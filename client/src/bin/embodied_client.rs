//! E0 optional native client. One Session, two cameras, no alternate simulation.
#![allow(dead_code)]
#[path = "../embodied/mod.rs"]
mod embodied;
#[path = "../metrics.rs"]
mod metrics;
#[path = "../world_presentation.rs"]
mod world_presentation;
use macroquad::prelude::*;
use std::{io::Write, path::PathBuf, time::Instant};
use world_of_nodes::provisioning::HOUR;
use world_of_nodes::provisioning::playable::session::embodied::{EmbodiedSession, Input};
use world_of_nodes::provisioning::playable::{MilitaryOrder, PlayerCommand};
use world_presentation::RenderSnapshot;
fn conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: Conf {
            window_title: "World of Nodes · E0 embodied ruler prototype".into(),
            window_width: 1440,
            window_height: 900,
            sample_count: 4,
            ..Default::default()
        },
        draw_call_vertex_capacity: 16000,
        draw_call_index_capacity: 48000,
        ..Default::default()
    }
}
fn text(font: &Font, s: impl AsRef<str>, x: f32, y: f32, size: u16, c: Color) {
    draw_text_ex(
        s.as_ref(),
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size,
            color: c,
            ..Default::default()
        },
    );
}
fn main_button(font: &Font, title: &str, x: f32, y: f32, w: f32) -> bool {
    let r = Rect::new(x, y, w, 32.);
    let hover = r.contains(Vec2::from(mouse_position()));
    draw_rectangle(
        x,
        y,
        w,
        32.,
        if hover {
            Color::new(0.2, 0.31, 0.31, 0.97)
        } else {
            Color::new(0.08, 0.13, 0.14, 0.95)
        },
    );
    text(font, title, x + 9., y + 22., 16, WHITE);
    hover && is_mouse_button_pressed(MouseButton::Left)
}
#[macroquad::main(conf)]
async fn main() {
    prevent_quit();
    let args: Vec<_> = std::env::args().collect();
    let capture = args
        .iter()
        .position(|a| a == "--capture")
        .map(|i| PathBuf::from(&args[i + 1]));
    let log_dir = args
        .iter()
        .position(|a| a == "--log-dir")
        .map(|i| PathBuf::from(&args[i + 1]));
    let out = capture.clone().or(log_dir);
    if let Some(out) = &out {
        std::fs::create_dir(out).expect("fresh evidence directory required");
    }
    let font = load_ttf_font_from_bytes(include_bytes!("../../assets/LiberationSans-Regular.ttf"))
        .unwrap();
    let mut session = EmbodiedSession::new(0).expect("physical E0 setup");
    let mut ui_log = out
        .as_ref()
        .map(|o| std::fs::File::create(o.join("ui.tsv")).unwrap());
    if let Some(l) = ui_log.as_mut() {
        writeln!(l,"frame\tsim_ms\tthird_person\tpaused\tspeed\tselected\tpose\tstate_hash\tevent_hash\tsim_cpu_ms\trender_cpu_ms\tvisible_people\tfps\trss_kib").unwrap();
    }
    let mut scene = embodied::Scene::new();
    let geography = embodied::geography();
    let mut third = false;
    let mut yaw = -2.3f32;
    let mut pitch = 0.28f32;
    let mut distance = 7f32;
    let mut strategic_zoom = 100f32;
    let mut pan = Vec2::ZERO;
    let mut paused = true;
    let mut speed = 1u32;
    let mut pending = 0f64;
    let mut selected = None;
    let mut commands = false;
    let mut notice="40 real Persons. Bridge/cart are physical prehistory; no free avatar. Press V to enter the ruler.".to_owned();
    let mut frames = 0usize;
    let mut last_mouse = Vec2::from(mouse_position());
    let mut old = session.snapshot();
    let mut current = session.snapshot();
    let mut cam_pos = vec3(35., 105., 90.);
    let mut cam_target = vec3(35., 0., 0.);

    loop {
        let wall = get_frame_time();
        let mouse = Vec2::from(mouse_position());
        let delta = mouse - last_mouse;
        last_mouse = mouse;

        if is_key_pressed(KeyCode::V) {
            third = !third;
            notice=if third{"Following sovereign Person 0. WASD refines position inside this place; travel uses real routes."}else{"Strategic camera, same time and ruler. Right-click a known location to request personal travel."}.into();
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::Equal) {
            speed = (speed * 4).min(16)
        }
        if is_key_pressed(KeyCode::Minus) {
            speed = (speed / 4).max(1)
        }
        if is_key_pressed(KeyCode::W) && !third {
            commands = !commands;
        }
        if is_key_pressed(KeyCode::Q) {
            notice = format!(
                "{:?}",
                session.dispatch(Input::Institutional(PlayerCommand::ObserveReport))
            );
        }
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        if is_mouse_button_down(MouseButton::Right) && third {
            yaw -= delta.x * 0.007;
            pitch = (pitch + delta.y * 0.005).clamp(0.12, 1.1);
        }
        if is_mouse_button_down(MouseButton::Middle) && !third {
            pan += vec2(-delta.x, delta.y) * strategic_zoom / 700.;
        }
        let scroll = mouse_wheel().1;
        if third {
            distance = (distance - scroll * 0.5).clamp(3., 16.);
        } else {
            strategic_zoom = (strategic_zoom - scroll * 6.).clamp(30., 200.);
        }
        let mut walk = vec2(0., 0.);
        if third {
            let front = vec2(yaw.sin(), yaw.cos());
            let side = vec2(front.y, -front.x);
            if is_key_down(KeyCode::W) {
                walk += front
            }
            if is_key_down(KeyCode::S) {
                walk -= front
            }
            if is_key_down(KeyCode::D) {
                walk += side
            }
            if is_key_down(KeyCode::A) {
                walk -= side
            }
        }
        if capture.is_some() {
            paused = false;
            speed = 1;
            pending += 10000.;
            if frames == 35 || frames == 180 || frames == 215 {
                third = !third;
            }
            walk = if (45..49).contains(&frames) {
                vec2(1., 0.)
            } else if (49..53).contains(&frames) {
                vec2(-1., 0.)
            } else {
                Vec2::ZERO
            };
            if frames == 90 {
                notice = format!(
                    "{:?}",
                    session.dispatch(Input::Institutional(PlayerCommand::ObserveReport))
                );
            }
            if frames == 110 {
                notice = format!(
                    "{:?}",
                    session.dispatch(Input::Institutional(PlayerCommand::Unit {
                        verb: MilitaryOrder::Move,
                        target: 2
                    }))
                );
            }
            if frames == 210 {
                notice = format!(
                    "{:?}",
                    session.dispatch(Input::Institutional(PlayerCommand::Unit {
                        verb: MilitaryOrder::Move,
                        target: 1
                    }))
                );
            }
            if frames == 240 {
                session.advance_to(20 * HOUR).unwrap();
                old = session.snapshot();
                current = old.clone();
                third = true;
                notice="NIGHT · accelerated same-world clock. Existing service expiry and survival remain active.".into();
            }
        } else if !paused {
            pending += wall as f64 * 10_000. * speed as f64;
        }
        let t = Instant::now();
        let mut steps = 0;
        while pending >= 1000. && steps < 160 {
            old = current.clone();
            session.advance_to(session.now() + 1000).unwrap();
            if walk.length_squared() > 0. {
                let dir = walk.normalize();
                if let Err(e) = session.dispatch(Input::Walk {
                    x: (dir.x * 1000.) as i32,
                    z: (dir.y * 1000.) as i32,
                }) {
                    notice = e;
                }
            }
            current = session.snapshot();
            pending -= 1000.;
            steps += 1;
        }
        let sim_ms = t.elapsed().as_secs_f64() * 1000.;
        let hash = (session.world().state_hash(), session.world().event_hash());
        let t = Instant::now();
        let f = RenderSnapshot::project(
            &current,
            Some(&old),
            if paused {
                1.
            } else {
                (pending / 1000.).min(1.) as f32
            },
            &geography,
        );
        let ruler = embodied::ruler_position(&f, session.pose());
        let target = if third {
            ruler + vec3(0., 1.1, 0.)
        } else {
            vec3(35. + pan.x, 0., pan.y)
        };
        let position = if third {
            target
                - vec3(
                    yaw.sin() * pitch.cos(),
                    -pitch.sin(),
                    yaw.cos() * pitch.cos(),
                ) * distance
        } else {
            target + vec3(0., 100., 75.)
        };
        let smoothing = if capture.is_some() {
            0.20
        } else {
            1. - (-wall * 7.).exp()
        };
        cam_pos = cam_pos.lerp(position, smoothing);
        cam_target = cam_target.lerp(target, smoothing);
        cam_pos.y = cam_pos.y.max(embodied::height(cam_pos.x, cam_pos.z) + 1.5);
        let camera = Camera3D {
            position: cam_pos,
            target: cam_target,
            up: Vec3::Y,
            projection: if third {
                Projection::Perspective
            } else {
                Projection::Orthographics
            },
            fovy: if third {
                55f32.to_radians()
            } else {
                strategic_zoom
            },
            z_near: 0.1,
            z_far: 450.,
            ..Default::default()
        };
        let picks = scene.render(
            &f,
            session.pose(),
            &camera,
            third,
            walk.length_squared() > 0.,
        );
        let render_ms = t.elapsed().as_secs_f64() * 1000.;
        assert_eq!(
            hash,
            (session.world().state_hash(), session.world().event_hash())
        );
        if is_mouse_button_pressed(MouseButton::Left)
            && mouse.y > 120.
            && mouse.y < screen_height() - 155.
        {
            selected = picks
                .iter()
                .filter(|(_, p, d)| p.distance(mouse) < 20. && (!third || *d < 15.))
                .min_by(|a, b| {
                    a.1.distance_squared(mouse)
                        .total_cmp(&b.1.distance_squared(mouse))
                })
                .map(|p| p.0);
        }
        if !third
            && is_mouse_button_pressed(MouseButton::Right)
            && let Some((node, _)) = current
                .nodes
                .iter()
                .filter_map(|n| {
                    let p = geography.node(n.id);
                    let x = (p.x - 460.) * 0.1;
                    let z = (p.y - 540.) * 0.1;
                    embodied::project(vec3(x, embodied::height(x, z), z), &camera)
                        .map(|s| (n.id, s))
                })
                .filter(|(_, s)| s.distance(mouse) < 70.)
                .min_by(|a, b| {
                    a.1.distance_squared(mouse)
                        .total_cmp(&b.1.distance_squared(mouse))
                })
        {
            notice = format!("{:?}", session.dispatch(Input::Travel { target: node }));
        }
        draw_rectangle(
            0.,
            0.,
            screen_width(),
            96.,
            Color::new(0.03, 0.06, 0.07, 0.94),
        );
        text(
            &font,
            format!(
                "WORLD OF NODES / E0     {}     T+{:.2}h",
                if third { "EMBODIED RULER" } else { "STRATEGIC" },
                session.now() as f64 / HOUR as f64
            ),
            22.,
            30.,
            22,
            WHITE,
        );
        text(
            &font,
            format!(
                "RULER KNOWLEDGE · {} · {}x (1x = 10 simulated seconds / second) · {} known Persons",
                if paused { "PAUSED" } else { "RUNNING" },
                speed,
                current.persons.len()
            ),
            22.,
            55.,
            15,
            Color::new(0.65, 0.78, 0.72, 1.),
        );
        text(
            &font,
            "V switch view   Space pause   +/- speed   WASD local walk   RMB orbit (3D) / travel (map)   Wheel zoom   Middle-drag pan",
            22.,
            80.,
            14,
            LIGHTGRAY,
        );
        if main_button(
            &font,
            if third {
                "V · Strategic"
            } else {
                "V · Enter ruler"
            },
            screen_width() - 175.,
            15.,
            153.,
        ) {
            third = !third;
        }
        let bottom = screen_height() - 140.;
        draw_rectangle(
            0.,
            bottom,
            screen_width(),
            140.,
            Color::new(0.03, 0.06, 0.07, 0.95),
        );
        text(&font, &notice, 20., bottom + 24., 15, GOLD);
        if main_button(
            &font,
            if paused { "Resume" } else { "Pause" },
            20.,
            bottom + 38.,
            90.,
        ) {
            paused = !paused;
        }
        if main_button(&font, "W · Orders", 120., bottom + 38., 125.) {
            commands = !commands;
        }
        if main_button(&font, "Q · Local report", 255., bottom + 38., 158.) {
            notice = format!(
                "{:?}",
                session.dispatch(Input::Institutional(PlayerCommand::ObserveReport))
            );
        }
        if commands {
            for (i, (label, command)) in [
                (
                    "Commission crossing",
                    PlayerCommand::Commission {
                        office: 0,
                        project: 0,
                    },
                ),
                (
                    "Militia: move east",
                    PlayerCommand::Unit {
                        verb: MilitaryOrder::Move,
                        target: 2,
                    },
                ),
                (
                    "Militia: hold here",
                    PlayerCommand::Unit {
                        verb: MilitaryOrder::Hold,
                        target: session.pose().node,
                    },
                ),
            ]
            .into_iter()
            .enumerate()
            {
                if main_button(&font, label, 435. + i as f32 * 190., bottom + 38., 180.) {
                    notice = format!("{:?}", session.dispatch(Input::Institutional(command)));
                }
            }
        }
        if commands {
            for (i, order) in current.orders.iter().rev().take(2).enumerate() {
                text(
                    &font,
                    format!(
                        "Order {}: {} · {:?}",
                        order.id, order.status, order.evidence.source
                    ),
                    22.,
                    120. + i as f32 * 22.,
                    15,
                    GOLD,
                );
            }
        }
        let here = current.persons.iter().find(|p| p.id == current.viewer);
        text(
            &font,
            format!(
                "Person 0 · {:?} · {} · local ({:.1}, {:.1})m",
                here.map(|p| &p.location),
                here.and_then(|p| p.action.as_deref()).unwrap_or("unknown"),
                session.pose().x_mm as f32 / 1000.,
                session.pose().z_mm as f32 / 1000.
            ),
            20.,
            bottom + 91.,
            14,
            WHITE,
        );
        text(
            &font,
            "Local detail stays inside this place. Return near (0,0), then right-click a known destination in strategic view. River/lights are study scenery.",
            20.,
            bottom + 117.,
            13,
            GRAY,
        );
        if let Some(id) = selected
            && let Some(p) = current.persons.iter().find(|p| p.id == id)
        {
            draw_rectangle(
                screen_width() - 335.,
                112.,
                315.,
                132.,
                Color::new(0.04, 0.08, 0.09, 0.92),
            );
            text(
                &font,
                format!("Person {} · {:?}", id, p.evidence.source),
                screen_width() - 320.,
                138.,
                19,
                WHITE,
            );
            text(
                &font,
                p.role.as_deref().unwrap_or("Role not locally known"),
                screen_width() - 320.,
                164.,
                14,
                LIGHTGRAY,
            );
            text(
                &font,
                p.action.as_deref().unwrap_or("Activity not observed"),
                screen_width() - 320.,
                187.,
                15,
                GOLD,
            );
            text(
                &font,
                "Household: not included in this report",
                screen_width() - 320.,
                211.,
                13,
                GRAY,
            );
        }
        if let Some(l) = ui_log.as_mut() {
            writeln!(l,"{frames}\t{}\t{third}\t{paused}\t{speed}\t{selected:?}\t{:?}\t{:016x}\t{:016x}\t{sim_ms}\t{render_ms}\t{}\t{}\t{:?}",session.now(),session.pose(),session.world().state_hash(),session.world().event_hash(),picks.len(),get_fps(),metrics::memory_kib()).unwrap();
        }
        if let Some(out) = &capture {
            if frames == 0 {
                std::fs::create_dir(out.join("motion")).unwrap();
            }
            get_screen_data().export_png(
                out.join(format!("motion/frame-{frames:04}.png"))
                    .to_str()
                    .unwrap(),
            );
            if [20, 65, 200, 300].contains(&frames) {
                get_screen_data()
                    .export_png(out.join(format!("view-{frames:03}.png")).to_str().unwrap());
            }
            if frames == 359 {
                break;
            }
        }
        frames += 1;
        next_frame().await;
    }
    if let Some(out) = &out {
        std::fs::write(out.join("inputs.txt"), format!("{:#?}", session.log())).unwrap();
        let replay = EmbodiedSession::replay(session.seed(), session.log(), session.now())
            .expect("input replay");
        let same = session.world().state_hash() == replay.world().state_hash()
            && session.world().event_hash() == replay.world().event_hash()
            && session.pose() == replay.pose();
        assert!(same);
        let replay_done = true;
        std::fs::write(out.join("result.json"),format!("{{\"replay_equal\":{replay_done},\"persons\":{},\"now\":{},\"frames\":{},\"state_hash\":\"{:016x}\",\"event_hash\":\"{:016x}\",\"same_ruler_id\":0,\"GPU_ms\":null}}",session.world().agents().len(),session.now(),frames+1,session.world().state_hash(),session.world().event_hash())).unwrap();
    }
}
