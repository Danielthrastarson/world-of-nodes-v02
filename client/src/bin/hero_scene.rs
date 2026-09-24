//! W1 isolated review fixture. No playable-client integration.
#![allow(dead_code)]
#[path = "../camera.rs"]
mod camera;
#[path = "../hero/mod.rs"]
mod hero;
#[path = "../world_presentation.rs"]
mod world_presentation;
use macroquad::prelude::*;
use std::{io::Write, path::PathBuf};
use world_of_nodes::provisioning::playable::{PlayerCommand, ScenarioKind, Session, ViewMode};
use world_of_nodes::provisioning::{HOUR, MINUTE};
use world_presentation::{Point, RenderSnapshot};
fn conf() -> Conf {
    Conf {
        window_title: "World of Nodes • W1 isolated art study".into(),
        window_width: 1680,
        window_height: 1050,
        sample_count: 4,
        ..Default::default()
    }
}
fn project<'a>(
    s: &'a world_of_nodes::provisioning::playable::snapshot::Snapshot,
    old: Option<&world_of_nodes::provisioning::playable::snapshot::Snapshot>,
    alpha: f32,
    g: &'a world_presentation::Geography,
) -> RenderSnapshot<'a> {
    let mut f = RenderSnapshot::project(s, old, alpha, g);
    // Authored arrangement of existing fields, not extra cultivation or land rights.
    let mut slots = std::collections::BTreeMap::<usize, usize>::new();
    for field in &s.fields {
        let slot = slots.entry(field.node).or_default();
        let n = g.node(field.node);
        f.fields.insert(
            field.id,
            Point::new(
                if field.node == 2 {
                    n.x + 82. + (*slot % 3) as f32 * 99.
                } else {
                    n.x - 101. + (*slot % 3) as f32 * 99.
                },
                if field.node == 2 {
                    n.y + 93. + (*slot / 3) as f32 * 68.
                } else {
                    n.y - 136. - (*slot / 3) as f32 * 68.
                },
            ),
        );
        *slot += 1;
    }
    f
}
fn caption(font: &Font, title: &str, mode: ViewMode, now: i64) {
    let ink = Color::from_rgba(225, 223, 200, 255);
    draw_rectangle(24., 24., 444., 80., Color::new(0.035, 0.052, 0.055, 0.86));
    draw_text_ex(
        "WORLD OF NODES",
        42.,
        52.,
        TextParams {
            font: Some(font),
            font_size: 21,
            color: ink,
            ..Default::default()
        },
    );
    draw_text_ex(
        title,
        42.,
        77.,
        TextParams {
            font: Some(font),
            font_size: 14,
            color: Color::from_rgba(166, 182, 170, 255),
            ..Default::default()
        },
    );
    let mode = if mode == ViewMode::Debug {
        "RESEARCH ART VIEW · full-state evidence"
    } else {
        "RULER VIEW · observed / remembered"
    };
    draw_rectangle(
        24.,
        screen_height() - 53.,
        screen_width() - 48.,
        29.,
        Color::new(0.035, 0.052, 0.055, 0.78),
    );
    draw_text_ex(
        format!(
            "{mode}    |    T+{:.2}h    |    river & stationary lights: presentation study props",
            now as f64 / HOUR as f64
        ),
        40.,
        screen_height() - 33.,
        TextParams {
            font: Some(font),
            font_size: 13,
            color: ink,
            ..Default::default()
        },
    );
}
#[macroquad::main(conf)]
async fn main() {
    let out = PathBuf::from(std::env::args().nth(1).expect("fresh output directory"));
    std::fs::create_dir(&out).unwrap();
    let font = load_ttf_font_from_bytes(include_bytes!("../../assets/LiberationSans-Regular.ttf"))
        .unwrap();
    let g = hero::geography();
    let mut world = Session::new(0, ScenarioKind::Local).unwrap();
    world
        .dispatch(PlayerCommand::Commission {
            office: 0,
            project: 0,
        })
        .unwrap();
    world.advance_to(30 * HOUR).unwrap();
    let mut evidence = std::fs::File::create(out.join("frames.tsv")).unwrap();
    writeln!(
        evidence,
        "file\tsim_ms\tmode\tzoom\tvisible_people\tstate_hash\tevent_hash\tdraw_cpu_ms"
    )
    .unwrap();
    let presets = [
        ("01-overview", 0.90, vec2(820., 475.), ViewMode::Debug),
        ("02-settlement", 1.60, vec2(620., 497.), ViewMode::Debug),
        ("03-detail", 2.65, vec2(492., 486.), ViewMode::Debug),
        ("04-ruler-knowledge", 1.15, vec2(695., 492.), ViewMode::Play),
    ];
    for (name, zoom, center, mode) in presets {
        let s = world.snapshot(mode);
        let frame = project(&s, None, 1., &g);
        let camera = camera::MapCamera { center, zoom };
        let hash = (world.world().state_hash(), world.world().event_hash());
        for i in 0..3 {
            let t = std::time::Instant::now();
            let count = hero::draw(&frame, &camera, false);
            let ms = t.elapsed().as_secs_f64() * 1000.;
            caption(
                &font,
                "W1 / THE FORD · illustrated landscape study",
                mode,
                s.now,
            );
            if i == 2 {
                get_screen_data().export_png(out.join(format!("{name}.png")).to_str().unwrap());
                writeln!(
                    evidence,
                    "{name}.png\t{}\t{mode:?}\t{zoom}\t{count}\t{:016x}\t{:016x}\t{ms}",
                    s.now, hash.0, hash.1
                )
                .unwrap();
            }
            next_frame().await;
        }
        assert_eq!(
            hash,
            (world.world().state_hash(), world.world().event_hash())
        );
    }
    std::fs::create_dir(out.join("motion")).unwrap();
    let mut old = world.snapshot(ViewMode::Debug);
    let mut num = 0;
    for step in 1..=60 {
        world.advance_to(30 * HOUR + step * 8 * MINUTE).unwrap();
        let s = world.snapshot(ViewMode::Debug);
        let hash = (world.world().state_hash(), world.world().event_hash());
        for sample in 0..4 {
            let a = sample as f32 / 4.;
            let frame = project(&s, Some(&old), a, &g);
            let camera = camera::MapCamera {
                center: vec2(670., 484.),
                zoom: 1.28,
            };
            let t = std::time::Instant::now();
            let count = hero::draw(&frame, &camera, false);
            let ms = t.elapsed().as_secs_f64() * 1000.;
            caption(
                &font,
                "W1 / THE FORD · day to night / accelerated real clock",
                ViewMode::Debug,
                s.now,
            );
            let name = format!("motion/frame-{num:04}.png");
            get_screen_data().export_png(out.join(&name).to_str().unwrap());
            writeln!(
                evidence,
                "{name}\t{}\tDebug\t1.28\t{count}\t{:016x}\t{:016x}\t{ms}",
                s.now, hash.0, hash.1
            )
            .unwrap();
            num += 1;
            next_frame().await;
            assert_eq!(
                hash,
                (world.world().state_hash(), world.world().event_hash())
            );
        }
        old = s;
    }
    let s = world.snapshot(ViewMode::Debug);
    let f = project(&s, None, 1., &g);
    let camera = camera::MapCamera {
        center: vec2(630., 495.),
        zoom: 1.65,
    };
    let t = std::time::Instant::now();
    let count = hero::draw(&f, &camera, false);
    let ms = t.elapsed().as_secs_f64() * 1000.;
    caption(
        &font,
        "W1 / THE FORD · night / stationary light study",
        ViewMode::Debug,
        s.now,
    );
    get_screen_data().export_png(out.join("05-night.png").to_str().unwrap());
    writeln!(
        evidence,
        "05-night.png\t{}\tDebug\t1.65\t{count}\t{:016x}\t{:016x}\t{ms}",
        s.now,
        world.world().state_hash(),
        world.world().event_hash()
    )
    .unwrap();
    next_frame().await;
    let replay = Session::replay(0, ScenarioKind::Local, world.command_log(), world.now()).unwrap();
    assert_eq!(
        (world.world().state_hash(), world.world().event_hash()),
        (replay.world().state_hash(), replay.world().event_hash())
    );
    std::fs::write(out.join("result.json"),format!("{{\"scenario\":\"Local\",\"seed\":0,\"command\":\"Commission office0 project0 at t0\",\"screenshots\":5,\"motion_frames\":{num},\"start_hour\":30,\"end_hour\":38,\"replay_equal\":true,\"render_does_not_mutate_world\":true,\"final_state\":\"{:016x}\",\"final_events\":\"{:016x}\",\"kernel_persons\":{},\"graph_overlay\":false,\"art_fixture_only\":true,\"presentation_props\":{{\"river\":\"authored crossing geography, no hydrology/collision\",\"stationary_lights\":2,\"light_anchor\":\"known current institutional node1\",\"fuel_or_heat_simulated\":false}}}}\n",world.world().state_hash(),world.world().event_hash(),world.world().agents().len())).unwrap();
}
