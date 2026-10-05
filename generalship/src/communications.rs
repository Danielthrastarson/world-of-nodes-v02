//! Communications operates on the single authoritative Game.
use crate::{model::*, sim::Game};

impl Game {
    pub(crate) fn source_pos(&self, side: usize, s: &Source) -> Result<Point, String> {
        match s {
            Source::Hq => Ok(self.terrain.headquarters[side]),
            Source::General { id } => self
                .people
                .get(*id)
                .filter(|p| p.side == side && p.alive() && p.role == Role::General)
                .map(|p| p.pos)
                .ok_or("General unavailable".into()),
            Source::Outpost { id } => self.knowledge[side]
                .posts
                .get(id)
                .filter(|p| p.side == side && p.complete)
                .map(|p| p.pos)
                .ok_or("No known friendly outpost".into()),
        }
    }
    pub(crate) fn known_destination(&self, side: usize, ids: &[usize]) -> Result<Point, String> {
        let points: Vec<_> = ids
            .iter()
            .filter_map(|i| self.knowledge[side].people.get(i))
            .map(|p| p.pos)
            .collect();
        if points.len() != ids.len() {
            return Err("Recipient has no known position".into());
        }
        Ok(Point::new(
            points.iter().map(|p| p.x as i64).sum::<i64>() as i32 / points.len() as i32,
            points.iter().map(|p| p.y as i64).sum::<i64>() as i32 / points.len() as i32,
        ))
    }
    pub(crate) fn dispatch(
        &mut self,
        side: usize,
        source: &Source,
        pos: Point,
        destination: Point,
        recipients: Vec<usize>,
        payload: Payload,
    ) {
        let mut path = vec![];
        let mut due = self.tick + self.config.local_exchange_ticks;
        if let Source::Outpost { id } = source {
            let relay = self.relay(side);
            if let Some((delay, p)) = relay.get(id) {
                due += delay;
                path = p.clone();
            } else {
                due = u64::MAX;
                path = vec![*id];
            }
        }
        let id = self.missions.len();
        self.missions.push(Mission {
            id,
            side,
            source: pos,
            destination,
            payload,
            courier: None,
            due,
            wait_since: None,
            returning: false,
            finished: false,
            relay_path: path,
            recipients,
        });
    }
    pub(crate) fn acknowledge(&mut self, side: usize, id: u64, status: &str) {
        if let Some(o) = self.knowledge[side].orders.iter_mut().find(|o| o.id == id) {
            o.status = status.into();
        }
        self.notice(side, &format!("Order {id}: {status}"));
    }
    pub(crate) fn deliver_to_command(
        &mut self,
        side: usize,
        pos: Point,
        payload: &Payload,
    ) -> bool {
        let direct = pos.near(self.terrain.headquarters[side], self.config.exchange_radius)
            || self.people.iter().any(|p| {
                p.side == side
                    && p.role == Role::General
                    && p.alive()
                    && p.pos.near(pos, self.config.exchange_radius)
            });
        let (reports, ack) = match payload {
            Payload::Reports { reports } => (reports.clone(), None),
            Payload::Ack {
                id,
                status,
                reports,
            } => (reports.clone(), Some((*id, status.clone()))),
            _ => return false,
        };
        if direct {
            for r in reports {
                self.receive(side, r);
            }
            if let Some((id, status)) = ack {
                self.acknowledge(side, id, &status);
            }
            return true;
        }
        for (id, (delay, path)) in self.relay(side) {
            if self.posts[id].pos.near(pos, self.config.exchange_radius) {
                self.signals.push(Signal {
                    side,
                    path,
                    due: self.tick + delay,
                    reports,
                    ack,
                });
                return true;
            }
        }
        false
    }
    pub(crate) fn apply_payload(
        &mut self,
        side: usize,
        pos: Point,
        payload: &Payload,
    ) -> Option<(u64, String)> {
        match payload {
            Payload::Order {
                id,
                sequence,
                recipients,
                plan,
            } => {
                let present: Vec<_> = recipients
                    .iter()
                    .copied()
                    .filter(|&i| {
                        self.people[i].alive()
                            && self.people[i].pos.near(pos, self.config.exchange_radius)
                    })
                    .collect();
                if present.is_empty() {
                    return None;
                }
                let mut received = 0;
                for &i in &present {
                    let p = &mut self.people[i];
                    if p.sequence >= *sequence
                        || matches!(p.role, Role::General | Role::Trainee)
                        || p.task.is_some()
                    {
                        continue;
                    }
                    p.sequence = *sequence;
                    p.plan = plan.clone();
                    p.route_index = 0;
                    p.path_goal = None;
                    p.path.clear();
                    p.accepted_group = present.clone();
                    p.accepted_strength = present.len();
                    received += 1;
                }
                Some((
                    *id,
                    format!(
                        "Acknowledged locally by {received} people; execution is not confirmed complete"
                    ),
                ))
            }
            Payload::Assign {
                id,
                sequence,
                people,
                role,
                target,
                post,
                escort,
            } => {
                let present: Vec<_> = people
                    .iter()
                    .copied()
                    .filter(|&i| {
                        self.people[i].alive()
                            && self.people[i].pos.near(pos, self.config.exchange_radius)
                    })
                    .collect();
                if present.is_empty() {
                    return None;
                }
                let mut count = 0;
                for i in present {
                    let p = &mut self.people[i];
                    if p.sequence >= *sequence || !p.role.assignable() || p.task.is_some() {
                        continue;
                    }
                    p.sequence = *sequence;
                    p.role = *role;
                    p.duty_target = *target;
                    p.post = *post;
                    p.escort = *escort;
                    p.path_goal = None;
                    p.path.clear();
                    p.plan = Plan::hold(*target);
                    p.material_quota = 0;
                    p.report_returning = false;
                    if *role == Role::Scout {
                        p.plan.verb = Verb::Observe;
                        p.plan.manner = Manner::Avoid;
                    }
                    count += 1;
                }
                Some((*id, format!("Duty received by {count} people")))
            }
            _ => {
                let _ = side;
                None
            }
        }
    }
    pub(crate) fn communications(&mut self) {
        for idx in 0..self.missions.len() {
            if self.missions[idx].finished {
                continue;
            }
            let mut m = self.missions[idx].clone();
            if !m.relay_path.is_empty() {
                let endpoint = *m.relay_path.last().unwrap();
                let routes = self.relay(m.side);
                if !self.signal_valid(m.side, &m.relay_path) {
                    m.due = u64::MAX;
                }
                if m.due == u64::MAX {
                    if let Some((delay, path)) = routes.get(&endpoint) {
                        m.relay_path = path.clone();
                        m.due = self.tick + delay;
                    }
                    self.missions[idx] = m;
                    continue;
                }
                if self.tick < m.due {
                    continue;
                }
                m.relay_path.clear();
            }
            if self.tick < m.due {
                continue;
            }
            if let Some(id) = m.courier {
                if !self.people[id].alive() {
                    m.finished = true;
                    self.missions[idx] = m;
                    continue;
                }
                if !self.people[id]
                    .pos
                    .near(m.destination, self.config.courier_arrival_radius)
                {
                    self.missions[idx] = m;
                    continue;
                }
            } else if !m.source.near(m.destination, self.config.exchange_radius) {
                if let Some(p) = self.people.iter_mut().find(|p| {
                    p.side == m.side
                        && p.alive()
                        && p.role == Role::Courier
                        && p.task.is_none()
                        && p.pos.near(m.source, self.config.exchange_radius)
                }) {
                    p.task = Some(idx);
                    p.path_goal = None;
                    m.courier = Some(p.id);
                }
                self.missions[idx] = m;
                continue;
            }
            let at = m.courier.map(|id| self.people[id].pos).unwrap_or(m.source);
            if m.returning || matches!(m.payload, Payload::Reports { .. }) {
                if self.deliver_to_command(m.side, at, &m.payload) {
                    m.finished = true;
                } else if m.courier.is_some() {
                    m.destination = self.terrain.headquarters[m.side];
                }
            } else {
                let result = self.apply_payload(m.side, at, &m.payload);
                let timed_out = m
                    .wait_since
                    .is_some_and(|t| self.tick - t >= self.config.courier_wait);
                if let Some((id, status)) = result {
                    let report = self.observe(
                        m.side,
                        at,
                        self.config.sight,
                        format!("courier:{}", m.courier.unwrap_or(999)),
                        250 + (id / 2 % 200),
                    );
                    m.payload = Payload::Ack {
                        id,
                        status,
                        reports: vec![report],
                    };
                    m.returning = true;
                    m.wait_since = None;
                    if m.courier.is_some() {
                        m.destination = m.source;
                    } else if self.deliver_to_command(m.side, at, &m.payload) {
                        m.finished = true;
                    } else {
                        m.destination = self.terrain.headquarters[m.side];
                    }
                } else if timed_out {
                    let id = match m.payload {
                        Payload::Order { id, .. } | Payload::Assign { id, .. } => id,
                        _ => 0,
                    };
                    m.payload = Payload::Ack {
                        id,
                        status: "Returned undelivered: recipient not found at rendezvous".into(),
                        reports: vec![],
                    };
                    m.returning = true;
                    m.wait_since = None;
                    if m.courier.is_some() {
                        m.destination = m.source;
                    } else {
                        self.deliver_to_command(m.side, at, &m.payload);
                        m.finished = true;
                    }
                } else if m.wait_since.is_none() {
                    m.wait_since = Some(self.tick);
                }
            }
            if m.finished
                && let Some(id) = m.courier
            {
                self.people[id].task = None;
                self.people[id].path_goal = None;
            }
            self.missions[idx] = m;
        }
    }
}
