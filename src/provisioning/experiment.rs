//! Fixed interventions for falsification. Scenario definitions are versioned inputs.
use super::*;
pub const SCENARIOS: &[&str] = &[
    "reference",
    "prior_information",
    "self_only",
    "no_transport",
    "no_food",
    "scarce",
    "unequal_access",
    "contested_requests",
    "reverse_order",
    "capacity_1",
    "capacity_2",
    "distance_6h",
    "distance_12h",
    "shelf_12h",
    "shelf_24h",
    "near_fresh_far_old",
    "stale_depletion",
    "cadence_30s",
    "cadence_120s",
];
#[derive(Clone, Debug)]
pub struct Checkpoint {
    pub at: Time,
    pub agents: Vec<Agent>,
}
pub struct Run {
    pub world: World,
    pub checkpoints: Vec<Checkpoint>,
    pub parameters: Config,
}
pub fn configuration(name: &str, seed: u64, trace: bool) -> Result<Config, String> {
    // Published P1 matrix is a historical reproduction entry point.
    let mut c = Config {
        diagnostic_policy: DiagnosticPolicy::LegacyForecast,
        seed,
        trace,
        ..Config::default()
    };
    match name {
        "reference" => {}
        "prior_information" => c.prior_memory = true,
        "self_only" => c.communal = false,
        "no_transport" => {
            for e in &mut c.edges {
                e.capacity = 0;
            }
        }
        "no_food" => c.food[0].1 = 0,
        "scarce" => c.food[0].1 = 300,
        "unequal_access" => {
            for n in &mut c.starts[25..] {
                *n = 2;
            }
        }
        "contested_requests" => {
            c.boards = vec![0, 1];
            for n in &mut c.starts[25..] {
                *n = 1;
            }
        }
        "reverse_order" => c.reverse_evaluation = true,
        "capacity_1" | "capacity_2" => {
            for e in &mut c.edges {
                e.capacity = if name == "capacity_1" { 1 } else { 2 };
            }
        }
        "distance_6h" | "distance_12h" => {
            for e in &mut c.edges {
                e.nominal_ms = if name == "distance_6h" {
                    6 * HOUR
                } else {
                    12 * HOUR
                };
            }
        }
        "shelf_12h" | "shelf_24h" => {
            c.food[0].3 = if name == "shelf_12h" {
                12 * HOUR
            } else {
                24 * HOUR
            }
        }
        "near_fresh_far_old" => {
            c.nodes = 4;
            c.edges.push(Edge {
                a: 0,
                b: 3,
                nominal_ms: HOUR / 2,
                capacity: 4,
            });
            c.food = vec![(3, 300, 0, 48 * HOUR), (2, 2700, -24 * HOUR, 48 * HOUR)];
        }
        "stale_depletion" => {
            c.food[0].1 = 6;
            c.starts[49] = 2;
            c.prior_memory = true;
        }
        "cadence_30s" => c.policy_ms = 30 * SECOND,
        "cadence_120s" => c.policy_ms = 120 * SECOND,
        _ => return Err(format!("unknown scenario {name}")),
    }
    Ok(c)
}
pub fn run(name: &str, seed: u64, trace: bool) -> Result<Run, String> {
    let parameters = configuration(name, seed, trace)?;
    let mut world = World::new(parameters.clone())?;
    if name == "stale_depletion" {
        // Recorded physical intervention: actor 49 is initially at F and actually
        // gathers and consumes its six quanta while others retain t=0 memories.
        world.config.autonomous = false;
        for _ in 0..6 {
            world.apply_intents(vec![(
                49,
                Intent::Gather {
                    batch: 0,
                    communal: false,
                },
            )])?;
            world.advance_to(world.now + 2000)?;
        }
        let batch = world
            .goods
            .batches
            .iter()
            .find(|b| b.holder == Holder::Agent(49) && b.quantity > 0)
            .unwrap()
            .id;
        world.apply_intents(vec![(49, Intent::Eat { batch, quantity: 6 })])?;
        world.log(
            None,
            "FixtureComplete",
            "source physically depleted by agent 49; other agents retain t=0 observations".into(),
        );
        world.config.autonomous = true;
    }
    let mut checkpoints = vec![];
    for h in [36, 48, 72] {
        world.advance_to(h * HOUR)?;
        checkpoints.push(Checkpoint {
            at: h * HOUR,
            agents: world.agents.clone(),
        });
    }
    Ok(Run {
        world,
        checkpoints,
        parameters,
    })
}
impl World {
    pub fn state_hash(&self) -> u64 {
        let mut text = format!(
            "{:?}|{:?}|{:?}|{:?}|{:?}",
            self.now, self.agents, self.goods, self.boards, self.metrics
        );
        if self.config.commands {
            text.push_str(&format!(
                "|commands={:?}|{}",
                self.commands.memories, self.commands.next_message
            ));
        }
        if self.config.information {
            text.push_str(&format!(
                "|information={:?}|{}",
                self.information.memories, self.information.next_report
            ));
        }
        if self.config.combat.is_some() {
            text.push_str(&format!(
                "|combat={:?}|{:?}|{:?}",
                self.config.combat, self.combat.bodies, self.combat.views
            ));
        }
        if self.config.formation {
            text.push_str(&format!(
                "|formation={:?}|{:?}|{}",
                self.formation.units, self.formation.views, self.formation.next_order
            ));
        }
        if !self.playable_retreats.is_empty() {
            text.push_str(&format!("|playable_retreats={:?}", self.playable_retreats));
        }
        if self.config.military.is_some() {
            text.push_str(&format!(
                "|military={:?}|{:?}",
                self.config.military, self.military.memories
            ));
        }
        if self.config.lifecycle.is_some() {
            text.push_str(&format!(
                "|lifecycle={:?}|{:?}|{:?}|{:?}",
                self.config.lifecycle,
                self.lifecycle.people,
                self.lifecycle.views,
                self.lifecycle.consents
            ));
        }
        if self.config.social.is_some() {
            text.push_str(&format!(
                "|social={:?}|{:?}",
                self.config.social, self.social.memories
            ));
        }
        if self.config.household.is_some() {
            text.push_str(&format!(
                "|household={:?}|{:?}",
                self.config.household, self.household.memories
            ));
        }
        if self.config.authority.is_some() {
            text.push_str(&format!(
                "|authority={:?}|{:?}|{:?}|{:?}",
                self.config.authority,
                self.polity(),
                self.offices(),
                self.commissions()
            ));
        }
        if self.config.farming.is_some() {
            text.push_str(&format!("|farming={:?}", self.farm));
        }
        if self.config.paths.is_some() {
            text.push_str(&format!("|paths={:?}", self.paths));
        }
        if self.config.infrastructure.is_some() {
            text.push_str(&format!("|infrastructure={:?}", self.infrastructure));
        }
        if self.config.capital.is_some() {
            text.push_str(&format!("|capital={:?}", self.capital));
        }
        if !self.config.market_nodes.is_empty() {
            text.push_str(&format!("|markets={:?}", self.config.market_nodes));
            text.push_str(&format!("|market_pending={:?}", self.barter.pending));
        }
        if self.config.barter {
            text.push_str(&format!("|barter={:?}", self.barter.offers));
        }
        let mut h = 0xcbf29ce484222325u64;
        for byte in text.bytes() {
            h ^= byte as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }
}
pub fn csv_header() -> &'static str {
    "scenario,seed,survivors_36h,survivors_48h,survivors_72h,deaths,food_initial,food_gathered,food_delivered,food_consumed,food_spoiled,spoiled_in_transit,spoiled_communal,food_stranded_at_death,food_carried_at_death,request_hours,request_quantum_hours,jobs_accepted,jobs_cancelled,failed_stale_trips,infeasible_abandons,food_search_distance_m,completed_food_trips,queue_person_hours,loaded_queue_hours,unloaded_queue_hours,queue_overtakes,idle_person_hours,sleep_person_hours,starvation_person_hours,lease_expiries,food_remaining,waste_remaining,inaccessible_dead_personal_food,alive_at_c,alive_at_f,accounting_error,event_hash,state_hash"
}
pub fn csv_row(name: &str, seed: u64, w: &World) -> String {
    let m = &w.metrics;
    let mut fields = vec![name.to_string(), seed.to_string()];
    for h in [36, 48, 72] {
        fields.push(m.survivors.get(&(h * HOUR)).unwrap_or(&0).to_string());
    }
    for n in [
        m.deaths as u128,
        w.goods.initial[0],
        m.gathered as u128,
        m.delivered as u128,
        m.consumed as u128,
        m.spoiled as u128,
        m.spoiled_transit as u128,
        m.spoiled_commons as u128,
        m.stranded as u128,
        m.carried_at_death as u128,
    ] {
        fields.push(n.to_string());
    }
    for n in [m.request_ms, m.request_quantum_ms] {
        fields.push(format!("{:.6}", n as f64 / HOUR as f64));
    }
    for n in [
        m.jobs,
        m.cancellations,
        m.stale_failures,
        m.infeasible_abandons,
    ] {
        fields.push(n.to_string());
    }
    fields.push(format!("{:.6}", m.food_travel_mm as f64 / 1000.0));
    fields.push(m.food_trips.to_string());
    for n in [m.queue_ms, m.loaded_queue_ms, m.unloaded_queue_ms] {
        fields.push(format!("{:.6}", n as f64 / HOUR as f64));
    }
    fields.push(m.overtakes.to_string());
    for n in [m.idle_ms, m.sleep_ms, m.deficit_ms] {
        fields.push(format!("{:.6}", n as f64 / HOUR as f64));
    }
    fields.push(m.lease_expiries.to_string());
    for material in [Material::Food, Material::Waste] {
        fields.push(
            w.goods
                .batches
                .iter()
                .filter(|b| b.material == material)
                .map(|b| b.quantity)
                .sum::<u64>()
                .to_string(),
        );
    }
    fields.push(
        w.goods
            .batches
            .iter()
            .filter(|b| {
                b.material == Material::Food
                    && matches!(b.owner,Owner::Agent(id)if w.agents[id].action==Action::Dead)
            })
            .map(|b| b.quantity)
            .sum::<u64>()
            .to_string(),
    );
    for node in [0, 2] {
        fields.push(
            w.agents
                .iter()
                .filter(|a| a.action != Action::Dead && a.location == Location::Node(node))
                .count()
                .to_string(),
        );
    }
    fields.push(if w.audit().is_ok() { "0" } else { "1" }.into());
    fields.push(format!("{:016x}", w.event_hash));
    fields.push(format!("{:016x}", w.state_hash()));
    fields.join(",")
}
