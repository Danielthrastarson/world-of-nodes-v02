//! Display-only geography and packing over the permitted snapshot.
//! Distances, footprints and within-node positions are schematic, never physics.
use crate::camera::{MapCamera, node_position};
use macroquad::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use world_of_nodes::provisioning::playable::snapshot::*;
use world_of_nodes::provisioning::{Holder, Location, SCALE};

pub const LAND: Color = Color::new(0.42, 0.48, 0.36, 1.);
pub const INK: Color = Color::new(0.13, 0.19, 0.16, 1.);
pub const SAND: Color = Color::new(0.67, 0.62, 0.44, 1.);

pub fn current(e: &Evidence) -> bool {
    matches!(
        e.source,
        KnowledgeSource::Local | KnowledgeSource::SelfKnowledge | KnowledgeSource::Debug
    )
}
pub fn tint(c: Color, e: &Evidence) -> Color {
    if current(e) {
        c
    } else {
        Color::new(
            c.r * 0.7 + LAND.r * 0.3,
            c.g * 0.7 + LAND.g * 0.3,
            c.b * 0.7 + LAND.b * 0.3,
            0.65,
        )
    }
}
/// Same visual curve in both directions. It does not add length to kernel travel.
pub fn route_point(from: usize, to: usize, fraction: f32) -> Vec2 {
    let (low, high, t) = if from <= to {
        (from, to, fraction)
    } else {
        (to, from, 1. - fraction)
    };
    let a = node_position(low);
    let b = node_position(high);
    let tangent = (b - a).normalize_or_zero();
    let bend = (((low.wrapping_mul(3).wrapping_add(high)) % 3) as f32 - 1.) * 27.;
    a.lerp(b, t) + vec2(-tangent.y, tangent.x) * (std::f32::consts::PI * t).sin() * bend
}
pub fn route_line(
    a: usize,
    b: usize,
    width: f32,
    color: Color,
    camera: &MapCamera,
    viewport: Rect,
    dashed: bool,
) {
    for segment in 0..32 {
        if dashed && segment % 3 == 2 {
            continue;
        }
        let p = camera.project(route_point(a, b, segment as f32 / 32.), viewport);
        let q = camera.project(route_point(a, b, (segment + 1) as f32 / 32.), viewport);
        draw_line(p.x, p.y, q.x, q.y, width, color);
    }
}

pub fn field_position(node: usize, slot: usize) -> Vec2 {
    node_position(node)
        + vec2(
            -83. + (slot % 3) as f32 * 66.,
            -113. - (slot / 3) as f32 * 47.,
        )
}
pub fn goods_position(node: usize, slot: usize, wood_deposit: bool) -> Vec2 {
    node_position(node)
        + if wood_deposit {
            vec2(126., 62. + slot as f32 * 34.)
        } else {
            vec2(
                66. + (slot % 2) as f32 * 41.,
                -33. - (slot / 2) as f32 * 32.,
            )
        }
}
pub fn crossing_position(c: &CrossingSnapshot, s: &Snapshot) -> Vec2 {
    c.edge
        .and_then(|id| s.edges.iter().find(|e| e.id == id))
        .map(|e| route_point(e.a, e.b, 0.5))
        .unwrap_or_else(|| node_position(c.node) + vec2(95., -8.))
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Packing {
    Military(usize),
    Waiting,
    Working,
    Civilian,
}
fn group(p: &PersonSnapshot) -> Packing {
    if p.action.as_deref() == Some("queued") {
        Packing::Waiting
    } else if let Some(unit) = p.unit {
        Packing::Military(unit)
    } else if p.action.as_deref() == Some("cultivating") {
        Packing::Working
    } else {
        Packing::Civilian
    }
}
fn anchor(group: Packing) -> Vec2 {
    match group {
        Packing::Military(_) => vec2(-65., 38.),
        Packing::Waiting => vec2(76., 20.),
        Packing::Working => vec2(-48., -69.),
        Packing::Civilian => vec2(14., 47.),
    }
}
/// One position per existing visible/remembered Person. No dot population is
/// generated from a count. Local ranks avoid huge IDs sending a person off-map.
pub fn person_positions(s: &Snapshot) -> BTreeMap<usize, Vec2> {
    let mut slots = BTreeMap::<(usize, Packing), usize>::new();
    let mut people: Vec<_> = s.persons.iter().collect();
    people.sort_by_key(|p| p.id);
    people
        .into_iter()
        .map(|p| {
            let at = match p.location {
                Location::Node(node) => {
                    let group = group(p);
                    let slot = slots.entry((node, group)).or_default();
                    let rank = *slot as f32;
                    *slot += 1;
                    // Low-count clusters read as groups; high-count area grows.
                    // This is packing only, not a Line/Column military formation.
                    let radius = rank.sqrt() * 15.;
                    let angle = rank * 2.399_963_1;
                    node_position(node) + anchor(group) + vec2(angle.cos(), angle.sin()) * radius
                }
                Location::Edge {
                    from, to, progress, ..
                } => {
                    let work = p.traversal_nominal_ms.unwrap_or(1).max(1) as f64 * SCALE as f64;
                    let t = (progress as f64 / work).clamp(0., 1.) as f32;
                    let tangent = (node_position(to) - node_position(from)).normalize_or_zero();
                    route_point(from, to, t)
                        + vec2(-tangent.y, tangent.x) * ((p.id % 3) as f32 - 1.) * 3.
                }
            };
            (p.id, at)
        })
        .collect()
}

/// Broad, irregular place footprints are visual grouping, not territory or fog.
/// Off-route background is neutral and says nothing about traversability.
pub fn ground(s: &Snapshot, camera: &MapCamera, viewport: Rect) {
    draw_rectangle(viewport.x, viewport.y, viewport.w, viewport.h, LAND);
    let scale = camera.zoom;
    let field_nodes: BTreeSet<_> = s.fields.iter().map(|f| f.node).collect();
    for n in &s.nodes {
        if !n.contents_known || (n.board_quantity.is_none() && !field_nodes.contains(&n.id)) {
            continue;
        }
        let at = camera.project(node_position(n.id), viewport);
        let c = if current(&n.evidence) {
            Color::new(0.47, 0.52, 0.38, 1.)
        } else {
            Color::new(0.46, 0.51, 0.40, 1.)
        };
        for (dx, dy, rx, ry) in [
            (-52., -30., 112., 93.),
            (32., -15., 119., 81.),
            (17., 49., 115., 78.),
        ] {
            draw_ellipse(
                at.x + dx * scale,
                at.y + dy * scale,
                rx * scale,
                ry * scale,
                0.,
                c,
            );
        }
        // Patchwork is soil/open land, never extra plots, crop yield or trees.
        let soil = Color::new(c.r + 0.02, c.g + 0.014, c.b - 0.02, 0.38);
        draw_ellipse(
            at.x - 30. * scale,
            at.y + 12. * scale,
            78. * scale,
            45. * scale,
            -13.,
            soil,
        );
    }
}

pub fn place(n: &NodeSnapshot, camera: &MapCamera, viewport: Rect, selected: bool) {
    let at = camera.project(node_position(n.id), viewport);
    let z = camera.zoom;
    if !n.contents_known {
        // Open chart mark: contents unknown, not an empty settlement.
        for i in 0..8 {
            let a = i as f32 * std::f32::consts::TAU / 8.;
            let b = a + 0.38;
            draw_line(
                at.x + a.cos() * 20. * z,
                at.y + a.sin() * 20. * z,
                at.x + b.cos() * 20. * z,
                at.y + b.sin() * 20. * z,
                1.5,
                Color::new(0.26, 0.33, 0.28, 0.65),
            );
        }
        draw_circle(at.x, at.y, 2. * z, INK);
    } else if n.board_quantity.is_some() {
        // One civic place glyph, not newly simulated housing/buildings.
        let c = tint(Color::new(0.83, 0.77, 0.59, 1.), &n.evidence);
        draw_ellipse(
            at.x + 4. * z,
            at.y + 10. * z,
            33. * z,
            17. * z,
            0.,
            Color::new(0.19, 0.25, 0.19, 0.25),
        );
        draw_rectangle(at.x - 24. * z, at.y - 13. * z, 48. * z, 35. * z, INK);
        draw_rectangle(at.x - 21. * z, at.y - 11. * z, 42. * z, 30. * z, c);
        draw_triangle(
            at + vec2(-31., -10.) * z,
            at + vec2(0., -35.) * z,
            at + vec2(31., -10.) * z,
            INK,
        );
        draw_triangle(
            at + vec2(-26., -12.) * z,
            at + vec2(0., -31.) * z,
            at + vec2(26., -12.) * z,
            tint(Color::new(0.48, 0.31, 0.23, 1.), &n.evidence),
        );
        draw_rectangle(at.x - 6. * z, at.y + 2. * z, 12. * z, 17. * z, INK);
        for x in [-14., 14.] {
            draw_rectangle(at.x + (x - 3.) * z, at.y - 4. * z, 6. * z, 7. * z, INK);
        }
        // The request board is an existing institution.
        draw_line(
            at.x + 33. * z,
            at.y + 2. * z,
            at.x + 33. * z,
            at.y + 24. * z,
            2. * z,
            INK,
        );
        draw_rectangle(
            at.x + 27. * z,
            at.y + 1. * z,
            14. * z,
            12. * z,
            tint(crate::widgets::GOLD, &n.evidence),
        );
    } else {
        draw_ellipse(
            at.x,
            at.y,
            21. * z,
            11. * z,
            0.,
            Color::new(0.57, 0.58, 0.43, 0.7),
        );
        draw_circle_lines(at.x, at.y, 7. * z, 1.5, tint(INK, &n.evidence));
    }
    if selected {
        draw_ellipse_lines(
            at.x,
            at.y + 1. * z,
            44. * z,
            34. * z,
            0.,
            5.,
            Color::new(0.98, 0.82, 0.40, 0.15),
        );
        draw_ellipse_lines(
            at.x,
            at.y + 1. * z,
            42. * z,
            32. * z,
            0.,
            2.,
            crate::widgets::GOLD,
        );
    }
}
/// Resource-zone symbol only when a known physical Wood deposit exists.
/// The canopy silhouette stands for the deposit, not individually simulated trees.
pub fn wood_zone(g: &GoodsSnapshot, at: Vec2, scale: f32) {
    if !matches!(g.holder, Holder::Deposit(_)) || !g.material.contains("Wood") || g.quantity == 0 {
        return;
    }
    let c = tint(Color::new(0.21, 0.34, 0.24, 1.), &g.evidence);
    draw_ellipse(
        at.x + 9. * scale,
        at.y - 13. * scale,
        37. * scale,
        29. * scale,
        -12.,
        tint(Color::new(0.36, 0.43, 0.28, 1.), &g.evidence),
    );
    for (x, y, r) in [(-17., -20., 13.), (7., -32., 16.), (27., -16., 12.)] {
        draw_rectangle(
            at.x + (x - 1.5) * scale,
            at.y + (y + 3.) * scale,
            3. * scale,
            17. * scale,
            tint(Color::new(0.33, 0.26, 0.20, 1.), &g.evidence),
        );
        draw_circle(at.x + x * scale, at.y + y * scale, r * scale, c);
        draw_circle(
            at.x + (x - 4.) * scale,
            at.y + (y - 3.) * scale,
            (r - 4.) * scale,
            tint(Color::new(0.29, 0.42, 0.27, 1.), &g.evidence),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use world_of_nodes::provisioning::playable::{ScenarioKind, Session};
    #[test]
    fn curved_routes_retrace_and_keep_authoritative_endpoints() {
        for (a, b) in [(0, 1), (1, 2), (1, 3), (3, 2), (4, 2)] {
            assert!(route_point(a, b, 0.).distance(node_position(a)) < 0.001);
            assert!(route_point(a, b, 1.).distance(node_position(b)) < 0.001);
            for step in 0..=20 {
                let t = step as f32 / 20.;
                assert!(route_point(a, b, t).distance(route_point(b, a, 1. - t)) < 0.001);
            }
        }
    }
    #[test]
    fn packing_has_exactly_one_position_per_known_person_and_no_world_access() {
        let session = Session::new(0, ScenarioKind::Local).unwrap();
        let s = session.snapshot(ViewMode::Play);
        let positions = person_positions(&s);
        assert_eq!(positions.len(), s.persons.len());
        assert!(positions.values().all(|p| p.is_finite()));
        let mut sparse = s.clone();
        sparse.persons.retain(|p| p.unit.is_none());
        assert_eq!(person_positions(&sparse).len(), sparse.persons.len());
        assert!(s.nodes.iter().any(|n| !n.contents_known));
    }
}
