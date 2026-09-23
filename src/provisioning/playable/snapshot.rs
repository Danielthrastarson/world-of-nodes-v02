//! Owned, read-only presentation data. Play is an epistemic view; Debug is truth.
//! Schematic screen coordinates deliberately do not belong in this module.
use super::super::{Holder, Location, Owner, Time, World};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Play,
    Debug,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnowledgeSource {
    Local,
    Memory,
    Report,
    SelfKnowledge,
    Debug,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evidence {
    pub source: KnowledgeSource,
    pub observed_at: Time,
    pub received_at: Option<Time>,
    pub author: Option<usize>,
    pub sender: Option<usize>,
}
impl Evidence {
    pub fn age(&self, now: Time) -> Time {
        now.saturating_sub(self.observed_at).max(0)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub now: Time,
    pub viewer: usize,
    pub mode: ViewMode,
    pub nodes: Vec<NodeSnapshot>,
    pub edges: Vec<EdgeSnapshot>,
    pub persons: Vec<PersonSnapshot>,
    pub goods: Vec<GoodsSnapshot>,
    pub fields: Vec<FieldSnapshot>,
    pub crossings: Vec<CrossingSnapshot>,
    pub carts: Vec<CartSnapshot>,
    pub units: Vec<UnitSnapshot>,
    pub orders: Vec<OrderSnapshot>,
    pub reports: Vec<ReportSnapshot>,
    pub inspect_lines: Vec<String>,
    pub debug: Option<DebugSnapshot>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeSnapshot {
    pub id: usize,
    /// False for an edge endpoint whose contents have never been observed.
    pub contents_known: bool,
    pub people_seen: usize,
    pub board_quantity: Option<u64>,
    pub evidence: Evidence,
    pub inspect_lines: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeSnapshot {
    pub id: usize,
    pub a: usize,
    pub b: usize,
    pub nominal_ms: Time,
    pub capacity: usize,
    pub queued: Option<usize>,
    /// A play queue count is one observed bank, never the unseen opposite bank.
    pub queue_scope_node: Option<usize>,
    pub occupied: Option<usize>,
    pub path_stage: Option<String>,
    pub evidence: Evidence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersonSnapshot {
    pub id: usize,
    pub location: Location,
    pub action: Option<String>,
    pub role: Option<String>,
    pub unit: Option<usize>,
    /// A locally recognized threat, not access to hidden raid objectives. None
    /// means unknown; a lack of observed harm does not establish peaceful intent.
    pub hostile: Option<bool>,
    pub nutrition: Option<Time>,
    pub fatigue: Option<Time>,
    pub starvation: Option<Time>,
    pub traversal_nominal_ms: Option<Time>,
    pub evidence: Evidence,
    pub inspect_lines: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoodsSnapshot {
    pub id: usize,
    pub material: String,
    pub quantity: u64,
    pub available: Option<u64>,
    pub owner: Owner,
    pub holder: Holder,
    pub node: Option<usize>,
    pub expires_at: Option<Time>,
    pub evidence: Evidence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldSnapshot {
    pub id: usize,
    pub node: usize,
    pub crop: Option<u64>,
    pub owner: Option<Owner>,
    pub stage: String,
    pub ready_at: Option<Time>,
    pub evidence: Evidence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrossingSnapshot {
    pub id: usize,
    pub node: usize,
    pub edge: Option<usize>,
    pub owner: Option<Owner>,
    pub wood_committed: Option<u64>,
    pub wood_required: Option<u64>,
    pub work_remaining: Option<u128>,
    pub completed: Option<bool>,
    pub authorized: Option<bool>,
    pub evidence: Evidence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CartSnapshot {
    pub id: usize,
    pub owner: Owner,
    pub location: Location,
    pub user: Option<usize>,
    pub capacity: u64,
    pub evidence: Evidence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitSnapshot {
    pub id: usize,
    pub member_ids: Vec<usize>,
    pub global_member_count: Option<usize>,
    pub objective: Option<usize>,
    pub muster: Option<usize>,
    pub local_only: bool,
    pub evidence: Evidence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderSnapshot {
    pub id: u64,
    pub issuer: usize,
    pub recipient: usize,
    pub office: usize,
    pub objective: String,
    pub issued_at: Time,
    pub expires_at: Option<Time>,
    pub received_at: Option<Time>,
    pub status: String,
    pub evidence: Evidence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportSnapshot {
    pub id: u64,
    pub holder: usize,
    pub author: usize,
    pub sender: usize,
    pub node: usize,
    pub observed_at: Time,
    pub received_at: Time,
    pub food_quantity: u64,
    pub evidence: Evidence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugSnapshot {
    pub state_hash: u64,
    pub event_hash: u64,
    pub global_person_count: usize,
    pub live_person_count: usize,
    pub metrics: Vec<(String, String)>,
    pub journals: Vec<String>,
}

use super::super::{
    Action, FoodView, Material, NodeView, capital, commands, farming, infrastructure,
};

fn evidence(source: KnowledgeSource, observed_at: Time) -> Evidence {
    Evidence {
        source,
        observed_at,
        received_at: None,
        author: None,
        sender: None,
    }
}
fn memory_evidence(world: &World, viewer: usize, node: usize, at: Time) -> Evidence {
    if let Some(memory) = world.information_memory(viewer)
        && let Some(report_id) = memory.reported_nodes.get(&node)
        && let Some(receipt) = memory.reports.get(report_id)
        && receipt.report.observation.observed_at == at
    {
        return Evidence {
            source: KnowledgeSource::Report,
            observed_at: at,
            received_at: Some(receipt.received_at),
            author: Some(receipt.report.author),
            sender: Some(receipt.sender),
        };
    }
    evidence(KnowledgeSource::Memory, at)
}
fn holder_node(holder: Holder) -> Option<usize> {
    match holder {
        Holder::Deposit(n) | Holder::Loose(n) | Holder::Commons(n) => Some(n),
        _ => None,
    }
}
fn action_label(action: &Action) -> &'static str {
    match action {
        Action::Strike { .. } => "striking",
        Action::RaidStock { .. } => "raiding",
        Action::MilitaryDuty { .. } => "military duty",
        Action::Reproduce { .. } => "reproductive undertaking",
        Action::BuildCrossing { .. } => "building crossing",
        Action::BuildCart { .. } => "building cart",
        Action::Farm { .. } => "cultivating",
        Action::Idle => "idle",
        Action::Queued { .. } => "queued",
        Action::Travel => "travelling",
        Action::Gather { .. } => "gathering",
        Action::Sleep { .. } => "sleeping",
        Action::Dead => "dead",
    }
}
fn food_snapshot(food: &FoodView, source: &Evidence) -> GoodsSnapshot {
    GoodsSnapshot {
        id: food.batch,
        material: "Food".into(),
        quantity: food.quantity,
        available: Some(food.available),
        owner: food.owner,
        holder: food.holder,
        node: holder_node(food.holder),
        expires_at: Some(food.expires_at),
        evidence: source.clone(),
    }
}
fn field_snapshot(plot: &farming::Plot, source: &Evidence) -> FieldSnapshot {
    FieldSnapshot {
        id: plot.id,
        node: plot.node,
        crop: plot.crop.as_ref().map(|c| c.id),
        owner: plot.crop.as_ref().map(|c| c.owner),
        stage: plot.crop.as_ref().map_or_else(
            || "available".into(),
            |c| match c.stage {
                farming::Stage::Sowing => "sowing".into(),
                farming::Stage::Growing { .. } => "growing".into(),
                farming::Stage::Mature => "mature".into(),
                farming::Stage::Harvesting => "harvesting".into(),
            },
        ),
        ready_at: plot.crop.as_ref().and_then(|c| match c.stage {
            farming::Stage::Growing { ready_at } => Some(ready_at),
            _ => None,
        }),
        evidence: source.clone(),
    }
}
fn crossing_snapshot(project: &infrastructure::Project, source: &Evidence) -> CrossingSnapshot {
    CrossingSnapshot {
        id: project.id,
        node: project.site.node,
        edge: Some(project.site.edge),
        owner: Some(project.owner),
        wood_committed: Some(project.wood),
        wood_required: Some(project.site.wood_cost),
        work_remaining: Some(project.remaining),
        completed: Some(project.completed_at.is_some()),
        authorized: None,
        evidence: source.clone(),
    }
}
fn cart_snapshot(cart: &capital::CartView, source: &Evidence) -> CartSnapshot {
    CartSnapshot {
        id: cart.id,
        owner: cart.owner,
        location: cart.location.clone(),
        user: cart.user,
        capacity: cart.capacity,
        evidence: source.clone(),
    }
}
fn put<T>(map: &mut BTreeMap<usize, T>, id: usize, value: T, source: impl Fn(&T) -> &Evidence) {
    if map
        .get(&id)
        .is_none_or(|old| source(&value).observed_at >= source(old).observed_at)
    {
        map.insert(id, value);
    }
}

/// Capture has no side effects, including no observation/memory refresh. Current
/// local visibility is a presentation projection, not a new NPC information channel.
/// An invalid viewer yields an empty play view; debug remains explicitly privileged.
pub fn capture(world: &World, viewer: usize, mode: ViewMode) -> Snapshot {
    let debug = mode == ViewMode::Debug;
    let now = world.now();
    let mut out = Snapshot {
        now,
        viewer,
        mode,
        nodes: vec![],
        edges: vec![],
        persons: vec![],
        goods: vec![],
        fields: vec![],
        crossings: vec![],
        carts: vec![],
        units: vec![],
        orders: vec![],
        reports: vec![],
        inspect_lines: vec![],
        debug: None,
    };
    let person = world.agents().get(viewer);
    if person.is_none() && !debug {
        return out;
    }
    let own_observation = world.observe(viewer);
    let local = own_observation.as_ref().and_then(|o| o.local.as_ref());
    let local_node = local.map(|v| v.node);
    let mut nodes = BTreeMap::new();
    let mut views: BTreeMap<usize, (NodeView, Evidence)> = BTreeMap::new();
    if debug {
        for node in 0..world.config.nodes {
            views.insert(
                node,
                (world.node_view(node), evidence(KnowledgeSource::Debug, now)),
            );
        }
    } else if let Some(person) = person {
        for (&node, view) in &person.memory.nodes {
            views.insert(
                node,
                (
                    view.clone(),
                    memory_evidence(world, viewer, node, view.observed_at),
                ),
            );
        }
        if let Some(local) = local {
            views.insert(
                local.node,
                (local.clone(), evidence(KnowledgeSource::Local, now)),
            );
        }
    }
    let mut edges = BTreeMap::new();
    let mut persons = BTreeMap::new();
    let mut goods = BTreeMap::new();
    let mut fields = BTreeMap::new();
    let mut crossings = BTreeMap::new();
    let mut carts = BTreeMap::new();
    for (view, source) in views.values() {
        nodes.insert(
            view.node,
            NodeSnapshot {
                id: view.node,
                contents_known: true,
                people_seen: view.people.len(),
                board_quantity: view.board.as_ref().map(|b| b.quantity),
                evidence: source.clone(),
                inspect_lines: vec![format!(
                    "{} people observed at t={}",
                    view.people.len(),
                    view.observed_at
                )],
            },
        );
        for edge in &view.edges {
            let visible_now = debug || source.source == KnowledgeSource::Local;
            let edge_view = EdgeSnapshot {
                id: edge.id,
                a: edge.edge.a,
                b: edge.edge.b,
                nominal_ms: edge.edge.nominal_ms,
                capacity: edge.edge.capacity,
                queued: Some(if debug {
                    world
                        .agents()
                        .iter()
                        .filter(|a| {
                            matches!(a.action,
                    Action::Queued {edge: e, ..} if e == edge.id)
                        })
                        .count()
                } else {
                    edge.queued
                }),
                queue_scope_node: (!debug).then_some(view.node),
                occupied: debug.then(|| {
                    world
                        .agents()
                        .iter()
                        .filter(|a| {
                            matches!(a.location,
                    Location::Edge {edge: e, ..} if e == edge.id)
                        })
                        .count()
                }),
                path_stage: visible_now
                    .then(|| {
                        world
                            .path_surfaces()
                            .get(&edge.id)
                            .map(|p| p.stage().into())
                    })
                    .flatten(),
                evidence: source.clone(),
            };
            // At equal observation time the actually local bank wins over memory.
            if edges.get(&edge.id).is_none_or(|old: &EdgeSnapshot| {
                old.evidence.observed_at < source.observed_at
                    || source.source == KnowledgeSource::Local
                    || debug
            }) {
                edges.insert(edge.id, edge_view);
            }
            for endpoint in [edge.edge.a, edge.edge.b] {
                nodes.entry(endpoint).or_insert_with(|| NodeSnapshot {
                    id: endpoint,
                    contents_known: false,
                    people_seen: 0,
                    board_quantity: None,
                    evidence: source.clone(),
                    inspect_lines: vec!["Known adjacent endpoint; contents unobserved".into()],
                });
            }
        }
        for food in &view.food {
            put(&mut goods, food.batch, food_snapshot(food, source), |g| {
                &g.evidence
            });
        }
        if !debug {
            for &id in &view.people {
                if id == viewer {
                    continue;
                }
                // A ghost comes only from the remembered record, even if the real
                // Person moved, died or disappeared from current world storage.
                let p = PersonSnapshot {
                    id,
                    location: Location::Node(view.node),
                    action: None,
                    role: None,
                    unit: None,
                    hostile: None,
                    nutrition: None,
                    fatigue: None,
                    starvation: None,
                    traversal_nominal_ms: None,
                    evidence: source.clone(),
                    inspect_lines: vec![format!(
                        "Last seen at node {} at t={}",
                        view.node, source.observed_at
                    )],
                };
                put(&mut persons, id, p, |p| &p.evidence);
            }
        }
    }
    if !debug {
        if let Some(memory) = world.farming_memory(viewer) {
            for view in memory.values() {
                let source = evidence(KnowledgeSource::Memory, view.observed_at);
                for plot in &view.plots {
                    put(&mut fields, plot.id, field_snapshot(plot, &source), |p| {
                        &p.evidence
                    });
                }
            }
        }
        if let Some(memory) = world.crossing_memory(viewer) {
            for view in memory.values() {
                let source = evidence(KnowledgeSource::Memory, view.observed_at);
                for (project, _) in &view.projects {
                    let mut p = crossing_snapshot(project, &source);
                    // This view only contains projects authorized when observed.
                    p.authorized = Some(true);
                    put(&mut crossings, project.id, p, |p| &p.evidence);
                }
            }
        }
        if let Some(memory) = world.cart_memory(viewer) {
            for view in memory.values() {
                let source = evidence(KnowledgeSource::Memory, view.observed_at);
                for cart in &view.carts {
                    put(&mut carts, cart.id, cart_snapshot(cart, &source), |c| {
                        &c.evidence
                    });
                }
            }
        }
        if let Some(memory) = world.command_memory(viewer) {
            for site in memory.known_sites.values() {
                crossings
                    .entry(site.project)
                    .or_insert_with(|| CrossingSnapshot {
                        id: site.project,
                        node: site.settlement,
                        edge: None,
                        owner: None,
                        wood_committed: None,
                        wood_required: None,
                        work_remaining: None,
                        completed: None,
                        authorized: None,
                        evidence: evidence(KnowledgeSource::Memory, site.observed_at),
                    });
            }
        }
    }
    // Local observable surfaces may be projected without refreshing any memories.
    let current_source = evidence(
        if debug {
            KnowledgeSource::Debug
        } else {
            KnowledgeSource::Local
        },
        now,
    );
    for plot in world
        .plots()
        .iter()
        .filter(|p| debug || local_node == Some(p.node))
    {
        fields.insert(plot.id, field_snapshot(plot, &current_source));
    }
    for project in world
        .crossing_projects()
        .iter()
        .filter(|p| debug || local_node == Some(p.site.node))
    {
        let mut p = crossing_snapshot(project, &current_source);
        p.authorized = Some(world.crossing_authorized(project.id));
        crossings.insert(project.id, p);
    }
    for cart in world.carts().iter().filter(|c| {
        debug
            || matches!(c.location,
        Location::Node(n) if Some(n) == local_node)
            || c.user == Some(viewer)
    }) {
        carts.insert(
            cart.id,
            cart_snapshot(
                &capital::CartView {
                    id: cart.id,
                    owner: cart.owner,
                    location: cart.location.clone(),
                    user: cart.user,
                    capacity: cart.capacity,
                },
                &current_source,
            ),
        );
    }
    for batch in world.batches().iter().filter(|b| b.quantity > 0) {
        let own =
            world.carried_by(viewer, batch.holder) || world.accessible_cart(viewer, batch.holder);
        if debug || own || holder_node(batch.holder).is_some_and(|n| Some(n) == local_node) {
            let source = if !debug && own {
                evidence(KnowledgeSource::SelfKnowledge, now)
            } else {
                current_source.clone()
            };
            goods.insert(
                batch.id,
                GoodsSnapshot {
                    id: batch.id,
                    material: format!("{:?}", batch.material),
                    quantity: batch.quantity,
                    available: (batch.material == Material::Food)
                        .then(|| world.food_view(batch).available),
                    owner: batch.owner,
                    holder: batch.holder,
                    node: holder_node(batch.holder),
                    expires_at: (batch.material == Material::Food).then(|| batch.expires_at()),
                    evidence: source,
                },
            );
        }
    }
    // Do not retain an old ghost or batch at a location that has just been
    // inspected and no longer contains it. Other stale locations remain stale.
    if !debug && let Some(local) = local {
        persons.retain(|_, p| {
            p.location != Location::Node(local.node) || local.people.contains(&p.id)
        });
        goods.retain(|_, g| {
            g.node != Some(local.node)
                || g.evidence.source != KnowledgeSource::Memory
                    && g.evidence.source != KnowledgeSource::Report
        });
    }
    let local_authority = world.observe_authority(viewer);
    let present_ids: Vec<usize> = if debug {
        world.agents().iter().map(|p| p.id).collect()
    } else {
        local.map_or_else(Vec::new, |v| v.people.clone())
    };
    for id in present_ids.into_iter().chain(person.map(|p| p.id)) {
        let Some(p) = world.agents().get(id) else {
            continue;
        };
        let private = debug || id == viewer;
        let source = if debug {
            current_source.clone()
        } else if id == viewer {
            evidence(KnowledgeSource::SelfKnowledge, now)
        } else {
            evidence(KnowledgeSource::Local, now)
        };
        let unit = world.military_role(id);
        let own_unit = world.military_role(viewer);
        let hostile = if debug {
            Some(world.is_raider(id))
        } else if id == viewer {
            own_observation
                .as_ref()
                .map(|observation| observation.raider)
        } else {
            let recognized = world
                .person_state(viewer)
                .and_then(|person| person.capabilities.combat)
                .is_some_and(|view| view.enemies.contains(&id));
            let visible_harm = matches!(p.action, Action::RaidStock { .. })
                || matches!(p.action, Action::Strike { target, .. }
                    if target == viewer || (own_unit.is_some()
                        && local.is_some_and(|view| view.people.contains(&target))
                        && world.military_role(target) == own_unit));
            if recognized || visible_harm {
                Some(true)
            } else if own_unit.is_some() && unit == own_unit {
                Some(false)
            } else {
                None
            }
        };
        let role = if private {
            world.person_state(id).map(|p| format!("{:?}", p.roles))
        } else if let Some(unit) = unit {
            Some(format!("Soldier({unit})"))
        } else {
            local_authority
                .as_ref()
                .and_then(|v| v.present_offices.iter().find(|o| o.holder == id))
                .map(|o| format!("Steward({})", o.settlement))
        };
        let mut lines = vec![format!("Person {id}; {}", action_label(&p.action))];
        if private {
            lines.push(format!("Goal {:?}; job {:?}", p.goal, p.job));
            lines.push(format!("Personal memory: {} nodes", p.memory.nodes.len()));
            lines.push(format!("Physical body: {:?}", world.body(id)));
        }
        if debug {
            lines.push(format!(
                "Body {:?}; social {:?}",
                world.biology(id),
                world.social_memory(id)
            ));
            lines.push(format!("Action {:?}; memory {:?}", p.action, p.memory));
        }
        persons.insert(
            id,
            PersonSnapshot {
                id,
                location: p.location.clone(),
                action: Some(action_label(&p.action).into()),
                role,
                unit,
                hostile,
                nutrition: private.then_some(p.nutrition),
                fatigue: private.then_some(p.fatigue),
                starvation: private.then_some(p.starvation),
                traversal_nominal_ms: if private {
                    match p.location {
                        Location::Edge { edge, .. } => Some(world.traversal_nominal(id, edge)),
                        _ => None,
                    }
                } else {
                    None
                },
                evidence: source,
                inspect_lines: lines,
            },
        );
    }
    if debug {
        for unit in world.units().values() {
            out.units.push(UnitSnapshot {
                id: unit.id,
                member_ids: unit.members.keys().copied().collect(),
                global_member_count: Some(unit.members.len()),
                objective: unit.objective.as_ref().map(|o| o.target),
                muster: Some(unit.muster),
                local_only: false,
                evidence: current_source.clone(),
            });
        }
    } else {
        let mut local_units: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for p in persons.values().filter(|p| {
            matches!(
                p.evidence.source,
                KnowledgeSource::Local | KnowledgeSource::SelfKnowledge
            )
        }) {
            if let Some(unit) = p.unit {
                local_units.entry(unit).or_default().push(p.id);
            }
        }
        for (id, member_ids) in local_units {
            let personal_order = world
                .unit_view(viewer)
                .filter(|v| v.unit == Some(id))
                .and_then(|v| v.order.as_ref());
            out.units.push(UnitSnapshot {
                id,
                member_ids,
                global_member_count: None,
                objective: personal_order.map(|o| o.target),
                muster: None,
                local_only: true,
                evidence: evidence(KnowledgeSource::Local, now),
            });
        }
    }
    append_information(world, viewer, debug, &mut out);
    if let Some(local) = local_authority {
        for office in local.present_offices {
            out.inspect_lines.push(format!(
                "Local office {}: steward {} at settlement {}",
                office.id, office.holder, office.settlement
            ));
        }
        for commission in local.posted_commissions {
            out.inspect_lines
                .push(format!("Locally posted commission: {commission:?}"));
        }
    }
    if debug {
        out.debug = Some(DebugSnapshot {
            state_hash: world.state_hash(),
            event_hash: world.event_hash(),
            global_person_count: world.agents().len(),
            live_person_count: world
                .agents()
                .iter()
                .filter(|p| p.action != Action::Dead)
                .count(),
            metrics: vec![("physical".into(), format!("{:?}", world.metrics()))],
            journals: vec![
                format!("commands {:?}", world.command_journal()),
                format!("information {:?}", world.information_journal()),
                format!("military {:?}", world.military_journal()),
                format!("formation {:?}", world.formation_journal()),
                format!("authority {:?}", world.authority_journal()),
                format!("combat {:?}", world.combat_journal()),
            ],
        });
    }
    out.nodes = nodes.into_values().collect();
    out.edges = edges.into_values().collect();
    out.persons = persons.into_values().collect();
    out.goods = goods.into_values().collect();
    out.fields = fields.into_values().collect();
    out.crossings = crossings.into_values().collect();
    out.carts = carts.into_values().collect();
    out
}

fn append_information(world: &World, viewer: usize, debug: bool, out: &mut Snapshot) {
    let holders: Vec<usize> = if debug {
        (0..world.agents().len()).collect()
    } else {
        vec![viewer]
    };
    let mut orders = BTreeMap::new();
    for holder in holders {
        if let Some(memory) = world.information_memory(holder) {
            for receipt in memory.reports.values() {
                let report = &receipt.report;
                out.reports.push(ReportSnapshot {
                    id: report.id,
                    holder,
                    author: report.author,
                    sender: receipt.sender,
                    node: report.observation.node,
                    observed_at: report.observation.observed_at,
                    received_at: receipt.received_at,
                    food_quantity: report.observation.food.iter().map(|f| f.quantity).sum(),
                    evidence: Evidence {
                        source: if debug {
                            KnowledgeSource::Debug
                        } else {
                            KnowledgeSource::Report
                        },
                        observed_at: report.observation.observed_at,
                        received_at: Some(receipt.received_at),
                        author: Some(report.author),
                        sender: Some(receipt.sender),
                    },
                });
            }
        }
        if let Some(memory) = world.command_memory(holder) {
            for receipt in memory.envelopes.values() {
                let env = &receipt.envelope;
                let source = Evidence {
                    source: if debug {
                        KnowledgeSource::Debug
                    } else {
                        KnowledgeSource::SelfKnowledge
                    },
                    observed_at: env.issued_at(),
                    received_at: Some(receipt.received_at),
                    author: Some(env.issuer()),
                    sender: Some(receipt.sender),
                };
                let (office, objective, expires_at, status, received_at) = match env {
                    commands::Envelope::Order(order) => {
                        let obligation = memory.obligations.get(&order.id);
                        let status = if let Some(obligation) = obligation {
                            format!("{:?}: {}", obligation.status, obligation.reason)
                        } else if let Some(reason) = memory.invalid.get(&order.id) {
                            format!("Invalid: {reason}")
                        } else {
                            "Recipient status unknown".into()
                        };
                        (
                            order.office,
                            format!("{:?}", order.objective),
                            Some(order.expires_at),
                            status,
                            (holder == order.recipient).then_some(receipt.received_at),
                        )
                    }
                    commands::Envelope::Revoke(revoke) => (
                        revoke.office,
                        format!("Revoke command {}", revoke.command),
                        None,
                        if holder == revoke.recipient {
                            "Revocation received".into()
                        } else {
                            "Revocation issued/held; recipient receipt unknown".into()
                        },
                        (holder == revoke.recipient).then_some(receipt.received_at),
                    ),
                };
                let value = OrderSnapshot {
                    id: env.id(),
                    issuer: env.issuer(),
                    recipient: env.recipient(),
                    office,
                    objective,
                    issued_at: env.issued_at(),
                    expires_at,
                    received_at,
                    status,
                    evidence: source,
                };
                // Debug favors recipient knowledge, while play never inspects it.
                if !orders.contains_key(&value.id) || holder == value.recipient {
                    orders.insert(value.id, value);
                }
            }
        }
    }
    out.orders = orders.into_values().collect();
}

#[cfg(test)]
mod tests {
    use super::super::super::{Config, Edge, HOUR, Intent, authority};
    use super::*;

    fn fixture(starts: Vec<usize>) -> World {
        World::new(Config {
            nodes: 3,
            starts,
            autonomous: false,
            information: true,
            food: vec![(2, 20, 0, 12 * HOUR)],
            boards: vec![],
            communal: false,
            initial_nutrition: 24 * HOUR,
            initial_fatigue: 0,
            trace: true,
            edges: vec![
                Edge {
                    a: 0,
                    b: 1,
                    nominal_ms: HOUR,
                    capacity: 4,
                },
                Edge {
                    a: 1,
                    b: 2,
                    nominal_ms: HOUR,
                    capacity: 4,
                },
            ],
            farming: Some(farming::Config {
                units: vec![2],
                rules: farming::Rules::default(),
            }),
            ..Config::default()
        })
        .unwrap()
    }
    fn travel(world: &mut World, id: usize, edge: usize, target: usize) {
        world
            .apply_intents(vec![(
                id,
                Intent::Travel {
                    edge,
                    target,
                    food: false,
                },
            )])
            .unwrap();
        world.advance_to(world.now() + HOUR).unwrap();
        assert_eq!(world.agents[id].location, Location::Node(target));
    }
    fn remember(world: &mut World, viewer: usize, node: usize) {
        // Explicit initial personal knowledge for noninterference tests. No
        // snapshot operation imports this global test-harness observation.
        let view = world.node_view(node);
        world.agents[viewer].memory.nodes.insert(node, view);
    }

    #[test]
    fn capture_is_pure_for_physics_events_memories_rng_and_institutions() {
        let mut world = fixture(vec![0, 2]);
        remember(&mut world, 0, 2);
        let state = world.state_hash();
        let events = world.event_hash();
        let agents = world.agents().to_vec();
        let information = world.information.memories.clone();
        let commands = world.commands.memories.clone();
        let first = capture(&world, 0, ViewMode::Play);
        assert_eq!(first, capture(&world, 0, ViewMode::Play));
        capture(&world, 0, ViewMode::Debug);
        assert_eq!(state, world.state_hash());
        assert_eq!(events, world.event_hash());
        assert_eq!(agents, world.agents());
        assert_eq!(information, world.information.memories);
        assert_eq!(commands, world.commands.memories);
        world.audit().unwrap();
    }

    #[test]
    fn play_hides_other_private_state_and_unseen_people_debug_exposes_truth() {
        let mut world = fixture(vec![0, 0, 2]);
        world.agents[1].goal = Some((2, true));
        world.agents[1].nutrition = 7 * HOUR;
        let play = capture(&world, 0, ViewMode::Play);
        assert!(play.debug.is_none());
        assert_eq!(
            play.persons.iter().map(|p| p.id).collect::<Vec<_>>(),
            vec![0, 1]
        );
        let other = play.persons.iter().find(|p| p.id == 1).unwrap();
        assert_eq!(other.nutrition, None);
        assert_eq!(other.fatigue, None);
        assert_eq!(other.starvation, None);
        assert!(
            other
                .inspect_lines
                .iter()
                .all(|line| !line.contains("Goal"))
        );
        assert_eq!(play.persons[0].nutrition, Some(24 * HOUR));
        assert!(!play.nodes.iter().any(|n| n.id == 2));
        assert!(play.fields.is_empty());
        assert!(play.goods.is_empty());
        let debug = capture(&world, 0, ViewMode::Debug);
        assert_eq!(debug.debug.as_ref().unwrap().global_person_count, 3);
        assert_eq!(debug.persons[1].nutrition, Some(7 * HOUR));
        assert_eq!(
            debug.persons[1]
                .inspect_lines
                .iter()
                .filter(|s| s.contains("Goal"))
                .count(),
            1
        );
        assert_eq!(debug.fields.len(), 1);
        assert_eq!(debug.goods.len(), 1);
    }

    #[test]
    fn unseen_remote_truth_and_global_journals_cannot_change_play_snapshot() {
        let mut world = fixture(vec![0, 2, 2]);
        remember(&mut world, 0, 2);
        let before = capture(&world, 0, ViewMode::Play);
        let debug = capture(&world, 0, ViewMode::Debug);
        // Counterfactual hidden states, same time and identical personal history.
        world.agents[1].location = Location::Node(1);
        world.agents[1].action = Action::Dead;
        world.agents[1].nutrition = 0;
        world.agents[2].goal = Some((0, false));
        world.goods.batches[0].quantity = 1;
        world.goods.batches[0].owner = Owner::Agent(2);
        world.goods.batches[0].material = Material::Waste;
        world.metrics.deaths += 10;
        world.information.journal.add("hidden_handoff");
        world.farm.plots[0].crop = Some(farming::Crop {
            id: 17,
            owner: Owner::Agent(2),
            started_at: 0,
            stage: farming::Stage::Mature,
            remaining: 0,
        });
        assert_eq!(before, capture(&world, 0, ViewMode::Play));
        assert_ne!(debug, capture(&world, 0, ViewMode::Debug));
        let ghost = &before.persons.iter().find(|p| p.id == 1).unwrap();
        assert_eq!(ghost.location, Location::Node(2));
        assert_eq!(ghost.evidence.source, KnowledgeSource::Memory);
        assert_eq!(ghost.action, None);
        assert_eq!(before.goods[0].material, "Food");
        assert_eq!(before.goods[0].quantity, 20);
    }

    #[test]
    fn stale_ghost_and_food_timestamps_do_not_refresh_when_clock_advances() {
        let mut world = fixture(vec![0, 2]);
        remember(&mut world, 0, 2);
        world.advance_to(HOUR).unwrap();
        let snapshot = capture(&world, 0, ViewMode::Play);
        let ghost = snapshot.persons.iter().find(|p| p.id == 1).unwrap();
        assert_eq!(ghost.evidence.observed_at, 0);
        assert_eq!(ghost.evidence.age(snapshot.now), HOUR);
        assert_eq!(ghost.nutrition, None);
        assert_eq!(snapshot.goods[0].evidence.observed_at, 0);
        assert_eq!(world.agents[0].memory.nodes[&2].observed_at, 0);
    }

    #[test]
    fn physically_carried_report_retains_author_sender_and_observation_age() {
        let mut world = fixture(vec![0, 2, 2]);
        let report = world.capture_report(1).unwrap();
        travel(&mut world, 1, 1, 1);
        travel(&mut world, 1, 0, 0);
        world.receive_report(1, 0, report).unwrap();
        let snapshot = capture(&world, 0, ViewMode::Play);
        let source = &snapshot.nodes.iter().find(|n| n.id == 2).unwrap().evidence;
        assert_eq!(source.source, KnowledgeSource::Report);
        assert_eq!(source.observed_at, 0);
        assert_eq!(source.received_at, Some(2 * HOUR));
        assert_eq!(source.author, Some(1));
        assert_eq!(source.sender, Some(1));
        assert_eq!(snapshot.goods[0].evidence, *source);
        let ghost = snapshot.persons.iter().find(|p| p.id == 2).unwrap();
        assert_eq!(ghost.evidence, *source);
        assert_eq!(snapshot.reports.len(), 1);
        assert_eq!(snapshot.reports[0].observed_at, 0);
        assert_eq!(snapshot.reports[0].received_at, 2 * HOUR);
        world.goods.batches[0].quantity = 0;
        assert_eq!(snapshot, capture(&world, 0, ViewMode::Play));
    }

    #[test]
    fn local_fresh_observation_replaces_ghost_without_mutating_old_memory() {
        let mut world = fixture(vec![0, 1]);
        remember(&mut world, 0, 1);
        travel(&mut world, 1, 0, 0);
        let snapshot = capture(&world, 0, ViewMode::Play);
        let observed = snapshot.persons.iter().find(|p| p.id == 1).unwrap();
        assert_eq!(observed.location, Location::Node(0));
        assert_eq!(observed.evidence.source, KnowledgeSource::Local);
        assert_eq!(observed.evidence.observed_at, HOUR);
        assert_eq!(world.agents[0].memory.nodes[&1].observed_at, 0);
        assert!(world.agents[0].memory.nodes[&1].people.contains(&1));
    }

    #[test]
    fn local_edges_never_reveal_unseen_opposite_bank_queue_or_occupancy() {
        let mut world = fixture(vec![0, 1, 1]);
        let play = capture(&world, 0, ViewMode::Play);
        let edge = &play.edges[0];
        assert_eq!(edge.queued, Some(0));
        assert_eq!(edge.queue_scope_node, Some(0));
        assert_eq!(edge.occupied, None);
        world.agents[1].action = Action::Queued {
            edge: 0,
            to: 0,
            joined: 0,
        };
        world.agents[2].location = Location::Edge {
            edge: 0,
            from: 1,
            to: 0,
            progress: 0,
        };
        world.agents[2].action = Action::Travel;
        assert_eq!(play, capture(&world, 0, ViewMode::Play));
        let debug = capture(&world, 0, ViewMode::Debug);
        assert_eq!(debug.edges[0].queued, Some(1));
        assert_eq!(debug.edges[0].occupied, Some(1));
    }

    #[test]
    fn remote_order_completion_is_unknown_to_issuer_until_information_arrives() {
        let mut world = World::new(Config {
            nodes: 3,
            starts: vec![1, 1],
            autonomous: false,
            information: true,
            commands: true,
            authority: Some(authority::Config {
                polity_id: 9,
                sovereign: 0,
                settlements: vec![1],
            }),
            initial_nutrition: 24 * HOUR,
            initial_fatigue: 0,
            food: vec![],
            boards: vec![],
            edges: vec![
                Edge {
                    a: 0,
                    b: 1,
                    nominal_ms: HOUR,
                    capacity: 4,
                },
                Edge {
                    a: 1,
                    b: 2,
                    nominal_ms: HOUR,
                    capacity: 4,
                },
            ],
            infrastructure: Some(infrastructure::Config {
                enabled: true,
                wood: vec![],
                sites: vec![infrastructure::Site {
                    edge: 1,
                    node: 1,
                    wood_cost: 1,
                    work_ms: HOUR,
                    opened_nominal_ms: HOUR / 2,
                    opened_capacity: 4,
                }],
            }),
            ..Config::default()
        })
        .unwrap();
        world
            .apply_intents(vec![(
                0,
                Intent::AppointSteward {
                    candidate: 1,
                    settlement: 1,
                },
            )])
            .unwrap();
        world.accept_office(1, 0).unwrap();
        let command = world
            .issue_order(
                0,
                0,
                commands::Objective::CommissionCrossing {
                    project: 0,
                    settlement: 1,
                },
                12 * HOUR,
            )
            .unwrap();
        world.receive_command(0, 1, command).unwrap();
        travel(&mut world, 0, 0, 0);
        let before = capture(&world, 0, ViewMode::Play);
        assert_eq!(before.orders[0].status, "Recipient status unknown");
        assert_eq!(before.orders[0].received_at, None);
        world.fulfil_order(1, command).unwrap();
        assert_eq!(before, capture(&world, 0, ViewMode::Play));
        let recipient = capture(&world, 1, ViewMode::Play);
        assert!(recipient.orders[0].status.starts_with("Completed"));
        let debug = capture(&world, 0, ViewMode::Debug);
        assert!(debug.orders[0].status.starts_with("Completed"));
        world.audit().unwrap();
    }

    #[test]
    fn local_presence_does_not_expose_personal_inventory() {
        let mut world = fixture(vec![0, 0]);
        // Controlled private holding; no claim that it was autonomously acquired.
        world.goods.batches[0].holder = Holder::Agent(1);
        world.goods.batches[0].owner = Owner::Agent(1);
        assert!(capture(&world, 0, ViewMode::Play).goods.is_empty());
        assert_eq!(capture(&world, 1, ViewMode::Play).goods[0].quantity, 20);
        assert_eq!(capture(&world, 0, ViewMode::Debug).goods[0].quantity, 20);
    }

    #[test]
    fn invalid_viewer_cannot_be_used_as_global_spectator_in_play() {
        let world = fixture(vec![0, 2]);
        let view = capture(&world, 999, ViewMode::Play);
        assert!(view.persons.is_empty());
        assert!(view.goods.is_empty());
        assert!(view.nodes.is_empty());
        assert!(view.debug.is_none());
    }
    #[test]
    fn hostility_uses_local_evidence_not_hidden_raid_objectives() {
        use crate::provisioning::combat;
        let mut world = combat::research::fixture("local", true, true, 0).unwrap();
        combat::research::prepare(&mut world, true).unwrap();
        world.config.autonomous = false;
        // Controlled contact initial condition. Presence alone reveals neither
        // the seeded raid objective nor an intention to harm somebody later.
        world.agents[6].location = Location::Node(0);
        world.advance_to(2 * HOUR).unwrap();
        let before = capture(&world, 0, ViewMode::Play);
        assert_eq!(
            before.persons.iter().find(|p| p.id == 6).unwrap().hostile,
            None
        );
        world.combat.views[6].raid.as_mut().unwrap().target = 3;
        assert_eq!(before, capture(&world, 0, ViewMode::Play));
        assert_eq!(
            capture(&world, 0, ViewMode::Debug)
                .persons
                .iter()
                .find(|p| p.id == 6)
                .unwrap()
                .hostile,
            Some(true)
        );
        world
            .apply_intents(vec![(6, Intent::Strike { target: 0 })])
            .unwrap();
        assert!(matches!(world.agents[6].action, Action::Strike { .. }));
        let attacked = capture(&world, 0, ViewMode::Play);
        assert_eq!(
            attacked.persons.iter().find(|p| p.id == 6).unwrap().hostile,
            Some(true)
        );
        assert_eq!(
            attacked.persons.iter().find(|p| p.id == 1).unwrap().hostile,
            Some(false)
        );
        // Even a current harmful action elsewhere is not remote information.
        world.agents[6].location = Location::Node(1);
        assert!(
            !capture(&world, 0, ViewMode::Play)
                .persons
                .iter()
                .any(|p| p.id == 6)
        );
    }
    #[test]
    fn render_poll_frequency_does_not_change_integrated_session_history() {
        use super::super::session::{PlayerCommand, ScenarioKind, Session};
        let mut rendered = Session::new(0, ScenarioKind::Local).unwrap();
        let mut headless = Session::new(0, ScenarioKind::Local).unwrap();
        let input = PlayerCommand::Commission {
            office: 0,
            project: 0,
        };
        assert_eq!(rendered.dispatch(input.clone()), headless.dispatch(input));
        let end = rendered.world().now() + 2 * HOUR;
        while rendered.world().now() < end {
            let next = (rendered.world().now() + HOUR / 12).min(end);
            for _ in 0..3 {
                capture(rendered.world(), rendered.ruler(), ViewMode::Play);
                capture(rendered.world(), rendered.ruler(), ViewMode::Debug);
            }
            rendered.advance_to(next).unwrap();
        }
        headless.advance_to(end).unwrap();
        assert_eq!(rendered.world().state_hash(), headless.world().state_hash());
        assert_eq!(rendered.world().event_hash(), headless.world().event_hash());
        assert_eq!(rendered.world().agents(), headless.world().agents());
    }
}
