//! Observation operates on the single authoritative Game.
use crate::{model::*, sim::Game};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
impl Game {
    pub(crate) fn staff(&self, id: usize) -> usize {
        let post = &self.posts[id];
        self.people
            .iter()
            .filter(|p| {
                p.alive()
                    && p.missed < self.config.staff_missed_meals
                    && p.side == post.side
                    && p.role == Role::Staff
                    && p.post == Some(id)
                    && p.pos.near(post.pos, self.config.logistics_radius)
            })
            .count()
    }
    pub fn relay(&self, side: usize) -> BTreeMap<usize, (Tick, Vec<usize>)> {
        let roots: Vec<_> = std::iter::once(self.terrain.headquarters[side])
            .chain(
                self.people
                    .iter()
                    .filter(|p| p.side == side && p.alive() && p.role == Role::General)
                    .map(|p| p.pos),
            )
            .collect();
        let eligible: Vec<_> = self
            .posts
            .iter()
            .filter(|p| {
                p.side == side
                    && p.hp > 0
                    && p.complete
                    && self.staff(p.id) >= self.config.required_staff
            })
            .map(|p| p.id)
            .collect();
        let mut found: BTreeMap<usize, (Tick, Vec<usize>)> = BTreeMap::new();
        for &id in &eligible {
            let p = &self.posts[id];
            let range = if p.upgraded {
                self.config.upgraded_range
            } else {
                self.config.relay_range
            };
            let delay = if p.upgraded {
                self.config.upgraded_ticks
            } else {
                self.config.relay_ticks
            };
            if roots
                .iter()
                .any(|&r| r.near(p.pos, range) && self.terrain.sight(r, p.pos))
            {
                found.insert(id, (delay, vec![id]));
            }
        }
        for _ in 0..eligible.len() {
            let old = found.clone();
            for &id in &eligible {
                let p = &self.posts[id];
                for (&other, (cost, path)) in &old {
                    if path.contains(&id) {
                        continue;
                    }
                    let q = &self.posts[other];
                    let upgraded = p.upgraded && q.upgraded;
                    let range = if upgraded {
                        self.config.upgraded_range
                    } else {
                        self.config.relay_range
                    };
                    let delay = if upgraded {
                        self.config.upgraded_ticks
                    } else {
                        self.config.relay_ticks
                    };
                    if p.pos.near(q.pos, range)
                        && self.terrain.sight(p.pos, q.pos)
                        && found.get(&id).is_none_or(|v| v.0 > cost + delay)
                    {
                        let mut pp = path.clone();
                        pp.push(id);
                        found.insert(id, (cost + delay, pp));
                    }
                }
            }
            if found == old {
                break;
            }
        }
        found
    }
    pub(crate) fn signal_valid(&self, side: usize, path: &[usize]) -> bool {
        path.last()
            .is_some_and(|id| self.relay(side).get(id).is_some_and(|v| v.1 == path))
    }
    pub(crate) fn observe(
        &self,
        side: usize,
        pos: Point,
        radius: i32,
        source: String,
        _key: u64,
    ) -> Report {
        let people = self
            .people
            .iter()
            .filter(|p| pos.near(p.pos, radius) && self.terrain.sight(pos, p.pos))
            .map(|p| Contact {
                absent_since: None,
                id: p.id,
                side: p.side,
                role: p.role,
                pos: p.pos,
                hp: p.hp,
                cargo: if p.side == side { Some(p.cargo) } else { None },
                missed_meals: if p.side == side { Some(p.missed) } else { None },
                received_order: if p.side == side {
                    Some(p.sequence)
                } else {
                    None
                },
                observed: self.tick,
                received: self.tick,
                source: source.clone(),
            })
            .collect();
        let posts = self
            .posts
            .iter()
            .filter(|p| {
                (p.complete || p.work > 0 || p.stock[1] > 0)
                    && pos.near(p.pos, radius)
                    && self.terrain.sight(pos, p.pos)
            })
            .map(|p| PostContact {
                id: p.id,
                side: p.side,
                pos: p.pos,
                hp: p.hp,
                complete: p.complete,
                upgraded: p.upgraded,
                staff: self.staff(p.id),
                stock: if p.side == side { Some(p.stock) } else { None },
                work: if p.side == side { Some(p.work) } else { None },
                upgrading_work: if p.side == side {
                    Some(p.upgrading_work)
                } else {
                    None
                },
                observed: self.tick,
                received: self.tick,
                source: source.clone(),
            })
            .collect();
        Report {
            id: format!("{side}:{}:{source}", self.tick),
            observed: self.tick,
            received: self.tick,
            source,
            pos,
            radius,
            people,
            posts,
        }
    }
    pub fn receive(&mut self, side: usize, mut report: Report) {
        report.received = self.tick;
        let k = &mut self.knowledge[side];
        let seen: BTreeSet<_> = report.people.iter().map(|p| p.id).collect();
        for old in k.people.values_mut() {
            if old.observed < report.observed
                && !seen.contains(&old.id)
                && old.pos.near(report.pos, report.radius)
                && self.terrain.sight(report.pos, old.pos)
            {
                old.absent_since = Some(report.observed);
            }
        }
        for p in &mut report.people {
            p.received = self.tick;
            if k.people
                .get(&p.id)
                .is_none_or(|old| old.observed <= p.observed)
            {
                k.people.insert(p.id, p.clone());
            }
        }
        for p in &mut report.posts {
            p.received = self.tick;
            if k.posts
                .get(&p.id)
                .is_none_or(|old| old.observed <= p.observed)
            {
                k.posts.insert(p.id, p.clone());
            }
        }
        if k.reports
            .iter()
            .any(|r| r.source == report.source && r.observed > report.observed)
        {
            return;
        }
        k.reports.retain(|r| r.source != report.source);
        k.reports.push(report);
        if k.reports.len() > 40 {
            k.reports.remove(0);
        }
    }
    pub(crate) fn sense(&mut self) {
        for side in 0..2 {
            let mut direct = vec![self.observe(
                side,
                self.terrain.headquarters[side],
                self.config.general_sight,
                "headquarters".into(),
                0,
            )];
            for p in self
                .people
                .iter()
                .filter(|p| p.side == side && p.role == Role::General && p.alive())
            {
                direct.push(self.observe(
                    side,
                    p.pos,
                    self.config.general_sight,
                    format!("general:{}", p.id),
                    p.id as u64 + 1,
                ));
            }
            for r in direct {
                self.receive(side, r);
            }
            if self
                .tick
                .is_multiple_of(self.config.relay_report_ticks.max(1))
            {
                for (id, (delay, path)) in self.relay(side) {
                    let post = &self.posts[id];
                    let report = self.observe(
                        side,
                        post.pos,
                        self.config.outpost_sight,
                        format!("outpost:{id}"),
                        150 + id as u64,
                    );
                    self.signals.push(Signal {
                        side,
                        path,
                        due: self.tick + delay,
                        reports: vec![report],
                        ack: None,
                    });
                }
            }
        }
    }
    pub(crate) fn deliver_signals(&mut self) {
        let signals = std::mem::take(&mut self.signals);
        for s in signals {
            if !self.signal_valid(s.side, &s.path) {
                continue;
            }
            if s.due <= self.tick {
                for r in s.reports {
                    self.receive(s.side, r);
                }
                if let Some((id, status)) = s.ack {
                    self.acknowledge(s.side, id, &status);
                }
            } else {
                self.signals.push(s);
            }
        }
    }
    pub fn view(&self, side: usize) -> Value {
        let k = &self.knowledge[side];
        // No authoritative troop, mission, relay or opponent counters cross this boundary.
        json!({"tick":self.tick,"side":side,"status":self.status,"ready":self.ready,"connected":self.connected,
            "terrain":self.terrain,"config":self.config,"stock":self.stock[side],"people":k.people.values().collect::<Vec<_>>(),
            "posts":k.posts.values().collect::<Vec<_>>(),"reports":k.reports.iter().map(|r|json!({"id":r.id,"observed":r.observed,"received":r.received,"source":r.source,"pos":r.pos,"radius":r.radius,"people":r.people.iter().map(|p|json!({"id":p.id,"side":p.side,"hp":p.hp})).collect::<Vec<_>>()})).collect::<Vec<_>>(),"log":k.log,"orders":k.orders.iter().rev().take(40).collect::<Vec<_>>()})
    }
}

impl Game {}
