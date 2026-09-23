//! F3: personal crop claims, local beliefs and an explicit natural-production ledger.
//! No land titles. Research parameters are not a claim about real agronomy.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rules {
    pub sow_ms: Time,
    pub grow_ms: Time,
    pub harvest_ms: Time,
    pub yield_quanta: u64,
}
impl Default for Rules {
    fn default() -> Self {
        Self {
            sow_ms: HOUR,
            grow_ms: 12 * HOUR,
            harvest_ms: HOUR,
            yield_quanta: 24,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// Each entry is one physical cultivation unit, never a worker assignment.
    pub units: Vec<usize>,
    pub rules: Rules,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stage {
    Sowing,
    Growing { ready_at: Time },
    Mature,
    Harvesting,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Crop {
    pub id: u64,
    pub owner: Owner,
    pub started_at: Time,
    pub stage: Stage,
    /// Retained physical work; cancelling an action does not undo completed labor.
    pub remaining: u128,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plot {
    pub id: usize,
    pub node: usize,
    pub crop: Option<Crop>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct View {
    pub observed_at: Time,
    pub plots: Vec<Plot>,
}
pub type Memory = BTreeMap<usize, View>;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct State {
    pub plots: Vec<Plot>,
    pub memories: Vec<Memory>,
    pub attempts: u64,
    pub started: u64,
    pub matured: u64,
    pub harvested: u64,
    pub produced: u64,
    pub origins: BTreeSet<usize>,
    pub labor_ms: u128,
    pub interruptions: u64,
    pub resumptions: u64,
    pub activity_ms: BTreeMap<String, u128>,
}
impl State {
    pub fn new(config: Option<&Config>, nodes: usize, agents: usize) -> Result<Self, String> {
        let Some(c) = config else {
            return Ok(Self::default());
        };
        if c.units.iter().any(|&n| n >= nodes)
            || c.rules.sow_ms <= 0
            || c.rules.grow_ms <= 0
            || c.rules.harvest_ms <= 0
            || c.rules.yield_quanta == 0
        {
            return Err("invalid cultivation configuration".into());
        }
        Ok(Self {
            plots: c
                .units
                .iter()
                .enumerate()
                .map(|(id, &node)| Plot {
                    id,
                    node,
                    crop: None,
                })
                .collect(),
            memories: vec![Memory::new(); agents],
            ..Self::default()
        })
    }
    pub fn next_boundary(&self, now: Time, target: Time) -> Time {
        self.plots
            .iter()
            .filter_map(|p| match p.crop.as_ref()?.stage {
                Stage::Growing { ready_at } if ready_at > now => Some(ready_at),
                _ => None,
            })
            .fold(target, Time::min)
    }
}
impl World {
    pub fn plots(&self) -> &[Plot] {
        &self.farm.plots
    }
    pub fn farming_memory(&self, id: usize) -> Option<&Memory> {
        self.farm.memories.get(id)
    }
    pub(super) fn observe_farming(&mut self, id: usize) {
        if self.config.farming.is_none() {
            return;
        }
        if let Location::Node(n) = self.agents[id].location {
            self.farm.memories[id].insert(
                n,
                View {
                    observed_at: self.now,
                    plots: self
                        .farm
                        .plots
                        .iter()
                        .filter(|p| p.node == n)
                        .cloned()
                        .collect(),
                },
            );
        }
    }
    pub(super) fn begin_farm(&mut self, id: usize, plot: usize) -> Result<(), String> {
        let rules = self
            .config
            .farming
            .as_ref()
            .ok_or("cultivation absent")?
            .rules
            .clone();
        self.farm.attempts += 1;
        let p = self
            .farm
            .plots
            .get(plot)
            .ok_or("unknown cultivation unit")?;
        if self.agents[id].location != Location::Node(p.node) {
            return Err("nonlocal cultivation".into());
        }
        if let Some(c) = &p.crop {
            if c.owner != Owner::Agent(id) {
                return Err("crop ownership forbids access".into());
            }
            if matches!(c.stage, Stage::Growing { .. }) {
                return Err("crop still growing".into());
            }
        }
        if matches!(self.agents[id].action, Action::Farm { plot: old } if old == plot) {
            return Ok(());
        }
        self.cancel_action(id, "cultivation work");
        if self.farm.plots[plot].crop.is_none() {
            let crop = Crop {
                id: self.farm.started,
                owner: Owner::Agent(id),
                started_at: self.now,
                stage: Stage::Sowing,
                remaining: rules.sow_ms as u128 * SCALE,
            };
            self.farm.started += 1;
            self.log(
                Some(id),
                "CropStarted",
                format!("plot={plot} crop={} owner={:?}", crop.id, crop.owner),
            );
            self.farm.plots[plot].crop = Some(crop);
        } else {
            let c = self.farm.plots[plot].crop.as_mut().unwrap();
            if c.stage == Stage::Mature {
                c.stage = Stage::Harvesting;
                c.remaining = rules.harvest_ms as u128 * SCALE;
            } else {
                self.farm.resumptions += 1;
            }
        }
        self.agents[id].action = Action::Farm { plot };
        self.log(
            Some(id),
            "FarmWorkStarted",
            format!("plot={plot} crop={:?}", self.farm.plots[plot].crop),
        );
        Ok(())
    }
    pub(super) fn finish_farming(&mut self) {
        let Some(config) = &self.config.farming else {
            return;
        };
        let rules = config.rules.clone();
        for plot in 0..self.farm.plots.len() {
            let Some(c) = self.farm.plots[plot].crop.clone() else {
                continue;
            };
            let Owner::Agent(id) = c.owner else {
                unreachable!()
            };
            if let Stage::Growing { ready_at } = c.stage {
                if self.now >= ready_at {
                    self.farm.plots[plot].crop.as_mut().unwrap().stage = Stage::Mature;
                    self.farm.matured += 1;
                    // No remote wake or memory update. Growth is world state only.
                    self.log(
                        None,
                        "CropMatured",
                        format!("plot={plot} crop={} owner={id}", c.id),
                    );
                }
            } else if c.remaining == 0 && self.agents[id].action == (Action::Farm { plot }) {
                self.agents[id].action = Action::Idle;
                self.wake.insert(id);
                if c.stage == Stage::Sowing {
                    self.farm.plots[plot].crop.as_mut().unwrap().stage = Stage::Growing {
                        ready_at: self.now + rules.grow_ms,
                    };
                    self.log(
                        Some(id),
                        "CropEstablished",
                        format!(
                            "plot={plot} crop={} ready_at={}",
                            c.id,
                            self.now + rules.grow_ms
                        ),
                    );
                } else {
                    assert_eq!(c.stage, Stage::Harvesting);
                    let node = self.farm.plots[plot].node;
                    let q = rules.yield_quanta;
                    let batch = self.goods.add(
                        Material::Food,
                        q,
                        c.owner,
                        Holder::Loose(node),
                        self.now,
                        48 * HOUR,
                    );
                    // Explicit natural input, never relabel production as initial stock.
                    self.goods.initial[0] -= q as u128;
                    self.farm.produced += q;
                    self.farm.origins.insert(batch);
                    self.farm.harvested += 1;
                    self.farm.plots[plot].crop = None;
                    self.log(
                        Some(id),
                        "Harvested",
                        format!(
                            "plot={plot} crop={} batch={batch} q={q} owner={:?} created_at={}",
                            c.id, c.owner, self.now
                        ),
                    );
                }
            }
        }
    }
    pub(super) fn audit_farming(&self) -> Result<(), String> {
        let Some(config) = &self.config.farming else {
            return if self.farm == State::default() {
                Ok(())
            } else {
                Err("farming state without physical configuration".into())
            };
        };
        if self.farm.produced != self.farm.harvested * config.rules.yield_quanta
            || self.farm.origins.len() as u64 != self.farm.harvested
        {
            return Err("production ledger mismatch".into());
        }
        for p in &self.farm.plots {
            if let Some(c) = &p.crop {
                let Owner::Agent(owner) = c.owner else {
                    return Err("nonpersonal crop ownership".into());
                };
                if owner >= self.agents.len() {
                    return Err("invalid crop owner".into());
                }
            }
        }
        for a in &self.agents {
            if let Action::Farm { plot } = a.action {
                let p = self.farm.plots.get(plot).ok_or("work without plot")?;
                let c = p.crop.as_ref().ok_or("work without crop")?;
                if a.location != Location::Node(p.node)
                    || c.owner != Owner::Agent(a.id)
                    || !matches!(c.stage, Stage::Sowing | Stage::Harvesting)
                {
                    return Err("cultivation exclusivity/locality".into());
                }
            }
        }
        Ok(())
    }
    pub fn farming_activity_ms(&self) -> &BTreeMap<String, u128> {
        &self.farm.activity_ms
    }
    pub fn farming_counters(&self) -> BTreeMap<String, u128> {
        let f = &self.farm;
        let outputs: Vec<_> = self
            .goods
            .batches
            .iter()
            .filter(|b| f.origins.contains(&b.origin))
            .collect();
        let left: u128 = outputs.iter().map(|b| b.quantity as u128).sum();
        let spoiled: u128 = outputs
            .iter()
            .filter(|b| b.material == Material::Waste)
            .map(|b| b.quantity as u128)
            .sum();
        let dead_crops = f.plots.iter().filter(|p| p.crop.as_ref().is_some_and(|c| matches!(c.owner, Owner::Agent(id) if self.agents[id].action == Action::Dead))).count();
        let inaccessible = f.plots.iter().filter(|p| p.crop.as_ref().is_some_and(|c| matches!(c.stage, Stage::Mature | Stage::Harvesting) && matches!(c.owner, Owner::Agent(id) if self.agents[id].action == Action::Dead))).count();
        let stranded = f.plots.iter().filter(|p| p.crop.as_ref().is_some_and(|c| matches!(c.owner, Owner::Agent(id) if self.agents[id].location != Location::Node(p.node)))).count();
        [
            ("attempts", f.attempts as u128),
            ("started", f.started as u128),
            ("matured", f.matured as u128),
            ("harvested", f.harvested as u128),
            ("produced", f.produced as u128),
            ("output_consumed", f.produced as u128 - left),
            ("output_spoiled", spoiled),
            ("labor_ms", f.labor_ms),
            ("interruptions", f.interruptions as u128),
            ("resumptions", f.resumptions as u128),
            ("dead_owner_crops", dead_crops as u128),
            ("inaccessible_mature", inaccessible as u128),
            ("stranded_crops", stranded as u128),
            (
                "active_crops",
                f.plots.iter().filter(|p| p.crop.is_some()).count() as u128,
            ),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
    }
}

/// Pure policy extension: no World reference and no configuration containing plot locations.
pub(crate) fn consider(
    o: &Observation,
    memory: &PolicyMemory,
    farms: &Memory,
    rules: &Rules,
    rng: &mut u64,
) -> Option<Decision> {
    let Location::Node(node) = o.location else {
        return None;
    };
    let own: Vec<_> = farms
        .values()
        .flat_map(|v| &v.plots)
        .filter(|p| {
            p.crop
                .as_ref()
                .is_some_and(|c| c.owner == Owner::Agent(o.agent))
        })
        .collect();
    if let Some(p) = own.iter().filter(|p| {
        !matches!(p.crop.as_ref().unwrap().stage, Stage::Growing { ready_at } if ready_at > o.now)
    }).min_by_key(|p| (p.node != node, p.id)) {
        if p.node == node {
            return Some(Decision { intent: Intent::Cultivate { plot: p.id }, reason: "work on locally observed personal crop; interruption remains legal".into() });
        }
        if o.nutrition > 6 * HOUR && let Some(r) = policy::coherent::route(o, memory, p.node)
            && r.starvation < 24 * HOUR {
            return Some(Decision { intent: Intent::Travel { edge: r.first, target: p.node, food: false }, reason: "return toward remembered personal crop, conditional on still existing".into() });
        }
    }
    if !own.is_empty() || o.nutrition <= 6 * HOUR {
        return None;
    }
    let available: Vec<_> = farms
        .get(&node)?
        .plots
        .iter()
        .filter(|p| p.crop.is_none())
        .collect();
    if available.is_empty() {
        return None;
    }
    let duration =
        policy::coherent::cultivation_forecast(o, rules.sow_ms, rules.grow_ms, rules.harvest_ms)?;
    // Count only personal or accessible unowned food already seen here/carried and
    // believed edible through that horizon. No promised future communal rations.
    let food: u64 = o
        .own
        .iter()
        .chain(o.local.iter().flat_map(|v| &v.food))
        .filter(|f| {
            (f.owner == Owner::Agent(o.agent) || f.owner == Owner::None)
                && f.expires_at > o.now + duration
        })
        .map(|f| f.available)
        .sum();
    if o.nutrition
        .saturating_add(food.min(i64::MAX as u64 / HOUR as u64) as Time * HOUR)
        >= duration + 6 * HOUR
    {
        return None;
    }
    let p = available[(policy::random(rng) as usize) % available.len()];
    Some(Decision {
        intent: Intent::Cultivate { plot: p.id },
        reason: format!(
            "personal future food investment; conditional harvest_ms={} known_food={food}; no crop title to land",
            o.now + duration
        ),
    })
}

#[cfg(test)]
mod tests;
