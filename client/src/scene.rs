//! Spatial projection of known authoritative objects; no World or Session access.
use crate::camera::{MapCamera, node_name, node_position};
use crate::widgets::*;
use crate::widgets::{GOLD, RED};
use crate::{glyphs, spatial};
use macroquad::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use world_of_nodes::provisioning::playable::snapshot::*;
use world_of_nodes::provisioning::{Holder, Location, SCALE};

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
    pub rect: Option<Rect>,
}
fn visible(p: Vec2, v: Rect) -> bool {
    v.contains(p)
}
fn label(value: &str, p: Vec2, size: f32, color: Color) -> Rect {
    let width = text_width(value, size);
    draw_rectangle(
        p.x - width * 0.5 - 7.,
        p.y - size - 1.,
        width + 14.,
        size + 9.,
        Color::new(0.12, 0.18, 0.15, 0.83),
    );
    text(value, p.x - width * 0.5, p.y, size, color);
    Rect::new(
        p.x - width * 0.5 - 7.,
        p.y - size - 1.,
        width + 14.,
        size + 9.,
    )
}
fn bounds_mask(viewport: Rect) {
    draw_rectangle(0., 0., screen_width(), viewport.y, PANEL);
    draw_rectangle(
        viewport.right(),
        viewport.y,
        (screen_width() - viewport.right()).max(0.),
        screen_height() - viewport.y,
        PANEL,
    );
    draw_rectangle(
        0.,
        viewport.bottom(),
        screen_width(),
        (screen_height() - viewport.bottom()).max(0.),
        PANEL,
    );
}
/// Normal play includes quiet place labels and compact group markers.
pub fn draw(
    s: &Snapshot,
    previous: Option<&Snapshot>,
    alpha: f32,
    camera: &MapCamera,
    viewport: Rect,
    selection: Option<Selection>,
) -> Vec<Hit> {
    draw_world(s, previous, alpha, camera, viewport, selection, true)
}
/// Screenshot/exploration projection with no panels, labels, IDs or quantities.
pub fn draw_unlabelled(
    s: &Snapshot,
    previous: Option<&Snapshot>,
    alpha: f32,
    camera: &MapCamera,
    viewport: Rect,
    selection: Option<Selection>,
) -> Vec<Hit> {
    draw_world(s, previous, alpha, camera, viewport, selection, false)
}
fn draw_world(
    s: &Snapshot,
    previous: Option<&Snapshot>,
    alpha: f32,
    camera: &MapCamera,
    viewport: Rect,
    selection: Option<Selection>,
    labels: bool,
) -> Vec<Hit> {
    let mut hits = Vec::new();
    let z = camera.zoom;
    spatial::ground(s, camera, viewport);
    // Only known graph edges create routes. Width means observed path wear;
    // screen length, texture and curvature have no effect on travel/capacity.
    for e in &s.edges {
        let selected = selection == Some(Selection::Edge(e.id));
        let historical = !spatial::current(&e.evidence);
        let width = match e.path_stage.as_deref() {
            Some("worn_path") => 11.,
            Some("trail") => 7.,
            _ => 4.,
        };
        if selected {
            spatial::route_line(
                e.a,
                e.b,
                (width + 8.) * z,
                Color::new(1., 0.84, 0.34, 0.45),
                camera,
                viewport,
                false,
            );
        }
        if e.capacity > 0 {
            if historical {
                spatial::route_line(
                    e.a,
                    e.b,
                    2.0 * z.max(0.7),
                    Color::new(0.26, 0.32, 0.26, 0.55),
                    camera,
                    viewport,
                    true,
                );
            } else {
                spatial::route_line(
                    e.a,
                    e.b,
                    (width + 3.) * z,
                    Color::new(0.32, 0.35, 0.24, 0.60),
                    camera,
                    viewport,
                    false,
                );
                spatial::route_line(e.a, e.b, width * z, spatial::SAND, camera, viewport, false);
                spatial::route_line(
                    e.a,
                    e.b,
                    (width * 0.32).max(1.) * z,
                    Color::new(0.76, 0.70, 0.51, 0.40),
                    camera,
                    viewport,
                    false,
                );
            }
        } else {
            spatial::route_line(
                e.a,
                e.b,
                2. * z,
                Color::new(0.27, 0.32, 0.24, 0.50),
                camera,
                viewport,
                true,
            );
        }
        for i in 2..15 {
            let p = camera.project(spatial::route_point(e.a, e.b, i as f32 / 16.), viewport);
            if visible(p, viewport) {
                hits.push(Hit {
                    selection: Selection::Edge(e.id),
                    point: p,
                    radius: (9. * z).max(6.),
                    rect: None,
                });
            }
        }
        if selected && labels {
            let p = camera.project(spatial::route_point(e.a, e.b, 0.5), viewport);
            let value = if e.capacity == 0 {
                "Unopened route".into()
            } else {
                format!("{} · capacity {}", hours(e.nominal_ms), e.capacity)
            };
            label(&value, p + vec2(0., -25.), 13., GOLD);
            if let Some(q) = e.queued.filter(|q| *q > 0) {
                label(
                    &format!("{q} queued · observed bank"),
                    p + vec2(0., 25.),
                    12.,
                    INK,
                );
            }
        }
    }
    for n in &s.nodes {
        let at = camera.project(node_position(n.id), viewport);
        if !visible(at, viewport) {
            continue;
        }
        spatial::place(
            n,
            camera,
            viewport,
            selection == Some(Selection::Node(n.id)),
        );
        if labels {
            let selected = selection == Some(Selection::Node(n.id));
            let name = node_name(n.id);
            if n.board_quantity.is_some() || selected {
                label(
                    &name,
                    at + vec2(-10., 106. * z),
                    17.,
                    if selected { GOLD } else { INK },
                );
            } else {
                text(
                    &name,
                    at.x - text_width(&name, 13.) * 0.5,
                    at.y + 32. * z,
                    13.,
                    spatial::tint(spatial::INK, &n.evidence),
                );
            }
        }
        hits.push(Hit {
            selection: Selection::Node(n.id),
            point: at,
            radius: 32. * z.max(0.65),
            rect: None,
        });
    }
    let mut field_slots = BTreeMap::<usize, usize>::new();
    for f in &s.fields {
        let slot = field_slots.entry(f.node).or_default();
        let at = camera.project(spatial::field_position(f.node, *slot), viewport);
        *slot += 1;
        if !visible(at, viewport) {
            continue;
        }
        let selected = selection == Some(Selection::Field(f.id));
        glyphs::field(f, at, z, selected);
        if selected && labels {
            label(
                &format!("Plot · {}", f.stage),
                at + vec2(0., -25. * z),
                13.,
                GOLD,
            );
        }
        hits.push(Hit {
            selection: Selection::Field(f.id),
            point: at,
            radius: 29. * z,
            rect: None,
        });
    }
    let mut stock_slots = BTreeMap::<usize, usize>::new();
    let mut wood_slots = BTreeMap::<usize, usize>::new();
    for g in s.goods.iter().filter(|g| g.node.is_some()) {
        let node = g.node.unwrap();
        let wood = matches!(g.holder, Holder::Deposit(_)) && g.material.contains("Wood");
        let slot = if wood {
            wood_slots.entry(node).or_default()
        } else {
            stock_slots.entry(node).or_default()
        };
        let at = camera.project(spatial::goods_position(node, *slot, wood), viewport);
        *slot += 1;
        if !visible(at, viewport) {
            continue;
        }
        spatial::wood_zone(g, at, z);
        let selected = selection == Some(Selection::Goods(g.id));
        glyphs::goods(g, at, z, selected);
        if labels && selected {
            let kind = if wood {
                "Wood source"
            } else {
                g.material.as_str()
            };
            label(
                &format!("{} {}", g.quantity, kind),
                at + vec2(0., 25. * z),
                12.,
                if selected { GOLD } else { INK },
            );
        }
        hits.push(Hit {
            selection: Selection::Goods(g.id),
            point: at,
            radius: 20. * z,
            rect: None,
        });
    }
    for c in &s.crossings {
        let at = camera.project(spatial::crossing_position(c, s), viewport);
        if !visible(at, viewport) {
            continue;
        }
        // Explicit schematic barrier. No unmodeled river, hydrology or extra
        // blocked edges are invented. Only this known crossing spans the gap.
        let gap = spatial::tint(Color::new(0.22, 0.27, 0.20, 1.), &c.evidence);
        // Irregular bank cut, not a pond or a claim of modeled water.
        let left = [
            vec2(-13., -48.),
            vec2(-21., -27.),
            vec2(-18., -7.),
            vec2(-22., 18.),
            vec2(-12., 45.),
        ];
        let right = [
            vec2(12., -48.),
            vec2(21., -27.),
            vec2(18., -7.),
            vec2(23., 18.),
            vec2(14., 45.),
        ];
        for i in 0..4 {
            let a = at + left[i] * z;
            let b = at + right[i] * z;
            let p3 = at + left[i + 1] * z;
            let d = at + right[i + 1] * z;
            draw_triangle(a, b, p3, gap);
            draw_triangle(b, p3, d, gap);
            let bank = spatial::tint(Color::new(0.65, 0.60, 0.43, 1.), &c.evidence);
            draw_line(a.x, a.y, p3.x, p3.y, 3. * z, bank);
            draw_line(b.x, b.y, d.x, d.y, 3. * z, bank);
        }
        let selected = selection == Some(Selection::Crossing(c.id));
        glyphs::crossing(c, at, z, selected);
        if labels {
            let state = if c.completed == Some(true) {
                "Crossing open"
            } else if c.authorized == Some(true) {
                "Crossing · work site"
            } else {
                "Crossing opportunity"
            };
            label(
                state,
                at + vec2(0., 62. * z),
                14.,
                if selected { GOLD } else { INK },
            );
        }
        hits.push(Hit {
            selection: Selection::Crossing(c.id),
            point: at,
            radius: 36. * z,
            rect: None,
        });
    }
    let poses = spatial::person_positions(s);
    let old_poses = previous.map(spatial::person_positions).unwrap_or_default();
    let old_current: BTreeSet<_> = previous
        .into_iter()
        .flat_map(|s| &s.persons)
        .filter(|p| spatial::current(&p.evidence))
        .map(|p| p.id)
        .collect();
    let selected_members: BTreeSet<_> = selection
        .and_then(|sel| {
            if let Selection::Unit(id) = sel {
                s.units.iter().find(|u| u.id == id)
            } else {
                None
            }
        })
        .into_iter()
        .flat_map(|u| u.member_ids.iter().copied())
        .collect();
    let mut drawn = BTreeMap::new();
    for p in &s.persons {
        let mut pos = poses[&p.id];
        if spatial::current(&p.evidence)
            && old_current.contains(&p.id)
            && let Some(old) = old_poses.get(&p.id)
        {
            pos = old.lerp(pos, alpha.clamp(0., 1.));
        }
        let at = camera.project(pos, viewport);
        drawn.insert(p.id, at);
        if !visible(at, viewport) {
            continue;
        }
        if selected_members.contains(&p.id) {
            draw_circle_lines(at.x, at.y, 12. * z, 1.5, GOLD);
        }
        glyphs::person(
            p,
            at,
            z,
            p.id == s.viewer,
            selection == Some(Selection::Person(p.id)),
        );
        // Known action marks describe the real current action, never extra work.
        if spatial::current(&p.evidence) {
            let mark = match p.action.as_deref() {
                Some("sleeping") => Some("z"),
                Some("queued") => Some("· ·"),
                Some("striking") => Some("×"),
                _ => None,
            };
            if labels && let Some(mark) = mark {
                text(
                    mark,
                    at.x + 7. * z,
                    at.y - 9. * z,
                    12.,
                    if p.action.as_deref() == Some("striking") {
                        RED
                    } else {
                        spatial::INK
                    },
                );
            }
        }
        if labels && (selection == Some(Selection::Person(p.id)) || camera.zoom > 1.8) {
            label(
                &format!("Person {}", p.id),
                at + vec2(0., -22. * z),
                12.,
                GOLD,
            );
        }
        hits.push(Hit {
            selection: Selection::Person(p.id),
            point: at,
            radius: (10. * z).max(5.),
            rect: None,
        });
    }
    let people: BTreeMap<_, _> = s.persons.iter().map(|p| (p.id, p)).collect();
    let edge_times: BTreeMap<_, _> = s.edges.iter().map(|e| (e.id, e.nominal_ms)).collect();
    for c in &s.carts {
        let pos = match c.location {
            Location::Node(n) => node_position(n) + vec2(-114., 35. + (c.id % 4) as f32 * 22.),
            Location::Edge {
                edge,
                from,
                to,
                progress,
                ..
            } => {
                let nominal = c
                    .user
                    .and_then(|id| people.get(&id))
                    .and_then(|p| p.traversal_nominal_ms)
                    .or_else(|| edge_times.get(&edge).copied())
                    .unwrap_or(1)
                    .max(1);
                spatial::route_point(
                    from,
                    to,
                    (progress as f64 / (nominal as f64 * SCALE as f64)).clamp(0., 1.) as f32,
                )
            }
        };
        let at = camera.project(pos, viewport);
        if !visible(at, viewport) {
            continue;
        }
        glyphs::cart(c, at, z, selection == Some(Selection::Cart(c.id)));
        hits.push(Hit {
            selection: Selection::Cart(c.id),
            point: at,
            radius: 20. * z,
            rect: None,
        });
    }
    // One marker per actually observed same-location group. No badge floats
    // at a fictitious global army centroid across different nodes/edges.
    for u in &s.units {
        let mut clusters = BTreeMap::<(u8, usize, usize, u8), Vec<Vec2>>::new();
        for id in &u.member_ids {
            let Some(p) = people.get(id) else {
                continue;
            };
            let Some(at) = drawn.get(id) else {
                continue;
            };
            let key = match p.location {
                Location::Node(n) => (0, n, 0, 0),
                Location::Edge {
                    edge,
                    from,
                    progress,
                    ..
                } => {
                    let work = p.traversal_nominal_ms.unwrap_or(1).max(1) as f64 * SCALE as f64;
                    (
                        1,
                        edge,
                        from,
                        ((progress as f64 / work).clamp(0., 0.999) * 4.) as u8,
                    )
                }
            };
            clusters.entry(key).or_default().push(*at);
        }
        for (cluster, points) in &clusters {
            let mean = points.iter().copied().fold(Vec2::ZERO, |a, b| a + b) / points.len() as f32;
            let at = mean + vec2(-34., -12.) * z;
            if !visible(at, viewport) {
                continue;
            }
            let selected = selection == Some(Selection::Unit(u.id));
            let color = if selected { GOLD } else { TEAL };
            draw_line(
                at.x,
                at.y - 9. * z,
                at.x,
                at.y + 23. * z,
                3. * z,
                spatial::INK,
            );
            draw_triangle(
                at + vec2(1., -8.) * z,
                at + vec2(23., -2.) * z,
                at + vec2(1., 9.) * z,
                spatial::INK,
            );
            draw_triangle(
                at + vec2(3., -6.) * z,
                at + vec2(19., -2.) * z,
                at + vec2(3., 6.) * z,
                color,
            );
            if labels {
                let suffix = if cluster.0 == 0 { "here" } else { "on route" };
                let area = label(
                    &format!("Unit {} · {} {suffix}", u.id, points.len()),
                    at + vec2(-18., -22.) * z,
                    13.,
                    color,
                );
                hits.push(Hit {
                    selection: Selection::Unit(u.id),
                    point: area.center(),
                    radius: 1.,
                    rect: Some(area),
                });
                if selected {
                    label(
                        &crate::presentation::unit_state(s, u.id),
                        mean + vec2(0., 42. * z),
                        12.,
                        INK,
                    );
                }
            }
            if selected {
                draw_circle_lines(at.x + 8. * z, at.y, 19. * z, 2., GOLD);
            }
            hits.push(Hit {
                selection: Selection::Unit(u.id),
                point: at + vec2(8., 0.) * z,
                radius: 20. * z.max(0.65),
                rect: None,
            });
        }
    }
    bounds_mask(viewport);
    hits
}

pub fn hit_test(hits: &[Hit], mouse: Vec2) -> Option<Selection> {
    if let Some(hit) = hits
        .iter()
        .rev()
        .find(|h| h.rect.is_some_and(|r| r.contains(mouse)))
    {
        return Some(hit.selection);
    }
    hits.iter()
        .rev()
        .filter(|h| h.rect.is_none() && h.point.distance(mouse) <= h.radius)
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
