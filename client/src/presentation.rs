//! Pure decision-support projections over permitted presentation knowledge.
//! No World access, graphics context, actor mutation or extra command authority.
use crate::scene::Selection;
use std::collections::{BTreeSet, VecDeque};
use world_of_nodes::provisioning::playable::snapshot::{
    Evidence, KnowledgeSource, OrderSnapshot, PersonSnapshot,
};
use world_of_nodes::provisioning::playable::{MilitaryOrder, Snapshot, ViewMode};
use world_of_nodes::provisioning::{HOUR, Location, MINUTE, Owner, Time};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verb {
    Observe,
    Order,
    Govern,
    Allocate,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionKind {
    Inspect,
    Reports,
    CaptureReport,
    Help,
    Commission { project: Option<usize> },
    Revoke { command: Option<u64> },
    Unit(MilitaryOrder),
    Unavailable,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextAction {
    pub slot: u8,
    pub label: String,
    pub kind: ActionKind,
    pub enabled: bool,
    pub reason: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Urgent,
    Caution,
    Information,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Concern {
    pub title: String,
    pub detail: String,
    pub tone: Tone,
    pub evidence: String,
    pub target: Option<Selection>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub heading: String,
    pub lines: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Panel {
    pub title: String,
    pub subtitle: String,
    pub sections: Vec<Section>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepState {
    Known,
    Unknown,
    Pending,
    Negative,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StageStep {
    pub label: String,
    pub state: StepState,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderStageView {
    pub label: String,
    pub detail: String,
    pub steps: Vec<StageStep>,
}

pub fn age_label(ms: Time) -> String {
    let ms = ms.max(0);
    if ms < MINUTE {
        "under 1m".into()
    } else if ms < HOUR {
        format!("{}m", ms / MINUTE)
    } else if ms % HOUR == 0 {
        format!("{}h", ms / HOUR)
    } else {
        format!("{}h {}m", ms / HOUR, (ms % HOUR) / MINUTE)
    }
}
pub fn evidence_label(e: &Evidence, now: Time) -> String {
    match e.source {
        KnowledgeSource::Local => "Observed locally now".into(),
        KnowledgeSource::SelfKnowledge => "Personally known".into(),
        KnowledgeSource::Debug => "DEBUG truth".into(),
        KnowledgeSource::Memory => format!("Seen {} ago", age_label(e.age(now))),
        KnowledgeSource::Report => format!("Report {} old", age_label(e.age(now))),
    }
}
fn current(e: &Evidence) -> bool {
    matches!(
        e.source,
        KnowledgeSource::Local | KnowledgeSource::SelfKnowledge | KnowledgeSource::Debug
    )
}
fn viewer(s: &Snapshot) -> Option<&PersonSnapshot> {
    s.persons.iter().find(|p| p.id == s.viewer)
}
fn own_node(s: &Snapshot) -> Option<usize> {
    viewer(s).and_then(|p| match p.location {
        Location::Node(n) => Some(n),
        _ => None,
    })
}
fn body_block(s: &Snapshot) -> Option<String> {
    let Some(p) = viewer(s) else {
        return Some("Ruler's current state is unavailable".into());
    };
    if p.action.as_deref() == Some("dead") {
        return Some("The ruler is dead".into());
    }
    if !current(&p.evidence) {
        return Some("Ruler's current position is unknown".into());
    }
    if !matches!(p.location, Location::Node(_)) {
        return Some("Arrive at a node to issue a local instruction".into());
    }
    None
}
fn survival_caution(s: &Snapshot) -> Option<String> {
    let p = viewer(s)?;
    if p.nutrition.is_some_and(|n| n <= 6 * HOUR) {
        Some("Critical hunger may interrupt physical execution".into())
    } else if p.fatigue.is_some_and(|f| f >= 22 * HOUR) {
        Some("Severe fatigue may delay physical execution".into())
    } else {
        None
    }
}
fn selected_crossing(s: &Snapshot, selected: Option<Selection>) -> Option<usize> {
    match selected {
        Some(Selection::Crossing(id)) => s.crossings.iter().find(|p| p.id == id).map(|p| p.id),
        Some(Selection::Node(n)) => s.crossings.iter().find(|p| p.node == n).map(|p| p.id),
        Some(Selection::Edge(e)) => s.crossings.iter().find(|p| p.edge == Some(e)).map(|p| p.id),
        _ => None,
    }
}
fn action(
    slot: u8,
    label: &str,
    kind: ActionKind,
    blocked: Option<String>,
    detail: &str,
) -> ContextAction {
    ContextAction {
        slot,
        label: label.into(),
        kind,
        enabled: blocked.is_none(),
        reason: blocked.unwrap_or_else(|| detail.into()),
    }
}

/// Availability means "may compose/attempt". The authoritative session validates
/// rights, command scope and physical contact again at confirmation.
pub fn contextual_actions(
    s: &Snapshot,
    selected: Option<Selection>,
    verb: Verb,
) -> Vec<ContextAction> {
    match verb {
        Verb::Observe => vec![
            action(
                1,
                "Inspect",
                ActionKind::Inspect,
                selected.is_none().then(|| "Select a known object".into()),
                "Inspect only currently known evidence",
            ),
            action(
                2,
                "Reports",
                ActionKind::Reports,
                None,
                "Read carried reports with their original dates",
            ),
            action(
                3,
                "Record observation",
                ActionKind::CaptureReport,
                body_block(s),
                "Record what the ruler can observe here; no remote query",
            ),
            action(
                4,
                "Controls",
                ActionKind::Help,
                None,
                "Stable verbs: Q Observe, W Order, E Govern, R Allocate",
            ),
        ],
        Verb::Order if matches!(selected, Some(Selection::Unit(_))) => {
            let Some(Selection::Unit(unit)) = selected else {
                unreachable!()
            };
            let block = body_block(s).or_else(|| {
                viewer(s)
                    .filter(|p| p.unit == Some(unit))
                    .is_none()
                    .then(|| "Ruler holds no active service role in this unit".into())
            });
            let note = survival_caution(s).unwrap_or_else(|| {
                "Only present members hear this order; absent members are not updated".into()
            });
            [
                MilitaryOrder::Move,
                MilitaryOrder::Attack,
                MilitaryOrder::Hold,
                MilitaryOrder::Withdraw,
            ]
            .into_iter()
            .enumerate()
            .map(|(i, order)| {
                let (label, detail) = match order {
                    MilitaryOrder::Move => ("Move", format!("Choose a known node. {note}")),
                    MilitaryOrder::Attack => (
                        "Attack",
                        format!("Approach a known node and engage recognized threats. {note}"),
                    ),
                    MilitaryOrder::Hold => (
                        "Hold here",
                        format!("Protect the commander's current node. {note}"),
                    ),
                    MilitaryOrder::Withdraw => (
                        "Withdraw",
                        format!("Choose a different known node; movement remains physical. {note}"),
                    ),
                };
                action(
                    i as u8 + 1,
                    label,
                    ActionKind::Unit(order),
                    block.clone(),
                    &detail,
                )
            })
            .collect()
        }
        Verb::Order => {
            let crossing = selected_crossing(s, selected);
            let known_any = !s.crossings.is_empty();
            let commission_block = body_block(s)
                .or_else(|| (!known_any).then(|| "No crossing site is personally known".into()));
            let order = match selected {
                Some(Selection::Order(id)) => s.orders.iter().find(|o| {
                    o.id == id && o.issuer == s.viewer && !o.objective.starts_with("Revoke command")
                }),
                _ => None,
            };
            let revoke_block = body_block(s).or_else(|| {
                order
                    .is_none()
                    .then(|| "Select an order issued by this ruler".into())
            });
            vec![
                action(
                    1,
                    "Commission crossing",
                    ActionKind::Commission { project: crossing },
                    commission_block,
                    "Choose a known site; accepted office scope is checked on confirmation",
                ),
                action(
                    2,
                    "Send revocation",
                    ActionKind::Revoke {
                        command: order.map(|o| o.id),
                    },
                    revoke_block,
                    "Revocation must travel; completed work is not undone",
                ),
                action(
                    3,
                    "Other projects",
                    ActionKind::Unavailable,
                    Some("No other public project command exists yet".into()),
                    "",
                ),
                action(
                    4,
                    "Other orders",
                    ActionKind::Unavailable,
                    Some("Arbitrary commands to people are not authorized".into()),
                    "",
                ),
            ]
        }
        Verb::Govern | Verb::Allocate => (1..=4)
            .map(|slot| {
                action(
                    slot,
                    "Not available",
                    ActionKind::Unavailable,
                    Some(
                        if verb == Verb::Govern {
                            "No new law or coercive power is implemented"
                        } else {
                            "No new resource allocation right is implemented"
                        }
                        .into(),
                    ),
                    "",
                )
            })
            .collect(),
    }
}

/// A route through known edges is a conditional possibility, not a guarantee of
/// current capacity, free passage, survival, service duration, or arrival.
pub fn known_route(s: &Snapshot, from: usize, target: usize) -> bool {
    if from == target {
        return true;
    }
    let mut visited = BTreeSet::from([from]);
    let mut queue = VecDeque::from([from]);
    while let Some(node) = queue.pop_front() {
        for edge in s.edges.iter().filter(|e| e.capacity > 0) {
            let other = if edge.a == node {
                edge.b
            } else if edge.b == node {
                edge.a
            } else {
                continue;
            };
            if other == target {
                return true;
            }
            if visited.insert(other) {
                queue.push_back(other);
            }
        }
    }
    false
}

pub fn unit_state(s: &Snapshot, unit: usize) -> String {
    let members: Vec<_> = s
        .persons
        .iter()
        .filter(|p| {
            p.unit == Some(unit) && current(&p.evidence) && p.action.as_deref() != Some("dead")
        })
        .collect();
    if members.is_empty() {
        return "No members currently observed".into();
    }
    let count = |name: &str| {
        members
            .iter()
            .filter(|p| p.action.as_deref() == Some(name))
            .count()
    };
    for (action, label) in [
        ("striking", "In contact"),
        ("queued", "Queued"),
        ("travelling", "Moving"),
        ("sleeping", "Resting"),
    ] {
        let n = count(action);
        if n > 0 {
            return format!("{label} · {n} observed");
        }
    }
    let nodes: BTreeSet<_> = members
        .iter()
        .filter_map(|p| match p.location {
            Location::Node(n) => Some(n),
            _ => None,
        })
        .collect();
    if nodes.len() == 1 {
        let node = *nodes.first().unwrap();
        let objective = s
            .units
            .iter()
            .find(|u| u.id == unit)
            .and_then(|u| u.objective);
        if objective == Some(node) {
            format!("At remembered target · node {node}")
        } else {
            format!("Gathered at node {node}")
        }
    } else {
        format!("{} members in view", members.len())
    }
}

pub fn concerns(s: &Snapshot) -> Vec<Concern> {
    let mut items = Vec::new();
    if let Some(p) = viewer(s) {
        if p.action.as_deref() == Some("dead") {
            items.push(Concern {
                title: "The ruler has died".into(),
                detail:
                    "No living ruler is available to issue new commands; succession is not modeled."
                        .into(),
                tone: Tone::Urgent,
                evidence: evidence_label(&p.evidence, s.now),
                target: Some(Selection::Person(p.id)),
            });
        } else if p.nutrition.is_some_and(|n| n <= 6 * HOUR) {
            items.push(Concern {
                title: "Ruler needs food".into(),
                detail: format!(
                    "{} nutrition remains. Survival can interrupt undertakings.",
                    age_label(p.nutrition.unwrap())
                ),
                tone: Tone::Urgent,
                evidence: evidence_label(&p.evidence, s.now),
                target: Some(Selection::Person(p.id)),
            });
        } else if p.fatigue.is_some_and(|f| f >= 22 * HOUR) {
            items.push(Concern {
                title: "Ruler needs rest".into(),
                detail: "Critical fatigue can delay movement and execution.".into(),
                tone: Tone::Caution,
                evidence: evidence_label(&p.evidence, s.now),
                target: Some(Selection::Person(p.id)),
            });
        }
    }
    if let Some(p) = s.persons.iter().find(|p| {
        p.hostile == Some(true) && current(&p.evidence) && p.action.as_deref() != Some("dead")
    }) {
        items.push(Concern {title:"Hostile person observed".into(),detail:format!("Person {} is locally recognized as hostile. This does not reveal a remote raid plan.",p.id),tone:Tone::Urgent,evidence:evidence_label(&p.evidence,s.now),target:Some(Selection::Person(p.id))});
    }
    if let Some(order) = s
        .orders
        .iter()
        .rev()
        .find(|o| o.issuer == s.viewer && !o.objective.starts_with("Revoke command"))
    {
        let stage = order_stage(s, order);
        if stage
            .steps
            .iter()
            .any(|step| step.state == StepState::Unknown)
        {
            items.push(Concern {
                title: format!(
                    "Order {} · {}",
                    order.id,
                    if stage.steps[2].state == StepState::Unknown {
                        "receipt unknown"
                    } else if stage.steps[3].state == StepState::Unknown {
                        "decision unknown"
                    } else {
                        "inspect progress"
                    }
                ),
                detail: stage.detail,
                tone: Tone::Information,
                evidence: format!("Issued {} ago", age_label(s.now - order.issued_at)),
                target: Some(Selection::Order(order.id)),
            });
        }
    }
    if let Some(node) = s
        .nodes
        .iter()
        .filter(|n| n.contents_known && n.board_quantity.is_some_and(|q| q > 0))
        .min_by_key(|n| {
            (
                !current(&n.evidence),
                n.evidence.observed_at.saturating_neg(),
            )
        })
    {
        items.push(Concern {
            title: format!("Observed request: {} food", node.board_quantity.unwrap()),
            detail: format!("The board at node {} requests food. This is an observed request, not a diagnosis of private hunger.", node.id),
            tone: Tone::Caution, evidence: evidence_label(&node.evidence, s.now), target: Some(Selection::Node(node.id)),
        });
    }
    if let Some(p) = s
        .crossings
        .iter()
        .find(|p| p.completed == Some(false) && p.authorized == Some(true))
    {
        let progress = match (p.wood_committed, p.wood_required) {
            (Some(have), Some(need)) => format!("Observed materials {have}/{need}."),
            _ => "Current material progress is unknown.".into(),
        };
        items.push(Concern {
            title: "Project authorization observed".into(),
            detail: format!("{progress} Construction still requires voluntary work and time."),
            tone: Tone::Information,
            evidence: evidence_label(&p.evidence, s.now),
            target: Some(Selection::Crossing(p.id)),
        });
    } else if let Some(p) = s
        .crossings
        .iter()
        .find(|p| p.completed == Some(false) && p.authorized == Some(false))
    {
        items.push(Concern{title:"Crossing site known".into(),detail:"A scoped commission can request this project. Materials and participation are not guaranteed.".into(),tone:Tone::Information,evidence:evidence_label(&p.evidence,s.now),target:Some(Selection::Crossing(p.id))});
    }
    if let Some(report) = s
        .reports
        .iter()
        .filter(|r| r.holder == s.viewer)
        .max_by_key(|r| r.received_at)
    {
        items.push(Concern {
            title: format!("Dated report · node {}", report.node),
            detail: format!(
                "Author {} saw {} public food quanta. Present quantity may differ.",
                report.author, report.food_quantity
            ),
            tone: Tone::Information,
            evidence: format!("Report {} old", age_label(s.now - report.observed_at)),
            target: Some(Selection::Report(report.id)),
        });
    }
    if !s.orders.iter().any(|o| o.issuer == s.viewer) && !s.crossings.is_empty() {
        items.push(Concern {
            title: "No office order recorded".into(),
            detail: "A known crossing can be commissioned through an accepted office. Unit orders are separate.".into(),
            tone: Tone::Information, evidence: "Own command record".into(),
            target: s.crossings.first().map(|p| Selection::Crossing(p.id)),
        });
    }
    if items.is_empty() {
        items.push(Concern{title:"Information is local".into(),detail:"Select a known place or Person to inspect evidence. No unobserved threat or shortage is inferred.".into(),tone:Tone::Information,evidence:if s.mode==ViewMode::Debug {"DEBUG truth".into()} else {"Current personal knowledge".into()},target:own_node(s).map(Selection::Node)});
    }
    items.truncate(4);
    items
}

pub fn order_stage(s: &Snapshot, order: &OrderSnapshot) -> OrderStageView {
    let status = order.status.as_str();
    let is_revoke = order.objective.starts_with("Revoke command");
    let known_terminal = status.starts_with("Completed:")
        || status.starts_with("Failed:")
        || status.starts_with("Refused:")
        || status.starts_with("Expired:")
        || status.starts_with("Revoked:")
        || status.starts_with("Invalid:");
    let valid_obligation = known_terminal
        || status.starts_with("Received:")
        || status.starts_with("Active:")
        || status.starts_with("Paused:");
    let rejected = status.starts_with("Invalid:");
    let received =
        order.received_at.is_some() || valid_obligation || status == "Revocation received";
    let handed = status.starts_with("Handed to agreed courier");
    let completed = status.starts_with("Completed:");
    let pending = status.starts_with("Active:")
        || status.starts_with("Paused:")
        || status.starts_with("Received:");
    let negative = known_terminal && !completed;
    let deadline = order.expires_at.is_some_and(|t| t <= s.now);
    let label = if rejected {
        "Rejected by recipient"
    } else if completed {
        "Local commission completed"
    } else if status.starts_with("Failed:") {
        "Role obligation failed"
    } else if status.starts_with("Refused:") {
        "Recipient refused"
    } else if status.starts_with("Revoked:") || status == "Revocation received" {
        "Revocation received"
    } else if status.starts_with("Expired:") {
        "Expired at recipient"
    } else if status.starts_with("Paused:") {
        "Role obligation paused"
    } else if status.starts_with("Active:") {
        "Role obligation active"
    } else if received {
        "Received; decision unreported"
    } else if handed {
        "Courier handoff known; receipt unknown"
    } else if deadline {
        "Deadline passed; result unknown"
    } else {
        "Issued; receipt unknown"
    };
    let detail = if completed {
        "The steward authorized the local project. This is not proof of construction or physical success.".into()
    } else if is_revoke {
        "Revocation is another travelling message. It cannot undo already completed work.".into()
    } else if valid_obligation {
        format!(
            "Known recipient state: {status}. Physical project completion remains a separate observation."
        )
    } else if deadline {
        "The known deadline has passed. Without a reply or local observation, do not infer whether the project was attempted or completed earlier.".into()
    } else {
        "Authority creates an obligation after receipt. The recipient's private decision and the physical outcome remain unknown until observed or reported.".into()
    };
    OrderStageView {
        label: label.into(),
        detail,
        steps: vec![
            StageStep {
                label: "Issued".into(),
                state: StepState::Known,
            },
            StageStep {
                label: "Transport / handoff".into(),
                state: if received || handed {
                    StepState::Known
                } else {
                    StepState::Unknown
                },
            },
            StageStep {
                label: "Recipient receipt".into(),
                state: if received {
                    StepState::Known
                } else {
                    StepState::Unknown
                },
            },
            StageStep {
                label: if is_revoke {
                    "Recipient revocation knowledge"
                } else {
                    "Office obligation"
                }
                .into(),
                state: if negative {
                    StepState::Negative
                } else if completed || status == "Revocation received" {
                    StepState::Known
                } else if pending {
                    StepState::Pending
                } else {
                    StepState::Unknown
                },
            },
            StageStep {
                label: "Physical project outcome".into(),
                state: StepState::Unknown,
            },
        ],
    }
}

fn section(heading: &str, lines: Vec<String>) -> Section {
    Section {
        heading: heading.into(),
        lines,
    }
}
fn location(p: &PersonSnapshot) -> String {
    match p.location {
        Location::Node(n) => format!("Node {n}"),
        Location::Edge { from, to, .. } => format!("On route {from} → {to}"),
    }
}
fn owner_label(owner: Owner) -> String {
    match owner {
        Owner::None => "No owner".into(),
        Owner::Agent(id) => format!("Person {id}"),
        Owner::Commons(id) => format!("Commons {id}"),
    }
}
fn missing() -> Panel {
    Panel {
        title: "Not currently known".into(),
        subtitle: "Selection has no record in this view".into(),
        sections: vec![section(
            "Knowledge",
            vec!["No current world truth was fetched to resolve this selection.".into()],
        )],
    }
}

fn evidence_for(s: &Snapshot, selected: Selection) -> Option<&Evidence> {
    match selected {
        Selection::Node(id) => s.nodes.iter().find(|v| v.id == id).map(|v| &v.evidence),
        Selection::Person(id) => s.persons.iter().find(|v| v.id == id).map(|v| &v.evidence),
        Selection::Edge(id) => s.edges.iter().find(|v| v.id == id).map(|v| &v.evidence),
        Selection::Crossing(id) => s.crossings.iter().find(|v| v.id == id).map(|v| &v.evidence),
        Selection::Field(id) => s.fields.iter().find(|v| v.id == id).map(|v| &v.evidence),
        Selection::Goods(id) => s.goods.iter().find(|v| v.id == id).map(|v| &v.evidence),
        Selection::Cart(id) => s.carts.iter().find(|v| v.id == id).map(|v| &v.evidence),
        Selection::Unit(id) => s.units.iter().find(|v| v.id == id).map(|v| &v.evidence),
        Selection::Order(id) => s.orders.iter().find(|v| v.id == id).map(|v| &v.evidence),
        Selection::Report(id) => s
            .reports
            .iter()
            .find(|v| v.id == id && v.holder == s.viewer)
            .or_else(|| s.reports.iter().find(|v| v.id == id))
            .map(|v| &v.evidence),
    }
}

fn timestamp(t: Time) -> String {
    let absolute = t.unsigned_abs();
    format!(
        "T{}{:02}:{:02}:{:02}.{:03}",
        if t < 0 { "−" } else { "+" },
        absolute / HOUR as u64,
        (absolute / MINUTE as u64) % 60,
        (absolute / 1000) % 60,
        absolute % 1000
    )
}

fn evidence_section(e: &Evidence, now: Time) -> Section {
    let mut lines = vec![
        evidence_label(e, now),
        format!("Evidence timestamp: {}", timestamp(e.observed_at)),
    ];
    if let Some(t) = e.received_at {
        lines.push(format!("Received: {}", timestamp(t)));
    }
    if let Some(id) = e.author {
        lines.push(format!("Source Person: {id}"));
    }
    if let Some(id) = e.sender {
        lines.push(format!("Last sender: Person {id}"));
    }
    section("Evidence / time", lines)
}

pub fn selected_panel(s: &Snapshot, selected: Option<Selection>) -> Panel {
    let Some(selected) = selected else {
        return Panel {
            title: "Select the world".into(),
            subtitle: "People, places, projects and dated reports".into(),
            sections: vec![section(
                "Command language",
                vec![
                    "Q Observe · W Order · E Govern · R Allocate".into(),
                    "People execute physical actions; the ruler issues scoped intentions.".into(),
                ],
            )],
        };
    };
    let mut panel = match selected {
        Selection::Person(id) => {
            let Some(p) = s.persons.iter().find(|p| p.id == id) else {
                return missing();
            };
            let mut body = vec![
                location(p),
                format!("Action: {}", p.action.as_deref().unwrap_or("unobserved")),
                format!("Role: {}", p.role.as_deref().unwrap_or("unknown")),
            ];
            if let Some(n) = p.nutrition {
                body.push(format!("Nutrition: {}", age_label(n)));
            }
            if let Some(f) = p.fatigue {
                body.push(format!("Fatigue: {}", age_label(f)));
            }
            if p.nutrition.is_none() {
                body.push("Private needs, goals and inventory are not visible.".into());
            }
            let mut sections = vec![section(
                if current(&p.evidence) {
                    "Observed Person"
                } else {
                    "Last known Person"
                },
                body,
            )];
            if let Some(unit) = p.unit {
                sections.push(section(
                    "Unit",
                    vec![
                        format!("Unit {unit}: {}", unit_state(s, unit)),
                        "Membership does not erase needs, property or mortality.".into(),
                    ],
                ));
            }
            Panel {
                title: format!("Person {id}"),
                subtitle: evidence_label(&p.evidence, s.now),
                sections,
            }
        }
        Selection::Node(id) => {
            let Some(n) = s.nodes.iter().find(|n| n.id == id) else {
                return missing();
            };
            let mut lines = if n.contents_known {
                vec![format!("People observed: {}", n.people_seen)]
            } else {
                vec!["Map location known; people, goods and requests unobserved.".into()]
            };
            if let Some(q) = n.board_quantity {
                lines.push(format!("Request: {q} food"));
            }
            let food: Vec<_> = s
                .goods
                .iter()
                .filter(|g| g.node == Some(id) && g.material == "Food")
                .collect();
            if !food.is_empty() {
                lines.push(format!(
                    "Public food observed: {} quanta",
                    food.iter().map(|g| g.quantity).sum::<u64>()
                ));
            }
            let routes = s
                .edges
                .iter()
                .filter(|e| e.a == id || e.b == id)
                .map(|e| {
                    format!(
                        "To {}: {} nominal, capacity {} · {}",
                        if e.a == id { e.b } else { e.a },
                        age_label(e.nominal_ms),
                        e.capacity,
                        evidence_label(&e.evidence, s.now)
                    )
                })
                .collect();
            Panel {
                title: format!("Node {id}"),
                subtitle: evidence_label(&n.evidence, s.now),
                sections: vec![
                    section("Known situation", lines),
                    section("Known connections", routes),
                    section("Provenance", vec!["Counts describe the stated observation time, not guaranteed current supply.".into()]),
                ],
            }
        }
        Selection::Crossing(id) => {
            let Some(p) = s.crossings.iter().find(|p| p.id == id) else {
                return missing();
            };
            let status = match p.completed {
                Some(true) => "Completed when observed",
                Some(false) if p.authorized == Some(true) => {
                    "Authorized; construction incomplete when observed"
                }
                Some(false) => "Unfinished when observed",
                None => "Site known; progress unobserved",
            };
            let materials = match (p.wood_committed, p.wood_required) {
                (Some(a), Some(b)) => format!("Wood committed: {a}/{b}"),
                _ => "Material progress unknown".into(),
            };
            Panel{title:format!("Crossing {id}"),subtitle:evidence_label(&p.evidence,s.now),sections:vec![section("Project",vec![status.into(),format!("Site: node {}",p.node),materials]),section("What a commission changes",vec!["A valid received order asks the steward to authorize a public objective.".into(),"People must still contribute actual materials, work and time.".into(),"A completed crossing can create capacity competition; no safety bonus is granted.".into()])]}
        }
        Selection::Unit(id) => {
            let unit = s.units.iter().find(|u| u.id == id);
            let observed: Vec<_> = s
                .persons
                .iter()
                .filter(|p| p.unit == Some(id) && current(&p.evidence))
                .collect();
            let mut presence = vec![
                unit_state(s, id),
                format!("Members observed: {}", observed.len()),
            ];
            if let Some(u) = unit {
                if let Some(n) = u.global_member_count {
                    presence.push(format!("DEBUG roster: {n}"));
                }
                if let Some(n) = u.objective {
                    presence.push(format!(
                        "Remembered target: node {n} (receipt date unavailable)"
                    ));
                }
            }
            let mut condition = Vec::new();
            for (key, label) in [
                ("travelling", "Moving"),
                ("queued", "Queued"),
                ("sleeping", "Resting"),
                ("striking", "In contact"),
            ] {
                let count = observed
                    .iter()
                    .filter(|p| p.action.as_deref() == Some(key))
                    .count();
                if count > 0 {
                    condition.push(format!("{label}: {count} observed"));
                }
            }
            if let Some(p) = viewer(s) {
                if let Some(n) = p.nutrition {
                    condition.push(format!("Ruler nutrition: {}", age_label(n)));
                }
                if let Some(f) = p.fatigue {
                    condition.push(format!("Ruler fatigue: {}", age_label(f)));
                }
            }
            condition.push("Other members' private needs are unknown.".into());
            let mut reach = vec![
                body_block(s)
                    .or_else(|| {
                        viewer(s)
                            .filter(|p| p.unit == Some(id))
                            .is_none()
                            .then(|| "Ruler has no active service role in this unit".into())
                    })
                    .unwrap_or_else(|| "Local instruction available to present members.".into()),
            ];
            if let Some(note) = survival_caution(s) {
                reach.push(note);
            }
            if let (Some(from), Some(target)) = (own_node(s), unit.and_then(|u| u.objective)) {
                reach.push(if known_route(s, from, target) {
                    format!("Known route to node {target}; execution is conditional.")
                } else {
                    format!("No traversable route to node {target} in current knowledge.")
                });
            }
            Panel {title:format!("Unit {id}"), subtitle:if s.mode==ViewMode::Debug {"DEBUG organization / physical truth".into()}else{"Local presence, not global readiness".into()},
                sections:vec![section("Presence / objective",presence), section("Observed condition",condition), section("Command reach",reach),
                    section("Limits",vec!["Unseen members and new remote orders are unknown. A known route does not guarantee capacity, supplies or arrival.".into()])]}
        }
        Selection::Order(id) => {
            let Some(o) = s.orders.iter().find(|o| o.id == id) else {
                return missing();
            };
            let stage = order_stage(s, o);
            Panel {
                title: format!("Order {id}"),
                subtitle: stage.label.clone(),
                sections: vec![
                    section(
                        "Intention",
                        vec![
                            o.objective.clone(),
                            format!(
                                "Issuer {} → office {} / Person {}",
                                o.issuer, o.office, o.recipient
                            ),
                            format!("Issued {} ago", age_label(s.now - o.issued_at)),
                        ],
                    ),
                    section(
                        "Known command chain",
                        stage
                            .steps
                            .iter()
                            .map(|step| format!("{}: {:?}", step.label, step.state))
                            .collect(),
                    ),
                    section("Meaning", vec![stage.detail]),
                ],
            }
        }
        Selection::Report(id) => {
            let Some(r) = s
                .reports
                .iter()
                .find(|r| r.id == id && r.holder == s.viewer)
                .or_else(|| s.reports.iter().find(|r| r.id == id))
            else {
                return missing();
            };
            Panel{title:format!("Report {id}"),subtitle:format!("Observation {} old",age_label(s.now-r.observed_at)),sections:vec![section("Evidence",vec![format!("Author {} · last sender {} · node {}",r.author,r.sender,r.node),format!("Observed {} public food quanta",r.food_quantity),format!("Received {} ago",age_label(s.now-r.received_at))]),section("Limits",vec!["The report transports an old observation, not a live remote view.".into(),"Receiving it does not refresh its observation timestamp or guarantee the food remains.".into()])]}
        }
        Selection::Edge(id) => {
            let Some(e) = s.edges.iter().find(|e| e.id == id) else {
                return missing();
            };
            Panel {
                title: format!("Route {} ↔ {}", e.a, e.b),
                subtitle: evidence_label(&e.evidence, s.now),
                sections: vec![section(
                    "Physical constraint",
                    vec![
                        format!(
                            "Nominal travel: {} · capacity {}",
                            age_label(e.nominal_ms),
                            e.capacity
                        ),
                        format!(
                            "Observed queue: {}",
                            e.queued.map_or_else(
                                || "unknown".into(),
                                |q| format!("{q} at bank {:?}", e.queue_scope_node)
                            )
                        ),
                        format!(
                            "Occupancy: {}",
                            e.occupied
                                .map_or_else(|| "unknown".into(), |q| q.to_string())
                        ),
                        "Opposite-bank demand and future arrivals may differ from this evidence."
                            .into(),
                    ],
                )],
            }
        }
        Selection::Goods(id) => {
            let Some(g) = s.goods.iter().find(|g| g.id == id) else {
                return missing();
            };
            let expiry=g.expires_at.map(|t|if t<=s.now{"Recorded shelf-life deadline has passed; actual remaining quantity unknown remotely.".into()}else{format!("Recorded edible time remaining: {}",age_label(t-s.now))});
            let mut lines = vec![
                format!("{}: {} quanta", g.material, g.quantity),
                format!("Owner: {} · custody {:?}", owner_label(g.owner), g.holder),
                "Ownership and physical custody are distinct.".into(),
            ];
            if let Some(x) = expiry {
                lines.push(x);
            }
            Panel {
                title: format!("Goods batch {id}"),
                subtitle: evidence_label(&g.evidence, s.now),
                sections: vec![section("Physical goods", lines)],
            }
        }
        Selection::Field(id) => {
            let Some(p) = s.fields.iter().find(|p| p.id == id) else {
                return missing();
            };
            Panel {
                title: format!("Cultivation unit {id}"),
                subtitle: evidence_label(&p.evidence, s.now),
                sections: vec![section(
                    "Observed crop",
                    vec![
                        format!("Node {} · {}", p.node, p.stage),
                        format!(
                            "Crop owner: {}",
                            p.owner.map_or_else(|| "none / unknown".into(), owner_label)
                        ),
                        "Crop ownership does not imply land ownership.".into(),
                        "Growth and output require authoritative time and physical work.".into(),
                    ],
                )],
            }
        }
        Selection::Cart(id) => {
            let Some(c) = s.carts.iter().find(|c| c.id == id) else {
                return missing();
            };
            Panel {
                title: format!("Cart {id}"),
                subtitle: evidence_label(&c.evidence, s.now),
                sections: vec![section(
                    "Personal capital",
                    vec![
                        format!("Owner: {} · user {:?}", owner_label(c.owner), c.user),
                        format!("Physical location: {:?}", c.location),
                        format!("Added cargo capacity: {}", c.capacity),
                        "Military membership grants no right to another Person's cart.".into(),
                    ],
                )],
            }
        }
    };
    if let Some(e) = evidence_for(s, selected) {
        panel.sections.push(evidence_section(e, s.now));
    }
    panel
}

#[cfg(test)]
mod tests {
    use super::*;
    use world_of_nodes::provisioning::playable::snapshot::{ReportSnapshot, capture};
    use world_of_nodes::provisioning::playable::{PlayerCommand, ScenarioKind, Session};
    use world_of_nodes::provisioning::{Config, World};
    fn local() -> Session {
        Session::new(0, ScenarioKind::Local).unwrap()
    }

    #[test]
    fn stable_top_level_verbs_do_not_depend_on_predicted_context() {
        let s = local().snapshot(ViewMode::Play);
        let a = contextual_actions(&s, Some(Selection::Node(1)), Verb::Observe);
        let b = contextual_actions(&s, Some(Selection::Unit(0)), Verb::Observe);
        assert_eq!(a, b);
        assert_eq!(
            a.iter().map(|a| a.kind).collect::<Vec<_>>(),
            vec![
                ActionKind::Inspect,
                ActionKind::Reports,
                ActionKind::CaptureReport,
                ActionKind::Help
            ]
        );
        for verb in [Verb::Govern, Verb::Allocate] {
            assert!(
                contextual_actions(&s, Some(Selection::Unit(0)), verb)
                    .iter()
                    .all(|a| !a.enabled && a.kind == ActionKind::Unavailable)
            );
        }
        assert_eq!(
            contextual_actions(&s, Some(Selection::Unit(0)), Verb::Order)[0].kind,
            ActionKind::Unit(MilitaryOrder::Move)
        );
        assert!(matches!(
            contextual_actions(&s, Some(Selection::Crossing(0)), Verb::Order)[0].kind,
            ActionKind::Commission { project: Some(0) }
        ));
    }

    #[test]
    fn node_zero_is_a_real_known_route_not_a_false_boolean_target() {
        let s = local().snapshot(ViewMode::Play);
        assert!(known_route(&s, 1, 0));
        assert!(known_route(&s, 0, 0));
        assert!(known_route(&s, 1, 2)); // Known physical detour, not closed crossing.
        let mut only_closed = s.clone();
        only_closed.edges.retain(|e| e.a == 1 && e.b == 2);
        assert!(!known_route(&only_closed, 1, 2));
        assert!(!known_route(&s, 1, 999));
    }

    #[test]
    fn accepted_role_and_survival_have_distinct_ui_meanings() {
        let session = local();
        let mut s = session.snapshot(ViewMode::Play);
        let selection = Some(Selection::Unit(0));
        assert!(
            contextual_actions(&s, selection, Verb::Order)
                .iter()
                .all(|a| a.enabled)
        );
        s.persons
            .iter_mut()
            .find(|p| p.id == s.viewer)
            .unwrap()
            .nutrition = Some(HOUR);
        let actions = contextual_actions(&s, selection, Verb::Order);
        assert!(actions.iter().all(|a| a.enabled)); // No invented survival issuance ban.
        assert!(actions.iter().all(|a| a.reason.contains("Critical hunger")));
        assert!(concerns(&s).iter().any(|c| c.title == "Ruler needs food"));
        s.persons
            .iter_mut()
            .find(|p| p.id == s.viewer)
            .unwrap()
            .unit = None;
        assert!(
            contextual_actions(&s, selection, Verb::Order)
                .iter()
                .all(|a| !a.enabled && a.reason.contains("no active service"))
        );
        s.persons
            .iter_mut()
            .find(|p| p.id == s.viewer)
            .unwrap()
            .action = Some("dead".into());
        assert!(
            contextual_actions(&s, selection, Verb::Order)
                .iter()
                .all(|a| !a.enabled && a.reason == "The ruler is dead")
        );
    }

    #[test]
    fn actual_remote_setup_cannot_offer_military_commands_after_service_ended() {
        let remote = Session::new(0, ScenarioKind::Remote).unwrap();
        let actions = contextual_actions(
            &remote.snapshot(ViewMode::Play),
            Some(Selection::Unit(0)),
            Verb::Order,
        );
        assert!(
            actions
                .iter()
                .all(|a| !a.enabled && a.reason.contains("no active service"))
        );
    }

    #[test]
    fn physically_witnessed_handoff_does_not_claim_decision_or_construction() {
        let mut session = local();
        session
            .dispatch(PlayerCommand::Commission {
                office: 0,
                project: 0,
            })
            .unwrap();
        let s = session.snapshot(ViewMode::Play);
        let stage = order_stage(&s, &s.orders[0]);
        assert_eq!(stage.steps[2].state, StepState::Known);
        assert_eq!(stage.steps[3].state, StepState::Unknown);
        assert_eq!(stage.steps[4].state, StepState::Unknown);
        session.advance_to(HOUR).unwrap();
        let s = session.snapshot(ViewMode::Play);
        // The recipient actually completed its commission, but the issuer's
        // witnessed handoff is still not a private-recipient-status channel.
        assert!(
            session.world().command_memory(1).unwrap().obligations[&0]
                .status
                .terminal()
        );
        assert_eq!(
            order_stage(&s, &s.orders[0]).steps[3].state,
            StepState::Unknown
        );
        let mut own_received = s.orders[0].clone();
        own_received.status = "Completed: LOCAL_COMMISSION_AUTHORIZED_NOT_PHYSICAL_SUCCESS".into();
        let stage = order_stage(&s, &own_received);
        assert_eq!(stage.steps[3].state, StepState::Known);
        assert_eq!(stage.steps[4].state, StepState::Unknown);
        assert!(stage.detail.contains("not proof of construction"));
    }

    #[test]
    fn courier_handoff_and_elapsed_deadline_do_not_imply_remote_receipt() {
        let mut session = Session::new(0, ScenarioKind::Remote).unwrap();
        session
            .dispatch(PlayerCommand::Commission {
                office: 0,
                project: 0,
            })
            .unwrap();
        let mut s = session.snapshot(ViewMode::Play);
        assert!(s.orders[0].status.starts_with("Handed to agreed courier"));
        let stage = order_stage(&s, &s.orders[0]);
        assert_eq!(stage.steps[1].state, StepState::Known);
        assert_eq!(stage.steps[2].state, StepState::Unknown);
        s.now = s.orders[0].expires_at.unwrap() + HOUR;
        let stage = order_stage(&s, &s.orders[0]);
        assert_eq!(stage.steps[2].state, StepState::Unknown);
        assert_eq!(stage.steps[4].state, StepState::Unknown);
        assert!(stage.detail.contains("do not infer"));
    }

    #[test]
    fn hidden_world_differences_cannot_change_any_play_presentation() {
        let make = |quantity| {
            World::new(Config {
                starts: vec![0, 2],
                food: vec![(2, quantity, 0, 48 * HOUR)],
                boards: vec![],
                autonomous: false,
                ..Config::default()
            })
            .unwrap()
        };
        let a = capture(&make(10), 0, ViewMode::Play);
        let b = capture(&make(10_000), 0, ViewMode::Play);
        assert_eq!(a, b);
        assert_eq!(concerns(&a), concerns(&b));
        assert_eq!(
            selected_panel(&a, Some(Selection::Node(2))),
            selected_panel(&b, Some(Selection::Node(2)))
        );
        assert_eq!(
            contextual_actions(&a, Some(Selection::Crossing(0)), Verb::Order),
            contextual_actions(&b, Some(Selection::Crossing(0)), Verb::Order)
        );
    }

    #[test]
    fn dated_reports_and_ghosts_never_become_current_by_formatting() {
        let mut s = local().snapshot(ViewMode::Play);
        let e = Evidence {
            source: KnowledgeSource::Report,
            observed_at: 0,
            received_at: Some(4 * HOUR),
            author: Some(2),
            sender: Some(5),
        };
        s.now = 6 * HOUR;
        s.reports.push(ReportSnapshot {
            id: 99,
            holder: s.viewer,
            author: 2,
            sender: 5,
            node: 2,
            observed_at: 0,
            received_at: 4 * HOUR,
            food_quantity: 200,
            evidence: e.clone(),
        });
        let before = s.clone();
        let panel = selected_panel(&s, Some(Selection::Report(99)));
        assert!(panel.subtitle.contains("6h"));
        assert!(
            panel.sections[0]
                .lines
                .iter()
                .any(|line| line.contains("Received 2h ago"))
        );
        let provenance = panel.sections.last().unwrap();
        assert_eq!(provenance.heading, "Evidence / time");
        assert!(
            provenance
                .lines
                .contains(&"Evidence timestamp: T+00:00:00.000".into())
        );
        assert!(
            provenance
                .lines
                .contains(&"Received: T+04:00:00.000".into())
        );
        assert!(provenance.lines.contains(&"Source Person: 2".into()));
        assert!(concerns(&s).iter().any(|c| c.evidence == "Report 6h old"));
        assert_eq!(s, before);
        let p = s.persons.iter_mut().find(|p| p.id == 6).unwrap();
        p.evidence = e;
        p.hostile = Some(true);
        p.action = None;
        p.unit = None;
        assert!(
            !concerns(&s)
                .iter()
                .any(|c| c.title == "Hostile person observed")
        );
        let panel = selected_panel(&s, Some(Selection::Person(6)));
        assert_eq!(panel.sections[0].heading, "Last known Person");
        assert_eq!(panel.subtitle, "Report 6h old");
        assert!(
            panel
                .sections
                .last()
                .unwrap()
                .lines
                .contains(&"Evidence timestamp: T+00:00:00.000".into())
        );
    }

    #[test]
    fn map_only_nodes_are_not_presented_as_empty_settlements() {
        let s = local().snapshot(ViewMode::Play);
        assert!(!s.nodes.iter().find(|n| n.id == 2).unwrap().contents_known);
        let panel = selected_panel(&s, Some(Selection::Node(2)));
        assert!(
            panel.sections[0]
                .lines
                .iter()
                .any(|l| l.contains("unobserved"))
        );
        assert!(
            !panel.sections[0]
                .lines
                .iter()
                .any(|l| l.contains("0 people"))
        );
    }

    #[test]
    fn unit_badge_uses_observed_actions_not_last_issued_global_intent() {
        let mut s = local().snapshot(ViewMode::Play);
        assert_eq!(unit_state(&s, 0), "Gathered at node 1");
        s.persons.iter_mut().find(|p| p.id == 6).unwrap().action = Some("sleeping".into());
        assert_eq!(unit_state(&s, 0), "Resting · 1 observed");
        s.persons.iter_mut().find(|p| p.id == 7).unwrap().action = Some("queued".into());
        assert_eq!(unit_state(&s, 0), "Queued · 1 observed");
        s.persons.retain(|p| p.unit != Some(0));
        assert_eq!(unit_state(&s, 0), "No members currently observed");
    }
}
