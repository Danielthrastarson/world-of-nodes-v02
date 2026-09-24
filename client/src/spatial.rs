//! Display-only geography and packing over the permitted snapshot.
//! Distances, footprints and within-node positions are schematic, never physics.
use crate::camera::MapCamera;
use crate::world_presentation::{RegionKind, RenderSnapshot, RouteGeometry};
use macroquad::prelude::*;
use world_of_nodes::provisioning::Holder;
use world_of_nodes::provisioning::playable::snapshot::*;

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
pub fn route_line(
    route: &RouteGeometry,
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
        let p = camera.project(v(route.point(route.a, segment as f32 / 32.)), viewport);
        let q = camera.project(
            v(route.point(route.a, (segment + 1) as f32 / 32.)),
            viewport,
        );
        draw_line(p.x, p.y, q.x, q.y, width, color);
    }
}

pub fn v(p: crate::world_presentation::Point) -> Vec2 {
    vec2(p.x, p.y)
}

/// Broad, irregular place footprints are visual grouping, not territory or fog.
/// Off-route background is neutral and says nothing about traversability.
pub fn ground(frame: &RenderSnapshot, camera: &MapCamera, viewport: Rect) {
    draw_rectangle(viewport.x, viewport.y, viewport.w, viewport.h, LAND);
    let scale = camera.zoom;
    for region in &frame.regions {
        if region.kind != RegionKind::Settlement {
            continue;
        }
        debug_assert_eq!(region.center, frame.geography.node(region.source_id));
        let at = camera.project(v(region.center), viewport);
        let c = if current(&region.evidence) {
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

pub fn place(n: &NodeSnapshot, at: Vec2, z: f32, selected: bool) {
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
