//! F13A integration control, not a kingdom policy or new command authority.
use super::*;

pub const CASES: &[&str] = &["alternate", "scarce", "long"];
pub const STEWARD: usize = 1;
pub const COURIER: usize = 2;
pub const TARGET: usize = 2;
pub const END: Time = 72 * HOUR;
const PROTECT_AT: Time = HOUR + 5 * MINUTE;
const RELEASE_AT: Time = 6 * HOUR;

pub struct Run {
    pub world: World,
    pub report: information::ReportId,
    pub target_origin: usize,
    pub protection_receipts: Option<usize>,
    pub protection_error: Option<String>,
    protect_attempted: bool,
    released: bool,
}

pub fn fixture(case: &str, commissioned: bool, seed: u64) -> Result<Run, String> {
    fixture_impl(case, commissioned, seed, false)
}

/// F13B reuses the registered physical world with explicit local office consent.
pub(crate) fn command_fixture(seed: u64) -> Result<Run, String> {
    fixture_impl("alternate", false, seed, true)
}

fn fixture_impl(case: &str, commissioned: bool, seed: u64, commands: bool) -> Result<Run, String> {
    if !CASES.contains(&case) {
        return Err("unregistered kingdom readiness case".into());
    }
    let detour = if case == "long" { 8 * HOUR } else { 2 * HOUR };
    let mut starts = vec![1; 18];
    starts[COURIER] = 2;
    starts[9..12].fill(4);
    starts[12..16].fill(2);
    let c = Config {
        seed,
        trace: true,
        information: true,
        commands,
        formation: true,
        nodes: 5,
        starts,
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: HOUR / 4,
                capacity: 4,
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: HOUR / 4,
                capacity: 0,
            },
            Edge {
                a: 1,
                b: 3,
                nominal_ms: detour,
                capacity: 4,
            },
            Edge {
                a: 3,
                b: 2,
                nominal_ms: detour,
                capacity: 4,
            },
            Edge {
                a: 4,
                b: 2,
                nominal_ms: 2 * HOUR,
                capacity: 4,
            },
        ],
        food: vec![(1, 24, 0, 48 * HOUR)],
        boards: vec![1],
        initial_request: 60,
        initial_nutrition: 24 * HOUR,
        authority: Some(authority::Config {
            polity_id: 17,
            sovereign: 0,
            settlements: vec![1, 2],
        }),
        infrastructure: Some(infrastructure::Config {
            enabled: true,
            wood: vec![(1, if case == "scarce" { 5 } else { 100 })],
            sites: vec![infrastructure::Site {
                edge: 1,
                node: 1,
                wood_cost: 20,
                work_ms: 8 * HOUR,
                opened_nominal_ms: HOUR / 4,
                opened_capacity: 4,
            }],
        }),
        paths: Some(paths::Config::ground(vec![0, 2, 3, 4], true)),
        farming: Some(farming::Config {
            units: vec![1, 1, 2, 2, 2, 2],
            rules: Default::default(),
        }),
        household: Some(household::Config {
            enabled: true,
            households: vec![
                household::Household {
                    id: 0,
                    members: vec![3, 16],
                    home: Some(1),
                },
                household::Household {
                    id: 1,
                    members: vec![4, 17],
                    home: Some(1),
                },
            ],
            kin: vec![
                household::Relation {
                    from: 3,
                    to: 16,
                    kind: household::Kin::Parent,
                },
                household::Relation {
                    from: 4,
                    to: 17,
                    kind: household::Kin::Parent,
                },
            ],
            care: vec![
                household::Caregiver {
                    person: 3,
                    dependent: 16,
                },
                household::Caregiver {
                    person: 4,
                    dependent: 17,
                },
            ],
        }),
        social: Some(social::Config::default()),
        lifecycle: Some(lifecycle::Config {
            people: (0..18)
                .map(|id| lifecycle::Seed {
                    birth_time: if id >= 16 { 0 } else { -24 * HOUR },
                    sex: if id % 2 == 0 {
                        lifecycle::Sex::Male
                    } else {
                        lifecycle::Sex::Female
                    },
                    drive: false,
                })
                .collect(),
            rules: Default::default(),
        }),
        military: Some(military::Config {
            enabled: true,
            requests: vec![military::Request {
                id: 0,
                posting: 1,
                muster: 1,
                attendance: 4,
                opens: 0,
                expires: 60 * HOUR,
                duration: 12 * HOUR,
                objective: military::Objective::ProtectCrossing,
            }],
        }),
        combat: Some(combat::Config {
            nodes: vec![0, 1, 2, 3, 4],
            raids: (9..12)
                .map(|person| combat::Raid {
                    person,
                    target: TARGET,
                    home: 4,
                    begins: 2 * HOUR,
                    quota: 10,
                })
                .collect(),
            strike_work: 15 * MINUTE,
            theft_work: 5 * MINUTE,
        }),
        ..Default::default()
    };
    let mut w = World::new(c)?;
    let target_origin = w.goods.add(
        Material::Food,
        200,
        Owner::Commons(0),
        Holder::Commons(TARGET),
        0,
        48 * HOUR,
    );
    for id in 0..18 {
        if id >= 16 {
            w.agents[id].nutrition = 8 * HOUR;
        } else {
            w.goods.add(
                Material::Food,
                6,
                Owner::Agent(id),
                Holder::Agent(id),
                0,
                48 * HOUR,
            );
        }
    }
    // Explicit controlled prior visits: node-local adjacency only, no remote stock,
    // persons, boards, farm state or current queue knowledge is seeded.
    for node in 0..w.config.nodes {
        let mut old_map = w.node_view(node);
        old_map.observed_at = -HOUR;
        old_map.food.clear();
        old_map.people.clear();
        old_map.board = None;
        for person in &mut w.agents {
            person.memory.nodes.insert(node, old_map.clone());
        }
    }
    w.log(None, "ResearchInitialConditions", "F13A: known prior map visits only; seeded threat, kinship, goods and voluntary local request; no remote current stock beliefs".into());
    let report = w.capture_report(COURIER)?;
    w.begin_report_delivery(COURIER, report, STEWARD, 1)?;
    w.apply_intents(vec![(
        0,
        Intent::AppointSteward {
            candidate: STEWARD,
            settlement: 1,
        },
    )])?;
    if commands {
        w.apply_intents(vec![(STEWARD, Intent::AcceptOffice { office: 0 })])?;
    }
    if commissioned {
        w.apply_intents(vec![(
            STEWARD,
            Intent::CommissionCrossing {
                office: 0,
                project: 0,
            },
        )])?;
        if w.commissions().len() != 1 {
            return Err("registered local commission was not legally issued".into());
        }
    }
    // Existing F12 recipient contact control, NOT an autonomous residence policy.
    w.transport_research.actors = Some((0..18).filter(|id| *id != STEWARD).collect());
    w.log(
        Some(STEWARD),
        "ResearchRecipientHold",
        "deliberation held until6h; physiology remains active; all other Persons autonomous".into(),
    );
    w.audit()?;
    Ok(Run {
        world: w,
        report,
        target_origin,
        protection_receipts: None,
        protection_error: None,
        protect_attempted: false,
        released: false,
    })
}

impl Run {
    pub fn advance_to(&mut self, to: Time) -> Result<(), String> {
        if to < self.world.now || to > END {
            return Err("outside registered monotonically increasing 72h horizon".into());
        }
        if !self.protect_attempted && to >= PROTECT_AT {
            self.world.advance_to(PROTECT_AT)?;
            self.protect_attempted = true;
            match self.world.issue_unit_protect(0, 1, TARGET) {
                Ok(n) => self.protection_receipts = Some(n),
                Err(e) => {
                    self.world
                        .log(None, "ResearchProtectionUnavailable", e.clone());
                    self.protection_error = Some(e);
                }
            }
        }
        if !self.released && to >= RELEASE_AT {
            self.world.advance_to(RELEASE_AT)?;
            self.world.transport_research.actors = None;
            self.world.log(
                Some(STEWARD),
                "ResearchRecipientReleased",
                "ordinary policy resumes; no residence or work assigned".into(),
            );
            self.released = true;
        }
        self.world.advance_to(to)?;
        self.world.audit()?;
        if self.world.agents.len() > 100 {
            return Err(
                "research safety bound exceeded; run terminated, no birth suppressed".into(),
            );
        }
        Ok(())
    }

    pub fn parameters(&self) -> String {
        format!(
            "{:?}\nreport={} target_origin={}\nsource=actual node2 at0; initial map-only visits at-1h; steward1 controlled hold0..6h; protect command1h05m; no retries; horizon72h; all other Persons autonomous\nSee docs/KINGDOM_READINESS_EXPERIMENT.md",
            self.world.config, self.report, self.target_origin
        )
    }

    pub fn metrics(&self) -> Result<BTreeMap<String, String>, String> {
        let w = &self.world;
        w.audit()?;
        let mut out = BTreeMap::new();
        macro_rules! put {
            ($key:expr, $value:expr) => {
                out.insert(($key).to_string(), format!("{}", $value));
            };
        }
        let hours = |t: u128| t as f64 / HOUR as f64;
        put!("hour", w.now as f64 / HOUR as f64);
        put!("persons", w.agents.len());
        put!("peak_persons", w.lifecycle.journal.peak);
        put!(
            "alive",
            w.agents.iter().filter(|a| a.action != Action::Dead).count()
        );
        put!("deaths", w.metrics.deaths);
        put!("consumed", w.metrics.consumed);
        put!("spoiled", w.metrics.spoiled);
        put!("delivered", w.metrics.delivered);
        put!("starvation_h", hours(w.metrics.deficit_ms));
        put!("queue_h", hours(w.metrics.queue_ms));
        put!("jobs", w.metrics.jobs);
        put!("overtakes", w.metrics.overtakes);
        put!("audit_ok", 1);
        put!("appointments", w.offices().len());
        put!("commissions", w.commissions().len());
        put!(
            "administered_settlements",
            w.administered_settlements().len()
        );
        put!(
            "authority_h",
            hours(w.authority.journal.active_ms.values().sum())
        );
        put!(
            "steward_controlled_hold_h",
            w.now.min(RELEASE_AT) as f64 / HOUR as f64
        );
        put!("steward_food", w.agents[STEWARD].consumed);
        put!("steward_alive", w.agents[STEWARD].action != Action::Dead);
        for (k, v) in w.infrastructure_counters() {
            put!(format!("crossing_{k}"), v);
        }
        put!("crossing_donors", w.crossing_projects()[0].donors.len());
        put!("crossing_workers", w.crossing_projects()[0].labor_ms.len());
        for (k, v) in w.farming_counters() {
            put!(format!("farm_{k}"), v);
        }
        put!(
            "crossing_completion_h",
            w.crossing_projects()[0]
                .completed_at
                .map(|t| (t as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        put!(
            "path_passages",
            w.path_surfaces().values().map(|s| s.passages).sum::<u64>()
        );
        put!(
            "path_compaction",
            w.path_surfaces()
                .values()
                .map(|s| s.compaction)
                .sum::<u64>()
        );
        put!(
            "path_trips",
            w.path_trips()
                .iter()
                .filter(|t| t.arrived.is_some())
                .count()
        );
        let receipt = w
            .information_memory(STEWARD)
            .and_then(|m| m.reports.get(&self.report));
        put!("report_received", receipt.is_some());
        put!(
            "report_receipt_h",
            receipt
                .map(|r| (r.received_at as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        put!(
            "report_receipt_age_h",
            receipt
                .map(
                    |r| ((r.received_at - r.report.observation.observed_at) as f64 / HOUR as f64)
                        .to_string()
                )
                .unwrap_or_default()
        );
        put!("report_handoffs", w.information.journal.handoffs.len());
        put!(
            "report_undertaking_open",
            w.information.memories[COURIER].delivery.is_some()
        );
        put!("protect_receipts", self.protection_receipts.unwrap_or(0));
        put!("protect_command_failed", self.protection_error.is_some());
        put!("volunteers", w.military.journal.count("volunteers"));
        put!("assembled", w.military.journal.count("arrivals"));
        let unit = w.unit_snapshot(0).unwrap_or_default();
        put!("unit_arrivals", unit.arrived);
        put!("active_soldiers", unit.active);
        put!("separated_soldiers", unit.separated);
        put!("military_food", w.military.journal.food);
        put!(
            "military_starvation_h",
            hours(w.military.journal.soldier_starvation_ms)
        );
        put!(
            "civilian_starvation_h",
            hours(w.military.journal.civilian_starvation_ms)
        );
        put!(
            "military_queue_h",
            hours(w.formation.journal.member_queue_ms)
        );
        put!(
            "civilian_queue_h",
            hours(w.formation.journal.civilian_queue_ms)
        );
        put!("stolen", w.combat.journal.stolen);
        put!("engaged", w.combat.journal.engaged.len());
        for k in [
            "hits",
            "incapacitations",
            "lethal_injuries",
            "thefts_completed",
            "strikes_invalidated",
        ] {
            put!(k, w.combat.journal.count(k));
        }
        put!(
            "contact_first_h",
            w.combat
                .journal
                .first_contact
                .map(|t| (t as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        let stock_food = |holder: Option<Holder>, material| {
            w.goods
                .batches
                .iter()
                .filter(|b| {
                    b.origin == self.target_origin
                        && b.material == material
                        && holder.as_ref().is_none_or(|h| b.holder == *h)
                })
                .map(|b| b.quantity)
                .sum::<u64>()
        };
        let food = stock_food(None, Material::Food);
        let waste = stock_food(None, Material::Waste);
        put!("target_food_all_custodies", food);
        put!(
            "target_food_at_stock",
            stock_food(Some(Holder::Commons(TARGET)), Material::Food)
        );
        put!("target_food_waste", waste);
        put!("target_food_consumed", 200 - food - waste);
        put!(
            "target_food_raider_custody",
            (9..12)
                .map(|id| stock_food(Some(Holder::Agent(id)), Material::Food))
                .sum::<u64>()
        );
        put!("care_food", w.household.journal.count("food_transferred"));
        put!("care_travel_h", hours(w.household.journal.care_travel_ms));
        put!("care_queue_h", hours(w.household.journal.care_queue_ms));
        put!(
            "household_separation_h",
            hours(w.household.journal.separation_ms)
        );
        put!("caregiver_losses", w.household.journal.losses.len());
        put!("residence_changes", w.household.journal.residence_changes);
        put!("appraisals", w.social.journal.appraisals.len());
        put!(
            "relationship_edges",
            w.social
                .memories
                .iter()
                .map(|m| m.relationships.len())
                .sum::<usize>()
        );
        put!("maturations", w.lifecycle.journal.count("maturations"));
        put!(
            "unsupported_dependent_h",
            hours(w.lifecycle.journal.unsupported_ms)
        );
        for (name, ids) in [
            ("initial_caregivers", vec![3, 4]),
            ("initial_dependents", vec![16, 17]),
            ("raiders", (9..12).collect()),
            (
                "other_initial_people",
                (0..18)
                    .filter(|id| ![3, 4, 9, 10, 11, 16, 17].contains(id))
                    .collect(),
            ),
        ] {
            put!(
                format!("{name}_food"),
                ids.iter().map(|&id| w.agents[id].consumed).sum::<u64>()
            );
            put!(
                format!("{name}_deaths"),
                ids.iter()
                    .filter(|&&id| w.agents[id].action == Action::Dead)
                    .count()
            );
            put!(
                format!("{name}_starvation_h"),
                hours(
                    ids.iter()
                        .map(|id| w
                            .household
                            .journal
                            .starvation_ms
                            .get(id)
                            .copied()
                            .unwrap_or(0))
                        .sum()
                )
            );
            for tag in [
                "farming",
                "construction",
                "travel",
                "queue",
                "military_duty",
                "combat",
                "raid",
                "gather",
                "sleep",
                "idle",
            ] {
                put!(
                    format!("{name}_{tag}_h"),
                    hours(
                        w.household
                            .journal
                            .activity_ms
                            .iter()
                            .filter(|((id, t), _)| ids.contains(id) && t == tag)
                            .map(|(_, q)| *q)
                            .sum()
                    )
                );
            }
        }
        put!("state_hash", format!("{:016x}", w.state_hash()));
        put!("event_hash", format!("{:016x}", w.event_hash()));
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commission_changes_authorization_without_free_goods_labor_or_remote_reports() {
        let pending = fixture("alternate", false, 0).unwrap();
        let commissioned = fixture("alternate", true, 0).unwrap();
        let a = &pending.world;
        let b = &commissioned.world;
        assert_eq!(a.config, b.config);
        for (left, right) in a.agents.iter().zip(&b.agents) {
            let mut left = left.clone();
            let mut right = right.clone();
            // The legal commission adds causal event provenance, not body,
            // knowledge, inventory, RNG, goal or current-action differences.
            left.last_event = None;
            right.last_event = None;
            assert_eq!(left, right);
        }
        assert_eq!(a.information.memories, b.information.memories);
        assert_eq!(a.social.memories, b.social.memories);
        assert_eq!(a.lifecycle.people, b.lifecycle.people);
        assert_eq!(a.goods.batches, b.goods.batches);
        assert_eq!(a.crossing_projects(), b.crossing_projects());
        assert_eq!(a.offices(), b.offices());
        assert!(a.commissions().is_empty());
        assert_eq!(b.commissions().len(), 1);
        assert_eq!(b.commissions()[0].issuer, STEWARD);
        assert!(b.office_active(0));
        assert_eq!(b.effective_edge(1).capacity, 0);
        assert_eq!(b.crossing_projects()[0].wood, 0);
        assert!(b.information_memory(STEWARD).unwrap().reports.is_empty());
        assert_eq!(
            b.information.memories[COURIER].reports[&commissioned.report]
                .report
                .observation,
            b.node_view(TARGET)
        );
        for id in 0..18 {
            assert_eq!(
                b.transport_research.actors.as_ref().unwrap().contains(&id),
                id != STEWARD
            );
            if id != COURIER {
                assert!(b.agents[id].memory.nodes[&TARGET].food.is_empty());
                assert_eq!(b.agents[id].memory.nodes[&TARGET].observed_at, -HOUR);
            }
        }
    }

    #[test]
    fn controlled_command_and_release_boundaries_are_partition_invariant() {
        let mut direct = fixture("alternate", true, 7).unwrap();
        let mut irregular = fixture("alternate", true, 7).unwrap();
        direct.advance_to(7 * HOUR).unwrap();
        for t in [
            17 * MINUTE + 1,
            PROTECT_AT - 1,
            PROTECT_AT + 1,
            RELEASE_AT - 1,
            RELEASE_AT + 1,
            7 * HOUR,
        ] {
            irregular.advance_to(t).unwrap();
        }
        if direct.world.state_hash() != irregular.world.state_hash() {
            for (n, (a, b)) in direct
                .world
                .events()
                .iter()
                .zip(irregular.world.events())
                .enumerate()
            {
                if a != b {
                    eprintln!("first event divergence index={n}\ndirect={a:?}\nirregular={b:?}");
                    break;
                }
            }
            for (a, b) in direct.world.agents.iter().zip(&irregular.world.agents) {
                if a.location != b.location
                    || a.action != b.action
                    || a.nutrition != b.nutrition
                    || a.fatigue != b.fatigue
                {
                    eprintln!(
                        "person={} direct={:?}/{:?}/{}/{} irregular={:?}/{:?}/{}/{}",
                        a.id,
                        a.location,
                        a.action,
                        a.nutrition,
                        a.fatigue,
                        b.location,
                        b.action,
                        b.nutrition,
                        b.fatigue
                    );
                }
            }
        }
        assert_eq!(direct.world.state_hash(), irregular.world.state_hash());
        assert_eq!(direct.world.event_hash(), irregular.world.event_hash());
        assert_eq!(direct.protection_receipts, irregular.protection_receipts);
        assert!(direct.world.transport_research.actors.is_none());
    }
}
