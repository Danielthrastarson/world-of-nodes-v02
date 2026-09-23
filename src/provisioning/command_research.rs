//! F13B controlled command-chain comparisons. Never a policy knowledge source.
use super::*;
pub const CASES: &[&str] = &[
    "local",
    "distant",
    "late",
    "stale",
    "revocation",
    "death",
    "scope",
    "resource",
    "refusal",
    "integrated",
];
enum Simulation {
    Small(World),
    Integrated(kingdom_research::Run),
}
pub struct Run {
    sim: Simulation,
    pub case: String,
    pub ordered: bool,
    pub seed: u64,
    pub end: Time,
    issue_at: Time,
    issued: bool,
    pub order: Option<u64>,
    packet_report: Option<u64>,
    revoke_packet: Option<(u64, u64)>,
    revoke_sent: bool,
    refusal_done: bool,
    released: bool,
    pub first_receipt: Option<Time>,
    courier: usize,
}
impl Run {
    pub fn world(&self) -> &World {
        match &self.sim {
            Simulation::Small(w) => w,
            Simulation::Integrated(r) => &r.world,
        }
    }
    fn world_mut(&mut self) -> &mut World {
        match &mut self.sim {
            Simulation::Small(w) => w,
            Simulation::Integrated(r) => &mut r.world,
        }
    }
    fn advance_kernel(&mut self, t: Time) -> Result<(), String> {
        match &mut self.sim {
            Simulation::Small(w) => w.advance_to(t),
            Simulation::Integrated(r) => r.advance_to(t),
        }
    }
    pub fn advance_to(&mut self, to: Time) -> Result<(), String> {
        if to < self.world().now || to > self.end {
            return Err("outside registered command horizon".into());
        }
        while self.world().now < to {
            let next = (self.world().now / MINUTE * MINUTE + MINUTE).min(to);
            self.advance_kernel(next)?;
            // Research actions occur only on fixed clock boundaries, never caller checkpoints.
            if next % MINUTE != 0 {
                continue;
            }
            if !self.issued && next >= self.issue_at {
                self.issue()?;
                self.issued = true;
            }
            if self.issued {
                self.after_receipt()?;
            }
            if self.case == "revocation" && !self.revoke_sent && next >= self.issue_at + HOUR {
                let report = {
                    let w = self.world_mut();
                    let report = w.capture_report(0)?;
                    w.receive_report(0, 3, report)?;
                    report
                };
                if let Some(order) = self.order {
                    let w = self.world_mut();
                    let msg = w.issue_revocation(0, order)?;
                    w.receive_command(0, 3, msg)?;
                    w.begin_command_delivery(3, msg, 1, 1)?;
                    self.revoke_packet = Some((msg, report));
                } else {
                    let w = self.world_mut();
                    w.begin_report_delivery(3, report, 1, 1)?;
                }
                self.world_mut()
                    .transport_research
                    .actors
                    .as_mut()
                    .unwrap()
                    .insert(3);
                self.revoke_sent = true;
            }
            if self.case != "integrated" && !self.released && next >= 20 * HOUR {
                self.world_mut().transport_research.actors = None;
                self.released = true;
            }
            self.world().audit()?;
        }
        Ok(())
    }
    fn issue(&mut self) -> Result<(), String> {
        let courier = self.courier;
        let revocation = self.case == "revocation";
        let objective = if self.case == "scope" {
            commands::Objective::Unsupported {
                label: "seize_private_food".into(),
            }
        } else {
            commands::Objective::CommissionCrossing {
                project: 0,
                settlement: 1,
            }
        };
        let deadline = if self.case == "late" {
            self.world().now + HOUR
        } else {
            self.end
        };
        let report = {
            let receiver = if revocation { 1 } else { courier };
            let w = self.world_mut();
            let report = w.capture_report(0)?;
            w.receive_report(0, receiver, report)?;
            report
        };
        self.packet_report = Some(report);
        if self.ordered {
            let w = self.world_mut();
            let order = w.issue_order(0, 0, objective, deadline)?;
            if revocation {
                w.receive_command(0, 1, order)?;
            } else {
                w.receive_command(0, courier, order)?;
                w.begin_command_delivery(courier, order, 1, 1)?;
            }
            self.order = Some(order);
        } else {
            let w = self.world_mut();
            if !revocation {
                w.begin_report_delivery(courier, report, 1, 1)?;
            }
        }
        let w = self.world_mut();
        if let Some(actors) = &mut w.transport_research.actors {
            actors.insert(if revocation { 1 } else { courier });
        }
        Ok(())
    }
    fn after_receipt(&mut self) -> Result<(), String> {
        let w = self.world();
        let receipt_time = if self.ordered {
            self.order.and_then(|id| {
                w.command_memory(1)?
                    .envelopes
                    .get(&id)
                    .map(|r| r.received_at)
            })
        } else {
            self.packet_report.and_then(|id| {
                w.information_memory(1)?
                    .reports
                    .get(&id)
                    .map(|r| r.received_at)
            })
        };
        if receipt_time.is_some() && self.first_receipt.is_none() {
            if self.ordered && self.case != "revocation" {
                let report = self.packet_report.unwrap();
                let courier = self.courier;
                self.world_mut().receive_report(courier, 1, report)?;
            }
            self.first_receipt = receipt_time;
            if self.case == "refusal" && self.ordered && !self.refusal_done {
                let order = self.order.unwrap();
                self.world_mut().refuse_order(
                    1,
                    order,
                    "CONTROLLED_PERSON_CHOICE_DECLINES_ACCEPTED_DUTY",
                )?;
                self.refusal_done = true;
            }
            if self.case != "integrated" && self.case != "revocation" {
                self.world_mut().transport_research.actors = None;
                self.released = true;
            }
        }
        if let Some((message, report)) = self.revoke_packet
            && self
                .world()
                .command_memory(1)
                .unwrap()
                .envelopes
                .contains_key(&message)
            && !self
                .world()
                .information_memory(1)
                .unwrap()
                .reports
                .contains_key(&report)
        {
            self.world_mut().receive_report(3, 1, report)?;
        }
        Ok(())
    }
    pub fn parameters(&self) -> String {
        format!(
            "case={}; ordered={}; seed={}; horizon={}; issue_at={}; courier={}; config={:?}\nControlled high-level issue; explicit local office acceptance; prior map visits; no route/food truth refreshed. Both arms carry identical dated factual reports; order arm additionally carries scoped order. Packet handoffs occur at physical contact before next recipient policy opportunity. Refusal is a controlled Person choice, not claimed emergent psychology.\n",
            self.case,
            self.ordered,
            self.seed,
            self.end,
            self.issue_at,
            self.courier,
            self.world().config
        )
    }
    pub fn metrics(&self) -> Result<BTreeMap<String, String>, String> {
        let w = self.world();
        w.audit()?;
        let mut r = BTreeMap::new();
        macro_rules! put {
            ($k:expr,$v:expr) => {
                r.insert($k.to_string(), format!("{}", $v));
            };
        }
        put!("case", self.case);
        put!("arm", if self.ordered { "order" } else { "control" });
        put!("seed", self.seed);
        put!("hour", w.now as f64 / HOUR as f64);
        put!("audit_ok", 1);
        put!("overtakes", w.metrics.overtakes);
        put!("state_hash", format!("{:016x}", w.state_hash()));
        put!("event_hash", format!("{:016x}", w.event_hash()));
        put!("persons", w.agents.len());
        put!("deaths", w.metrics.deaths);
        put!("consumed", w.metrics.consumed);
        put!("spoiled", w.metrics.spoiled);
        put!("delivered", w.metrics.delivered);
        put!("starvation_h", w.metrics.deficit_ms as f64 / HOUR as f64);
        put!("queue_h", w.metrics.queue_ms as f64 / HOUR as f64);
        put!("commissioned", w.commissions().len());
        let p = &w.crossing_projects()[0];
        put!("constructed", usize::from(p.completed_at.is_some()));
        put!("wood_committed", p.wood);
        put!(
            "construction_labor_h",
            p.labor_ms.values().sum::<u128>() as f64 / HOUR as f64
        );
        put!(
            "improved_trips",
            w.crossing_trips()
                .iter()
                .filter(|t| t.improved && t.arrived.is_some())
                .count()
        );
        put!(
            "farm_output",
            w.farming_counters().get("produced").copied().unwrap_or(0)
        );
        put!("food_stolen", w.combat_journal().stolen);
        put!("care_food", w.care_journal().count("food_transferred"));
        put!(
            "first_receipt_h",
            self.first_receipt
                .map(|t| (t as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        put!(
            "information_age_h",
            self.first_receipt
                .map(|t| ((t - self.issue_at) as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        let courier_arrival = w.events().iter().find(|e| {
            e.at >= self.issue_at
                && e.agent == Some(self.courier)
                && e.kind == "Arrived"
                && e.detail.starts_with("node=1 ")
        });
        put!(
            "courier_arrival_h",
            courier_arrival
                .map(|e| (e.at as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        put!(
            "arrival_without_handoff",
            usize::from(courier_arrival.is_some() && self.first_receipt.is_none())
        );
        let j = w.command_journal();
        for key in [
            "orders_issued",
            "orders_received",
            "orders_valid",
            "orders_invalid",
            "obligations_formed",
            "revocations_sent",
            "revocations_received",
            "orders_transmitted",
            "messages_transmitted",
            "revocations_transmitted",
            "orders_expired_at_receipt",
            "orders_revoked_at_receipt",
            "obligations_paused",
            "obligations_active",
            "deliveries_started",
            "deliveries_completed",
            "deliveries_cancelled",
        ] {
            put!(key, j.count(key));
        }
        for status in [
            commands::Status::Received,
            commands::Status::Active,
            commands::Status::Paused,
            commands::Status::Completed,
            commands::Status::Failed,
            commands::Status::Expired,
            commands::Status::Refused,
            commands::Status::Revoked,
        ] {
            put!(
                format!("obligation_{status:?}"),
                w.command_memory(1).map_or(0, |m| m
                    .obligations
                    .values()
                    .filter(|o| o.status == status)
                    .count())
            );
        }
        let first = |status| {
            j.transitions
                .iter()
                .find(|x| x.person == 1 && x.status == status)
                .map(|x| x.at)
        };
        let active = first(commands::Status::Active);
        let completed = first(commands::Status::Completed);
        put!(
            "first_undertaking_h",
            active
                .map(|t| (t as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        put!(
            "receipt_to_undertaking_h",
            active
                .zip(self.first_receipt)
                .map(|(t, r)| ((t - r) as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        put!(
            "duty_completed_h",
            completed
                .map(|t| (t as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        let revoke = w.command_memory(1).and_then(|m| {
            m.envelopes
                .values()
                .find(|r| matches!(r.envelope, commands::Envelope::Revoke(_)))
        });
        put!(
            "revocation_receipt_h",
            revoke
                .map(|r| (r.received_at as f64 / HOUR as f64).to_string())
                .unwrap_or_default()
        );
        put!(
            "revocation_age_h",
            revoke
                .map(
                    |r| ((r.received_at - r.envelope.issued_at()) as f64 / HOUR as f64).to_string()
                )
                .unwrap_or_default()
        );
        put!(
            "commission_before_revocation",
            usize::from(
                completed
                    .zip(revoke)
                    .is_some_and(|(t, r)| t < r.received_at)
            )
        );
        put!(
            "terminal_reason",
            w.command_memory(1)
                .and_then(|m| self.order.and_then(|id| m.obligations.get(&id)))
                .map(|o| o.reason.replace(',', ";"))
                .unwrap_or_default()
        );
        put!("issuer_alive", w.agents[0].action != Action::Dead);
        put!("steward_alive", w.agents[1].action != Action::Dead);
        put!(
            "farm_labor_h",
            w.farming_counters().get("labor_ms").copied().unwrap_or(0) as f64 / HOUR as f64
        );
        put!(
            "caregiver_starvation_h",
            [3usize, 4]
                .iter()
                .map(|i| w.care_journal().starvation_ms.get(i).copied().unwrap_or(0))
                .sum::<u128>() as f64
                / HOUR as f64
        );
        put!(
            "initial_dependents_starvation_h",
            [16usize, 17]
                .iter()
                .map(|i| w.care_journal().starvation_ms.get(i).copied().unwrap_or(0))
                .sum::<u128>() as f64
                / HOUR as f64
        );
        put!(
            "project_outcome",
            if p.completed_at.is_some() {
                "COMPLETED"
            } else if w.commissions().is_empty() {
                "NOT_COMMISSIONED"
            } else if p.wood < p.site.wood_cost {
                "INSUFFICIENT_COMMITTED_MATERIAL"
            } else {
                "INCOMPLETE_LABOR"
            }
        );
        Ok(r)
    }
}

pub fn fixture(case: &str, ordered: bool, seed: u64) -> Result<Run, String> {
    if !CASES.contains(&case) {
        return Err("unregistered command case".into());
    }
    let integrated = case == "integrated";
    let long = matches!(case, "distant" | "late" | "stale" | "death" | "revocation");
    let duration = if long { 8 * HOUR } else { HOUR / 4 };
    let mut sim = if integrated {
        Simulation::Integrated(kingdom_research::command_fixture(seed)?)
    } else {
        let c = Config {
            seed,
            trace: true,
            information: true,
            commands: true,
            nodes: 4,
            starts: vec![1; 4],
            initial_nutrition: 24 * HOUR,
            food: vec![
                (1, 24, 0, 48 * HOUR),
                (
                    2,
                    120,
                    0,
                    if case == "stale" {
                        10 * HOUR
                    } else {
                        48 * HOUR
                    },
                ),
            ],
            boards: vec![1],
            edges: vec![
                Edge {
                    a: 0,
                    b: 1,
                    nominal_ms: duration,
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
                    nominal_ms: 2 * HOUR,
                    capacity: 4,
                },
                Edge {
                    a: 3,
                    b: 2,
                    nominal_ms: 2 * HOUR,
                    capacity: 4,
                },
            ],
            authority: Some(authority::Config {
                polity_id: 17,
                sovereign: 0,
                settlements: vec![1],
            }),
            infrastructure: Some(infrastructure::Config {
                enabled: true,
                wood: vec![(1, if case == "resource" { 5 } else { 100 })],
                sites: vec![infrastructure::Site {
                    edge: 1,
                    node: 1,
                    wood_cost: 20,
                    work_ms: 8 * HOUR,
                    opened_nominal_ms: HOUR / 4,
                    opened_capacity: 4,
                }],
            }),
            household: Some(household::Config {
                enabled: true,
                ..Default::default()
            }),
            farming: Some(farming::Config {
                units: vec![1, 2],
                rules: Default::default(),
            }),
            ..Default::default()
        };
        let mut w = World::new(c)?;
        for id in 0..4 {
            w.goods.add(
                Material::Food,
                6,
                Owner::Agent(id),
                Holder::Agent(id),
                0,
                48 * HOUR,
            );
        }
        for node in 0..4 {
            let mut map = w.node_view(node);
            map.observed_at = -HOUR;
            map.food.clear();
            map.people.clear();
            map.board = None;
            for a in &mut w.agents {
                a.memory.nodes.insert(node, map.clone());
            }
        }
        w.apply_intents(vec![
            (
                0,
                Intent::AppointSteward {
                    candidate: 1,
                    settlement: 1,
                },
            ),
            (1, Intent::AcceptOffice { office: 0 }),
        ])?;
        if !w.office_active(0) {
            return Err("controlled explicit office acceptance failed".into());
        }
        if case == "death" {
            w.agents[1].nutrition = 0;
            w.agents[1].starvation = 23 * HOUR;
        }
        Simulation::Small(w)
    };
    let w = match &mut sim {
        Simulation::Small(w) => w,
        Simulation::Integrated(r) => &mut r.world,
    };
    let courier = if integrated { 5 } else { 2 };
    w.observe_command_sites(0)?;
    let departing: Vec<usize> = if case == "revocation" {
        vec![0, 1, 2, 3]
    } else {
        vec![0, courier]
    };
    let actors = if integrated {
        (0..w.agents.len())
            .filter(|id| ![0, 1, courier].contains(id))
            .collect()
    } else {
        BTreeSet::new()
    };
    w.transport_research.actors = Some(actors);
    w.log(None,"ResearchCommandConditions",format!("F13B {case}: explicit consensual office; controlled sovereign/courier trip to node0; retained prior map observations; no remote command on issuance; hold recipient physiology active until contact (integrated fixed6h); controlled refusal only in refusal case"));
    w.apply_intents(
        departing
            .into_iter()
            .map(|id| {
                (
                    id,
                    Intent::Travel {
                        edge: 0,
                        target: 0,
                        food: false,
                    },
                )
            })
            .collect(),
    )?;
    w.audit()?;
    Ok(Run {
        sim,
        case: case.into(),
        ordered,
        seed,
        end: if integrated { 72 * HOUR } else { 24 * HOUR },
        issue_at: if integrated { HOUR } else { duration + MINUTE },
        issued: false,
        order: None,
        packet_report: None,
        revoke_packet: None,
        revoke_sent: false,
        refusal_done: false,
        released: false,
        first_receipt: None,
        courier,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matched_packets_supply_same_facts_before_role_choice() {
        for case in ["local", "distant", "late", "resource"] {
            let mut a = fixture(case, false, 0).unwrap();
            let mut b = fixture(case, true, 0).unwrap();
            for minute in 1..24 * 60 {
                a.advance_to(minute * MINUTE).unwrap();
                b.advance_to(minute * MINUTE).unwrap();
                if a.first_receipt.is_some() || b.first_receipt.is_some() {
                    assert_eq!(a.first_receipt, b.first_receipt, "{case}");
                    assert_eq!(
                        a.world().agents[1].memory,
                        b.world().agents[1].memory,
                        "{case}"
                    );
                    assert_eq!(
                        a.world().agents[1].nutrition,
                        b.world().agents[1].nutrition,
                        "{case}"
                    );
                    assert_eq!(
                        a.world().agents[1].fatigue,
                        b.world().agents[1].fatigue,
                        "{case}"
                    );
                    assert_eq!(
                        a.world().agents[1].location,
                        b.world().agents[1].location,
                        "{case}"
                    );
                    break;
                }
            }
            assert_eq!(a.first_receipt, b.first_receipt, "{case}");
            if matches!(case, "local" | "resource") {
                assert!(a.first_receipt.is_some(), "{case}");
            }
        }
    }
    #[test]
    fn voluntary_service_start_uses_simulation_time_before_any_ruler_order_exists() {
        let mut direct = fixture("integrated", false, 0).unwrap();
        let mut partitioned = fixture("integrated", false, 0).unwrap();
        direct.advance_to(MINUTE).unwrap();
        partitioned.advance_to(1).unwrap();
        partitioned.advance_to(MINUTE).unwrap();
        assert!(direct.order.is_none() && partitioned.order.is_none());
        assert_eq!(
            direct.world().military.memories,
            partitioned.world().military.memories
        );
        assert_eq!(
            direct.world().state_hash(),
            partitioned.world().state_hash()
        );
        assert_eq!(
            direct.world().event_hash(),
            partitioned.world().event_hash()
        );
    }
    #[test]
    fn research_actions_do_not_depend_on_caller_checkpoints() {
        for case in ["local", "revocation", "integrated"] {
            let mut a = fixture(case, true, 0).unwrap();
            let mut b = fixture(case, true, 0).unwrap();
            a.advance_to(a.end).unwrap();
            for t in [
                1,
                MINUTE - 1,
                17 * MINUTE + 1,
                HOUR + 1,
                6 * HOUR + 1,
                9 * HOUR + 1,
                17 * HOUR + 1,
            ] {
                b.advance_to(t).unwrap();
            }
            b.advance_to(b.end).unwrap();
            if a.world().state_hash() != b.world().state_hash() {
                eprintln!(
                    "partition diagnosis case={case}: event lengths {} / {}",
                    a.world().events().len(),
                    b.world().events().len()
                );
                for (i, (x, y)) in a
                    .world()
                    .events()
                    .iter()
                    .zip(b.world().events())
                    .enumerate()
                {
                    if x != y {
                        eprintln!("first event mismatch {i}: A={x:?}\nB={y:?}");
                        for e in a.world().events().iter().skip(i.saturating_sub(4)).take(10) {
                            eprintln!("A {e:?}");
                        }
                        for e in b.world().events().iter().skip(i.saturating_sub(4)).take(10) {
                            eprintln!("B {e:?}");
                        }
                        break;
                    }
                }
                for (x, y) in a.world().agents.iter().zip(&b.world().agents) {
                    if x != y {
                        eprintln!("person mismatch A={x:?}\nB={y:?}");
                    }
                }
            }
            assert_eq!(a.world().state_hash(), b.world().state_hash(), "{case}");
            assert_eq!(a.world().event_hash(), b.world().event_hash(), "{case}");
            assert_eq!(a.metrics().unwrap(), b.metrics().unwrap(), "{case}");
        }
    }
}
