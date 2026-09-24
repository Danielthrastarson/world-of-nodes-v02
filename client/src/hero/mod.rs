//! One authored W1 art study. Not imported by the playable client.
//! Only RenderSnapshot and presentation metadata enter this renderer.
use crate::{
    camera::MapCamera,
    world_presentation::{self as wp, Geography, LAYERS, Layer, Point, RenderSnapshot},
};
use macroquad::prelude::*;
use world_of_nodes::provisioning::playable::snapshot::*;
use world_of_nodes::provisioning::{HOUR, Holder, Location};

pub fn geography() -> Geography {
    let mut g = Geography {
        nodes: [
            (0, Point::new(110., 700.)),
            (1, Point::new(460., 540.)),
            (2, Point::new(1200., 540.)),
            (3, Point::new(880., 140.)),
            (4, Point::new(1510., 265.)),
        ]
        .into(),
        ..Geography::default()
    };
    g.route_bends.insert((1, 2), 25.);
    g.route_bends.insert((1, 3), -80.);
    g.route_bends.insert((2, 3), 60.);
    g
}
const RIVER: [(f32, f32); 10] = [
    (924., 365.),
    (920., 380.),
    (915., 400.),
    (900., 424.),
    (856., 494.),
    (830., 565.),
    (825., 645.),
    (762., 727.),
    (788., 850.),
    (730., 1100.),
];
const GROUND: Color = Color::new(0.43, 0.48, 0.32, 1.);
fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
}
fn alpha(mut c: Color, a: f32) -> Color {
    c.a = a;
    c
}
fn mix(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}
fn noise(n: u32) -> f32 {
    let mut x = n.wrapping_mul(747796405).wrapping_add(2891336453);
    x = ((x >> ((x >> 28) + 4)) ^ x).wrapping_mul(277803737);
    ((x >> 22) ^ x) as f32 / u32::MAX as f32
}
fn v(p: Point) -> Vec2 {
    vec2(p.x, p.y)
}
struct Brush<'a> {
    camera: &'a MapCamera,
    viewport: Rect,
    daylight: f32,
}
impl Brush<'_> {
    fn p(&self, p: Vec2) -> Vec2 {
        self.camera.project(p, self.viewport)
    }
    fn lit(&self, c: Color) -> Color {
        mix(
            Color::new(c.r * 0.25, c.g * 0.32, c.b * 0.49, c.a),
            c,
            self.daylight,
        )
    }
    fn disk(&self, p: Vec2, r: f32, c: Color) {
        let p = self.p(p);
        draw_circle(p.x, p.y, r * self.camera.zoom, self.lit(c));
    }
    fn ellipse(&self, p: Vec2, rx: f32, ry: f32, c: Color) {
        let p = self.p(p);
        draw_ellipse(
            p.x,
            p.y,
            rx * self.camera.zoom,
            ry * self.camera.zoom,
            0.,
            self.lit(c),
        );
    }
    fn line(&self, a: Vec2, b: Vec2, w: f32, c: Color) {
        let a = self.p(a);
        let b = self.p(b);
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            (w * self.camera.zoom).max(0.4),
            self.lit(c),
        );
    }
    fn quad(&self, p: [Vec2; 4], c: Color) {
        let p = p.map(|p| self.p(p));
        let c = self.lit(c);
        draw_triangle(p[0], p[1], p[2], c);
        draw_triangle(p[0], p[2], p[3], c);
    }
    fn rect(&self, p: Vec2, w: f32, h: f32, c: Color) {
        self.quad([p, p + vec2(w, 0.), p + vec2(w, h), p + vec2(0., h)], c);
    }
}
fn region(b: &Brush, c: Vec2, rx: f32, ry: f32, seed: u32, color: Color) {
    for i in 0..36 {
        let a = i as f32 / 36. * std::f32::consts::TAU;
        let d = (i + 1) as f32 / 36. * std::f32::consts::TAU;
        let r = 0.94 + noise(seed + i) * 0.12;
        let s = 0.94 + noise(seed + (i + 1) % 36) * 0.12;
        let x = b.p(c);
        let y = b.p(c + vec2(a.cos() * rx, a.sin() * ry) * r);
        let z = b.p(c + vec2(d.cos() * rx, d.sin() * ry) * s);
        draw_triangle(x, y, z, b.lit(color));
    }
}
pub fn river_points() -> Vec<Vec2> {
    let points: Vec<_> = RIVER.iter().map(|&(x, y)| vec2(x, y)).collect();
    let mut out = Vec::new();
    for k in 0..points.len() - 1 {
        let p0 = points[k.saturating_sub(1)];
        let p1 = points[k];
        let p2 = points[k + 1];
        let p3 = points[(k + 2).min(points.len() - 1)];
        for i in 0..12 {
            let t = i as f32 / 12.;
            out.push(
                (p1 * 2.
                    + (p2 - p0) * t
                    + (p0 * 2. - p1 * 5. + p2 * 4. - p3) * t * t
                    + (-p0 + p1 * 3. - p2 * 3. + p3) * t * t * t)
                    * 0.5,
            );
        }
    }
    out.push(*points.last().unwrap());
    out
}
fn river(b: &Brush, time: f32) {
    let points = river_points();
    for (width, c) in [
        (92., rgb(72, 78, 52)),
        (77., rgb(133, 132, 96)),
        (63., rgb(53, 90, 90)),
        (49., rgb(64, 111, 111)),
    ] {
        for pair in points.windows(2) {
            b.line(pair[0], pair[1], width, c);
            b.disk(pair[0], width * 0.5, c);
        }
    }
    for i in 0..110u32 {
        let seg = i as usize % (points.len() - 1);
        let a = points[seg];
        let z = points[seg + 1];
        let t = (noise(i + 51) + time * 0.012).fract();
        let at = a.lerp(z, t) + vec2((noise(i + 62) - 0.5) * 40., 0.);
        b.line(
            at,
            at + vec2(4. + noise(i + 66) * 7., -1.),
            0.7,
            alpha(rgb(160, 192, 173), 0.28),
        );
    }
    for i in 0..85u32 {
        let seg = i as usize % (points.len() - 1);
        let a = points[seg];
        let z = points[seg + 1];
        let side = if i % 2 == 0 { 1. } else { -1. };
        let at = a.lerp(z, noise(i + 501)) + vec2(side * (40. + noise(i + 88) * 13.), 0.);
        b.ellipse(at, 3. + noise(i + 38) * 4., 2.5, rgb(100, 107, 80));
    }
}
fn tree(b: &Brush, p: Vec2, size: f32, seed: u32) {
    b.ellipse(
        p + vec2(8., 10.),
        size * 0.90,
        size * 0.45,
        alpha(rgb(28, 37, 30), 0.3),
    );
    b.line(p, p - vec2(1., size * 0.7), 3., rgb(64, 53, 39));
    region(
        b,
        p - vec2(0., size * 0.65),
        size,
        size * 0.78,
        seed,
        rgb(41, 64, 47),
    );
    region(
        b,
        p - vec2(2., size * 0.82),
        size * 0.83,
        size * 0.64,
        seed + 4,
        rgb(60, 82, 51),
    );
    region(
        b,
        p - vec2(5., size * 1.02),
        size * 0.52,
        size * 0.39,
        seed + 8,
        rgb(81, 99, 60),
    );
    if b.camera.zoom > 0.8 {
        for i in 0..7 {
            let at = p + vec2(
                (noise(seed + i) - 0.5) * size,
                -size * 0.7 - noise(seed + i + 17) * size * 0.45,
            );
            b.disk(at, 2., alpha(rgb(134, 143, 86), 0.45));
        }
    }
}
fn hall(b: &Brush, at: Vec2, known: bool) {
    let c = if known { 1. } else { 0.45 };
    let at = at + vec2(-8., -24.);
    b.quad(
        [
            at + vec2(-48., -20.),
            at + vec2(53., -13.),
            at + vec2(87., 47.),
            at + vec2(-13., 46.),
        ],
        alpha(rgb(31, 39, 28), 0.25 * c),
    );
    b.rect(
        at + vec2(-43., -10.),
        89.,
        48.,
        alpha(rgb(168, 156, 119), c),
    );
    b.rect(at + vec2(4., -8.), 42., 46., alpha(rgb(126, 120, 93), c));
    for x in [-42., -18., 5., 24., 44.] {
        b.line(
            at + vec2(x, -8.),
            at + vec2(x, 38.),
            2.6,
            alpha(rgb(64, 60, 46), c),
        );
    }
    b.line(
        at + vec2(-43., 23.),
        at + vec2(46., 23.),
        2.,
        rgb(76, 69, 50),
    );
    b.rect(at + vec2(-13., 12.), 15., 26., rgb(45, 45, 35));
    for x in [-31., 17., 32.] {
        b.rect(at + vec2(x, 3.), 8., 11., rgb(52, 59, 46));
    }
    b.quad(
        [
            at + vec2(-52., -9.),
            at + vec2(-8., -48.),
            at + vec2(56., -28.),
            at + vec2(9., 13.),
        ],
        alpha(rgb(142, 83, 59), c),
    );
    b.quad(
        [
            at + vec2(9., 13.),
            at + vec2(56., -28.),
            at + vec2(56., -6.),
            at + vec2(49., 16.),
        ],
        alpha(rgb(85, 56, 44), c),
    );
    b.line(
        at + vec2(-52., -9.),
        at + vec2(9., 13.),
        3.,
        rgb(191, 126, 82),
    );
    b.line(
        at + vec2(-8., -48.),
        at + vec2(56., -28.),
        3.,
        rgb(177, 113, 76),
    );
    if b.camera.zoom > 0.8 {
        for i in 1..7 {
            let t = i as f32 / 7.;
            b.line(
                (at + vec2(-52., -9.)).lerp(at + vec2(-8., -48.), t),
                (at + vec2(9., 13.)).lerp(at + vec2(56., -28.), t),
                1.,
                alpha(rgb(88, 58, 43), 0.6),
            );
        }
    }
    // One place glyph; the awning is part of it, not another simulated building.
    b.quad(
        [
            at + vec2(-42., 30.),
            at + vec2(-42., 16.),
            at + vec2(-17., 18.),
            at + vec2(-13., 32.),
        ],
        rgb(143, 132, 88),
    );
    b.line(
        at + vec2(-42., 30.),
        at + vec2(-42., 42.),
        2.,
        rgb(61, 54, 41),
    );
    b.line(
        at + vec2(-13., 32.),
        at + vec2(-13., 44.),
        2.,
        rgb(61, 54, 41),
    );
}
fn field(b: &Brush, at: Vec2, f: &FieldSnapshot) {
    let color = if f.crop.is_none() {
        rgb(111, 94, 63)
    } else {
        rgb(136, 137, 70)
    };
    let corners = [
        at + vec2(-44., -26.),
        at + vec2(41., -30.),
        at + vec2(45., 23.),
        at + vec2(-39., 29.),
    ];
    b.quad(corners, rgb(71, 70, 47));
    b.quad(corners.map(|p| at + (p - at) * 0.91), color);
    let ripe = f.stage.contains("mature") || f.stage.contains("ready");
    for row in 0..8 {
        let y = -20. + row as f32 * 6.;
        b.line(
            at + vec2(-35., y),
            at + vec2(35., y - 4.),
            1.2,
            rgb(90, 79, 48),
        );
        if f.crop.is_some() {
            for col in 0..14 {
                let p = at + vec2(-33. + col as f32 * 5., y - 2. - col as f32 * 0.25);
                let color = if ripe {
                    rgb(215, 181, 91)
                } else {
                    rgb(155, 166, 91)
                };
                b.line(p, p + vec2(0., -3.), 1.1, color);
                if b.camera.zoom > 1. {
                    b.line(p + vec2(0., -1.), p + vec2(1.7, -3.), 0.7, color);
                }
            }
        }
    }
}
fn person(b: &Brush, p: &PersonSnapshot, at: Vec2, now: i64) {
    let visible = wp::current(&p.evidence);
    let fade = if visible { 1. } else { 0.38 };
    let mut cloth = match p.id % 4 {
        0 => rgb(172, 147, 105),
        1 => rgb(131, 119, 94),
        2 => rgb(164, 155, 125),
        _ => rgb(112, 124, 106),
    };
    if p.unit.is_some() {
        cloth = rgb(67, 111, 119)
    }
    if p.hostile == Some(true) {
        cloth = rgb(149, 79, 62)
    }
    if b.camera.zoom < 0.5 {
        b.disk(at, 1.6, alpha(cloth, fade));
        return;
    }
    if p.action.as_deref() == Some("dead") {
        b.line(at - vec2(3., 0.), at + vec2(3., 1.), 3., alpha(cloth, fade));
        return;
    }
    let moving = p.action.as_deref() == Some("travelling") && visible;
    let step = if moving {
        ((now as f64 / 28000.) + p.id as f64).sin() as f32 * 1.4
    } else {
        0.
    };
    b.ellipse(
        at + vec2(3., 2.),
        4.,
        1.8,
        alpha(rgb(26, 35, 30), 0.30 * fade),
    );
    b.line(
        at + vec2(-1., 0.),
        at + vec2(-1., 3. + step),
        1.2,
        alpha(rgb(53, 49, 39), fade),
    );
    b.line(
        at + vec2(1., 0.),
        at + vec2(1., 3. - step),
        1.2,
        alpha(rgb(53, 49, 39), fade),
    );
    b.ellipse(at - vec2(0., 2.), 2.5, 3.8, alpha(cloth, fade));
    b.disk(at - vec2(0., 6.), 1.8, alpha(rgb(195, 168, 126), fade));
    if p.unit.is_some() {
        b.ellipse(at + vec2(3., -2.), 1.7, 2.8, alpha(rgb(77, 86, 77), fade));
    }
    if p.role.as_deref().is_some_and(|r| r.contains("Steward")) {
        b.rect(at + vec2(-3., -4.), 1.5, 4., rgb(193, 165, 92));
    }
}
/// Returns state-backed visible Person count. No graph diagnostics unless explicit.
pub fn draw(frame: &RenderSnapshot, camera: &MapCamera, debug_graph: bool) -> usize {
    let phase = (frame.observation.now.rem_euclid(24 * HOUR) as f64 / (24 * HOUR) as f64) as f32;
    let sun = (phase * std::f32::consts::TAU).sin();
    let daylight = ((sun + 0.12) / 0.5).clamp(0., 1.);
    let b = Brush {
        camera,
        viewport: Rect::new(0., 0., screen_width(), screen_height()),
        daylight,
    };
    let s = frame.observation;
    let mut count = 0;
    for layer in LAYERS {
        match layer {
            Layer::Terrain => {
                clear_background(b.lit(GROUND));
                for i in 0..1050u32 {
                    let at = vec2(
                        -450. + noise(i * 3 + 1) * 2650.,
                        -450. + noise(i * 3 + 2) * 1850.,
                    );
                    let col = if i % 3 == 0 {
                        rgb(151, 155, 96)
                    } else {
                        rgb(76, 103, 66)
                    };
                    b.ellipse(
                        at,
                        8. + noise(i + 74) * 35.,
                        4. + noise(i + 29) * 17.,
                        alpha(col, 0.09),
                    );
                }
                if camera.zoom > 0.48 {
                    for i in 0..2800u32 {
                        let at = vec2(
                            -150. + noise(i * 7 + 9) * 2070.,
                            -120. + noise(i * 7 + 10) * 1310.,
                        );
                        b.line(
                            at,
                            at + vec2(1., -2. - noise(i + 18) * 2.),
                            0.5,
                            alpha(rgb(178, 174, 119), 0.22),
                        );
                    }
                }
                for n in &s.nodes {
                    if n.contents_known && n.board_quantity.is_some() {
                        region(
                            &b,
                            v(frame.geography.node(n.id)),
                            137.,
                            95.,
                            n.id as u32 + 122,
                            alpha(rgb(166, 151, 107), 0.48),
                        );
                    }
                }
            }
            Layer::Water => river(&b, s.now as f32 / HOUR as f32),
            Layer::ResourceAreas => {
                for f in &s.fields {
                    field(&b, v(frame.fields[&f.id]), f);
                }
                for g in &s.goods {
                    if matches!(g.holder, Holder::Deposit(_))
                        && g.material.contains("Wood")
                        && g.quantity > 0
                        && let Some(at) = frame.goods.get(&g.id)
                    {
                        let at = v(*at);
                        region(
                            &b,
                            at + vec2(23., 20.),
                            100.,
                            70.,
                            g.id as u32,
                            rgb(67, 85, 51),
                        );
                        let mut trees: Vec<_> = (0..28u32)
                            .map(|i| {
                                let angle = i as f32 * 2.399;
                                let r = (i as f32).sqrt() * 14.;
                                at + vec2(angle.cos() * r, angle.sin() * r * 0.68)
                            })
                            .collect();
                        trees.sort_by(|a, b| a.y.total_cmp(&b.y));
                        for (i, p) in trees.into_iter().enumerate() {
                            tree(
                                &b,
                                p,
                                13. + noise(i as u32 + 51) * 12.,
                                i as u32 + g.id as u32,
                            );
                        }
                    }
                }
            }
            Layer::Routes => {
                for e in &s.edges {
                    if e.capacity == 0 {
                        continue;
                    }
                    let r = frame.routes[&e.id];
                    let width = 6. + r.visual_width * 0.65;
                    for (w, c) in [
                        (width + 6., alpha(rgb(145, 137, 93), 0.35)),
                        (width + 2., rgb(105, 99, 70)),
                        (width, rgb(166, 150, 103)),
                    ] {
                        let mut last = v(r.point(r.a, 0.));
                        for i in 1..=70 {
                            let at = v(r.point(r.a, i as f32 / 70.));
                            b.line(last, at, w, c);
                            last = at;
                        }
                    }
                    for i in 1..70u32 {
                        let p = v(r.point(r.a, i as f32 / 70.));
                        let q = v(r.point(r.a, (i + 1) as f32 / 70.));
                        let n = (q - p).normalize_or_zero().perp();
                        for side in [-1., 1.] {
                            b.line(
                                p + n * width * 0.23 * side,
                                q + n * width * 0.23 * side,
                                0.8,
                                alpha(rgb(111, 103, 72), 0.32),
                            );
                        }
                        if i % 3 == 0 {
                            b.disk(
                                p + n * (width * 0.6) * (noise(i + e.id as u32) - 0.5) * 2.,
                                0.7,
                                rgb(106, 102, 80),
                            );
                        }
                    }
                }
            }
            Layer::Infrastructure => {
                for n in &s.nodes {
                    if n.contents_known && n.board_quantity.is_some() {
                        hall(&b, v(frame.geography.node(n.id)), wp::current(&n.evidence));
                    }
                }
                for c in &s.crossings {
                    if c.edge.is_none() {
                        continue;
                    }
                    let at = v(frame.crossings[&c.id]);
                    let completed = c.completed == Some(true);
                    let progress = if completed {
                        1.
                    } else {
                        c.wood_committed.unwrap_or(0) as f32
                            / c.wood_required.unwrap_or(1).max(1) as f32
                    };
                    for x in [-47., 47.] {
                        b.line(at + vec2(x, -14.), at + vec2(x, 23.), 5., rgb(69, 55, 39));
                    }
                    if progress > 0. {
                        b.quad(
                            [
                                at + vec2(-47., -12.),
                                at + vec2(-47. + 94. * progress, -10.),
                                at + vec2(-47. + 94. * progress, 16.),
                                at + vec2(-47., 14.),
                            ],
                            rgb(103, 75, 47),
                        );
                        for i in 0..(24. * progress) as usize {
                            let x = -46. + i as f32 * 4.;
                            b.line(
                                at + vec2(x, -11.),
                                at + vec2(x, 14.),
                                3.,
                                if i % 3 == 0 {
                                    rgb(162, 129, 81)
                                } else {
                                    rgb(144, 111, 68)
                                },
                            );
                        }
                    }
                    if completed {
                        for y in [-16., 12.] {
                            b.line(
                                at + vec2(-50., y),
                                at + vec2(50., y + 2.),
                                2.5,
                                rgb(193, 157, 100),
                            );
                            for x in [-46., -25., 0., 25., 46.] {
                                b.line(at + vec2(x, y), at + vec2(x, y - 8.), 2., rgb(79, 63, 44));
                            }
                        }
                    }
                }
                for g in &s.goods {
                    let Some(at) = frame.goods.get(&g.id) else {
                        continue;
                    };
                    if g.quantity == 0 {
                        continue;
                    }
                    let at = v(*at);
                    let food = g.material.contains("Food");
                    let piles = (g.quantity.div_ceil(12)).min(9);
                    for i in 0..piles {
                        let p = at + vec2((i % 3) as f32 * 5., (i / 3) as f32 * 4.);
                        if food {
                            b.ellipse(p, 3., 4., rgb(174, 148, 85));
                            b.line(p - vec2(2., 2.), p + vec2(2., -2.), 0.8, rgb(93, 77, 43));
                        } else {
                            b.line(p, p + vec2(9., -4.), 3., rgb(121, 87, 51));
                            b.disk(p + vec2(9., -4.), 1.5, rgb(180, 144, 89));
                        }
                    }
                }
            }
            Layer::Actors => {
                let mut people: Vec<_> = s.persons.iter().collect();
                people.sort_by(|a, b| {
                    frame.persons[&a.id]
                        .y
                        .total_cmp(&frame.persons[&b.id].y)
                        .then(a.id.cmp(&b.id))
                });
                for p in people {
                    let at = v(frame.persons[&p.id]);
                    if b.viewport.contains(b.p(at)) {
                        count += 1;
                        person(&b, p, at, s.now);
                    }
                }
                for c in &s.carts {
                    let at = if let Some(at) = c.user.and_then(|id| frame.persons.get(&id)) {
                        v(*at) + vec2(7., 7.)
                    } else {
                        match c.location {
                            Location::Node(n) => v(frame.geography.node(n)) + vec2(49., 28.),
                            _ => continue,
                        }
                    };
                    b.rect(at, 12., 8., rgb(121, 88, 53));
                    for x in [1., 11.] {
                        b.disk(at + vec2(x, 8.), 2., rgb(52, 49, 37));
                    }
                }
            }
            Layer::Lighting => {
                // Explicit W1 presentation props, never inferred torches on arbitrary Persons.
                if let Some(n) = s.nodes.iter().find(|n| {
                    n.id == 1
                        && n.contents_known
                        && n.board_quantity.is_some()
                        && wp::current(&n.evidence)
                }) {
                    for offset in [vec2(-49., 26.), vec2(67., 19.)] {
                        let at = v(frame.geography.node(n.id)) + offset;
                        let q = b.p(at);
                        let night = 1. - daylight;
                        let flicker = 0.92 + 0.08 * (s.now as f64 / 87000.).sin() as f32;
                        for i in (1..=24).rev() {
                            let t = i as f32 / 24.;
                            draw_circle(
                                q.x,
                                q.y,
                                87. * t * camera.zoom,
                                Color::new(1., 0.51, 0.15, night * 0.052 * (1. - t) * flicker),
                            );
                        }
                        b.line(at + vec2(0., 3.), at - vec2(0., 5.), 2., rgb(50, 42, 33));
                        b.ellipse(at - vec2(0., 5.), 4., 2., rgb(68, 52, 33));
                        if night > 0.05 {
                            draw_ellipse(
                                q.x,
                                q.y - 7. * camera.zoom,
                                2.4 * camera.zoom,
                                (4. + flicker) * camera.zoom,
                                0.,
                                Color::new(1., 0.58, 0.14, night),
                            );
                            draw_circle(
                                q.x,
                                q.y - 7. * camera.zoom,
                                1.3 * camera.zoom,
                                Color::new(1., 0.90, 0.52, night),
                            );
                        }
                    }
                }
            }
            Layer::Overlays => {
                if debug_graph {
                    for n in &s.nodes {
                        let p = b.p(v(frame.geography.node(n.id)));
                        draw_circle_lines(p.x, p.y, 12., 1., MAGENTA);
                        draw_text(format!("node {}", n.id), p.x + 14., p.y, 16., MAGENTA);
                    }
                }
            }
            Layer::Effects | Layer::Ui => {}
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use world_of_nodes::provisioning::playable::{ScenarioKind, Session};
    #[test]
    fn authored_river_does_not_imply_an_extra_crossing() {
        let session = Session::new(0, ScenarioKind::Local).unwrap();
        let snapshot = session.snapshot(ViewMode::Debug);
        let geography = geography();
        let river = river_points();
        let crossing = snapshot.crossings[0].edge.unwrap();
        for e in &snapshot.edges {
            let route = geography.route(e.a, e.b, 0.);
            let minimum = (0..=200)
                .map(|i| v(route.point(route.a, i as f32 / 200.)))
                .flat_map(|p| river.iter().map(move |r| p.distance(*r)))
                .fold(f32::INFINITY, f32::min);
            if e.id == crossing {
                assert!(minimum < 8., "bridge must cross actual rendered river");
            } else {
                assert!(
                    minimum > 48.,
                    "route {} touches authored river bank: {minimum}",
                    e.id
                );
            }
        }
    }
}
