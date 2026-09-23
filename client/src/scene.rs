use crate::camera::{MapCamera, node_name, node_position};
use crate::widgets::*;
use crate::widgets::{GOLD, RED};
use macroquad::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use world_of_nodes::provisioning::playable::snapshot::*;
use world_of_nodes::provisioning::{Location, SCALE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selection {
    Node(usize),
    Person(usize),
    Edge(usize),
    Crossing(usize),
    Field(usize),
    Goods(usize),
    Cart(usize),
    Unit(usize),
    Order(u64),
    Report(u64),
}
#[derive(Clone)]
pub struct Hit {
    pub selection: Selection,
    pub point: Vec2,
    pub radius: f32,
}

fn current(e: &Evidence) -> bool {
    matches!(
        e.source,
        KnowledgeSource::Local | KnowledgeSource::SelfKnowledge | KnowledgeSource::Debug
    )
}
fn faded(c: Color, e: &Evidence) -> Color {
    if current(e) {
        c
    } else {
        Color::new(c.r * 0.65, c.g * 0.65, c.b * 0.65, 1.0)
    }
}
pub fn person_position(p: &PersonSnapshot) -> Vec2 {
    match p.location {
        Location::Node(n) => {
            // Stable schematic packing; this is not continuous physical geometry.
            let (a, radius) = if p.id < 12 {
                (p.id as f32 * std::f32::consts::TAU / 12.0, 37.0)
            } else {
                // Area grows with count; avoid radial spokes for dense crowds.
                let index = (p.id - 12) as f32;
                (index * 2.399_963_1, 60.0 + index.sqrt() * 11.0)
            };
            node_position(n) + vec2(a.cos(), a.sin()) * radius
        }
        Location::Edge {
            from, to, progress, ..
        } => {
            let work = p.traversal_nominal_ms.unwrap_or(1).max(1) as f64 * SCALE as f64;
            let t = (progress as f64 / work).clamp(0., 1.) as f32;
            let a = node_position(from);
            let b = node_position(to);
            let tangent = (b - a).normalize_or_zero();
            a.lerp(b, t) + vec2(-tangent.y, tangent.x) * ((p.id % 5) as f32 - 2.) * 4.
        }
    }
}
fn visible(p: Vec2, viewport: Rect) -> bool {
    viewport.contains(p)
}
fn dashed(a: Vec2, b: Vec2, width: f32, color: Color) {
    let length = a.distance(b);
    let direction = (b - a).normalize_or_zero();
    let mut at = 0.0;
    while at < length {
        let x = a + direction * at;
        let y = a + direction * (at + 8.).min(length);
        draw_line(x.x, x.y, y.x, y.y, width, color);
        at += 15.;
    }
}

pub fn draw(
    snapshot: &Snapshot,
    previous: Option<&Snapshot>,
    alpha: f32,
    camera: &MapCamera,
    viewport: Rect,
    selection: Option<Selection>,
) -> Vec<Hit> {
    let mut hits = Vec::new();
    let old_people: BTreeMap<_, _> = previous
        .into_iter()
        .flat_map(|s| s.persons.iter())
        .filter(|p| current(&p.evidence))
        .map(|p| (p.id, p))
        .collect();
    draw_rectangle(viewport.x, viewport.y, viewport.w, viewport.h, BG);
    // Quiet schematic grid; it denotes the view, never hidden terrain or resource state.
    for x in (0..(viewport.w as usize)).step_by(48) {
        draw_line(
            viewport.x + x as f32,
            viewport.y,
            viewport.x + x as f32,
            viewport.bottom(),
            1.,
            Color::new(0.07, 0.11, 0.12, 1.),
        );
    }
    for y in (0..(viewport.h as usize)).step_by(48) {
        draw_line(
            viewport.x,
            viewport.y + y as f32,
            viewport.right(),
            viewport.y + y as f32,
            1.,
            Color::new(0.07, 0.11, 0.12, 1.),
        );
    }
    for e in &snapshot.edges {
        let a = camera.project(node_position(e.a), viewport);
        let b = camera.project(node_position(e.b), viewport);
        if !visible(a, viewport) && !visible(b, viewport) {
            continue;
        }
        let color = faded(
            if e.capacity == 0 {
                MUTED
            } else {
                Color::new(0.38, 0.41, 0.34, 1.)
            },
            &e.evidence,
        );
        let width = match e.path_stage.as_deref() {
            Some("worn_path") => 5.,
            Some("trail") => 3.,
            _ => 1.8,
        };
        if e.capacity == 0 || !current(&e.evidence) {
            dashed(a, b, width, color);
        } else {
            draw_line(a.x, a.y, b.x, b.y, width, color);
        }
        let m = a.lerp(b, 0.5);
        if visible(m, viewport) {
            let label = if e.capacity == 0 {
                "crossing unopened".into()
            } else {
                format!("{} / cap {}", hours(e.nominal_ms), e.capacity)
            };
            text(&label, m.x - 45., m.y - 12., 14., MUTED);
            if let Some(q) = e.queued.filter(|&q| q > 0) {
                text(&format!("queue {q}"), m.x - 25., m.y + 18., 15., GOLD);
            }
            hits.push(Hit {
                selection: Selection::Edge(e.id),
                point: m,
                radius: 18.,
            });
        }
    }
    for n in &snapshot.nodes {
        let p = camera.project(node_position(n.id), viewport);
        if !visible(p, viewport) {
            continue;
        }
        draw_circle(
            p.x,
            p.y,
            26. * camera.zoom.min(1.6),
            Color::new(0.11, 0.17, 0.17, 1.),
        );
        draw_circle_lines(
            p.x,
            p.y,
            26. * camera.zoom.min(1.6),
            1.5,
            faded(TEAL, &n.evidence),
        );
        if n.board_quantity.is_some() {
            draw_rectangle(p.x - 7., p.y - 9., 14., 18., faded(GOLD, &n.evidence));
        } else {
            draw_circle(p.x, p.y, 4., faded(MUTED, &n.evidence));
        }
        text(
            &node_name(n.id),
            p.x - 68.,
            p.y + 75.,
            19.,
            faded(INK, &n.evidence),
        );
        text(
            &format!("{} · {}", n.id, age(snapshot.now, n.evidence.observed_at)),
            p.x - 44.,
            p.y + 94.,
            14.,
            MUTED,
        );
        hits.push(Hit {
            selection: Selection::Node(n.id),
            point: p,
            radius: 28. * camera.zoom.max(0.7),
        });
    }
    for (i, f) in snapshot.fields.iter().enumerate() {
        let base =
            node_position(f.node) + vec2(-72. + (i % 3) as f32 * 31., -78. - (i / 3) as f32 * 21.);
        let p = camera.project(base, viewport);
        if !visible(p, viewport) {
            continue;
        }
        let green = faded(
            if f.crop.is_some() {
                Color::new(0.48, 0.62, 0.33, 1.)
            } else {
                MUTED
            },
            &f.evidence,
        );
        draw_rectangle_lines(p.x - 12., p.y - 8., 24., 16., 1., green);
        for x in 0..4 {
            draw_line(
                p.x - 9. + x as f32 * 6.,
                p.y - 6.,
                p.x - 9. + x as f32 * 6.,
                p.y + 6.,
                1.,
                green,
            );
        }
        hits.push(Hit {
            selection: Selection::Field(f.id),
            point: p,
            radius: 14.,
        });
    }
    for (i, g) in snapshot
        .goods
        .iter()
        .filter(|g| g.node.is_some())
        .enumerate()
    {
        let p = camera.project(
            node_position(g.node.unwrap())
                + vec2(60. + (i % 3) as f32 * 60., -26. - (i / 3) as f32 * 22.),
            viewport,
        );
        if !visible(p, viewport) {
            continue;
        }
        let color = faded(
            if g.material.contains("Food") {
                GOLD
            } else if g.material.contains("Wood") {
                Color::new(0.62, 0.43, 0.27, 1.)
            } else {
                MUTED
            },
            &g.evidence,
        );
        draw_rectangle(p.x - 7., p.y - 7., 14., 14., color);
        text(&g.quantity.to_string(), p.x + 10., p.y + 5., 14., color);
        hits.push(Hit {
            selection: Selection::Goods(g.id),
            point: p,
            radius: 12.,
        });
    }
    for c in &snapshot.crossings {
        let p = if let Some(e) = c
            .edge
            .and_then(|id| snapshot.edges.iter().find(|e| e.id == id))
        {
            camera.project(node_position(e.a).lerp(node_position(e.b), 0.5), viewport)
        } else {
            camera.project(node_position(c.node) + vec2(90., 40.), viewport)
        };
        if !visible(p, viewport) {
            continue;
        }
        let color = faded(
            if c.completed == Some(true) {
                TEAL
            } else {
                GOLD
            },
            &c.evidence,
        );
        draw_rectangle(p.x - 24., p.y - 8., 48., 16., BG);
        draw_line(p.x - 23., p.y - 7., p.x + 23., p.y - 7., 3., color);
        draw_line(p.x - 23., p.y + 7., p.x + 23., p.y + 7., 3., color);
        for x in -2..=2 {
            draw_line(
                p.x + x as f32 * 9.,
                p.y - 7.,
                p.x + x as f32 * 9.,
                p.y + 7.,
                1.,
                color,
            );
        }
        let label = if c.completed == Some(true) {
            "CROSSING OPEN"
        } else if c.authorized == Some(true) {
            "COMMISSIONED"
        } else {
            "CROSSING SITE"
        };
        text(label, p.x - 58., p.y + 38., 14., color);
        hits.push(Hit {
            selection: Selection::Crossing(c.id),
            point: p,
            radius: 26.,
        });
    }
    for c in &snapshot.carts {
        let position = match c.location {
            Location::Node(n) => node_position(n) + vec2(-60., 20. + (c.id % 4) as f32 * 17.),
            Location::Edge {
                edge,
                from,
                to,
                progress,
                ..
            } => {
                let nominal = c
                    .user
                    .and_then(|id| snapshot.persons.iter().find(|p| p.id == id))
                    .and_then(|p| p.traversal_nominal_ms)
                    .or_else(|| {
                        snapshot
                            .edges
                            .iter()
                            .find(|e| e.id == edge)
                            .map(|e| e.nominal_ms)
                    })
                    .unwrap_or(1)
                    .max(1);
                let fraction =
                    (progress as f64 / (nominal as f64 * SCALE as f64)).clamp(0., 1.) as f32;
                node_position(from).lerp(node_position(to), fraction)
            }
        };
        let p = camera.project(position, viewport);
        if !visible(p, viewport) {
            continue;
        }
        draw_rectangle_lines(p.x - 10., p.y - 6., 20., 12., 2., faded(GOLD, &c.evidence));
        draw_circle(p.x - 7., p.y + 8., 3., GOLD);
        draw_circle(p.x + 7., p.y + 8., 3., GOLD);
        hits.push(Hit {
            selection: Selection::Cart(c.id),
            point: p,
            radius: 14.,
        });
    }
    for p in &snapshot.persons {
        let mut pos = person_position(p);
        if current(&p.evidence)
            && let Some(old) = old_people.get(&p.id)
        {
            // Interpolation is bounded by two already-executed poses, never predicts physics.
            pos = person_position(old).lerp(pos, alpha.clamp(0., 1.));
        }
        let at = camera.project(pos, viewport);
        if !visible(at, viewport) {
            continue;
        }
        let dead = p.action.as_deref().is_some_and(|a| a == "dead");
        let hostile = p.hostile == Some(true);
        let color = if p.id == snapshot.viewer {
            GOLD
        } else if dead {
            MUTED
        } else if hostile {
            RED
        } else if p.unit.is_some() {
            TEAL
        } else {
            INK
        };
        let radius = (4.7 * camera.zoom.sqrt()).clamp(3., 8.);
        if current(&p.evidence) {
            draw_circle(at.x, at.y, radius, color);
        } else {
            draw_circle_lines(at.x, at.y, radius, 1., faded(color, &p.evidence));
        }
        if dead {
            draw_line(at.x - 4., at.y - 4., at.x + 4., at.y + 4., 1., BG);
        }
        if p.id == snapshot.viewer {
            draw_circle_lines(at.x, at.y, radius + 4., 1.2, GOLD);
        }
        if snapshot.mode == ViewMode::Debug || camera.zoom > 1.6 {
            text(&p.id.to_string(), at.x + 7., at.y - 5., 13., MUTED);
        }
        hits.push(Hit {
            selection: Selection::Person(p.id),
            point: at,
            radius: radius + 5.,
        });
    }
    for u in &snapshot.units {
        let member_ids: BTreeSet<_> = u.member_ids.iter().copied().collect();
        let members: Vec<_> = snapshot
            .persons
            .iter()
            .filter(|p| member_ids.contains(&p.id))
            .collect();
        if members.is_empty() {
            continue;
        }
        let mean = members
            .iter()
            .map(|p| person_position(p))
            .fold(Vec2::ZERO, |a, b| a + b)
            / members.len() as f32;
        let p = camera.project(mean + vec2(0., -58.), viewport);
        if !visible(p, viewport) {
            continue;
        }
        draw_rectangle(p.x - 47., p.y - 13., 94., 26., PANEL);
        draw_rectangle_lines(p.x - 47., p.y - 13., 94., 26., 1., TEAL);
        text(
            &format!("UNIT {} · {}", u.id, u.member_ids.len()),
            p.x - 40.,
            p.y + 5.,
            15.,
            TEAL,
        );
        hits.push(Hit {
            selection: Selection::Unit(u.id),
            point: p,
            radius: 33.,
        });
    }
    if let Some(h) = hits.iter().find(|h| Some(h.selection) == selection) {
        draw_circle_lines(h.point.x, h.point.y, h.radius + 6., 2., GOLD);
    }
    // Mask schematic geometry outside the map; inspectors own those pixels.
    draw_rectangle(0., 0., screen_width(), viewport.y, PANEL);
    draw_rectangle(
        viewport.right(),
        viewport.y,
        screen_width() - viewport.right(),
        screen_height() - viewport.y,
        PANEL,
    );
    draw_rectangle(
        0.,
        viewport.bottom(),
        screen_width(),
        screen_height() - viewport.bottom(),
        PANEL,
    );
    hits
}

pub fn hit_test(hits: &[Hit], mouse: Vec2) -> Option<Selection> {
    hits.iter()
        .rev()
        .filter(|h| h.point.distance(mouse) <= h.radius)
        .min_by(|a, b| {
            (a.point.distance(mouse) / a.radius).total_cmp(&(b.point.distance(mouse) / b.radius))
        })
        .map(|h| h.selection)
}
pub fn selected_node(s: &Snapshot, selection: Selection) -> Option<usize> {
    match selection {
        Selection::Node(id) => Some(id),
        Selection::Person(id) => {
            s.persons
                .iter()
                .find(|p| p.id == id)
                .and_then(|p| match p.location {
                    Location::Node(n) => Some(n),
                    _ => None,
                })
        }
        Selection::Crossing(id) => s.crossings.iter().find(|p| p.id == id).map(|p| p.node),
        Selection::Field(id) => s.fields.iter().find(|p| p.id == id).map(|p| p.node),
        Selection::Goods(id) => s.goods.iter().find(|p| p.id == id).and_then(|p| p.node),
        _ => None,
    }
}
pub fn evidence_line(e: &Evidence, now: i64) -> String {
    let source = match e.source {
        KnowledgeSource::Local => "Local observation".into(),
        KnowledgeSource::Memory => "Personal memory".into(),
        KnowledgeSource::Report => format!(
            "Report from Person {}",
            e.author.map_or("?".into(), |n| n.to_string())
        ),
        KnowledgeSource::SelfKnowledge => "Ruler's own knowledge".into(),
        KnowledgeSource::Debug => "DEBUG: authoritative truth".into(),
    };
    format!("{source} · {}", age(now, e.observed_at))
}
pub fn inspect(s: &Snapshot, selected: Option<Selection>) -> (String, Vec<String>) {
    let Some(selected) = selected else {
        return("Your kingdom".into(),vec!["Select a place, person, crossing site or unit.".into(),"Filled dots are observed now. Hollow dots are last-seen people; they are not live tracking.".into(),"The ruler is a real Person, marked in gold. Their location determines current sight and local command reach.".into()]);
    };
    let mut lines = Vec::new();
    let title = match selected {
        Selection::Node(id) => {
            if let Some(n) = s.nodes.iter().find(|x| x.id == id) {
                lines.push(evidence_line(&n.evidence, s.now));
                lines.push(if n.contents_known {
                    format!("{} people observed", n.people_seen)
                } else {
                    "Contents unknown; only this location is known.".into()
                });
                if let Some(q) = n.board_quantity {
                    lines.push(format!("Request board: {q} food quanta"));
                }
                lines.extend(n.inspect_lines.clone());
            }
            node_name(id)
        }
        Selection::Person(id) => {
            if let Some(p) = s.persons.iter().find(|x| x.id == id) {
                lines.push(evidence_line(&p.evidence, s.now));
                lines.push(format!("Location: {:?}", p.location));
                if let Some(a) = &p.action {
                    lines.push(format!("Action: {a}"));
                }
                if let Some(r) = &p.role {
                    lines.push(format!("Role: {r}"));
                }
                if let Some(n) = p.nutrition {
                    lines.push(format!(
                        "Nutrition {} / fatigue {}",
                        hours(n),
                        hours(p.fatigue.unwrap_or(0))
                    ));
                } else {
                    lines.push("Private needs, beliefs and intentions are not disclosed.".into());
                }
                lines.extend(p.inspect_lines.clone());
            }
            format!("Person {id}")
        }
        Selection::Edge(id) => {
            if let Some(e) = s.edges.iter().find(|x| x.id == id) {
                lines.push(evidence_line(&e.evidence, s.now));
                lines.push(format!("{} — {}", node_name(e.a), node_name(e.b)));
                lines.push(format!(
                    "Nominal {} · capacity {} · FIFO",
                    hours(e.nominal_ms),
                    e.capacity
                ));
                lines.push(format!(
                    "Path: {}",
                    e.path_stage.as_deref().unwrap_or("not known")
                ));
                lines.push(format!(
                    "Queue {:?}; observed bank {:?}",
                    e.queued, e.queue_scope_node
                ));
            }
            format!("Route {id}")
        }
        Selection::Crossing(id) => {
            if let Some(c) = s.crossings.iter().find(|x| x.id == id) {
                lines.push(evidence_line(&c.evidence, s.now));
                lines.push(format!("At {}", node_name(c.node)));
                lines.push(format!("Authorized: {}", known_bool(c.authorized)));
                lines.push(format!("Constructed: {}", known_bool(c.completed)));
                lines.push(format!(
                    "Wood committed: {} / {}",
                    known(c.wood_committed),
                    known(c.wood_required)
                ));
                if let Some(w) = c.work_remaining {
                    lines.push(format!(
                        "Work remaining: {:.2} nominal hours",
                        w as f64 / (SCALE as f64 * 3_600_000.)
                    ));
                }
                lines.push("W → 1 → select site → confirm. An order creates an obligation, never a finished crossing.".into());
            }
            format!("Crossing {id}")
        }
        Selection::Field(id) => {
            if let Some(f) = s.fields.iter().find(|x| x.id == id) {
                lines.push(evidence_line(&f.evidence, s.now));
                lines.push(format!("Stage: {}", f.stage));
                lines.push(format!("Crop owner: {:?}", f.owner));
                if let Some(at) = f.ready_at {
                    lines.push(format!("Expected maturity: {}", hours(at)));
                }
            }
            format!("Cultivation unit {id}")
        }
        Selection::Goods(id) => {
            if let Some(g) = s.goods.iter().find(|x| x.id == id) {
                lines.push(evidence_line(&g.evidence, s.now));
                lines.push(format!("{} · {} quanta", g.material, g.quantity));
                lines.push(format!("Owner: {:?}", g.owner));
                lines.push(format!("Custody: {:?}", g.holder));
                if let Some(at) = g.expires_at {
                    lines.push(format!("Edible deadline: {}", hours(at)));
                }
                lines.push(
                    "Ownership and custody are separate. A known quantity may have changed.".into(),
                );
            }
            format!("Goods batch {id}")
        }
        Selection::Cart(id) => {
            if let Some(c) = s.carts.iter().find(|x| x.id == id) {
                lines.push(evidence_line(&c.evidence, s.now));
                lines.push(format!("Owner {:?} / user {:?}", c.owner, c.user));
                lines.push(format!("Capacity {} · {:?}", c.capacity, c.location));
            }
            format!("Cart {id}")
        }
        Selection::Unit(id) => {
            if let Some(u) = s.units.iter().find(|x| x.id == id) {
                lines.push(evidence_line(&u.evidence, s.now));
                lines.push(format!("{} members in this view", u.member_ids.len()));
                lines.push(format!("Members: {:?}", u.member_ids));
                if let Some(n) = u.global_member_count {
                    lines.push(format!("DEBUG roster: {n}"));
                }
                lines.push("W: 1 Move · 2 Attack · 3 Hold · 4 Withdraw".into());
                lines.push("Orders reach only active members beside the embodied commander. Absent people keep their previous orders.".into());
            }
            format!("Voluntary unit {id}")
        }
        Selection::Order(id) => {
            if let Some(o) = s.orders.iter().find(|x| x.id == id) {
                lines.push(evidence_line(&o.evidence, s.now));
                lines.push(format!("{} · {}", o.status, o.objective));
                lines.push(format!(
                    "Issued {}; recipient Person {}",
                    hours(o.issued_at),
                    o.recipient
                ));
                lines.push(format!(
                    "Receipt: {}",
                    o.received_at.map_or("unknown".into(), hours)
                ));
                lines.push("Completed duty means commissioned, not physically constructed. Remote progress requires observation or a returned report.".into());
            }
            format!("Sovereign order {id}")
        }
        Selection::Report(id) => {
            if let Some(r) = s.reports.iter().find(|x| x.id == id) {
                lines.push(format!(
                    "Author Person {} · sender Person {}",
                    r.author, r.sender
                ));
                lines.push(format!(
                    "{} observed at {}",
                    node_name(r.node),
                    hours(r.observed_at)
                ));
                lines.push(format!(
                    "Received {} · {}",
                    hours(r.received_at),
                    age(s.now, r.observed_at)
                ));
                lines.push(format!("Observed food: {} quanta", r.food_quantity));
                lines.push("This is a dated claim about an observation. It does not refresh when the world changes.".into());
            }
            format!("Report {id}")
        }
    };
    if lines.is_empty() {
        lines.push("No longer present in your current knowledge view. Debug does not grant player command rights.".into());
    }
    (title, lines)
}
fn known(x: Option<u64>) -> String {
    x.map_or("?".into(), |n| n.to_string())
}
fn known_bool(x: Option<bool>) -> &'static str {
    match x {
        Some(true) => "yes",
        Some(false) => "no",
        None => "unknown",
    }
}
