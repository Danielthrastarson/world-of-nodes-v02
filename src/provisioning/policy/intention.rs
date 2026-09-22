//! Pure, research-only arbitration: own decision memory plus permitted observations.
use super::*;
use crate::provisioning::persistence::{Arm, Leg, State, VisitBoardForRequest};

fn normalized(o: &Observation, m: &PolicyMemory) -> PolicyMemory {
    let mut out = m.clone();
    if let (Location::Node(n), Action::Queued { edge, .. }) = (&o.location, &o.action)
        && let Some(v) = out.nodes.get_mut(n)
    {
        for e in &mut v.edges {
            if e.id == *edge {
                e.queued = e.queued.saturating_sub(1);
            }
        }
    }
    out
}
fn material_changes(
    old: &PolicyMemory,
    new: &PolicyMemory,
    created: Time,
    now: Time,
    id: usize,
    q: u64,
) -> BTreeSet<String> {
    let mut changes = BTreeSet::new();
    for (&n, v) in &new.nodes {
        if v.observed_at <= created {
            continue;
        }
        let Some(prior) = old.nodes.get(&n) else {
            changes.insert(format!("NEW_NODE:{n}"));
            continue;
        };
        for e in &v.edges {
            let Some(p) = prior.edges.iter().find(|p| p.id == e.id) else {
                changes.insert(format!("NEW_EDGE:{}", e.id));
                continue;
            };
            if e.edge != p.edge {
                changes.insert(format!("EDGE_CHANGED:{}", e.id));
            }
            if e.edge.capacity > 0 && p.edge.capacity > 0 {
                let a = (p.queued + p.occupied) / p.edge.capacity;
                let b = (e.queued + e.occupied) / e.edge.capacity;
                if a != b {
                    changes.insert(format!("QUEUE_SERVICE:{}:{a}->{b}", e.id));
                }
            }
        }
        if prior
            .edges
            .iter()
            .any(|e| !v.edges.iter().any(|p| p.id == e.id))
        {
            changes.insert(format!("EDGE_REMOVED:{n}"));
        }
        let food = |view: &NodeView| {
            let f: Vec<_> = view
                .food
                .iter()
                .filter(|f| accessible(f, id) && f.expires_at > now)
                .collect();
            (
                f.iter().map(|f| f.quantity).sum::<u64>().min(q),
                f.iter().map(|f| f.available).sum::<u64>().min(q),
                f.iter().map(|f| f.expires_at).collect::<BTreeSet<_>>(),
            )
        };
        if food(prior) != food(v) {
            changes.insert(format!("FOOD_REQUIREMENT_OR_EXPIRY:{n}"));
        }
        let board = |view: &NodeView| {
            view.board
                .as_ref()
                .map(|b| (b.quantity.min(q), b.unclaimed(now).min(q)))
        };
        if board(prior) != board(v) {
            changes.insert(format!("REQUEST_AVAILABILITY:{n}"));
        }
    }
    changes
}
fn visit_target(o: &Observation, m: &PolicyMemory, d: &Decision) -> Option<usize> {
    if !d.reason.contains("visit remembered board to claim locally") {
        return None;
    }
    let target = match d.intent {
        Intent::Travel {
            target,
            food: false,
            ..
        } => target,
        Intent::Wait => o.goal.filter(|(_, food)| !*food)?.0,
        _ => return None,
    };
    m.nodes.get(&target)?.board.as_ref()?;
    Some(target)
}
fn serves(o: &Observation, d: &Decision, board: usize) -> bool {
    matches!(d.intent,Intent::Travel{target,food:false,..} if target==board)
        || (d.intent == Intent::Wait
            && matches!(o.action, Action::Queued { .. })
            && o.goal == Some((board, false)))
}
fn reason(o: &Observation, d: &Decision) -> &'static str {
    if d.intent == Intent::Sleep {
        "CRITICAL_FATIGUE"
    } else if o.nutrition <= 6 * HOUR {
        "CRITICAL_HUNGER"
    } else if matches!(d.intent, Intent::Eat { .. }) {
        "EATING"
    } else if o.nutrition < 12 * HOUR
        && (d.reason.contains("remembered food") || d.reason.contains("observed deposit"))
    {
        "ORDINARY_SELF_PROVISION_OVERRIDE"
    } else if d.reason.contains("observed deposit") || d.reason.contains("remembered food") {
        "PERSONAL_BUFFER_PROVISIONING"
    } else if d.reason.contains("no locally justified work") {
        "NO_JUSTIFIED_CURRENT_ACTION"
    } else {
        "OTHER_POLICY_OVERRIDE"
    }
}
fn valid(
    o: &Observation,
    m: &PolicyMemory,
    p: &VisitBoardForRequest,
) -> Result<Route, &'static str> {
    let r = route(o, m, p.board_node).ok_or("TARGET_UNREACHABLE")?;
    if let Some(step) = r.steps.first() {
        let next = route(o, m, step.to).ok_or("TARGET_UNREACHABLE")?;
        if next.starvation >= 24 * HOUR {
            return Err("PREDICTED_DEATH");
        }
    }
    if let Some(v) = m.nodes.get(&p.board_node)
        && v.observed_at > p.created_at
        && v.board.as_ref().is_none_or(|b| b.quantity == 0)
    {
        return Err("NEW_LOCAL_EVIDENCE");
    }
    let mut edible = false;
    let mut opportunity = false;
    for v in m.nodes.values().filter(|v| v.node != p.board_node) {
        for f in v
            .food
            .iter()
            .filter(|f| f.quantity > 0 && accessible(f, o.agent))
        {
            if f.expires_at > o.now {
                edible = true;
            }
            let mut at_board = o.clone();
            at_board.location = Location::Node(p.board_node);
            at_board.action = Action::Idle;
            let Some(out) = route(&at_board, m, v.node) else {
                continue;
            };
            let nominal: Time = out.steps.iter().map(|s| s.nominal_ms).sum();
            let work = if matches!(f.holder, Holder::Deposit(_)) {
                2000
            } else {
                0
            };
            // Necessary timing condition only; not a guarantee of future acquisition.
            if o.now + r.time + 2 * nominal + work < f.expires_at {
                opportunity = true;
            }
        }
    }
    if !opportunity {
        if !edible && p.changes.iter().any(|c| c.starts_with("FOOD_REQUIREMENT")) {
            return Err("NEW_LOCAL_EVIDENCE");
        }
        return Err("FOOD_TIMING_INVALID");
    }
    Ok(r)
}
pub(crate) fn decide_persistent(
    o: &Observation,
    m: &PolicyMemory,
    rng: &mut u64,
    arm: Arm,
    s: &mut State,
) -> Decision {
    let information = normalized(o, m);
    if let Some(leg) = &mut s.queued_leg {
        leg.changes.extend(material_changes(
            &leg.evidence,
            &information,
            leg.selected_at,
            o.now,
            o.agent,
            leg.quantity,
        ));
    }
    if let Some(i) = s.active {
        let p = &mut s.plans[i];
        p.changes.extend(material_changes(
            &p.evidence,
            &information,
            p.created_at,
            o.now,
            o.agent,
            p.quantity,
        ));
    }
    if let Some(leg) = &mut s.last_leg {
        leg.changes.extend(material_changes(
            &leg.evidence,
            &information,
            leg.selected_at,
            o.now,
            o.agent,
            leg.quantity,
        ));
    }
    let mut candidate_rng = *rng;
    let mut audit = AcceptanceAudit::new(o, m, ForecastMode::Meals);
    let base = decide_impl(
        o,
        m,
        &mut candidate_rng,
        ForecastMode::Meals,
        &mut audit,
        false,
    );
    audit.finish(o, &base);
    let mut d = base.clone();
    let mut used_base = true;
    let mut cancelled = false;
    // An immediately executable legal meal can save the carrier before any trip.
    // Do not erase the goal by forecasting the pre-meal starvation state instead.
    let mut assessment = o.clone();
    if let Intent::Eat { quantity, .. } = base.intent {
        assessment.nutrition += quantity as Time * HOUR;
        assessment.starvation = 0;
    }
    if arm.retains()
        && let Some(i) = s.active
    {
        match valid(&assessment, m, &s.plans[i]) {
            Err(why) => {
                s.end(o, why);
                cancelled = true;
                if arm != Arm::Remembered
                    && (visit_target(o, m, &base).is_some()
                        || (base.intent == Intent::Wait
                            && matches!(o.action, Action::Queued { .. })))
                {
                    d = choice(
                        Intent::CancelAction,
                        format!("invalid board intention: {why}"),
                    );
                    used_base = false;
                }
            }
            Ok(r) => {
                if matches!(arm, Arm::Resumable | Arm::Committed) {
                    let urgent = matches!(base.intent, Intent::Eat { .. } | Intent::Sleep)
                        || o.nutrition <= 6 * HOUR
                        || matches!(o.action, Action::Gather { .. });
                    let ordinary = o.nutrition < 12 * HOUR
                        && sources(o, m).iter().any(|f| f.route.starvation < 24 * HOUR);
                    if !urgent && !(arm == Arm::Resumable && ordinary) {
                        d = toward(
                            o,
                            &r,
                            s.plans[i].board_node,
                            false,
                            "resume remembered board intention",
                        );
                        used_base = false;
                    } else if arm == Arm::Resumable
                        && !urgent
                        && ordinary
                        && matches!(base.intent, Intent::Travel { food: true, .. })
                    {
                        s.end(o, "ORDINARY_SELF_PROVISION_OVERRIDE");
                        cancelled = true;
                    }
                }
            }
        }
    }
    // This arm reads only an uninterrupted route marker, not a retained intention.
    if arm == Arm::SelfPriority
        && let Some(board) = s.route_marker
        && o.goal == Some((board, false))
        && route(o, m, board).is_some_and(|r| r.starvation < 24 * HOUR)
        && o.nutrition > 6 * HOUR
        && o.nutrition < 12 * HOUR
    {
        let mut ablation_rng = *rng;
        let mut ignored = AcceptanceAudit::disabled();
        let alternative = decide_impl(
            o,
            m,
            &mut ablation_rng,
            ForecastMode::Meals,
            &mut ignored,
            true,
        );
        s.record(
            o,
            s.active,
            "ABLATION_ELIGIBLE",
            "ORDINARY_12H_BRANCH_ONLY",
            format!("base={base:?} alternative={alternative:?}"),
        );
        if alternative != base {
            s.record(
                o,
                s.active,
                "ABLATION_CHANGED_DECISION",
                "ORDINARY_12H_BRANCH_ONLY",
                format!("base={base:?} alternative={alternative:?}"),
            );
        }
        d = alternative;
        candidate_rng = ablation_rng;
    }
    if used_base {
        *rng = candidate_rng;
    }
    if let Some(board) = visit_target(o, m, &d) {
        if s.active.is_none() && !cancelled {
            let id = s.plans.len();
            let q = m.nodes[&board]
                .board
                .as_ref()
                .unwrap()
                .unclaimed(o.now)
                .min(o.capacity - o.load);
            let plan = VisitBoardForRequest {
                id,
                request_id: board,
                board_node: board,
                created_at: o.now,
                reason: d.reason.clone(),
                quantity: q,
                paused: false,
                reached_x: false,
                status: "ACTIVE".into(),
                evidence: information.clone(),
                changes: BTreeSet::new(),
                created_nutrition: o.nutrition,
                created_fatigue: o.fatigue,
            };
            s.plans.push(plan);
            s.active = Some(id);
            s.record(
                o,
                Some(id),
                "INTENT_CREATED",
                "VOLUNTARY_BOARD_DECISION",
                format!("plan={:?} forecast={audit:?}", s.plans[id]),
            );
        }
        s.route_marker = Some(board);
    }
    if let Some(i) = s.active {
        let serving = serves(o, &d, s.plans[i].board_node);
        if serving && s.plans[i].paused {
            s.plans[i].paused = false;
            s.plans[i].status = "ACTIVE".into();
            s.record(
                o,
                Some(i),
                "INTENT_RESUMED",
                "BOARD_ROUTE_SELECTED",
                d.reason.clone(),
            );
        } else if !serving && !s.plans[i].paused {
            if arm.retains() {
                s.plans[i].paused = true;
                s.plans[i].status = "PAUSED".into();
                s.record(o, Some(i), "INTENT_PAUSED", reason(o, &d), d.reason.clone());
            } else {
                s.end(o, reason(o, &d));
            }
        }
    }
    if let Some(board) = s.route_marker
        && !serves(o, &d, board)
    {
        s.route_marker = None;
    }
    if let Intent::Travel { edge, target, food } = d.intent
        && let Location::Node(from) = o.location
        && let Some(e) = o
            .local
            .as_ref()
            .and_then(|v| v.edges.iter().find(|e| e.id == edge))
    {
        let to = if e.edge.a == from { e.edge.b } else { e.edge.a };
        if let Some(mut last) = s.last_leg.take() {
            if !last.reversal_counted && last.edge == edge && last.from == to && last.to == from {
                let detail = format!(
                    "edge={edge} from={from} to={to} target={target} food={food} previous_plan={:?} selected_at={} new_information={} changes={:?} decision={d:?}",
                    last.plan,
                    last.selected_at,
                    !last.changes.is_empty(),
                    last.changes
                );
                s.record(
                    o,
                    last.plan,
                    "DIRECTION_REVERSAL",
                    reason(o, &d),
                    detail.clone(),
                );
                if last.changes.is_empty() {
                    s.record(
                        o,
                        last.plan,
                        "UNINFORMED_REVERSAL",
                        reason(o, &d),
                        detail.clone(),
                    );
                }
                if last.plan.is_some() {
                    s.record(
                        o,
                        last.plan,
                        "BOARD_REVERSAL",
                        reason(o, &d),
                        detail.clone(),
                    );
                    if last.changes.is_empty() {
                        s.record(
                            o,
                            last.plan,
                            "UNINFORMED_BOARD_REVERSAL",
                            reason(o, &d),
                            detail,
                        );
                    }
                }
                last.reversal_counted = true;
            }
            s.last_leg = Some(last);
        }
        let plan = s
            .active
            .filter(|&i| target == s.plans[i].board_node && !food);
        let (evidence, selected_at) = plan.map_or((information.clone(), o.now), |i| {
            (s.plans[i].evidence.clone(), s.plans[i].created_at)
        });
        let changes = plan.map_or_else(BTreeSet::new, |i| s.plans[i].changes.clone());
        s.pending_leg = Some(Leg {
            edge,
            from,
            to,
            plan,
            evidence,
            changes,
            selected_at,
            quantity: plan.map_or(o.capacity, |i| s.plans[i].quantity),
            reversal_counted: false,
        });
    }
    if s.active.is_some()
        || visit_target(o, m, &d).is_some()
        || s.plans.iter().any(|p| p.status == "COMPLETED")
    {
        s.record(
            o,
            s.active,
            "DECISION",
            "ARBITRATION",
            format!(
                "baseline={base:?} chosen={d:?} baseline_gate={} local={:?}",
                audit.outcome, o.local
            ),
        );
    }
    s.memory = information;
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provisioning::{Config, World};
    fn input() -> (Observation, PolicyMemory) {
        let w = World::new(Config {
            starts: vec![2],
            initial_nutrition: 18 * HOUR,
            initial_fatigue: 9 * HOUR,
            initial_request: 600,
            ..Config::default()
        })
        .unwrap();
        let mut o = w.observe(0).unwrap();
        o.now = 5 * HOUR;
        let mut m = PolicyMemory::default();
        for n in 0..3 {
            let mut v = w.node_view(n);
            v.observed_at = if n == 2 { o.now } else { 0 };
            m.nodes.insert(n, v);
        }
        o.local = Some(m.nodes[&2].clone());
        (o, m)
    }
    fn begin(arm: Arm) -> (Observation, PolicyMemory, State) {
        let (o, m) = input();
        let mut s = State::default();
        let d = decide_persistent(&o, &m, &mut 0, arm, &mut s);
        assert!(
            matches!(
                d.intent,
                Intent::Travel {
                    target: 0,
                    food: false,
                    ..
                }
            ),
            "{d:?}"
        );
        assert_eq!(s.active, Some(0));
        assert!(o.job.is_none());
        (o, m, s)
    }
    fn at_x(o: &mut Observation, m: &mut PolicyMemory) {
        o.now += 2 * HOUR;
        o.location = Location::Node(1);
        o.goal = Some((0, false));
        o.nutrition = 10 * HOUR;
        o.fatigue = 12 * HOUR;
        m.nodes.get_mut(&1).unwrap().observed_at = o.now;
        o.local = Some(m.nodes[&1].clone());
    }
    #[test]
    fn commitment_distinguishes_ordinary_self_provisioning_from_survival() {
        for arm in [Arm::Remembered, Arm::Resumable, Arm::Committed] {
            let (mut o, mut m, mut s) = begin(arm);
            at_x(&mut o, &mut m);
            let d = decide_persistent(&o, &m, &mut 0, arm, &mut s);
            if arm == Arm::Committed {
                assert!(matches!(d.intent, Intent::Travel { target: 0, .. }));
                o.nutrition = 6 * HOUR;
                let d = decide_persistent(&o, &m, &mut 0, arm, &mut s);
                assert!(matches!(
                    d.intent,
                    Intent::Travel {
                        target: 2,
                        food: true,
                        ..
                    }
                ));
                assert_eq!(s.active, Some(0));
                assert!(s.plans[0].paused);
            } else {
                assert!(matches!(
                    d.intent,
                    Intent::Travel {
                        target: 2,
                        food: true,
                        ..
                    }
                ));
                assert_eq!(s.active.is_some(), arm == Arm::Remembered);
            }
        }
    }
    #[test]
    fn eating_and_sleep_pause_then_resume_without_claiming_or_reserving() {
        for arm in [Arm::Resumable, Arm::Committed] {
            let (mut o, m, mut s) = begin(arm);
            o.goal = Some((0, false));
            o.nutrition = 11 * HOUR;
            o.own.push(FoodView {
                batch: 17,
                quantity: 1,
                available: 1,
                owner: Owner::Agent(0),
                holder: Holder::Agent(0),
                expires_at: 48 * HOUR,
            });
            o.load = 1;
            let d = decide_persistent(&o, &m, &mut 0, arm, &mut s);
            assert!(matches!(d.intent, Intent::Eat { quantity: 1, .. }));
            assert_eq!(s.active, Some(0));
            o.own.clear();
            o.load = 0;
            o.nutrition = 12 * HOUR;
            let d = decide_persistent(&o, &m, &mut 0, arm, &mut s);
            assert!(matches!(d.intent, Intent::Travel { target: 0, .. }));
            o.nutrition = 18 * HOUR;
            o.fatigue = 22 * HOUR;
            assert_eq!(
                decide_persistent(&o, &m, &mut 0, arm, &mut s).intent,
                Intent::Sleep
            );
            assert_eq!(s.active, Some(0));
            o.fatigue = 4 * HOUR;
            o.action = Action::Sleep { until: o.now };
            assert!(matches!(
                decide_persistent(&o, &m, &mut 0, arm, &mut s).intent,
                Intent::Travel { target: 0, .. }
            ));
            assert!(m.nodes[&0].board.as_ref().unwrap().claims.is_empty());
            assert_eq!(s.plans.len(), 1);
            assert_eq!(o.job, None);
        }
    }
    #[test]
    fn commitment_cancels_predicted_death_and_locally_learned_blockage() {
        let (mut o, mut m, mut s) = begin(Arm::Committed);
        at_x(&mut o, &mut m);
        o.nutrition = 0;
        o.starvation = 23 * HOUR;
        decide_persistent(&o, &m, &mut 0, Arm::Committed, &mut s);
        assert_eq!(s.plans[0].status, "CANCELLED:PREDICTED_DEATH");
        let (mut o, mut m, mut s) = begin(Arm::Committed);
        at_x(&mut o, &mut m);
        m.nodes
            .get_mut(&1)
            .unwrap()
            .edges
            .iter_mut()
            .find(|e| e.id == 0)
            .unwrap()
            .edge
            .capacity = 0;
        decide_persistent(&o, &m, &mut 0, Arm::Committed, &mut s);
        assert_eq!(s.plans[0].status, "CANCELLED:TARGET_UNREACHABLE");
    }
    #[test]
    fn self_priority_ablation_has_no_goal_memory_across_meal_interrupt() {
        let (mut o, mut m, mut s) = begin(Arm::SelfPriority);
        at_x(&mut o, &mut m);
        decide_persistent(&o, &m, &mut 0, Arm::SelfPriority, &mut s);
        assert!(s.records.iter().any(|r| r.kind == "ABLATION_ELIGIBLE"));
        o.nutrition = 5 * HOUR;
        assert!(matches!(
            decide_persistent(&o, &m, &mut 0, Arm::SelfPriority, &mut s).intent,
            Intent::Travel {
                target: 2,
                food: true,
                ..
            }
        ));
        assert!(s.route_marker.is_none());
    }
    #[test]
    fn material_information_excludes_clock_and_small_stock_changes_but_includes_queue_service() {
        let (o, m) = input();
        let mut newer = m.clone();
        newer.nodes.get_mut(&2).unwrap().observed_at = o.now + HOUR;
        newer.nodes.get_mut(&2).unwrap().food[0].quantity -= 1;
        assert!(material_changes(&m, &newer, o.now, o.now + HOUR, 0, 10).is_empty());
        newer.nodes.get_mut(&2).unwrap().edges[0].queued = 4;
        assert!(
            material_changes(&m, &newer, o.now, o.now + HOUR, 0, 10)
                .iter()
                .any(|v| v.starts_with("QUEUE_SERVICE"))
        );
        newer.nodes.get_mut(&2).unwrap().food.clear();
        assert!(
            material_changes(&m, &newer, o.now, o.now + HOUR, 0, 10)
                .iter()
                .any(|v| v.starts_with("FOOD_REQUIREMENT"))
        );
        newer.nodes.get_mut(&2).unwrap().edges[0].queued = 0;
        assert!(material_changes(&m, &newer, o.now, 49 * HOUR, 0, 10).is_empty());
    }
    #[test]
    fn intention_cannot_access_changed_remote_truth() {
        let (mut o, mut m, s) = begin(Arm::Committed);
        at_x(&mut o, &mut m);
        let mut a = s.clone();
        let mut b = s;
        // No World is accepted by this API. Identical beliefs remain identical even
        // when a separate world has neither a board nor food.
        let changed = World::new(Config {
            boards: vec![],
            food: vec![],
            ..Config::default()
        })
        .unwrap();
        assert!(changed.boards.is_empty());
        assert_eq!(
            decide_persistent(&o, &m, &mut 0, Arm::Committed, &mut a),
            decide_persistent(&o, &m, &mut 0, Arm::Committed, &mut b)
        );
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }
    #[test]
    fn an_immediate_legal_meal_does_not_erase_a_survivable_intention() {
        let (mut o, m, mut s) = begin(Arm::Committed);
        o.nutrition = 0;
        o.starvation = 23 * HOUR;
        o.load = 1;
        o.own.push(FoodView {
            batch: 17,
            quantity: 1,
            available: 1,
            owner: Owner::Agent(0),
            holder: Holder::Agent(0),
            expires_at: 48 * HOUR,
        });
        assert!(matches!(
            decide_persistent(&o, &m, &mut 0, Arm::Committed, &mut s).intent,
            Intent::Eat { quantity: 1, .. }
        ));
        assert_eq!(s.active, Some(0));
        assert!(s.plans[0].paused);
    }
    #[test]
    fn expired_committed_goal_does_not_leave_a_cancelled_agent_in_its_old_queue() {
        let (mut o, m, mut s) = begin(Arm::Committed);
        o.now = 50 * HOUR;
        o.action = Action::Queued {
            edge: 1,
            to: 1,
            joined: 5 * HOUR,
        };
        o.goal = Some((0, false));
        let d = decide_persistent(&o, &m, &mut 0, Arm::Committed, &mut s);
        assert_eq!(s.plans[0].status, "CANCELLED:FOOD_TIMING_INVALID");
        assert_eq!(d.intent, Intent::CancelAction);
    }
}
