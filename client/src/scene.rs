//! Spatial projection of known authoritative objects; no World or Session access.
use crate::camera::{MapCamera, node_name};
use crate::spatial::v;
use crate::widgets::*;
use crate::widgets::{GOLD, RED};
use crate::world_presentation::{LAYERS, Layer, Lod, RegionKind, RenderSnapshot};
use crate::{glyphs, spatial};
use macroquad::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;
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
#[derive(Default, Debug)]
pub struct DrawStats {
    pub visible_persons: usize,
    pub effect_cues: usize,
    pub visible_effects: usize,
    pub layer_ms: [f64; 10],
}
pub struct DrawResult {
    pub hits: Vec<Hit>,
    pub stats: DrawStats,
}
struct Overlay {
    value: String,
    at: Vec2,
    size: f32,
    color: Color,
    boxed: bool,
}
fn overlay_label(
    overlays: &mut Vec<Overlay>,
    value: &str,
    at: Vec2,
    size: f32,
    color: Color,
) -> Rect {
    let width = text_width(value, size);
    overlays.push(Overlay {
        value: value.into(),
        at,
        size,
        color,
        boxed: true,
    });
    Rect::new(
        at.x - width * 0.5 - 7.,
        at.y - size - 1.,
        width + 14.,
        size + 9.,
    )
}
fn overlay_text(overlays: &mut Vec<Overlay>, value: &str, x: f32, y: f32, size: f32, color: Color) {
    overlays.push(Overlay {
        value: value.into(),
        at: vec2(x, y),
        size,
        color,
        boxed: false,
    });
}
/// Macroquad backend entry point. UI is composed by the caller after this pass.
pub fn render(
    frame: &RenderSnapshot,
    camera: &MapCamera,
    viewport: Rect,
    selection: Option<Selection>,
    labels: bool,
) -> DrawResult {
    let s = frame.observation;
    let mut hits = Vec::new();
    let z = camera.zoom;
    let lod = Lod::at_zoom(z);
    let labels = labels && lod != Lod::Far;
    let mut overlays = vec![];
    let mut stats = DrawStats {
        effect_cues: 0,
        ..Default::default()
    };
    let mut drawn = BTreeMap::new();
    for (layer_index, layer) in LAYERS.iter().enumerate() {
        let started = Instant::now();
        match layer {
            Layer::Terrain => spatial::ground(frame, camera, viewport),
            Layer::Water => {
                for region in frame.regions.iter().filter(|r| r.kind == RegionKind::Water) {
                    let at = camera.project(v(region.center), viewport);
                    draw_ellipse(
                        at.x,
                        at.y,
                        region.half_size.x * z,
                        region.half_size.y * z,
                        0.,
                        spatial::tint(Color::new(0.25, 0.42, 0.48, 1.), &region.evidence),
                    );
                }
            }
            Layer::ResourceAreas => {
                for f in &s.fields {
                    let at = camera.project(v(frame.fields[&f.id]), viewport);
                    if !visible(at, viewport) {
                        continue;
                    }
                    let selected = selection == Some(Selection::Field(f.id));
                    if lod == Lod::Close {
                        glyphs::field(f, at, z, false);
                    } else {
                        let color = if f.crop.is_some() {
                            Color::new(0.63, 0.60, 0.32, 1.)
                        } else {
                            Color::new(0.51, 0.59, 0.35, 1.)
                        };
                        draw_rectangle(
                            at.x - 28. * z,
                            at.y - 19. * z,
                            56. * z,
                            38. * z,
                            spatial::tint(color, &f.evidence),
                        );
                    }
                    if selected && labels {
                        overlay_label(
                            &mut overlays,
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
                for g in s.goods.iter().filter(|g| g.node.is_some()) {
                    let wood =
                        matches!(g.holder, Holder::Deposit(_)) && g.material.contains("Wood");
                    let at = camera.project(v(frame.goods[&g.id]), viewport);
                    if !visible(at, viewport) {
                        continue;
                    }
                    spatial::wood_zone(g, at, z);
                    let selected = selection == Some(Selection::Goods(g.id));
                    if lod == Lod::Close {
                        glyphs::goods(g, at, z, false);
                    } else if g.quantity > 0 {
                        draw_rectangle(
                            at.x - 4. * z,
                            at.y - 4. * z,
                            8. * z,
                            8. * z,
                            spatial::tint(
                                if wood {
                                    Color::new(0.49, 0.32, 0.19, 1.)
                                } else {
                                    GOLD
                                },
                                &g.evidence,
                            ),
                        );
                    }
                    if labels && selected {
                        let kind = if wood {
                            "Wood source"
                        } else {
                            g.material.as_str()
                        };
                        overlay_label(
                            &mut overlays,
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
            }
            Layer::Routes => {
                // Only known graph edges create routes. Width means observed path wear;
                // screen length, texture and curvature have no effect on travel/capacity.
                for e in &s.edges {
                    let selected = selection == Some(Selection::Edge(e.id));
                    let route = &frame.routes[&e.id];
                    let historical = !spatial::current(&e.evidence);
                    let width = route.visual_width;
                    if e.capacity > 0 {
                        if historical {
                            spatial::route_line(
                                route,
                                2.0 * z.max(0.7),
                                Color::new(0.26, 0.32, 0.26, 0.55),
                                camera,
                                viewport,
                                true,
                            );
                        } else {
                            spatial::route_line(
                                route,
                                (width + 3.) * z,
                                Color::new(0.32, 0.35, 0.24, 0.60),
                                camera,
                                viewport,
                                false,
                            );
                            spatial::route_line(
                                route,
                                width * z,
                                spatial::SAND,
                                camera,
                                viewport,
                                false,
                            );
                            spatial::route_line(
                                route,
                                (width * 0.32).max(1.) * z,
                                Color::new(0.76, 0.70, 0.51, 0.40),
                                camera,
                                viewport,
                                false,
                            );
                        }
                    } else {
                        spatial::route_line(
                            route,
                            2. * z,
                            Color::new(0.27, 0.32, 0.24, 0.50),
                            camera,
                            viewport,
                            true,
                        );
                    }
                    for i in 2..15 {
                        let p = camera.project(v(route.point(e.a, i as f32 / 16.)), viewport);
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
                        let p = camera.project(v(route.point(e.a, 0.5)), viewport);
                        let value = if e.capacity == 0 {
                            "Unopened route".into()
                        } else {
                            format!("{} · capacity {}", hours(e.nominal_ms), e.capacity)
                        };
                        overlay_label(&mut overlays, &value, p + vec2(0., -25.), 13., GOLD);
                        if let Some(q) = e.queued.filter(|q| *q > 0) {
                            overlay_label(
                                &mut overlays,
                                &format!("{q} queued · observed bank"),
                                p + vec2(0., 25.),
                                12.,
                                INK,
                            );
                        }
                    }
                }
            }
            Layer::Infrastructure => {
                for n in &s.nodes {
                    let at = camera.project(v(frame.geography.node(n.id)), viewport);
                    if !visible(at, viewport) {
                        continue;
                    }
                    if lod == Lod::Far {
                        draw_rectangle_lines(
                            at.x - 5.,
                            at.y - 4.,
                            10.,
                            8.,
                            1.,
                            spatial::tint(spatial::INK, &n.evidence),
                        );
                    } else {
                        spatial::place(n, at, z, false);
                    }
                    if labels {
                        let selected = selection == Some(Selection::Node(n.id));
                        let name = node_name(n.id);
                        if n.board_quantity.is_some() || selected {
                            overlay_label(
                                &mut overlays,
                                &name,
                                at + vec2(-10., 106. * z),
                                17.,
                                if selected { GOLD } else { INK },
                            );
                        } else {
                            overlay_text(
                                &mut overlays,
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
                for c in &s.crossings {
                    let at = camera.project(v(frame.crossings[&c.id]), viewport);
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
                    glyphs::crossing(c, at, z, false);
                    if labels {
                        let state = if c.completed == Some(true) {
                            "Crossing open"
                        } else if c.authorized == Some(true) {
                            "Crossing · work site"
                        } else {
                            "Crossing opportunity"
                        };
                        overlay_label(
                            &mut overlays,
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
            }
            Layer::Actors => {
                let mut ordered: Vec<_> = s.persons.iter().collect();
                ordered.sort_by(|a, b| {
                    frame.persons[&a.id]
                        .y
                        .total_cmp(&frame.persons[&b.id].y)
                        .then(a.id.cmp(&b.id))
                });
                for p in ordered {
                    let at = camera.project(v(frame.persons[&p.id]), viewport);
                    drawn.insert(p.id, at);
                    if !visible(at, viewport) {
                        continue;
                    }
                    stats.visible_persons += 1;
                    if lod == Lod::Close && spatial::current(&p.evidence) {
                        let sun = frame.lighting.sun_direction;
                        draw_ellipse(
                            at.x - sun.x * 5. * z,
                            at.y + 4. * z - sun.y * 3. * z,
                            7. * z,
                            2.5 * z,
                            0.,
                            Color::new(0.1, 0.15, 0.12, 0.15),
                        );
                    }
                    person_mark(p, at, z, lod, p.id == s.viewer);
                    // Known action marks describe the real current action, never extra work.
                    if spatial::current(&p.evidence) {
                        let mark = match p.action.as_deref() {
                            Some("sleeping") => Some("z"),
                            Some("queued") => Some("· ·"),
                            Some("striking") => Some("×"),
                            _ => None,
                        };
                        if labels
                            && lod == Lod::Close
                            && let Some(mark) = mark
                        {
                            overlay_text(
                                &mut overlays,
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
                        overlay_label(
                            &mut overlays,
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
                let edge_times: BTreeMap<_, _> =
                    s.edges.iter().map(|e| (e.id, e.nominal_ms)).collect();
                for c in &s.carts {
                    let pos = match c.location {
                        Location::Node(n) => {
                            v(frame.geography.node(n)) + vec2(-114., 35. + (c.id % 4) as f32 * 22.)
                        }
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
                            v(frame.geography.route(from, to, 0.).point(
                                from,
                                (progress as f64 / (nominal as f64 * SCALE as f64)).clamp(0., 1.)
                                    as f32,
                            ))
                        }
                    };
                    let at = camera.project(pos, viewport);
                    if !visible(at, viewport) {
                        continue;
                    }
                    glyphs::cart(c, at, z, false);
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
                                let work = p.traversal_nominal_ms.unwrap_or(1).max(1) as f64
                                    * SCALE as f64;
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
                        let mean = points.iter().copied().fold(Vec2::ZERO, |a, b| a + b)
                            / points.len() as f32;
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
                            let area = overlay_label(
                                &mut overlays,
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
                                overlay_label(
                                    &mut overlays,
                                    &crate::presentation::unit_state(s, u.id),
                                    mean + vec2(0., 42. * z),
                                    12.,
                                    INK,
                                );
                            }
                        }
                        hits.push(Hit {
                            selection: Selection::Unit(u.id),
                            point: at + vec2(8., 0.) * z,
                            radius: 20. * z.max(0.65),
                            rect: None,
                        });
                    }
                }
            }
            Layer::Effects => {
                // Validate source, observation date and viewport before admitting
                // a cue to an eventual effect renderer. W0 emits no particles.
                stats.effect_cues = frame
                    .effects
                    .iter()
                    .filter(|cue| {
                        let supported = matches!(
                            cue.kind,
                            crate::world_presentation::EffectKind::Movement
                                | crate::world_presentation::EffectKind::Construction
                        );
                        supported
                            && source_present(frame, cue.source)
                            && cue.observed_at <= s.now
                            && visible(camera.project(v(cue.position), viewport), viewport)
                    })
                    .count();
            }
            Layer::Lighting => draw_lighting(frame, camera, viewport),
            Layer::Overlays => {
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
                for (id, at) in &drawn {
                    if selected_members.contains(id) {
                        draw_circle_lines(at.x, at.y, (12. * z).max(4.), 1.5, GOLD);
                    }
                }
                if let Some(Selection::Edge(id)) = selection
                    && let Some(route) = frame.routes.get(&id)
                {
                    spatial::route_line(
                        route,
                        (route.visual_width + 5.) * z,
                        Color::new(1., 0.84, 0.34, 0.35),
                        camera,
                        viewport,
                        false,
                    );
                }
                for h in hits.iter().filter(|h| {
                    Some(h.selection) == selection && !matches!(h.selection, Selection::Edge(_))
                }) {
                    if let Some(r) = h.rect {
                        draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., GOLD);
                    } else {
                        draw_circle_lines(h.point.x, h.point.y, h.radius, 2., GOLD);
                    }
                }
                for overlay in &overlays {
                    if overlay.boxed {
                        label(&overlay.value, overlay.at, overlay.size, overlay.color);
                    } else {
                        text(
                            &overlay.value,
                            overlay.at.x,
                            overlay.at.y,
                            overlay.size,
                            overlay.color,
                        );
                    }
                }
            }
            Layer::Ui => {} // Existing chrome is rendered last by the client.
        }
        stats.layer_ms[layer_index] = started.elapsed().as_secs_f64() * 1000.;
    }
    bounds_mask(viewport);
    DrawResult { hits, stats }
}

/// LOD changes only a record's mark, never identity, knowledge or selection.
pub fn person_mark(p: &PersonSnapshot, at: Vec2, z: f32, lod: Lod, viewer: bool) {
    if lod == Lod::Close {
        glyphs::person(p, at, z, viewer, false);
        return;
    }
    let color = spatial::tint(
        if viewer {
            GOLD
        } else if p.hostile == Some(true) {
            RED
        } else if p.unit.is_some() {
            TEAL
        } else {
            Color::new(0.90, 0.86, 0.68, 1.)
        },
        &p.evidence,
    );
    let radius = if lod == Lod::Far {
        1.25
    } else {
        (4. * z).max(2.)
    };
    if spatial::current(&p.evidence) {
        draw_rectangle(
            at.x - radius,
            at.y - radius,
            radius * 2.,
            radius * 2.,
            color,
        );
    } else {
        draw_circle_lines(at.x, at.y, radius, 0.8, color);
    }
}
fn source_present(frame: &RenderSnapshot, source: crate::world_presentation::VisualSource) -> bool {
    match source {
        crate::world_presentation::VisualSource::Person(id) => frame.persons.contains_key(&id),
        crate::world_presentation::VisualSource::Crossing(id) => frame.crossings.contains_key(&id),
    }
}
fn draw_lighting(frame: &RenderSnapshot, camera: &MapCamera, viewport: Rect) {
    draw_rectangle(
        viewport.x,
        viewport.y,
        viewport.w,
        viewport.h,
        Color::new(0.04, 0.06, 0.10, 1. - frame.lighting.ambient),
    );
    // No sources are seeded in this scenario. A source must come from permitted
    // state; these radial marks cannot reveal anything outside the snapshot.
    for light in frame
        .lighting
        .local
        .iter()
        .filter(|light| source_present(frame, light.source))
    {
        let at = camera.project(v(light.position), viewport);
        for ring in (1..=8).rev() {
            draw_circle(
                at.x,
                at.y,
                light.radius * camera.zoom * ring as f32 / 8.,
                Color::new(1., 0.85, 0.55, light.intensity.clamp(0., 1.) * 0.025),
            );
        }
    }
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
