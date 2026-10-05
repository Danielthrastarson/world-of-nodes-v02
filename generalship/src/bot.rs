//! A practice opponent fed only the same serialized view as a human seat.
//! All actions go through Game::input in the server and enter the replay journal.
use crate::{model::*, terrain::Terrain};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Default)]
pub struct Bot {
    orders: BTreeMap<usize, (Tick, Plan)>,
    supplied: bool,
    last_training: Option<Tick>,
}

#[derive(Deserialize)]
struct View {
    tick: Tick,
    side: usize,
    status: Status,
    people: Vec<Contact>,
    terrain: Terrain,
    config: Config,
    stock: Stock,
}

fn centre(people: &[&Contact]) -> Point {
    Point::new(
        people.iter().map(|p| p.pos.x).sum::<i32>() / people.len() as i32,
        people.iter().map(|p| p.pos.y).sum::<i32>() / people.len() as i32,
    )
}

impl Bot {
    pub fn decide(&mut self, snapshot: Value) -> Vec<Input> {
        let Ok(v) = serde_json::from_value::<View>(snapshot) else {
            return vec![];
        };
        if v.status != Status::Running || !v.tick.is_multiple_of(30) {
            return vec![];
        }
        let mut actions = vec![];
        let home = v.terrain.headquarters[v.side];
        let enemy_hq = v.terrain.headquarters[1 - v.side];
        let mirror = |x, y| Point::new(if v.side == 1 { x } else { 90000 - x }, y);
        let own: Vec<_> = v
            .people
            .iter()
            .filter(|p| p.side == v.side && p.hp > 0)
            .collect();
        let generals: Vec<_> = own
            .iter()
            .copied()
            .filter(|p| p.role == Role::General)
            .collect();
        let enemies: Vec<_> = v
            .people
            .iter()
            .filter(|p| {
                p.side != v.side
                    && p.hp > 0
                    && p.absent_since.is_none()
                    && v.tick.saturating_sub(p.observed) <= 150
            })
            .collect();
        for (group, role) in [Role::Line, Role::Ranged, Role::Light]
            .into_iter()
            .enumerate()
        {
            let people: Vec<_> = own.iter().copied().filter(|p| p.role == role).collect();
            if people.is_empty() {
                continue;
            }
            let pos = centre(&people);
            // Generals physically accompany the two columns. They cannot see an unseen flank.
            if group != 1
                && let Some(general) = generals.get(usize::from(group == 2))
            {
                let behind = if v.side == 1 { 1400 } else { -1400 };
                let threatened = enemies.iter().any(|p| p.pos.near(general.pos, 6500));
                let offset = if threatened { behind * 3 } else { behind };
                let target = Point::new((pos.x + offset).clamp(1000, 89000), pos.y);
                if !general.pos.near(target, 1000) && v.terrain.walkable(target) {
                    actions.push(Input::MoveGeneral {
                        id: general.id,
                        target,
                    });
                }
            }
            let depleted = people.len() < 5
                || people.iter().map(|p| p.hp).sum::<i32>() < 40 * people.len() as i32;
            let contact = enemies
                .iter()
                .filter(|p| p.pos.near(pos, 16000))
                .min_by_key(|p| (p.pos.distance2(pos), p.id));
            let mut plan = Plan::hold(home);
            if depleted {
                plan.verb = Verb::Withdraw;
                plan.manner = Manner::Avoid;
            } else if let Some(enemy) = contact {
                plan.verb = Verb::Engage;
                plan.target = enemy.pos;
            } else if group == 2 && v.tick < 2700 {
                plan.verb = Verb::Move;
                plan.target = mirror(31000, 48000);
                // Use the longer ford deliberately; no hidden enemy coordinates enter the route.
                plan.route = vec![mirror(55000, 48500), mirror(40000, 48500)];
                plan.then = Some(Verb::Observe);
            } else {
                plan.verb = Verb::Engage;
                plan.target = if v.tick >= 2700 {
                    enemy_hq
                } else {
                    mirror(if group == 1 { 37000 } else { 33000 }, 27500)
                };
            }
            // Do not constantly correct the same march or restart a partially completed route.
            let due = self.orders.get(&group).is_none_or(|(tick, last)| {
                let changed = last.verb != plan.verb || !last.target.near(plan.target, 6000);
                (changed && v.tick - tick >= 150)
                    || (v.tick - tick >= 1200 && last.verb != Verb::Move)
            });
            if due {
                let (source, _) = generals
                    .iter()
                    .map(|p| (Source::General { id: p.id }, p.pos.distance2(pos)))
                    .chain(std::iter::once((Source::Hq, home.distance2(pos))))
                    .min_by_key(|(_, distance)| *distance)
                    .unwrap();
                actions.push(Input::Order {
                    rendezvous: None,
                    source,
                    recipients: people.iter().map(|p| p.id).collect(),
                    plan: plan.clone(),
                });
                self.orders.insert(group, (v.tick, plan));
            }
        }
        // Finite supplies travel on ordinary carrier routes; one carrier feeds the starting post.
        if !self.supplied && v.tick >= 300 {
            let carriers: Vec<_> = own
                .iter()
                .copied()
                .filter(|p| p.role == Role::Carrier)
                .collect();
            for (i, carrier) in carriers.iter().enumerate() {
                let target = match i {
                    0 => mirror(64000, 20000),
                    1 => mirror(56000, 48500),
                    _ => mirror(50000, 27500),
                };
                actions.push(Input::Assign {
                    source: Source::Hq,
                    people: vec![carrier.id],
                    role: Role::Carrier,
                    target,
                    post: None,
                    escort: None,
                });
            }
            self.supplied = true;
        }
        let commanders = own
            .iter()
            .filter(|p| matches!(p.role, Role::General | Role::Trainee))
            .count();
        if commanders < 2
            && v.stock[0] >= v.config.training_food
            && self
                .last_training
                .is_none_or(|tick| v.tick - tick >= v.config.training_ticks)
            && let Some(person) = own
                .iter()
                .find(|p| p.role == Role::Guard && p.pos.near(home, v.config.exchange_radius))
        {
            actions.push(Input::Train { person: person.id });
            self.last_training = Some(v.tick);
        }
        actions
    }
}
