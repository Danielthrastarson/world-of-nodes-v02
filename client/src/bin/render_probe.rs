//! Render-load measurement over 1,000 actual kernel Persons. No fake dot copies.
#![allow(dead_code)]
#[path = "../camera.rs"]
mod camera;
#[path = "../glyphs.rs"]
mod glyphs;
#[path = "../presentation.rs"]
mod presentation;
#[path = "../scene.rs"]
mod scene;
#[path = "../spatial.rs"]
mod spatial;
#[path = "../widgets.rs"]
mod widgets;
use macroquad::prelude::*;
use std::time::Instant;
use world_of_nodes::provisioning::playable::snapshot::{ViewMode, capture};
use world_of_nodes::provisioning::{Config, World};
fn conf() -> Conf {
    Conf {
        window_title: "World of Nodes · render probe".into(),
        window_width: 1440,
        window_height: 900,
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    let path = std::env::args().nth(1).expect("output directory");
    widgets::install_font(
        load_ttf_font_from_bytes(include_bytes!("../../assets/LiberationSans-Regular.ttf"))
            .unwrap(),
    );
    let world = World::new(Config {
        starts: vec![0; 1000],
        ..Default::default()
    })
    .unwrap();
    world.audit().unwrap();
    let before = (world.state_hash(), world.event_hash());
    let t = Instant::now();
    let snapshot = capture(&world, 0, ViewMode::Play);
    let snapshot_ms = t.elapsed().as_secs_f64() * 1000.;
    let camera = camera::MapCamera {
        center: camera::node_position(0),
        zoom: 0.72,
    };
    let start = Instant::now();
    let mut draw_ms = 0.;
    let mut max_ms = 0.0_f64;
    let mut visible = 0;
    for frame in 0..240 {
        let t = Instant::now();
        clear_background(widgets::BG);
        let hits = scene::draw(
            &snapshot,
            None,
            1.,
            &camera,
            Rect::new(0., 104., 1440., 730.),
            None,
        );
        visible = hits
            .iter()
            .filter(|h| matches!(h.selection, scene::Selection::Person(_)))
            .count();
        widgets::text(
            "RENDER PROBE · 1,000 actual Persons · paused kernel",
            24.,
            50.,
            26.,
            widgets::INK,
        );
        let elapsed = t.elapsed().as_secs_f64() * 1000.;
        draw_ms += elapsed;
        max_ms = max_ms.max(elapsed);
        if frame == 239 {
            get_screen_data().export_png(&format!("{path}/1000-persons.png"));
        }
        next_frame().await;
    }
    assert_eq!(before, (world.state_hash(), world.event_hash()));
    std::fs::write(format!("{path}/1000-persons.txt"),format!("actual_persons={}\nvisible_persons={}\nframes=240\nsnapshot_ms={snapshot_ms:.6}\nmean_cpu_draw_ms={:.6}\nmax_cpu_draw_ms={max_ms:.6}\nfps_including_capture={:.3}\nstate_unchanged=true\nclock_paused=true\n",world.agents().len(),visible,draw_ms/240.,240./start.elapsed().as_secs_f64())).unwrap();
}
