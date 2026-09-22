//! Research-only intention memory and observation/event journals. No world queries by policy.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    Reactive,
    Remembered,
    Resumable,
    Committed,
    SelfPriority,
}
impl Arm {
    pub fn parse(name: &str) -> Result<Self, String> {
        match name {
            "reactive" => Ok(Self::Reactive),
            "remembered" => Ok(Self::Remembered),
            "resumable" => Ok(Self::Resumable),
            "committed" => Ok(Self::Committed),
            "self_priority" => Ok(Self::SelfPriority),
            _ => Err(format!("unknown arm {name}")),
        }
    }
    pub fn retains(self) -> bool {
        matches!(self, Self::Remembered | Self::Resumable | Self::Committed)
    }
}
#[derive(Clone, Debug)]
pub struct VisitBoardForRequest {
    pub id: usize,
    pub request_id: usize,
    pub board_node: usize,
    pub created_at: Time,
    pub reason: String,
    pub quantity: u64,
    pub paused: bool,
    pub reached_x: bool,
    pub status: String,
    pub evidence: PolicyMemory,
    pub changes: BTreeSet<String>,
    pub created_nutrition: Time,
    pub created_fatigue: Time,
}
#[derive(Clone, Debug)]
pub(crate) struct Leg {
    pub edge: usize,
    pub from: usize,
    pub to: usize,
    pub plan: Option<usize>,
    pub evidence: PolicyMemory,
    pub changes: BTreeSet<String>,
    pub selected_at: Time,
    pub quantity: u64,
    pub reversal_counted: bool,
}
#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub plans: Vec<VisitBoardForRequest>,
    pub active: Option<usize>,
    pub route_marker: Option<usize>,
    pub queued_leg: Option<Leg>,
    pub pending_leg: Option<Leg>,
    pub moving: Option<Leg>,
    pub last_leg: Option<Leg>,
    pub records: Vec<Record>,
    pub memory: PolicyMemory,
}
#[derive(Clone, Debug)]
pub struct Record {
    pub at: Time,
    pub agent: usize,
    pub plan: Option<usize>,
    pub kind: String,
    pub reason: String,
    pub node: String,
    pub nutrition: Time,
    pub fatigue: Time,
    pub detail: String,
}
pub const HEADER: &str =
    "time_ms\tagent\tplan\tkind\treason\tlocation\tnutrition_ms\tfatigue_ms\tdetail";
impl Record {
    pub fn tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.at,
            self.agent,
            self.plan.map_or(String::new(), |v| v.to_string()),
            self.kind,
            self.reason,
            self.node,
            self.nutrition,
            self.fatigue,
            self.detail
        )
    }
}
impl State {
    pub fn record(
        &mut self,
        o: &Observation,
        plan: Option<usize>,
        kind: &str,
        reason: &str,
        detail: String,
    ) {
        self.records.push(Record {
            at: o.now,
            agent: o.agent,
            plan,
            kind: kind.into(),
            reason: reason.into(),
            node: format!("{:?}", o.location),
            nutrition: o.nutrition,
            fatigue: o.fatigue,
            detail,
        });
    }
    pub fn end(&mut self, o: &Observation, reason: &str) {
        if let Some(i) = self.active.take() {
            self.plans[i].status = format!("CANCELLED:{reason}");
            self.record(
                o,
                Some(i),
                "INTENT_CANCELLED",
                reason,
                format!("changes={:?}", self.plans[i].changes),
            );
        }
    }
}
#[derive(Clone, Debug, Default)]
pub(crate) struct Journal {
    pub states: BTreeMap<usize, State>,
    pub counts: BTreeMap<String, u64>,
    pub reasons: BTreeMap<String, u64>,
    pub records: Vec<Record>,
}
pub const COUNTERS: &[&str] = &[
    "INTENT_CREATED",
    "INTENT_PAUSED",
    "INTENT_RESUMED",
    "INTENT_CANCELLED",
    "INTENT_COMPLETED",
    "INTENT_REACHED_X",
    "QUEUE_INTERRUPTION",
    "DIRECTION_REVERSAL",
    "BOARD_REVERSAL",
    "UNINFORMED_REVERSAL",
    "UNINFORMED_BOARD_REVERSAL",
    "OPPOSITE_TRAVERSAL",
    "F_X_F",
    "BOARD_ARRIVAL",
    "ABLATION_ELIGIBLE",
    "ABLATION_CHANGED_DECISION",
];
impl Journal {
    fn collect(&mut self, id: usize) {
        let state = self.states.get_mut(&id).unwrap();
        for r in std::mem::take(&mut state.records) {
            *self.counts.entry(r.kind.clone()).or_default() += 1;
            if r.kind == "INTENT_CANCELLED"
                || r.kind == "INTENT_PAUSED"
                || r.kind == "DIRECTION_REVERSAL"
            {
                *self
                    .reasons
                    .entry(format!("{}:{}", r.kind, r.reason))
                    .or_default() += 1;
            }
            self.records.push(r);
        }
    }
    pub fn collect_decision(&mut self, id: usize) {
        self.collect(id);
    }
    pub fn event(&mut self, e: &Event, agents: &[Agent]) {
        let Some(id) = e.agent else { return };
        let Some(s) = self.states.get_mut(&id) else {
            return;
        };
        let a = &agents[id];
        let o = Observation {
            now: e.at,
            agent: id,
            location: a.location.clone(),
            action: a.action.clone(),
            nutrition: a.nutrition,
            fatigue: a.fatigue,
            starvation: a.starvation,
            own: vec![],
            load: 0,
            capacity: 0,
            local: None,
            job: a.job.clone(),
            goal: a.goal,
            communal: true,
            decision_interval: MINUTE,
        };
        if e.kind == "Queued" {
            s.queued_leg = s.pending_leg.take();
        } else if e.kind == "Admitted" {
            if let Some(mut leg) = s.queued_leg.take() {
                if let Some(i) = leg.plan {
                    leg.changes.extend(s.plans[i].changes.clone());
                }
                if let Some(last) = &s.last_leg
                    && last.edge == leg.edge
                    && last.from == leg.to
                    && last.to == leg.from
                {
                    s.record(
                        &o,
                        leg.plan,
                        "OPPOSITE_TRAVERSAL",
                        "PHYSICAL_ADMISSION",
                        format!(
                            "edge={} from={} to={} previous_plan={:?}",
                            leg.edge, leg.from, leg.to, last.plan
                        ),
                    );
                }
                s.moving = Some(leg);
            }
        } else if e.kind == "Interrupted" {
            if let Some(leg) = s.queued_leg.take() {
                s.record(
                    &o,
                    leg.plan,
                    "QUEUE_INTERRUPTION",
                    "ACTION_INTERRUPT",
                    e.detail.clone(),
                );
            }
        } else if e.kind == "Arrived" {
            if let Some(leg) = s.moving.take() {
                if let Some(last) = &s.last_leg
                    && last.from == 2
                    && last.to == 1
                    && leg.from == 1
                    && leg.to == 2
                {
                    s.record(
                        &o,
                        last.plan,
                        "F_X_F",
                        "PHYSICAL_RETURN",
                        format!("edge={} prior_plan={:?}", leg.edge, last.plan),
                    );
                }
                s.last_leg = Some(leg);
            }
            if let Some(i) = s.active {
                if a.location == Location::Node(1) && !s.plans[i].reached_x {
                    s.plans[i].reached_x = true;
                    s.record(
                        &o,
                        Some(i),
                        "INTENT_REACHED_X",
                        "PHYSICAL_ARRIVAL",
                        e.detail.clone(),
                    );
                }
                if a.location == Location::Node(s.plans[i].board_node) {
                    s.plans[i].status = "COMPLETED".into();
                    s.active = None;
                    s.record(
                        &o,
                        Some(i),
                        "INTENT_COMPLETED",
                        "PHYSICAL_BOARD_ARRIVAL",
                        e.detail.clone(),
                    );
                }
            }
            if a.location == Location::Node(0) {
                s.record(
                    &o,
                    None,
                    "BOARD_ARRIVAL",
                    "PHYSICAL_ARRIVAL",
                    e.detail.clone(),
                );
            }
        } else if e.kind == "Died" {
            s.end(&o, "DEATH");
            s.route_marker = None;
            s.queued_leg = None;
            s.moving = None;
        }
        self.collect(id);
    }
}
pub fn fixture(name: &str, seed: u64, trace: bool) -> Result<World, String> {
    World::new(Config {
        persistence: Some(Arm::parse(name)?),
        diagnostic_policy: DiagnosticPolicy::MealAware,
        acceptance_audit: true,
        initial_request: 600,
        seed,
        trace,
        ..Config::default()
    })
}
impl World {
    pub fn take_persistence_records(&mut self) -> Vec<Record> {
        std::mem::take(&mut self.persistence.records)
    }
    pub fn persistence_count(&self, key: &str) -> u64 {
        self.persistence.counts.get(key).copied().unwrap_or(0)
    }
    pub fn persistence_plans(&self) -> Vec<(usize, VisitBoardForRequest)> {
        self.persistence
            .states
            .iter()
            .flat_map(|(&id, s)| s.plans.iter().cloned().map(move |p| (id, p)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reactive_and_remembered_reproduce_frozen_meal_aware_physics() {
        for name in ["reactive", "remembered"] {
            let mut w = fixture(name, 0, false).unwrap();
            for h in 0..=72 {
                w.advance_to(h * HOUR).unwrap();
                w.take_persistence_records();
            }
            assert_eq!(w.event_hash(), 0x00b2d22978a370eb, "{name}");
            assert_eq!(w.state_hash(), 0x98c779b899d8487d, "{name}");
            assert_eq!(w.metrics.jobs, 0);
            assert_eq!(w.metrics.delivered, 0);
            assert_eq!(w.persistence_count("INTENT_COMPLETED"), 0);
            assert_eq!(
                w.acceptance.zero_ms.values().sum::<u128>(),
                w.metrics.deficit_ms
            );
        }
    }
    #[test]
    fn own_arrival_completes_intention_without_creating_claim_and_death_cancels() {
        let mut w = fixture("committed", 0, false).unwrap();
        let p = VisitBoardForRequest {
            id: 0,
            request_id: 0,
            board_node: 0,
            created_at: 0,
            reason: "test own decision".into(),
            quantity: 10,
            paused: true,
            reached_x: false,
            status: "PAUSED".into(),
            evidence: PolicyMemory::default(),
            changes: BTreeSet::new(),
            created_nutrition: 12 * HOUR,
            created_fatigue: 4 * HOUR,
        };
        for id in [0, 1] {
            let s = w.persistence.states.entry(id).or_default();
            s.plans.push(p.clone());
            s.active = Some(0);
        }
        let e = Event {
            id: 0,
            at: 0,
            agent: Some(0),
            cause: None,
            kind: "Arrived".into(),
            detail: "node=0 edge=0".into(),
        };
        w.persistence.event(&e, &w.agents);
        assert_eq!(w.persistence_count("INTENT_COMPLETED"), 1);
        assert_eq!(w.metrics.jobs, 0);
        assert!(w.boards[0].claims.is_empty());
        assert!(w.agents[0].job.is_none());
        w.persistence.event(
            &Event {
                agent: Some(1),
                kind: "Died".into(),
                ..e
            },
            &w.agents,
        );
        assert_eq!(w.persistence.states[&1].plans[0].status, "CANCELLED:DEATH");
    }
}
