//! Combat operates on the single authoritative Game.
use crate::{model::*, sim::Game};

impl Game {
    pub(crate) fn combat(&mut self) {
        let mut hits = vec![0i32; self.people.len()];
        let mut post_hits = vec![0i32; self.posts.len()];
        for id in 0..self.people.len() {
            let p = &self.people[id];
            if !p.alive()
                || p.next_attack > self.tick
                || (!p.role.combatant() && p.role != Role::General)
            {
                continue;
            }
            let retreat = matches!(p.plan.verb, Verb::Withdraw | Verb::Observe)
                || p.plan.manner == Manner::Avoid;
            let ranged = p.role == Role::Ranged && !retreat;
            let range = if ranged {
                self.config.ranged_range
            } else {
                self.config.melee_range
            };
            let target = self
                .local_enemies(id)
                .into_iter()
                .filter(|&j| p.pos.near(self.people[j].pos, range))
                .min_by_key(|&j| (p.pos.distance2(self.people[j].pos), j));
            let raw = if ranged {
                self.config.ranged_damage
            } else {
                match p.role {
                    Role::Line => self.config.melee_damage[0],
                    Role::Light => self.config.melee_damage[2],
                    _ => self.config.melee_damage[1],
                }
            };
            let damage = (raw
                * (100
                    - (p.missed.min(self.config.starvation_max_meals)
                        * self.config.starvation_step_percent) as i32)
                / 100)
                .max(1);
            if let Some(j) = target {
                let q = &self.people[j];
                let armor = if q.role == Role::Line {
                    self.config.line_armor
                } else {
                    0
                };
                let cover = if ranged && self.terrain.kind(q.pos) == 2 {
                    self.config.woods_cover_divisor
                } else {
                    1
                };
                hits[j] += ((damage - armor) / cover).max(1);
                self.people[id].next_attack = self.tick
                    + if ranged {
                        self.config.ranged_ticks
                    } else {
                        self.config.melee_ticks
                    };
            } else if p.plan.verb == Verb::Engage
                && let Some(post) = self
                    .posts
                    .iter()
                    .filter(|q| {
                        q.side != p.side
                            && q.hp > 0
                            && (q.complete || q.work > 0 || q.stock[1] > 0)
                            && p.pos.near(q.pos, range)
                            && self.terrain.sight(p.pos, q.pos)
                    })
                    .min_by_key(|q| (p.pos.distance2(q.pos), q.id))
            {
                post_hits[post.id] += damage;
                self.people[id].next_attack = self.tick
                    + if ranged {
                        self.config.ranged_ticks
                    } else {
                        self.config.melee_ticks
                    };
            }
        }
        let mut lost_generals = vec![];
        for (id, damage) in hits.into_iter().enumerate() {
            let was_alive = self.people[id].alive();
            self.people[id].hp = (self.people[id].hp - damage).max(0);
            if was_alive && !self.people[id].alive() && self.people[id].role == Role::General {
                lost_generals.push(id);
            }
        }
        for id in lost_generals {
            let p = &self.people[id];
            let report = self.observe(p.side, p.pos, 0, format!("general:{id}"), id as u64 + 1);
            self.receive(p.side, report);
        }
        for (id, damage) in post_hits.into_iter().enumerate() {
            self.posts[id].hp = (self.posts[id].hp - damage).max(0);
        }
    }
    pub(crate) fn victory(&mut self) {
        for attacker in 0..2 {
            let pos = self.terrain.headquarters[1 - attacker];
            let attackers = self
                .people
                .iter()
                .filter(|p| {
                    p.side == attacker
                        && p.alive()
                        && p.role.combatant()
                        && p.pos.near(pos, self.config.capture_radius)
                })
                .count();
            let defenders = self.people.iter().any(|p| {
                p.side != attacker
                    && p.alive()
                    && p.role.combatant()
                    && p.pos.near(pos, self.config.capture_radius)
            });
            if attackers >= self.config.capture_people && !defenders {
                self.capture[attacker] += 1;
            } else {
                self.capture[attacker] = 0;
            }
        }
        let captures = [
            self.capture[0] >= self.config.capture_ticks,
            self.capture[1] >= self.config.capture_ticks,
        ];
        if captures == [true; 2] {
            self.status = Status::Finished {
                winner: None,
                reason: "simultaneous capture".into(),
            };
        } else if captures.iter().any(|&v| v) {
            self.status = Status::Finished {
                winner: Some(usize::from(captures[1])),
                reason: "headquarters captured".into(),
            };
        } else if self.connected == [false; 2]
            && self
                .disconnected_at
                .iter()
                .flatten()
                .any(|t| self.tick - t >= self.config.disconnect_ticks)
        {
            self.status = Status::Finished {
                winner: None,
                reason: "both players disconnected".into(),
            };
        } else if let Some(side) = (0..2).find(|&s| {
            self.disconnected_at[s].is_some_and(|t| self.tick - t >= self.config.disconnect_ticks)
        }) {
            self.status = Status::Finished {
                winner: Some(1 - side),
                reason: "disconnect timeout".into(),
            };
        } else if self.tick >= self.config.max_ticks {
            self.status = Status::Finished {
                winner: None,
                reason: "twenty-minute time limit".into(),
            };
        }
    }
}
