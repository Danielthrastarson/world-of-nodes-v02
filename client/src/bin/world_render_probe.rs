//! Renderer-only stress fixtures. Synthetic presentation records NEVER enter
//! World/Session. Kernel population stays the existing 18-Person scenario.
#![allow(dead_code)]
#[path = "../camera.rs"]
mod camera;
#[path = "../glyphs.rs"]
mod glyphs;
#[path = "../metrics.rs"]
mod metrics;
#[path = "../presentation.rs"]
mod presentation;
#[path = "../scene.rs"]
mod scene;
#[path = "../spatial.rs"]
mod spatial;
#[path = "../widgets.rs"]
mod widgets;
#[path = "../world_presentation.rs"]
mod world_presentation;
use macroquad::prelude::*;
use std::{path::PathBuf, time::Instant};
use world_of_nodes::provisioning::playable::snapshot::KnowledgeSource;
use world_of_nodes::provisioning::playable::{ScenarioKind, Session, ViewMode};
use world_presentation::{Geography, Lod, Point, RenderSnapshot};
fn conf() -> Conf {
    Conf {
        window_title: "W0 RENDER BENCHMARK · synthetic records, not population".into(),
        window_width: 1440,
        window_height: 900,
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    let out = PathBuf::from(args.get(1).expect("new output directory"));
    std::fs::create_dir(&out).expect("new output path required");
    let count: usize = args.get(2).expect("count").parse().unwrap();
    assert!([1000, 10000, 50000].contains(&count));
    let zoom: f32 = args.get(3).expect("zoom").parse().unwrap();
    widgets::install_font(
        load_ttf_font_from_bytes(include_bytes!("../../assets/LiberationSans-Regular.ttf"))
            .unwrap(),
    );
    let session = Session::new(0, ScenarioKind::Local).unwrap();
    let before = (session.world().state_hash(), session.world().event_hash());
    let population = session.world().agents().len();
    let mut input = session.snapshot(ViewMode::Debug);
    let template = input.persons[0].clone();
    input.persons = (0..count)
        .map(|id| {
            let mut p = template.clone();
            p.id = id;
            p.unit = None;
            p.role = None;
            p.action = Some("idle".into());
            p.hostile = None;
            p.evidence.source = KnowledgeSource::Debug;
            p
        })
        .collect();
    input.nodes.clear();
    input.edges.clear();
    input.fields.clear();
    input.goods.clear();
    input.crossings.clear();
    input.carts.clear();
    input.units.clear();
    let geography = Geography::default();
    let start = Instant::now();
    let mut frame = RenderSnapshot::project(&input, None, 1., &geography);
    let project_ms = start.elapsed().as_secs_f64() * 1000.;
    let viewport = Rect::new(0., 100., 1440., 760.);
    let camera = camera::MapCamera {
        center: vec2(0., 0.),
        zoom,
    };
    let cols = ((count as f32 * viewport.w / viewport.h).sqrt().ceil()) as usize;
    let rows = count.div_ceil(cols);
    // Layout is synthetic stress geometry. No kernel coordinates or population
    // are changed, and these records cannot be dispatched to the live client.
    for id in 0..count {
        let at = vec2(
            18. + (id % cols) as f32 * (viewport.w - 36.) / cols as f32,
            118. + (id / cols) as f32 * (viewport.h - 36.) / rows as f32,
        );
        let world = camera.unproject(at, viewport);
        frame.persons.insert(id, Point::new(world.x, world.y));
    }
    let mut samples = vec![];
    let mut visible = 0;
    let mut last_stats = scene::DrawStats::default();
    let mut guarded = false;
    let wall_start = Instant::now();
    for index in 0..65 {
        clear_background(widgets::BG);
        let t = Instant::now();
        let result = scene::render(&frame, &camera, viewport, None, false);
        let ms = t.elapsed().as_secs_f64() * 1000.;
        visible = result.stats.visible_persons;
        last_stats = result.stats;
        widgets::text(
            &format!(
                "RENDER STRESS · {count} synthetic records · {:?} · kernel still {population} Persons",
                Lod::at_zoom(zoom)
            ),
            20.,
            48.,
            20.,
            widgets::INK,
        );
        if index >= 5 {
            samples.push(ms);
        }
        if index == 64 {
            get_screen_data().export_png(out.join("view.png").to_str().unwrap());
        }
        next_frame().await;
        if index >= 14 && ms > 500. {
            guarded = true;
            get_screen_data().export_png(out.join("view.png").to_str().unwrap());
            break;
        }
    }
    let wall_seconds = wall_start.elapsed().as_secs_f64();
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let max = samples.iter().copied().fold(0., f64::max);
    let rss = metrics::memory_kib();
    // Expensive backend batch telemetry runs OUTSIDE the timing/memory sample.
    // It captures preview textures per batch; do not enable in ordinary play.
    let batches = if mean < 100. {
        macroquad::telemetry::capture_frame();
        next_frame().await;
        clear_background(widgets::BG);
        scene::render(&frame, &camera, viewport, None, false);
        next_frame().await;
        Some(macroquad::telemetry::drawcalls().len())
    } else {
        None
    };
    assert_eq!(visible, count);
    assert_eq!(session.world().agents().len(), population);
    assert_eq!(
        before,
        (session.world().state_hash(), session.world().event_hash())
    );
    let result = format!(
        "{{\"kind\":\"synthetic_render_load_only\",\"records\":{count},\"visible_records\":{visible},\"kernel_persons\":{population},\"zoom\":{zoom},\"lod\":\"{:?}\",\"projection_ms\":{project_ms},\"cpu_draw_mean_ms\":{mean},\"cpu_draw_max_ms\":{max},\"samples\":{},\"wall_fps_with_warmup_and_capture\":{},\"gpu_frame_ms\":null,\"gpu_reason\":\"pinned miniquad elapsed-query implementation unavailable\",\"captured_scene_batches\":{},\"rss_kib_before_batch_capture\":{},\"visible_effects\":{},\"effect_cues\":{},\"guarded_500ms\":{guarded},\"kernel_unchanged\":true,\"layer_cpu_ms_last_frame\":{:?}}}\n",
        Lod::at_zoom(zoom),
        samples.len(),
        (samples.len() + 5) as f64 / wall_seconds,
        batches.map_or("null".into(), |v| v.to_string()),
        rss.map_or("null".into(), |v| v.to_string()),
        last_stats.visible_effects,
        last_stats.effect_cues,
        last_stats.layer_ms
    );
    std::fs::write(out.join("result.json"), result).unwrap();
}
