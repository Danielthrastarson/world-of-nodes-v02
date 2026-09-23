//! F8 observational accounting. No policy can read this instrument; it never
//! submits commands, updates beliefs, advances time, or enters physical hashes.
use super::*;
pub const CORRIDOR: usize = 2;
pub const TRANSIT: usize = 3;
pub const ARMS: &[&str] = &[
    "complementary",
    "symmetric",
    "high_cost",
    "low_cost",
    "no_carts",
    "paths",
    "paths_no_carts",
];

pub fn configuration(arm: &str, seed: u64, trace: bool) -> Result<Config, String> {
    if !ARMS.contains(&arm) {
        return Err("unregistered F8 arm".into());
    }
    let symmetric = arm == "symmetric";
    let time = match arm {
        "high_cost" => 4 * HOUR,
        "low_cost" => HOUR / 2,
        _ => 2 * HOUR,
    };
    let mut units = vec![0; if symmetric { 25 } else { 40 }];
    units.extend(vec![2; 50 - units.len()]);
    Ok(Config {
        seed,
        trace,
        starts: (0..50).map(|id| if id % 2 == 0 { 0 } else { 2 }).collect(),
        boards: vec![0, 2],
        initial_request: 0,
        food: vec![
            (0, if symmetric { 1500 } else { 2400 }, 0, 48 * HOUR),
            (2, if symmetric { 1500 } else { 600 }, 0, 48 * HOUR),
        ],
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: time,
                capacity: 4,
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: time,
                capacity: 4,
            },
        ],
        farming: Some(farming::Config {
            units,
            rules: farming::Rules::default(),
        }),
        capital: Some(capital::Config {
            rules: capital::Rules {
                enabled: !matches!(arm, "no_carts" | "paths_no_carts"),
                ..capital::Rules::default()
            },
            wood: if symmetric {
                vec![(0, 250), (2, 250)]
            } else {
                vec![(2, 500)]
            },
        }),
        paths: Some(paths::Config::ground(
            vec![0, 1],
            matches!(arm, "paths" | "paths_no_carts"),
        )),
        ..Config::default()
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flow {
    pub at: Time,
    pub agent: usize,
    pub from: usize,
    pub to: usize,
    pub origin: usize,
    pub material: Material,
    pub expires_at: Time,
    pub quantity: u64,
    pub owner: Owner,
    pub communal_job: bool,
    pub cart: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub at: Time,
    pub agent: usize,
    pub place: usize,
    pub kind: String,
    pub origin: usize,
    pub material: Material,
    pub quantity: u64,
    pub before: Owner,
    pub after: Owner,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Journal {
    pub node_places: Vec<usize>,
    pub cohorts: Vec<usize>,
    last_settlement: Vec<usize>,
    /// (starting cohort, current physical place, mutually exclusive activity).
    pub labor_ms: BTreeMap<(usize, usize, String), u128>,
    pub stats: BTreeMap<(usize, String), u128>,
    pub events: BTreeMap<(usize, String), u64>,
    pub flows: Vec<Flow>,
    pub transfers: Vec<Transfer>,
    pub visits: Vec<(Time, usize, usize, usize)>,
}
fn quantity(text: &str, key: &str) -> u64 {
    let marker = format!("{key}=");
    let s = text.split(&marker).nth(1).expect("existing event contract");
    s.chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .expect("integer event field")
}
pub(crate) fn observed_event(kind: &str) -> bool {
    matches!(
        kind,
        "Arrived"
            | "Gathered"
            | "Harvested"
            | "Consumed"
            | "Spoiled"
            | "Pickup"
            | "CargoLoaded"
            | "Delivered"
            | "JobAccepted"
            | "JobCancelled"
            | "Queued"
            | "Interrupted"
            | "SleepStarted"
            | "Died"
            | "Rejected"
            | "CartStarted"
            | "CartCompleted"
            | "CapitalForecast"
    )
}
impl Journal {
    fn place(&self, location: &Location) -> usize {
        match location {
            Location::Node(n) => self.node_places[*n],
            Location::Edge { .. } => TRANSIT,
        }
    }
    fn add(&mut self, place: usize, key: &str, value: u128) {
        *self.stats.entry((place, key.into())).or_default() += value;
    }
    pub(crate) fn integrate(&mut self, w: &World, dt: Time) {
        for a in &w.agents {
            if a.action == Action::Dead {
                continue;
            }
            let place = self.place(&a.location);
            let activity = match a.action {
                Action::Strike { .. } => "combat",
                Action::RaidStock { .. } => "raid",
                Action::Farm { plot } => {
                    if w.farm.plots[plot].crop.as_ref().unwrap().stage == farming::Stage::Sowing {
                        "sow"
                    } else {
                        "harvest"
                    }
                }
                Action::BuildCart { .. } => "cart_construction",
                Action::BuildCrossing { .. } => "crossing_construction",
                Action::Gather { batch, .. } => {
                    if w.goods.batches[batch].material == Material::Wood {
                        "gather_wood"
                    } else {
                        "gather_food"
                    }
                }
                Action::Travel => {
                    if w.carrier_load(a.id) > 0 {
                        "loaded_travel"
                    } else {
                        "empty_travel"
                    }
                }
                Action::Queued { .. } => "queue",
                Action::Sleep { .. } => "sleep",
                Action::MilitaryDuty { .. } => "military_duty",
                Action::Reproduce { .. } => "reproduction",
                Action::Idle => "idle",
                Action::Dead => unreachable!(),
            };
            *self
                .labor_ms
                .entry((self.cohorts[a.id], place, activity.into()))
                .or_default() += dt as u128;
            self.add(place, "living_ms", dt as u128);
            self.add(place, "starvation_ms", (dt - a.nutrition).max(0) as u128);
        }
        for p in &w.farm.plots {
            let place = self.node_places[p.node];
            if let Some(c) = &p.crop {
                let Owner::Agent(owner) = c.owner else {
                    unreachable!()
                };
                if w.agents[owner].action == Action::Dead {
                    self.add(place, "dead_owner_plot_ms", dt as u128);
                }
                if matches!(c.stage, farming::Stage::Mature | farming::Stage::Harvesting)
                    && w.agents[owner].action != (Action::Farm { plot: p.id })
                {
                    self.add(place, "mature_unworked_plot_ms", dt as u128);
                }
            } else {
                self.add(place, "unused_plot_ms", dt as u128);
            }
        }
        for board in &w.boards {
            self.add(
                self.node_places[board.node],
                "request_quantum_ms",
                board.quantity as u128 * dt as u128,
            );
        }
    }
    pub(crate) fn event(&mut self, e: &Event, w: &World) {
        let place = e
            .agent
            .map_or(CORRIDOR, |id| self.place(&w.agents[id].location));
        let key = match e.kind.as_str() {
            "Interrupted" => format!("Interrupted:{}", e.detail.split(';').next().unwrap()),
            "JobCancelled" => format!(
                "JobCancelled:{}",
                e.detail.split("reason=").nth(1).unwrap_or(&e.detail)
            ),
            "CapitalForecast" => format!(
                "CapitalGate:{}",
                e.detail.split_whitespace().next().unwrap()
            ),
            _ => e.kind.clone(),
        };
        *self.events.entry((place, key)).or_default() += 1;
        match e.kind.as_str() {
            "Arrived" => {
                let id = e.agent.unwrap();
                // A graph has no physical length unit: report actual quantity-edge
                // passages, not invented kilometres. Same-node barter adds none.
                for b in w
                    .goods
                    .batches
                    .iter()
                    .filter(|b| b.quantity > 0 && w.carried_by(id, b.holder))
                {
                    let material = if b.material == Material::Food && b.expires_at() <= w.now {
                        Material::Waste
                    } else {
                        b.material
                    };
                    self.add(
                        place,
                        &format!("{material:?}_quantity_edges"),
                        b.quantity as u128,
                    );
                    if w.barter.exchanges.iter().any(|x| {
                        x.at < w.now && (x.give.origin == b.origin || x.payment.origin == b.origin)
                    }) {
                        // Fungible descendants can merge with unexchanged siblings.
                        // This is explicitly an ancestry upper bound, not exact trade distance.
                        self.add(
                            place,
                            &format!("barter_ancestry_{material:?}_quantity_edges_upper"),
                            b.quantity as u128,
                        );
                    }
                }
                if place > 1 {
                    return;
                }
                let from = self.last_settlement[id];
                if from != place && from <= 1 {
                    self.visits.push((w.now, id, from, place));
                    let cart = w.capital.carts.iter().any(|c| c.user == Some(id));
                    for b in w
                        .goods
                        .batches
                        .iter()
                        .filter(|b| b.quantity > 0 && w.carried_by(id, b.holder))
                    {
                        self.flows.push(Flow {
                            at: w.now,
                            agent: id,
                            from,
                            to: place,
                            origin: b.origin,
                            // Arrival is logged before simultaneous spoilage in the
                            // boundary sequence. Report timestamp-level edibility,
                            // not that transient pre-spoil event ordering.
                            material: if b.material == Material::Food && b.expires_at() <= w.now {
                                Material::Waste
                            } else {
                                b.material
                            },
                            expires_at: b.expires_at(),
                            quantity: b.quantity,
                            owner: b.owner,
                            communal_job: w.agents[id].job.is_some(),
                            cart,
                        });
                    }
                }
                self.last_settlement[id] = place;
            }
            "Harvested" => self.add(place, "produced_food", quantity(&e.detail, "q") as u128),
            "Consumed" => {
                let b = &w.goods.batches[quantity(&e.detail, "batch") as usize];
                let q = quantity(&e.detail, "q") as u128;
                self.add(place, "consumed_food", q);
                let origin_place = self.holder_place(w, w.goods.batches[b.origin].holder);
                self.add(place, &format!("consumed_food_from_{origin_place}"), q);
                if w.farm.origins.contains(&b.origin) {
                    self.add(place, "consumed_crop_food", q);
                }
                if b.owner == Owner::Commons(0) {
                    self.add(place, "consumed_communal_food", q);
                }
            }
            "Spoiled" => {
                let b = &w.goods.batches[quantity(&e.detail, "batch") as usize];
                let p = self.holder_place(w, b.holder);
                self.add(p, "spoiled_food", quantity(&e.detail, "q") as u128);
            }
            "Gathered" | "Pickup" | "CargoLoaded" => {
                let id = e.agent.unwrap();
                let key = if e.kind == "Gathered" {
                    "source_batch"
                } else {
                    "batch"
                };
                let b = &w.goods.batches[quantity(&e.detail, key) as usize];
                let q = quantity(&e.detail, "q");
                let after = if e.kind == "CargoLoaded" || e.detail.contains("owner=Commons(0)") {
                    Owner::Commons(0)
                } else {
                    Owner::Agent(id)
                };
                self.transfers.push(Transfer {
                    at: w.now,
                    agent: id,
                    place,
                    kind: e.kind.clone(),
                    origin: b.origin,
                    material: b.material,
                    quantity: q,
                    before: b.owner,
                    after,
                });
                if e.kind == "Gathered" {
                    self.add(
                        place,
                        if b.material == Material::Wood {
                            "gathered_wood"
                        } else {
                            "gathered_food"
                        },
                        q as u128,
                    );
                }
            }
            "Delivered" => self.add(
                place,
                "communal_delivered",
                quantity(&e.detail, "q") as u128,
            ),
            "CartStarted" => {
                let project = w
                    .capital
                    .projects
                    .iter()
                    .find(|p| p.investor == e.agent.unwrap() && p.started_at == w.now)
                    .unwrap();
                self.add(place, "wood_invested", project.wood as u128);
            }
            _ => {}
        }
    }
    pub fn holder_place(&self, w: &World, holder: Holder) -> usize {
        match holder {
            Holder::Deposit(n) | Holder::Loose(n) | Holder::Commons(n) => self.node_places[n],
            Holder::Agent(id) => self.place(&w.agents[id].location),
            Holder::Cart(id) => self.place(&w.capital.carts[id].location),
            Holder::Edge { .. } => TRANSIT,
            Holder::MarketEscrow(id) => self.node_places[w.barter.offers[id].location],
        }
    }
}
impl World {
    pub fn enable_settlement_research(&mut self, node_places: Vec<usize>) -> Result<(), String> {
        if self.now != 0
            || self.initialized
            || self.settlement_research.is_some()
            || node_places.len() != self.config.nodes
            || node_places.iter().any(|p| *p > CORRIDOR)
        {
            return Err("research mapping must be set once before execution".into());
        }
        let cohorts = self
            .agents
            .iter()
            .map(|a| match a.location {
                Location::Node(n) => node_places[n],
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        self.settlement_research = Some(Journal {
            node_places,
            last_settlement: cohorts.clone(),
            cohorts,
            labor_ms: BTreeMap::new(),
            stats: BTreeMap::new(),
            events: BTreeMap::new(),
            flows: vec![],
            transfers: vec![],
            visits: vec![],
        });
        Ok(())
    }
    pub fn settlement_journal(&self) -> &Journal {
        self.settlement_research
            .as_ref()
            .expect("enabled research journal")
    }
    pub fn audit_settlement_research(&self) -> Result<(), String> {
        let j = self.settlement_journal();
        let labor: u128 = j.labor_ms.values().sum();
        let total = |key: &str| {
            j.stats
                .iter()
                .filter(|((_, k), _)| k == key)
                .map(|(_, v)| *v)
                .sum::<u128>()
        };
        if labor != self.metrics.physiology_ms
            || total("starvation_ms") != self.metrics.deficit_ms
            || total("consumed_food") != self.metrics.consumed as u128
            || total("spoiled_food") != self.metrics.spoiled as u128
            || total("produced_food") != self.farm.produced as u128
        {
            return Err("research accounting disagrees with kernel".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
