//! E0 3D projection only. No World access; no separate simulation or character AI.
use crate::world_presentation::{Geography, Point, RenderSnapshot, current};
use macroquad::{camera::Camera, prelude::*};
use world_of_nodes::provisioning::playable::session::embodied::Pose;
use world_of_nodes::provisioning::playable::snapshot::*;
use world_of_nodes::provisioning::{HOUR, Holder, Location};
pub fn geography() -> Geography {
    Geography {
        nodes: [
            (0, Point::new(60., 760.)),
            (1, Point::new(460., 540.)),
            (2, Point::new(1200., 540.)),
            (3, Point::new(880., 140.)),
            (4, Point::new(1510., 265.)),
        ]
        .into(),
        route_bends: [((1, 2), 25.), ((1, 3), -80.), ((2, 3), 60.)].into(),
    }
}
pub fn river_x(z: f32) -> f32 {
    37. + (z * 0.07).sin() * 2.5
}
pub fn height(x: f32, z: f32) -> f32 {
    let river = (x - river_x(z)).abs();
    if z > -26. && river < 3.3 {
        return -1.5;
    }
    let base = ((x * 0.023).sin() * (z * 0.019).sin() * 3. + (z + 8.).abs() * 0.025).max(0.);
    let village = ((x * x + z * z).sqrt() / 28.).clamp(0., 1.);
    let east = (((x - 74.).powi(2) + z * z).sqrt() / 24.).clamp(0., 1.);
    base * village * east
}
fn point(p: Point) -> Vec3 {
    let x = (p.x - 460.) * 0.1;
    let z = (p.y - 540.) * 0.1;
    vec3(x, height(x, z), z)
}
fn floor(p: Point, frame: &RenderSnapshot) -> Vec3 {
    let mut p = point(p);
    if (p.x - 37.).abs() < 5.2
        && (p.z - 2.5).abs() < 3.
        && frame
            .observation
            .crossings
            .iter()
            .any(|c| c.completed == Some(true))
    {
        p.y = 0.55;
    }
    p
}
pub fn ruler_position(f: &RenderSnapshot, pose: Pose) -> Vec3 {
    let id = f.observation.viewer;
    let Some(person) = f.observation.persons.iter().find(|p| p.id == id) else {
        return Vec3::ZERO;
    };
    if let Location::Node(n) = person.location {
        let p = f.geography.node(n);
        point(Point::new(
            p.x + pose.x_mm as f32 / 100.,
            p.y + pose.z_mm as f32 / 100.,
        ))
    } else {
        floor(f.persons[&id], f)
    }
}
pub struct Scene {
    terrain: Mesh,
    colors: Vec<[u8; 4]>,
}
impl Scene {
    pub fn new() -> Self {
        let mut vertices = vec![];
        let mut indices = vec![];
        for z in 0..=60 {
            for x in 0..=80 {
                let xx = -80. + x as f32 * 3.;
                let zz = -100. + z as f32 * 3.;
                let y = height(xx, zz);
                let hash = ((x as u32).wrapping_mul(73856093) ^ (z as u32).wrapping_mul(19349663))
                    .wrapping_mul(2654435761);
                let n = (hash % 1000) as f32 / 50000.;
                vertices.push(Vertex::new(
                    xx,
                    y,
                    zz,
                    0.,
                    0.,
                    Color::new(0.36 + n, 0.43 + n, 0.28 + n * 0.5, 1.),
                ));
            }
        }
        for z in 0..60 {
            for x in 0..80 {
                let a = (z * 81 + x) as u16;
                indices.extend([a, a + 81, a + 1, a + 1, a + 81, a + 82]);
            }
        }
        let colors = vertices.iter().map(|v| v.color).collect();
        Self {
            terrain: Mesh {
                vertices,
                indices,
                texture: None,
            },
            colors,
        }
    }
    pub fn render(
        &mut self,
        f: &RenderSnapshot,
        pose: Pose,
        cam: &Camera3D,
        third: bool,
        walking: bool,
    ) -> Vec<(usize, Vec2, f32)> {
        let phase = ((f.observation.now - 6 * HOUR).rem_euclid(24 * HOUR) as f64
            / (24 * HOUR) as f64) as f32;
        let day = ((phase * std::f32::consts::TAU).sin() + 0.15).clamp(0.12, 1.);
        clear_background(Color::new(
            0.16 + day * 0.44,
            0.21 + day * 0.48,
            0.28 + day * 0.45,
            1.,
        ));
        set_camera(cam);
        for (v, c) in self.terrain.vertices.iter_mut().zip(&self.colors) {
            v.color = lit(Color::from_rgba(c[0], c[1], c[2], 255), day).into();
        }
        draw_mesh(&self.terrain);
        for i in 0..68 {
            let z = -25. + i as f32 * 1.8;
            draw_cube(
                vec3(river_x(z), -0.42, z),
                vec3(6.3, 0.12, 2.),
                None,
                lit(Color::new(0.20, 0.43, 0.45, 1.), day),
            );
        }
        for e in &f.observation.edges {
            if e.capacity == 0 {
                continue;
            }
            let r = f.routes[&e.id];
            for i in 0..110 {
                let a = floor(r.point(r.a, i as f32 / 110.), f);
                let b = floor(r.point(r.a, (i + 1) as f32 / 110.), f);
                road(
                    a,
                    b,
                    0.7 + r.visual_width * 0.07,
                    lit(Color::new(0.59, 0.53, 0.36, 1.), day),
                );
            }
        }
        for n in &f.observation.nodes {
            if n.contents_known && n.board_quantity.is_some() {
                let p = point(f.geography.node(n.id));
                let fade = if current(&n.evidence) { 1. } else { 0.45 };
                building(p + vec3(-8.5, 0., -8.), vec3(10., 4., 7.), day * fade);
            }
        }
        for c in &f.observation.crossings {
            if c.edge.is_none() {
                continue;
            }
            let mut p = point(f.crossings[&c.id]);
            p.y = 0.5;
            let progress = if c.completed == Some(true) {
                1.
            } else {
                c.wood_committed.unwrap_or(0) as f32 / c.wood_required.unwrap_or(1).max(1) as f32
            };
            for i in 0..(22. * progress) as usize {
                box3(
                    p + vec3(-4.8 + i as f32 * 0.45, 0., 0.),
                    vec3(0.40, 0.28, 3.5),
                    Color::new(0.54, 0.37, 0.21, 1.),
                    day,
                );
            }
            if c.completed == Some(true) {
                for z in [-1.8, 1.8] {
                    for x in [-4.5, -2.5, 0., 2.5, 4.5] {
                        box3(p + vec3(x, 0.45, z), vec3(0.13, 1.2, 0.13), BROWN, day);
                    }
                    box3(p + vec3(0., 1., z), vec3(10., 0.15, 0.15), BROWN, day);
                }
            }
        }
        for field in &f.observation.fields {
            let at = point(f.fields[&field.id]);
            let at = at + vec3(-5., 0., -5.);
            let p = vec3(at.x, height(at.x, at.z) + 0.05, at.z);
            box3(p, vec3(6., 0.10, 4.), Color::new(0.37, 0.30, 0.20, 1.), day);
            for row in 0..7 {
                let q = p + vec3(-2.5 + row as f32 * 0.8, 0.1, 0.);
                if field.crop.is_some() {
                    box3(
                        q,
                        vec3(0.35, 0.22, 3.6),
                        if field.stage == "mature" {
                            GOLD
                        } else {
                            Color::new(0.47, 0.56, 0.26, 1.)
                        },
                        day,
                    );
                } else {
                    road(
                        q - vec3(0., 0., 1.8),
                        q + vec3(0., 0., 1.8),
                        0.13,
                        lit(DARKBROWN, day),
                    );
                }
            }
        }
        for g in &f.observation.goods {
            if g.quantity == 0 {
                continue;
            }
            let Some(at) = f.goods.get(&g.id) else {
                continue;
            };
            let at = point(*at);
            if g.material.contains("Wood") && matches!(g.holder, Holder::Deposit(_)) {
                for i in 0..18 {
                    let a = i as f32 * 2.4;
                    let r = (i as f32).sqrt() * 1.2;
                    let p = at + vec3(a.cos() * r, 0., a.sin() * r);
                    tree(vec3(p.x, height(p.x, p.z), p.z), day, i);
                }
            }
            for i in 0..g.quantity.div_ceil(12).min(7) {
                let p = at + vec3((i % 3) as f32 * 0.55, 0.28, (i / 3) as f32 * 0.5);
                box3(
                    p,
                    vec3(0.45, 0.55, 0.45),
                    if g.material.contains("Food") {
                        Color::new(0.68, 0.54, 0.28, 1.)
                    } else {
                        BROWN
                    },
                    day,
                );
            }
        }
        let ruler = ruler_position(f, pose);
        let mut picks = vec![];
        for p in &f.observation.persons {
            let mut pos = if p.id == f.observation.viewer {
                ruler
            } else {
                floor(f.persons[&p.id], f)
            };
            if matches!(p.location, Location::Node(1))
                && pos.x > -14.
                && pos.x < -3.
                && pos.z > -12.
                && pos.z < -4.
            {
                pos.z += 12.;
                pos.y = height(pos.x, pos.z);
            }
            if pos.distance(cam.position) > 230. {
                continue;
            }
            let shade = if current(&p.evidence) {
                day
            } else {
                day * 0.38
            };
            let active = walking && p.id == f.observation.viewer
                || p.action.as_deref() == Some("travelling");
            let detail = if third {
                pos.distance(ruler) < 35.
            } else {
                cam.fovy < 105.
            };
            person(
                pos,
                p,
                shade,
                active,
                f.observation.now,
                detail,
                p.id == f.observation.viewer,
            );
            if let Some(screen) = project(pos + vec3(0., 1., 0.), cam) {
                picks.push((p.id, screen, pos.distance(ruler)));
            }
        }
        for c in &f.observation.carts {
            let pos = c
                .user
                .and_then(|id| f.persons.get(&id))
                .copied()
                .or_else(|| {
                    if let Location::Node(n) = c.location {
                        Some(f.geography.node(n))
                    } else {
                        None
                    }
                });
            if let Some(p) = pos {
                let p = floor(p, f) + vec3(0.8, 0., 0.6);
                box3(p + vec3(0., 0.55, 0.), vec3(1.6, 0.35, 1.), BROWN, day);
                for x in [-0.65, 0.65] {
                    for z in [-0.6, 0.6] {
                        box3(p + vec3(x, 0.35, z), vec3(0.2, 0.65, 0.2), DARKBROWN, day);
                    }
                }
            }
        }
        for node in f
            .observation
            .nodes
            .iter()
            .filter(|n| (n.id == 1 || n.id == 2) && n.contents_known)
        {
            let center = point(f.geography.node(node.id));
            for offset in [vec3(-4., 0., 1.), vec3(5., 0., 4.)] {
                let mut p = center + offset;
                p.y = height(p.x, p.z);
                box3(
                    p + vec3(0., 0.75, 0.),
                    vec3(0.15, 1.5, 0.15),
                    DARKBROWN,
                    day,
                );
                if day < 0.7 {
                    draw_sphere_ex(
                        p + vec3(0., 1.6, 0.),
                        0.30,
                        None,
                        ORANGE,
                        DrawSphereParams {
                            rings: 4,
                            slices: 6,
                            ..Default::default()
                        },
                    );
                    for i in 0..8 {
                        let r = 0.7 + i as f32 * 0.4;
                        draw_cylinder_ex(
                            p + vec3(0., 0.025 + i as f32 * 0.001, 0.),
                            r,
                            r,
                            0.002,
                            None,
                            Color::new(1., 0.5, 0.12, 0.13 * (1. - day)),
                            DrawCylinderParams {
                                sides: 16,
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
        set_default_camera();
        picks
    }
}
fn lit(c: Color, d: f32) -> Color {
    Color::new(
        c.r * (0.12 + d * 0.88),
        c.g * (0.18 + d * 0.82),
        c.b * (0.30 + d * 0.70),
        c.a,
    )
}
fn box3(p: Vec3, s: Vec3, c: Color, d: f32) {
    let h = s * 0.5;
    let corners = [
        vec3(-h.x, -h.y, -h.z),
        vec3(h.x, -h.y, -h.z),
        vec3(h.x, h.y, -h.z),
        vec3(-h.x, h.y, -h.z),
        vec3(-h.x, -h.y, h.z),
        vec3(h.x, -h.y, h.z),
        vec3(h.x, h.y, h.z),
        vec3(-h.x, h.y, h.z),
    ];
    let mut vertices = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);
    for (face, shade) in [
        ([0, 1, 2, 3], 0.74),
        ([4, 7, 6, 5], 0.90),
        ([0, 3, 7, 4], 0.68),
        ([1, 5, 6, 2], 0.84),
        ([3, 2, 6, 7], 1.0),
        ([0, 4, 5, 1], 0.55),
    ] {
        let at = vertices.len() as u16;
        let color = lit(Color::new(c.r * shade, c.g * shade, c.b * shade, c.a), d);
        vertices.extend(face.map(|i| Vertex::new2(p + corners[i], Vec2::ZERO, color)));
        indices.extend([at, at + 1, at + 2, at, at + 2, at + 3]);
    }
    draw_mesh(&Mesh {
        vertices,
        indices,
        texture: None,
    });
}
fn road(a: Vec3, b: Vec3, width: f32, c: Color) {
    let n = vec3(-(b.z - a.z), 0., b.x - a.x).normalize_or_zero() * width / 2.;
    let p = [a - n, a + n, b + n, b - n].map(|v| v + vec3(0., 0.06, 0.));
    let mesh = Mesh {
        vertices: p
            .into_iter()
            .map(|v| Vertex::new2(v, Vec2::ZERO, c))
            .collect(),
        indices: vec![0, 1, 2, 0, 2, 3],
        texture: None,
    };
    draw_mesh(&mesh);
}
fn building(p: Vec3, s: Vec3, d: f32) {
    box3(
        p + vec3(0., s.y / 2., 0.),
        s,
        Color::new(0.64, 0.60, 0.45, 1.),
        d,
    );
    let a = p + vec3(-s.x / 2. - 0.5, s.y, -s.z / 2. - 0.5);
    let b = a + vec3(s.x + 1., 0., 0.);
    let c = a + vec3(0., 0., s.z + 1.);
    let e = b + vec3(0., 0., s.z + 1.);
    let ridge = p + vec3(0., s.y + 2., 0.);
    let verts = [
        a,
        b,
        c,
        e,
        ridge + vec3(-s.x / 2. - 0.5, 0., 0.),
        ridge + vec3(s.x / 2. + 0.5, 0., 0.),
    ];
    draw_mesh(&Mesh {
        vertices: verts
            .into_iter()
            .map(|v| Vertex::new2(v, Vec2::ZERO, lit(Color::new(0.45, 0.23, 0.16, 1.), d)))
            .collect(),
        indices: vec![0, 1, 5, 0, 5, 4, 4, 5, 3, 4, 3, 2, 0, 4, 2, 1, 3, 5],
        texture: None,
    });
    box3(
        p + vec3(0., 1.1, s.z / 2. + 0.02),
        vec3(1.5, 2.2, 0.08),
        DARKBROWN,
        d,
    );
    for x in [-s.x / 2. + 0.2, s.x / 2. - 0.2] {
        box3(
            p + vec3(x, s.y / 2., s.z / 2. + 0.04),
            vec3(0.2, s.y, 0.12),
            DARKBROWN,
            d,
        );
    }
    road(
        p + vec3(0., 0.01, 0.),
        p + vec3(6., 0.01, 3.),
        s.x * 0.9,
        Color::new(0.06, 0.08, 0.07, 0.30),
    );
}
fn tree(p: Vec3, d: f32, i: usize) {
    box3(p + vec3(0., 1.3, 0.), vec3(0.3, 2.6, 0.3), BROWN, d);
    draw_sphere_ex(
        p + vec3(0., 3., 0.),
        1.8 + (i % 3) as f32 * 0.2,
        None,
        lit(Color::new(0.20, 0.34, 0.20, 1.), d),
        DrawSphereParams {
            rings: 4,
            slices: 7,
            ..Default::default()
        },
    );
}
fn person(p: Vec3, s: &PersonSnapshot, d: f32, moving: bool, time: i64, detail: bool, ruler: bool) {
    let cloth = if ruler {
        Color::new(0.60, 0.22, 0.18, 1.)
    } else if s.unit.is_some() {
        Color::new(0.21, 0.43, 0.48, 1.)
    } else if s.hostile == Some(true) {
        MAROON
    } else {
        Color::new(0.58 + (s.id % 3) as f32 * 0.07, 0.53, 0.39, 1.)
    };
    if s.action.as_deref() == Some("dead") {
        box3(p + vec3(0., 0.2, 0.), vec3(1.2, 0.3, 0.5), cloth, d);
        return;
    }
    if !detail {
        box3(p + vec3(0., 0.65, 0.), vec3(0.45, 1.3, 0.45), cloth, d);
        return;
    }
    let gait = if moving {
        (time as f32 / 7000. + s.id as f32).sin() * 0.19
    } else {
        0.
    };
    for (x, g) in [(-0.14, gait), (0.14, -gait)] {
        box3(p + vec3(x, 0.34, g), vec3(0.17, 0.68, 0.20), DARKBROWN, d);
    }
    box3(p + vec3(0., 1., 0.), vec3(0.50, 0.70, 0.30), cloth, d);
    for x in [-0.33, 0.33] {
        box3(p + vec3(x, 0.95, -gait), vec3(0.14, 0.65, 0.15), cloth, d);
    }
    draw_sphere_ex(
        p + vec3(0., 1.58, 0.),
        0.22,
        None,
        lit(Color::new(0.74, 0.62, 0.45, 1.), d),
        DrawSphereParams {
            rings: 4,
            slices: 6,
            ..Default::default()
        },
    );
    if s.unit.is_some() {
        box3(p + vec3(0.4, 0.9, 0.), vec3(0.12, 0.58, 0.42), GRAY, d);
    }
    if ruler {
        box3(p + vec3(0., 1.77, 0.), vec3(0.40, 0.09, 0.38), GOLD, d);
    }
    road(
        p - vec3(0.3, -0.01, 0.),
        p + vec3(0.6, 0.01, 0.4),
        0.5,
        Color::new(0.02, 0.03, 0.03, 0.30),
    );
}
pub fn project(p: Vec3, c: &Camera3D) -> Option<Vec2> {
    let q = c.matrix() * p.extend(1.);
    if q.w <= 0. {
        return None;
    }
    let ndc = q.truncate() / q.w;
    if ndc.x.abs() > 1. || ndc.y.abs() > 1. || ndc.z.abs() > 1. {
        return None;
    }
    Some(vec2(
        (ndc.x + 1.) * 0.5 * screen_width(),
        (1. - ndc.y) * 0.5 * screen_height(),
    ))
}
