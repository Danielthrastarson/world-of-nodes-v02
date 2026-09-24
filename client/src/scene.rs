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

// Icons describe known places and objects, not additional simulated buildings.
fn place_outline(p: Vec2, radius: f32, width: f32, color: Color) {
    draw_poly_lines(p.x, p.y, 6, radius, 30., width, color);
}
fn centered_label(label: &str, p: Vec2, size: f32, color: Color) {
    // Approximate centering is display-only; the bundled UI font has a similar
    // average advance. Hit positions never depend on rendered text width.
    text(
        label,
        p.x - label.chars().count() as f32 * size * 0.25,
        p.y,
        size,
        color,
    );
}
fn quiet(color: Color, strength: f32) -> Color {
    Color::new(color.r, color.g, color.b, strength)
}
fn selection_box(p: Vec2, half: Vec2) {
    draw_rectangle_lines(
        p.x - half.x - 4.,
        p.y - half.y - 4.,
        2. * half.x + 8.,
        2. * half.y + 8.,
        5.,
        quiet(GOLD, 0.13),
    );
    draw_rectangle_lines(
        p.x - half.x - 2.,
        p.y - half.y - 2.,
        2. * half.x + 4.,
        2. * half.y + 4.,
        2.,
        GOLD,
    );
}
fn short_resource(material: &str) -> &str {
    if material.contains("Food") {
        "Food"
    } else if material.contains("Wood") {
        "Wood"
    } else if material.contains("Stone") {
        "Stone"
    } else {
        material
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
    // A low-contrast drafting grid belongs to the camera, never to terrain truth.
    let grid = Color::new(0.10, 0.16, 0.18, 0.24);
    for x in (0..(viewport.w as usize)).step_by(64) {
        draw_line(
            viewport.x + x as f32,
            viewport.y,
            viewport.x + x as f32,
            viewport.bottom(),
            1.,
            grid,
        );
    }
    for y in (0..(viewport.h as usize)).step_by(64) {
        draw_line(
            viewport.x,
            viewport.y + y as f32,
            viewport.right(),
            viewport.y + y as f32,
            1.,
            grid,
        );
    }
    // Connections are the quietest meaningful layer. Only selected routes show
    // timing/capacity; visible queue pressure remains noticeable without hover.
    for e in &snapshot.edges {
        let a = camera.project(node_position(e.a), viewport);
        let b = camera.project(node_position(e.b), viewport);
        if !visible(a, viewport) && !visible(b, viewport) {
            continue;
        }
        let selected = selection == Some(Selection::Edge(e.id));
        let width = match e.path_stage.as_deref() {
            Some("worn_path") => 4.,
            Some("trail") => 2.7,
            _ => 1.5,
        };
        let color = if selected {
            GOLD
        } else {
            quiet(
                faded(
                    if e.capacity == 0 {
                        MUTED
                    } else {
                        Color::new(0.43, 0.50, 0.43, 1.)
                    },
                    &e.evidence,
                ),
                0.65,
            )
        };
        if selected {
            draw_line(a.x, a.y, b.x, b.y, width + 7., quiet(GOLD, 0.10));
        }
        if e.capacity == 0 || !current(&e.evidence) {
            dashed(a, b, width, color);
        } else {
            draw_line(a.x, a.y, b.x, b.y, width, color);
        }
        let m = a.lerp(b, 0.5);
        if visible(m, viewport) {
            if selected || camera.zoom > 1.65 {
                let label = if e.capacity == 0 {
                    "Route unopened".into()
                } else {
                    format!("{} · capacity {}", hours(e.nominal_ms), e.capacity)
                };
                centered_label(
                    &label,
                    m + vec2(0., -19.),
                    13.,
                    if selected { GOLD } else { MUTED },
                );
                if selected && !current(&e.evidence) {
                    centered_label(
                        &format!("Last seen {}", age(snapshot.now, e.evidence.observed_at)),
                        m + vec2(0., 36.),
                        12.,
                        MUTED,
                    );
                }
            }
            if let Some(q) = e.queued.filter(|&q| q > 0) {
                centered_label(
                    &format!("{q} queued"),
                    m + vec2(0., 20.),
                    14.,
                    faded(GOLD, &e.evidence),
                );
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
        let selected = selection == Some(Selection::Node(n.id));
        let local = current(&n.evidence);
        let settlement = n.board_quantity.is_some();
        let radius = (if settlement { 29. } else { 22. }) * camera.zoom.clamp(0.8, 1.35);
        let color = if selected {
            GOLD
        } else if settlement {
            faded(TEAL, &n.evidence)
        } else {
            faded(MUTED, &n.evidence)
        };
        if selected {
            draw_poly(p.x, p.y, 6, radius + 9., 30., quiet(GOLD, 0.08));
            place_outline(p, radius + 7., 2., GOLD);
        }
        draw_poly(p.x, p.y, 6, radius, 30., if local { PANEL } else { BG });
        place_outline(
            p,
            radius,
            if local { 1.8 } else { 1. },
            quiet(color, if local { 1. } else { 0.6 }),
        );
        if settlement {
            // A roof-like civic marker means a known local request institution;
            // it is not a claim that a physical house/building was created.
            draw_line(p.x - 12., p.y - 3., p.x, p.y - 13., 2., color);
            draw_line(p.x, p.y - 13., p.x + 12., p.y - 3., 2., color);
            draw_rectangle_lines(p.x - 9., p.y - 3., 18., 15., 1.5, color);
            draw_rectangle(p.x - 2.5, p.y + 3., 5., 9., color);
        } else {
            draw_circle_lines(p.x, p.y, 4.5, 1.3, color);
            if local {
                draw_circle(p.x, p.y, 2., color);
            }
        }
        centered_label(
            &node_name(n.id),
            p + vec2(0., 66.),
            if settlement || selected { 19. } else { 16. },
            color,
        );
        if selected || !local {
            let detail = if !n.contents_known {
                "Map knowledge only".into()
            } else if local {
                "Observed here".into()
            } else {
                format!("Last seen {}", age(snapshot.now, n.evidence.observed_at))
            };
            centered_label(&detail, p + vec2(0., 84.), 12., quiet(MUTED, 0.8));
        }
        hits.push(Hit {
            selection: Selection::Node(n.id),
            point: p,
            radius: radius + 4.,
        });
    }
    // Use node-local packing: discovering goods/fields elsewhere must not shift
    // the icons attached to a different location.
    let mut field_slots = BTreeMap::<usize, usize>::new();
    for f in &snapshot.fields {
        let slot = field_slots.entry(f.node).or_default();
        let base = node_position(f.node)
            + vec2(
                -70. + (*slot % 3) as f32 * 30.,
                -81. - (*slot / 3) as f32 * 23.,
            );
        *slot += 1;
        let p = camera.project(base, viewport);
        if !visible(p, viewport) {
            continue;
        }
        let selected = selection == Some(Selection::Field(f.id));
        let green = faded(
            if f.crop.is_some() {
                Color::new(0.48, 0.62, 0.33, 1.)
            } else {
                MUTED
            },
            &f.evidence,
        );
        draw_rectangle(p.x - 12., p.y - 8., 24., 16., BG);
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
        if selected {
            selection_box(p, vec2(12., 8.));
        }
        hits.push(Hit {
            selection: Selection::Field(f.id),
            point: p,
            radius: 14.,
        });
    }
    let mut goods_slots = BTreeMap::<usize, usize>::new();
    for g in snapshot.goods.iter().filter(|g| g.node.is_some()) {
        let node = g.node.unwrap();
        let slot = goods_slots.entry(node).or_default();
        let p = camera.project(
            node_position(node)
                + vec2(
                    69. + (*slot % 2) as f32 * 87.,
                    -34. - (*slot / 2) as f32 * 27.,
                ),
            viewport,
        );
        *slot += 1;
        if !visible(p, viewport) {
            continue;
        }
        let selected = selection == Some(Selection::Goods(g.id));
        let details = selected || selection == Some(Selection::Node(node)) || camera.zoom > 1.45;
        let name = short_resource(&g.material);
        let color = faded(
            match name {
                "Food" => GOLD,
                "Wood" => Color::new(0.62, 0.46, 0.32, 1.),
                _ => MUTED,
            },
            &g.evidence,
        );
        draw_rectangle(p.x - 7., p.y - 7., 14., 14., BG);
        if current(&g.evidence) {
            draw_rectangle(p.x - 6., p.y - 6., 12., 12., quiet(color, 0.22));
        }
        draw_rectangle_lines(p.x - 7., p.y - 7., 14., 14., 1.5, color);
        draw_line(p.x - 5., p.y + 4., p.x + 5., p.y - 4., 1., color);
        if details {
            text(
                &format!("{} {}", g.quantity, name),
                p.x + 12.,
                p.y + 5.,
                13.,
                color,
            );
        }
        if selected {
            selection_box(p, vec2(8., 8.));
        }
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
        let selected = selection == Some(Selection::Crossing(c.id));
        let color = faded(
            if c.completed == Some(true) {
                TEAL
            } else {
                GOLD
            },
            &c.evidence,
        );
        draw_rectangle(p.x - 31., p.y - 17., 62., 34., BG);
        if selected {
            selection_box(p, vec2(32., 18.));
        }
        draw_line(p.x - 24., p.y - 9., p.x + 24., p.y - 9., 2.5, color);
        draw_line(p.x - 24., p.y + 9., p.x + 24., p.y + 9., 2.5, color);
        for x in -2..=2 {
            draw_line(
                p.x + x as f32 * 9.,
                p.y - 8.,
                p.x + x as f32 * 9.,
                p.y + 8.,
                1.,
                color,
            );
        }
        for x in [-28., 28.] {
            draw_rectangle(p.x + x - 3., p.y - 14., 6., 28., color);
        }
        let label = if c.completed == Some(true) {
            "Crossing open"
        } else if c.authorized == Some(true) {
            "Crossing commissioned"
        } else {
            "Crossing site"
        };
        centered_label(
            label,
            p + vec2(0., 42.),
            15.,
            if selected { GOLD } else { color },
        );
        if selected && !current(&c.evidence) {
            centered_label(
                &format!("Last seen {}", age(snapshot.now, c.evidence.observed_at)),
                p + vec2(0., 59.),
                12.,
                MUTED,
            );
        }
        hits.push(Hit {
            selection: Selection::Crossing(c.id),
            point: p,
            radius: 32.,
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
        let color = faded(GOLD, &c.evidence);
        draw_rectangle_lines(p.x - 10., p.y - 6., 20., 12., 1.5, color);
        draw_circle_lines(p.x - 7., p.y + 8., 3., 1.5, color);
        draw_circle_lines(p.x + 7., p.y + 8., 3., 1.5, color);
        if selection == Some(Selection::Cart(c.id)) {
            selection_box(p, vec2(12., 12.));
        }
        hits.push(Hit {
            selection: Selection::Cart(c.id),
            point: p,
            radius: 14.,
        });
    }
    let selected_members: BTreeSet<_> = selection
        .and_then(|s| match s {
            Selection::Unit(id) => snapshot.units.iter().find(|u| u.id == id),
            _ => None,
        })
        .into_iter()
        .flat_map(|u| u.member_ids.iter().copied())
        .collect();
    let mut drawn_people = BTreeMap::new();
    for p in &snapshot.persons {
        let mut pos = person_position(p);
        if current(&p.evidence)
            && let Some(old) = old_people.get(&p.id)
        {
            // Both poses have already executed; no extrapolation or movement law.
            pos = person_position(old).lerp(pos, alpha.clamp(0., 1.));
        }
        let at = camera.project(pos, viewport);
        drawn_people.insert(p.id, at);
        if !visible(at, viewport) {
            continue;
        }
        let dead = p.action.as_deref().is_some_and(|a| a == "dead");
        let color = if p.id == snapshot.viewer {
            GOLD
        } else if dead {
            MUTED
        } else if p.hostile == Some(true) {
            RED
        } else if p.unit.is_some() {
            TEAL
        } else {
            INK
        };
        let radius = (4.7 * camera.zoom.sqrt()).clamp(3., 8.);
        if selected_members.contains(&p.id) {
            draw_circle_lines(at.x, at.y, radius + 5., 1.4, quiet(GOLD, 0.65));
        }
        if current(&p.evidence) {
            draw_circle(at.x, at.y, radius, color);
        } else {
            draw_circle_lines(
                at.x,
                at.y,
                radius,
                1.,
                quiet(faded(color, &p.evidence), 0.65),
            );
        }
        if dead {
            draw_line(at.x - 4., at.y - 4., at.x + 4., at.y + 4., 1., BG);
        }
        if p.id == snapshot.viewer {
            draw_circle_lines(at.x, at.y, radius + 3.5, 1.3, GOLD);
        }
        if selection == Some(Selection::Person(p.id)) {
            draw_circle_lines(at.x, at.y, radius + 8., 3., quiet(GOLD, 0.18));
            draw_circle_lines(at.x, at.y, radius + 7., 2., GOLD);
        }
        if snapshot.mode == ViewMode::Debug || camera.zoom > 1.6 {
            text(&p.id.to_string(), at.x + 7., at.y - 5., 12., MUTED);
        }
        hits.push(Hit {
            selection: Selection::Person(p.id),
            point: at,
            radius: radius + 5.,
        });
    }
    for u in &snapshot.units {
        let points: Vec<_> = u
            .member_ids
            .iter()
            .filter_map(|id| drawn_people.get(id).copied())
            .collect();
        if points.is_empty() {
            continue;
        }
        let mean = points.iter().copied().fold(Vec2::ZERO, |a, b| a + b) / points.len() as f32;
        let p = mean + vec2(0., -73.);
        if !visible(p, viewport) {
            continue;
        }
        let selected = selection == Some(Selection::Unit(u.id));
        let color = if selected { GOLD } else { TEAL };
        // A leader line links the badge to observed members, not a territory or
        // continuous formation boundary. No unseen member contributes a point.
        draw_line(p.x, p.y + 24., mean.x, mean.y - 13., 1., quiet(color, 0.4));
        draw_rectangle(p.x - 90., p.y - 31., 180., 62., PANEL);
        draw_rectangle_lines(
            p.x - 90.,
            p.y - 31.,
            180.,
            62.,
            if selected { 2. } else { 1. },
            color,
        );
        if selected {
            selection_box(p, vec2(91., 32.));
        }
        let flag = p + vec2(-75., -14.);
        draw_line(flag.x, flag.y - 3., flag.x, flag.y + 18., 1.5, color);
        draw_triangle(
            flag + vec2(0., -3.),
            flag + vec2(13., 1.),
            flag + vec2(0., 6.),
            color,
        );
        text(&format!("UNIT {}", u.id), p.x - 52., p.y - 10., 17., color);
        text(
            &format!("{} in view", u.member_ids.len()),
            p.x - 52.,
            p.y + 7.,
            12.,
            MUTED,
        );
        let state = crate::presentation::unit_state(snapshot, u.id);
        fitted(&state, p.x - 79., p.y + 24., 158., 12., MUTED);
        // Overlapping hit disks cover the full badge without turning the space
        // between badge and soldiers into an oversized invisible button.
        for x in [-58., 0., 58.] {
            hits.push(Hit {
                selection: Selection::Unit(u.id),
                point: p + vec2(x, 0.),
                radius: 34.,
            });
        }
    }
    // Inspectors and command bars own pixels outside the schematic map.
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
    let node = match selection {
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
    };
    node.filter(|id| s.nodes.iter().any(|n| n.id == *id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use world_of_nodes::provisioning::playable::{ScenarioKind, Session};

    #[test]
    fn composing_target_never_promotes_a_node_absent_from_supplied_knowledge() {
        let session = Session::new(0, ScenarioKind::Local).unwrap();
        let mut play = session.snapshot(ViewMode::Play);
        let node = selected_node(&play, Selection::Person(play.viewer)).unwrap();
        assert_eq!(selected_node(&play, Selection::Node(node)), Some(node));
        assert_eq!(selected_node(&play, Selection::Node(usize::MAX)), None);
        play.nodes.retain(|n| n.id != node);
        assert_eq!(selected_node(&play, Selection::Node(node)), None);
        assert_eq!(selected_node(&play, Selection::Person(play.viewer)), None);
    }
}
