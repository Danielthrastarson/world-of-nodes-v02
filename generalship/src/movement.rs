//! Shared physical movement: policies request destinations; this layer owns positions.
use crate::{model::*, sim::Game, terrain::Terrain};

impl Game {
    fn body_fits(&self, p: Point) -> bool {
        let r = self.config.person_radius;
        if !self.terrain.walkable(p) {
            return false;
        }
        for y in (p.y - r).div_euclid(1000)..=(p.y + r).div_euclid(1000) {
            for x in (p.x - r).div_euclid(1000)..=(p.x + r).div_euclid(1000) {
                if !self
                    .terrain
                    .walkable(Point::new(x * 1000 + 500, y * 1000 + 500))
                {
                    let nearest = Point::new(
                        p.x.clamp(x * 1000, (x + 1) * 1000),
                        p.y.clamp(y * 1000, (y + 1) * 1000),
                    );
                    if p.distance2(nearest) < i64::from(r).pow(2) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn corridor_clear(&self, from: Point, to: Point, obstacles: &[Point]) -> bool {
        if obstacles
            .iter()
            .any(|&p| !segment_clear(from, to, p, self.config.person_radius * 2))
        {
            return false;
        }
        let steps = ((to.x - from.x).abs().max((to.y - from.y).abs()) / 200).max(1);
        (1..=steps).all(|i| {
            self.body_fits(Point::new(
                from.x + (to.x - from.x) * i / steps,
                from.y + (to.y - from.y) * i / steps,
            ))
        })
    }

    pub(crate) fn place_deployment(&mut self) {
        let spacing = self.config.person_radius * 2;
        if spacing == 0 {
            return;
        }
        for id in 0..self.people.len() {
            let origin = self.people[id].pos;
            let free = |p: Point, game: &Self| {
                game.body_fits(p)
                    && game.people[..id]
                        .iter()
                        .all(|q| p.distance2(q.pos) >= i64::from(spacing).pow(2))
            };
            if free(origin, self) {
                continue;
            }
            'search: for ring in 1i32..=20 {
                for y in -ring..=ring {
                    for x in -ring..=ring {
                        if x.abs().max(y.abs()) != ring {
                            continue;
                        }
                        let p = Point::new(origin.x + x * spacing, origin.y + y * spacing);
                        if free(p, self) {
                            self.people[id].pos = p;
                            if self.people[id].plan.target == origin {
                                self.people[id].plan.target = p;
                            }
                            break 'search;
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn arrived(&self, id: usize, target: Point, tolerance: i32) -> bool {
        let p = &self.people[id];
        if p.pos.near(target, tolerance) {
            return true;
        }
        let spacing = self.config.person_radius * 2;
        // A shared rendezvous has room around it, not an infinitely stackable centre.
        spacing > 0
            && p.pos.near(target, spacing + tolerance)
            && self.people.iter().any(|q| {
                q.id != id
                    && q.alive()
                    && q.pos.near(target, spacing - 1)
                    && self.terrain.sight(p.pos, q.pos)
            })
    }

    pub(crate) fn resolve_movement(&mut self) {
        let count = self.people.len();
        let original: Vec<_> = self.people.iter().map(|p| p.pos).collect();
        self.bridge_queue.retain(|id, _| {
            self.people[*id].alive()
                && self.movement_intents[*id].is_some()
                && self.terrain.kind(self.people[*id].pos) != 4
        });
        // Rotation removes permanent low-ID priority. All ties remain replayable.
        for offset in 0..count {
            let id = (offset + self.tick as usize % count) % count;
            let Some(target) = self.movement_intents[id] else {
                continue;
            };
            if !self.people[id].alive() || self.arrived(id, target, self.config.arrival_radius) {
                continue;
            }
            let p = &self.people[id];
            let obstacles: Vec<_> = self
                .people
                .iter()
                .filter(|q| {
                    q.id != id
                        && q.alive()
                        && q.pos.near(p.pos, self.config.sight)
                        && self.terrain.sight(p.pos, q.pos)
                })
                .map(|q| q.pos)
                .collect();
            if p.path_goal
                .is_none_or(|old| Terrain::cell(old) != Terrain::cell(target))
                || p.path.is_empty()
                || (self.tick + id as u64).is_multiple_of(10)
            {
                self.people[id].path = self.terrain.path_avoiding(
                    p.pos,
                    target,
                    &obstacles,
                    self.config.person_radius * 2,
                );
            } else if let Some(last) = self.people[id].path.last_mut() {
                *last = target;
            }
            self.people[id].path_goal = Some(target);
            // Grid cells define a walkable corridor, not compulsory single-file
            // centre points. String-pull a short clear section before steering.
            let from = self.people[id].pos;
            let ahead = self.people[id]
                .path
                .iter()
                .take(8)
                .enumerate()
                .rev()
                .find(|(_, point)| self.corridor_clear(from, **point, &obstacles))
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.people[id].path.drain(..ahead);
            let p = &self.people[id];
            let Some(next) = p.path.first().copied() else {
                continue;
            };
            let from = p.pos;
            let speed = self.speed(p).max(1);
            let (dx, dy) = (next.x - from.x, next.y - from.y);
            let length = (from.distance2(next) as u64).isqrt().max(1) as i32;
            let step = speed.min(length);
            let spacing = self.config.person_radius * 2;
            let bridge = self.terrain.kind(from) == 4;
            let checks = ((next.x - from.x).abs().max((next.y - from.y).abs()) / 200).max(1);
            let crosses_bridge = (1..=checks).any(|i| {
                self.terrain.kind(Point::new(
                    from.x + (next.x - from.x) * i / checks,
                    from.y + (next.y - from.y) * i / checks,
                )) == 4
            });
            if !bridge && crosses_bridge {
                self.bridge_queue.entry(id).or_insert(self.tick);
            } else {
                self.bridge_queue.remove(&id);
            }
            let bridge_room = self
                .people
                .iter()
                .filter(|q| q.alive() && self.terrain.kind(q.pos) == 4)
                .count()
                < self.config.bridge_capacity;
            let head = self
                .bridge_queue
                .iter()
                .min_by_key(|&(who, when)| (*when, *who))
                .map(|(&who, _)| who);
            let mut candidates = vec![];
            // Prefer forward travel and consistent right-hand passing. No pushing,
            // teleporting, displacement of standing people, or extra speed.
            for (cos, sin) in [
                (1000, 0),
                (924, 383),
                (924, -383),
                (707, 707),
                (707, -707),
                (383, 924),
                (383, -924),
                (0, 1000),
                (0, -1000),
                (-707, 707),
                (-707, -707),
            ] {
                let x = (i64::from(dx) * cos - i64::from(dy) * sin) * i64::from(step)
                    / (i64::from(length) * 1000);
                let y = (i64::from(dx) * sin + i64::from(dy) * cos) * i64::from(step)
                    / (i64::from(length) * 1000);
                let to = Point::new(from.x + x as i32, from.y + y as i32);
                if to == from || !self.body_fits(to) {
                    continue;
                }
                if !bridge && self.terrain.kind(to) == 4 && (!bridge_room || head != Some(id)) {
                    continue;
                }
                let clear = self
                    .people
                    .iter()
                    .filter(|q| q.id != id && q.alive())
                    .all(|q| {
                        if spacing == 0 {
                            return true;
                        }
                        let before = from.distance2(q.pos);
                        // Invalid externally edited fixtures may separate gradually;
                        // normal play always starts from a non-overlapping deployment.
                        if before < i64::from(spacing).pow(2) {
                            return to.distance2(q.pos) > before;
                        }
                        segment_clear(from, to, q.pos, spacing)
                            && (original[q.id] == q.pos
                                || (segment_clear(from, to, original[q.id], spacing)
                                    && segment_clear(original[q.id], q.pos, to, spacing)))
                    });
                if clear {
                    candidates.push(to);
                }
            }
            if let Some(to) = candidates.into_iter().min_by_key(|p| p.distance2(next)) {
                self.people[id].pos = to;
                if self.terrain.kind(to) == 4 {
                    self.bridge_queue.remove(&id);
                }
                if to.near(next, self.config.waypoint_radius) {
                    self.people[id].path.remove(0);
                }
            }
        }
    }
}
