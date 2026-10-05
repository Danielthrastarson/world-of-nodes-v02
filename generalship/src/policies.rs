//! Policies operates on the single authoritative Game.
use crate::terrain::Terrain;
use crate::{model::*, sim::Game};
impl Game {
    pub(crate) fn local_enemies(&self, id: usize) -> Vec<usize> {
        let p = &self.people[id];
        self.people
            .iter()
            .filter(|q| {
                q.alive()
                    && q.side != p.side
                    && p.pos.near(q.pos, self.config.sight)
                    && self.terrain.sight(p.pos, q.pos)
            })
            .map(|q| q.id)
            .collect()
    }
    pub(crate) fn nearest_enemy(&self, id: usize) -> Option<usize> {
        let p = &self.people[id];
        self.local_enemies(id)
            .into_iter()
            .min_by_key(|&j| (p.pos.distance2(self.people[j].pos), j))
    }
    pub(crate) fn speed(&self, p: &Person) -> i32 {
        let speed = match p.role {
            Role::Line | Role::Guard | Role::Staff => self.config.speeds[0],
            Role::Ranged => self.config.speeds[1],
            Role::Light | Role::Scout => self.config.speeds[2],
            Role::General | Role::Escort => self.config.speeds[3],
            _ => self.config.speeds[4],
        };
        let rate = match self.terrain.kind(p.pos) {
            1 => self.config.road_speed_percent,
            2 => self.config.woods_speed_percent,
            5 => self.config.ford_speed_percent,
            _ => 100,
        };
        speed * rate / 100
            * (100
                - (p.missed.min(self.config.starvation_max_meals)
                    * self.config.starvation_step_percent) as i32)
            / 100
    }
    pub(crate) fn move_toward(&mut self, id: usize, target: Point) {
        if !self.terrain.walkable(target) {
            return;
        }
        let p = &self.people[id];
        if p.pos.near(target, self.config.arrival_radius) {
            return;
        }
        if p.path_goal
            .is_none_or(|old| Terrain::cell(old) != Terrain::cell(target))
            || p.path.is_empty()
        {
            let path = self.terrain.path(p.pos, target);
            self.people[id].path = path;
            self.people[id].path_goal = Some(target);
        }
        let p = &self.people[id];
        let Some(next) = p.path.first().copied() else {
            return;
        };
        let speed = self.speed(p).max(1);
        let dx = next.x - p.pos.x;
        let dy = next.y - p.pos.y;
        let dist = ((dx as i64 * dx as i64 + dy as i64 * dy as i64) as u64)
            .isqrt()
            .max(1) as i32;
        let candidate = if dist <= speed {
            next
        } else {
            Point::new(p.pos.x + dx * speed / dist, p.pos.y + dy * speed / dist)
        };
        // FIFO bridge admission, shared across both travel directions.
        if self.terrain.kind(candidate) == 4 && self.terrain.kind(p.pos) != 4 {
            self.bridge_queue.entry(id).or_insert(self.tick);
            let occupants = self
                .people
                .iter()
                .filter(|q| q.alive() && self.terrain.kind(q.pos) == 4)
                .count();
            let head = self
                .bridge_queue
                .iter()
                .min_by_key(|&(who, when)| (*when, *who))
                .map(|(&who, _)| who);
            if occupants >= self.config.bridge_capacity || head != Some(id) {
                return;
            }
            self.bridge_queue.remove(&id);
        }
        let p = &mut self.people[id];
        p.pos = candidate;
        if candidate == next {
            p.path.remove(0);
        }
    }
    pub(crate) fn work_and_move(&mut self) {
        // Queue entries cannot retain priority after an actor dies or changes route.
        self.bridge_queue.retain(|id, _| {
            self.people[*id].alive()
                && self.people[*id]
                    .path
                    .first()
                    .is_some_and(|p| self.terrain.kind(*p) == 4)
        });
        for id in 0..self.people.len() {
            if !self.people[id].alive() {
                continue;
            }
            let p = self.people[id].clone();
            match p.role {
                Role::Trainee => {
                    if p.pos.near(
                        self.terrain.headquarters[p.side],
                        self.config.exchange_radius,
                    ) {
                        self.people[id].training = self.people[id].training.saturating_sub(1);
                        if self.people[id].training == 0 {
                            self.people[id].role = Role::General;
                            self.people[id].plan = Plan::hold(p.pos);
                            self.notice(p.side, "General training completed at headquarters.");
                        }
                    }
                }
                Role::Courier => {
                    if let Some(task) = p.task {
                        if let Some(m) = self.missions.get(task)
                            && !m.finished
                        {
                            self.move_toward(id, m.destination);
                        }
                    } else {
                        self.move_toward(id, p.duty_target);
                    }
                }
                Role::Carrier => self.carrier(id),
                Role::Builder => self.builder(id),
                Role::Staff => {
                    if let Some(post) = p.post {
                        let dest = self.posts[post].pos;
                        self.move_toward(id, dest);
                    }
                }
                Role::Escort => {
                    if let Some(g) = p.escort {
                        let q = &self.people[g];
                        if q.alive()
                            && p.pos.near(q.pos, self.config.sight)
                            && self.terrain.sight(p.pos, q.pos)
                        {
                            let target = Point::new(
                                q.pos.x + self.config.escort_offset,
                                q.pos.y + self.config.escort_offset,
                            );
                            if !p.pos.near(target, self.config.escort_radius) {
                                self.move_toward(id, target);
                            }
                        }
                    }
                }
                Role::Scout => {
                    self.scout(id);
                }
                _ => self.execute_plan(id),
            }
        }
    }
    pub(crate) fn carrier(&mut self, id: usize) {
        let p = self.people[id].clone();
        let hq = self.terrain.headquarters[p.side];
        if p.pos.near(hq, self.config.logistics_radius) {
            let room = self.config.carrier_capacity - p.cargo.iter().sum::<u64>();
            let amount = room.min(self.stock[p.side][0]);
            self.stock[p.side][0] -= amount;
            self.people[id].cargo[0] += amount;
        }
        if self.people[id].cargo[0] <= self.config.reserve_food {
            self.move_toward(id, hq);
            return;
        }
        if !p.pos.near(p.duty_target, self.config.rendezvous_radius) {
            self.move_toward(id, p.duty_target);
            return;
        }
        // Only people physically encountered at the assigned rendezvous can receive food.
        for other in 0..self.people.len() {
            if self.people[id].cargo[0] <= self.config.reserve_food {
                break;
            }
            let q = &self.people[other];
            if other != id
                && q.alive()
                && q.side == p.side
                && q.role != Role::Carrier
                && q.pos.near(p.pos, self.config.exchange_radius)
                && q.cargo[0] < self.config.refill_target
                && q.cargo.iter().sum::<u64>() < self.config.carrier_capacity
            {
                self.people[other].cargo[0] += 1;
                self.people[id].cargo[0] -= 1;
            }
        }
        if self.people[id].cargo[0] <= self.config.reserve_food {
            self.move_toward(id, hq);
        }
    }
    pub(crate) fn builder(&mut self, id: usize) {
        let p = self.people[id].clone();
        let Some(post_id) = p.post else { return };
        let post = self.posts[post_id].clone();
        let hq = self.terrain.headquarters[p.side];
        if p.material_quota > 0 && p.cargo[1] == 0 {
            if !p.pos.near(hq, self.config.logistics_radius) {
                self.move_toward(id, hq);
                return;
            }
            let amount = p
                .material_quota
                .min(self.config.carrier_capacity - p.cargo.iter().sum::<u64>())
                .min(self.stock[p.side][1]);
            self.stock[p.side][1] -= amount;
            self.people[id].cargo[1] += amount;
            self.people[id].material_quota -= amount;
            if amount == 0 {
                return;
            }
        }
        if !p.pos.near(post.pos, self.config.logistics_radius) {
            self.move_toward(id, post.pos);
            return;
        }
        if post.hp <= 0 {
            self.people[id].role = Role::Carrier;
            self.people[id].post = None;
            self.people[id].duty_target = hq;
            return;
        }
        // No duplicate structures at a site, including an unseen enemy construction.
        if self
            .posts
            .iter()
            .any(|q| q.id != post_id && q.pos == post.pos && q.hp > 0 && q.complete)
        {
            self.people[id].role = Role::Guard;
            self.people[id].plan = Plan::hold(p.pos);
            return;
        }
        let material = self.people[id].cargo[1];
        self.people[id].cargo[1] = 0;
        self.posts[post_id].stock[1] += material;
        if self.people[id].material_quota > 0 {
            return;
        }
        let post = &mut self.posts[post_id];
        if !post.complete && post.stock[1] >= self.config.build_material {
            post.work += 1;
            if post.work >= self.config.build_work {
                post.stock[1] -= self.config.build_material;
                self.consumed[p.side][1] += self.config.build_material;
                post.complete = true;
            }
        } else if post.complete && post.upgrading && post.stock[1] >= self.config.upgrade_material {
            post.upgrading_work += 1;
            if post.upgrading_work >= self.config.upgrade_work {
                post.stock[1] -= self.config.upgrade_material;
                self.consumed[p.side][1] += self.config.upgrade_material;
                post.upgraded = true;
                post.upgrading = false;
            }
        }
        if self.posts[post_id].complete && !self.posts[post_id].upgrading {
            self.people[id].role = if self.staff(post_id) < self.config.required_staff {
                Role::Staff
            } else {
                Role::Guard
            };
            self.people[id].plan = Plan::hold(p.pos);
        }
    }
    pub(crate) fn scout(&mut self, id: usize) {
        let p = self.people[id].clone();
        let hq = self.terrain.headquarters[p.side];
        if p.report_returning {
            let report_payload = self
                .missions
                .iter()
                .find(|m| m.courier == Some(id) && !m.finished)
                .map(|m| (m.id, m.payload.clone(), m.destination, m.wait_since));
            if let Some((mid, payload, destination, wait_since)) = report_payload {
                if self.deliver_to_command(p.side, p.pos, &payload) {
                    self.missions[mid].finished = true;
                    self.people[id].report_returning = false;
                    self.people[id].last_report = self.tick;
                    self.people[id].path_goal = None;
                } else {
                    // Travel to a fixed, briefed rendezvous, never a hidden moving unit.
                    if p.pos.near(destination, self.config.exchange_radius) {
                        if wait_since.is_some_and(|t| self.tick - t >= self.config.courier_wait) {
                            self.missions[mid].destination = hq;
                        } else if wait_since.is_none() {
                            self.missions[mid].wait_since = Some(self.tick);
                        }
                    }
                    self.move_toward(id, self.missions[mid].destination);
                }
                return;
            }
            self.people[id].report_returning = false;
        }
        // Scouts carry their own observations home; they do not spawn couriers.
        if self.tick - p.last_report >= self.config.report_ticks
            && (!self.local_enemies(id).is_empty()
                || p.pos
                    .near(p.plan.target, self.config.observation_arrival_radius))
        {
            let report = self.observe(
                p.side,
                p.pos,
                self.config.sight,
                format!("scout:{id}"),
                300 + id as u64,
            );
            let payload = Payload::Reports {
                reports: vec![report],
            };
            if self.deliver_to_command(p.side, p.pos, &payload) {
                self.people[id].last_report = self.tick;
            } else {
                let mid = self.missions.len();
                self.missions.push(Mission {
                    id: mid,
                    side: p.side,
                    source: p.pos,
                    destination: [hq, self.terrain.sites[if p.side == 0 { 0 } else { 3 }]]
                        .into_iter()
                        .min_by_key(|q| p.pos.distance2(*q))
                        .unwrap(),
                    payload,
                    courier: Some(id),
                    due: u64::MAX,
                    wait_since: None,
                    returning: true,
                    finished: false,
                    relay_path: vec![],
                    recipients: vec![],
                });
                self.people[id].report_returning = true;
            }
        }
        self.execute_plan(id);
    }
    pub(crate) fn execute_plan(&mut self, id: usize) {
        let p = self.people[id].clone();
        if p.plan.verb == Verb::Observe
            && p.role.combatant()
            && self.tick - p.last_report >= self.config.report_ticks
        {
            let report = self.observe(
                p.side,
                p.pos,
                self.config.sight,
                format!("observer:{id}"),
                id as u64,
            );
            if report.people.iter().any(|q| q.side != p.side) {
                let payload = Payload::Reports {
                    reports: vec![report],
                };
                if self.deliver_to_command(p.side, p.pos, &payload) {
                    self.people[id].last_report = self.tick;
                } else if let Some(courier) = self
                    .people
                    .iter()
                    .find(|q| {
                        q.side == p.side
                            && q.alive()
                            && q.role == Role::Courier
                            && q.task.is_none()
                            && q.pos.near(p.pos, self.config.exchange_radius)
                    })
                    .map(|q| q.id)
                {
                    let mid = self.missions.len();
                    self.missions.push(Mission {
                        id: mid,
                        side: p.side,
                        source: p.pos,
                        destination: self.terrain.headquarters[p.side],
                        payload,
                        courier: Some(courier),
                        due: self.tick + self.config.local_exchange_ticks,
                        wait_since: None,
                        returning: true,
                        finished: false,
                        relay_path: vec![],
                        recipients: vec![],
                    });
                    self.people[courier].task = Some(mid);
                    self.people[courier].path_goal = None;
                    self.people[courier].path.clear();
                    self.people[id].last_report = self.tick;
                }
            }
        }
        let mut plan = p.plan.clone();
        if let Some(fallback) = plan.fallback {
            let local_alive = p
                .accepted_group
                .iter()
                .filter(|&&i| {
                    self.people[i].alive()
                        && self.people[i].pos.near(p.pos, self.config.sight)
                        && self.terrain.sight(p.pos, self.people[i].pos)
                })
                .count();
            // Local absent members do not count as confirmed casualties. Only locally seen dead members do.
            let local_dead = p
                .accepted_group
                .iter()
                .filter(|&&i| {
                    !self.people[i].alive()
                        && self.people[i].pos.near(p.pos, self.config.sight)
                        && self.terrain.sight(p.pos, self.people[i].pos)
                })
                .count();
            let _ = local_alive;
            if p.accepted_strength > 0
                && (p.accepted_strength - local_dead) * 100
                    < p.accepted_strength * self.config.fallback_percent as usize
            {
                plan.verb = Verb::Withdraw;
                plan.target = fallback;
                plan.fallback = None;
                self.people[id].plan = plan.clone();
            }
        }
        let enemy = self.nearest_enemy(id);
        if (plan.manner == Manner::Avoid || matches!(plan.verb, Verb::Observe | Verb::Withdraw))
            && let Some(e) = enemy
        {
            let q = &self.people[e];
            if p.pos.near(q.pos, self.config.avoidance_radius) && plan.verb != Verb::Withdraw {
                let target = Point::new(
                    (p.pos.x + (p.pos.x - q.pos.x).signum() * self.config.avoidance_step)
                        .clamp(500, 89500),
                    (p.pos.y + (p.pos.y - q.pos.y).signum() * self.config.avoidance_step)
                        .clamp(500, 59500),
                );
                self.move_toward(id, target);
                return;
            }
        }
        let mut target = if plan.verb == Verb::Move && p.route_index < plan.route.len() {
            plan.route[p.route_index]
        } else {
            plan.target
        };
        if plan.verb == Verb::Hold
            && p.pos
                .near(target, plan.area_radius.max(self.config.hold_radius))
        {
            return;
        }
        if plan.verb == Verb::Observe && plan.area_radius > 0 {
            let offsets = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            let (dx, dy) = offsets[p.route_index % 4];
            let candidate = Point::new(
                plan.target.x + dx * plan.area_radius,
                plan.target.y + dy * plan.area_radius,
            );
            if self.terrain.walkable(candidate) {
                target = candidate;
            }
        }
        if plan.verb == Verb::Engage
            && let Some(e) = enemy
        {
            let q = &self.people[e];
            if (plan.area_radius == 0 || q.pos.near(plan.target, plan.area_radius))
                && (plan.manner != Manner::NoPursuit
                    || q.pos.near(plan.target, self.config.pursuit_radius))
            {
                target = q.pos;
                let range = if p.role == Role::Ranged {
                    self.config.ranged_range * self.config.firing_standoff_percent / 100
                } else {
                    self.config.melee_range * self.config.firing_standoff_percent / 100
                };
                if p.pos.near(target, range) {
                    return;
                }
            }
        }
        if plan.manner == Manner::Together && p.accepted_group.len() > 1 {
            let nearby: Vec<_> = p
                .accepted_group
                .iter()
                .map(|&i| &self.people[i])
                .filter(|q| {
                    q.alive()
                        && q.pos.near(p.pos, self.config.sight)
                        && self.terrain.sight(p.pos, q.pos)
                })
                .collect();
            if !nearby.is_empty() {
                let center = Point::new(
                    nearby.iter().map(|q| q.pos.x as i64).sum::<i64>() as i32 / nearby.len() as i32,
                    nearby.iter().map(|q| q.pos.y as i64).sum::<i64>() as i32 / nearby.len() as i32,
                );
                if !p.pos.near(center, self.config.cohesion_radius)
                    && p.pos.distance2(target) < center.distance2(target)
                {
                    return;
                }
            }
        }
        if p.pos.near(target, self.config.waypoint_radius) {
            if (plan.verb == Verb::Move && p.route_index < plan.route.len())
                || (plan.verb == Verb::Observe && plan.area_radius > 0)
            {
                self.people[id].route_index += 1;
            } else if let Some(next) = plan.then {
                self.people[id].plan.verb = next;
                self.people[id].plan.then = None;
            }
            return;
        }
        self.move_toward(id, target);
    }
}
