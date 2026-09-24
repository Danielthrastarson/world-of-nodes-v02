//! A controlled typed move, rendered at multiple samples between executed ticks.
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
#[path = "../world_presentation.rs"]
mod world_presentation;
use macroquad::prelude::*;
use std::{io::Write, path::PathBuf};
use world_of_nodes::provisioning::MINUTE;
use world_of_nodes::provisioning::playable::{
    MilitaryOrder, PlayerCommand, ScenarioKind, Session, ViewMode,
};
fn conf() -> Conf {
    Conf {
        window_title: "W0 motion evidence · actual Persons".into(),
        window_width: 1440,
        window_height: 900,
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    let out = PathBuf::from(std::env::args().nth(1).expect("new output path"));
    std::fs::create_dir(&out).unwrap();
    widgets::install_font(
        load_ttf_font_from_bytes(include_bytes!("../../assets/LiberationSans-Regular.ttf"))
            .unwrap(),
    );
    let mut session = Session::new(0, ScenarioKind::Local).unwrap();
    session
        .dispatch(PlayerCommand::Unit {
            verb: MilitaryOrder::Move,
            target: 3,
        })
        .unwrap();
    let g = world_presentation::Geography::default();
    let camera = camera::MapCamera::default();
    let mut old = session.snapshot(ViewMode::Play);
    let mut count = 0;
    let mut trace = std::fs::File::create(out.join("motion.tsv")).unwrap();
    writeln!(trace, "frame\tsim_ms\talpha\tperson\tx\ty\tevidence").unwrap();
    for step in 1..=60 {
        session.advance_to(step * MINUTE).unwrap();
        let now = session.snapshot(ViewMode::Play);
        let hash = (session.world().state_hash(), session.world().event_hash());
        for sample in 0..4 {
            let alpha = sample as f32 / 4.;
            let frame = world_presentation::RenderSnapshot::project(&now, Some(&old), alpha, &g);
            clear_background(widgets::BG);
            scene::render(
                &frame,
                &camera,
                Rect::new(0., 90., 1440., 810.),
                None,
                false,
            );
            widgets::text(
                "RULER KNOWLEDGE · actual Persons · controlled Move to known ridge",
                20.,
                35.,
                20.,
                widgets::INK,
            );
            widgets::text(
                &format!(
                    "Executed time {}m · render fraction {:.2} · no extrapolation",
                    step, alpha
                ),
                20.,
                65.,
                16.,
                widgets::INK,
            );
            for p in &now.persons {
                let at = frame.persons[&p.id];
                writeln!(
                    trace,
                    "{count}\t{}\t{alpha}\t{}\t{}\t{}\t{:?}",
                    now.now, p.id, at.x, at.y, p.evidence.source
                )
                .unwrap();
            }
            get_screen_data()
                .export_png(out.join(format!("frame-{count:04}.png")).to_str().unwrap());
            count += 1;
            next_frame().await;
            assert_eq!(
                hash,
                (session.world().state_hash(), session.world().event_hash())
            );
        }
        old = now;
    }
    let replay =
        Session::replay(0, ScenarioKind::Local, session.command_log(), session.now()).unwrap();
    assert_eq!(
        (session.world().state_hash(), session.world().event_hash()),
        (replay.world().state_hash(), replay.world().event_hash())
    );
    std::fs::write(out.join("result.json"),format!("{{\"frames\":{count},\"kind\":\"controlled typed Move, actual Persons\",\"mode\":\"Play\",\"replay_equal\":true,\"render_samples_never_mutate_world\":true,\"state_hash\":\"{:016x}\",\"event_hash\":\"{:016x}\"}}\n",session.world().state_hash(),session.world().event_hash())).unwrap();
}
