mod camera;
mod chrome;
mod clock;
mod metrics;
mod presentation;
mod scene;
mod widgets;
use camera::MapCamera;
use chrome::{footer, header, matters, sidebar};
use clock::Playback;
use macroquad::prelude::*;
use metrics::Metrics;
use presentation::{ActionKind, Verb};
use scene::{Hit, Selection};
use std::time::Instant;
use widgets::*;
use widgets::{GOLD, RED};
use world_of_nodes::provisioning::playable::session::{CommandResult, DispatchState};
use world_of_nodes::provisioning::playable::{
    MilitaryOrder, PlayerCommand, ScenarioKind, Session, Snapshot, ViewMode,
};
use world_of_nodes::provisioning::{HOUR, Location};

#[derive(Clone, Debug)]
enum Draft {
    Crossing,
    Unit(MilitaryOrder),
    Confirm(PlayerCommand),
}
struct Ui {
    selection: Option<Selection>,
    verb: Verb,
    draft: Option<Draft>,
    camera: MapCamera,
    mode: ViewMode,
    help: bool,
    tab: usize,
    scroll: usize,
    notice: String,
    mouse: Vec2,
    right_origin: Option<Vec2>,
    dragged: bool,
    last_unit_order: Option<(MilitaryOrder, usize, i64, usize)>,
}
impl Default for Ui {
    fn default() -> Self {
        Self{selection:Some(Selection::Node(1)),verb:Verb::Observe,draft:None,camera:MapCamera::default(),mode:ViewMode::Play,help:false,tab:0,scroll:0,notice:"PAUSED · Accepted steward duties and voluntary militia are initial conditions. No order issued.".into(),mouse:Vec2::ZERO,right_origin:None,dragged:false,last_unit_order:None}
    }
}
impl Ui {
    fn issue(&mut self, session: &mut Session, command: PlayerCommand) {
        let issued = command.clone();
        let result = match session.dispatch(command) {
            Ok(CommandResult::Order { message, dispatch }) => {
                format!("ORDER {message} ISSUED · {}", dispatch_text(dispatch))
            }
            Ok(CommandResult::Revocation { message, dispatch }) => {
                format!("REVOCATION {message} ISSUED · {}", dispatch_text(dispatch))
            }
            Ok(CommandResult::Unit { receivers }) => {
                if let PlayerCommand::Unit { verb, target } = issued {
                    self.last_unit_order = Some((verb, target, session.now(), receivers));
                }
                format!(
                    "UNIT ORDER RECEIVED · {receivers} local members. Physical execution may be delayed."
                )
            }
            Ok(CommandResult::Report { report }) => {
                format!("Local observation recorded as report {report}. No distant state revealed.")
            }
            Err(e) => format!("NOT ISSUED · {e}"),
        };
        self.notice = format!("{} · {result}", hours(session.now()));
        self.draft = None;
    }
    fn choose(&mut self, index: usize, s: &Snapshot) {
        self.scroll = 0;
        let actions = presentation::contextual_actions(s, self.selection, self.verb);
        let Some(action) = actions.get(index) else {
            return;
        };
        if !action.enabled {
            self.notice = format!("UNAVAILABLE · {}", action.reason);
            return;
        }
        self.draft = None;
        match action.kind {
            ActionKind::Inspect => self.tab = 0,
            ActionKind::Reports => self.tab = 2,
            ActionKind::CaptureReport => {
                self.draft = Some(Draft::Confirm(PlayerCommand::ObserveReport))
            }
            ActionKind::Help => self.help = true,
            ActionKind::Commission { .. } => self.draft = Some(Draft::Crossing),
            ActionKind::Revoke {
                command: Some(command),
            } => self.draft = Some(Draft::Confirm(PlayerCommand::Revoke { command })),
            ActionKind::Unit(verb) => {
                if verb == MilitaryOrder::Hold {
                    if let Some(target) =
                        s.persons
                            .iter()
                            .find(|p| p.id == s.viewer)
                            .and_then(|p| match p.location {
                                Location::Node(n) => Some(n),
                                _ => None,
                            })
                    {
                        self.draft = Some(Draft::Confirm(PlayerCommand::Unit { verb, target }));
                    }
                } else {
                    self.draft = Some(Draft::Unit(verb));
                }
            }
            _ => self.notice = action.reason.clone(),
        }
    }
    fn target(&mut self, hit: Selection, s: &Snapshot, office: usize) {
        match self.draft.clone() {
            Some(Draft::Crossing) => {
                let id = match hit {
                    Selection::Crossing(n) => s.crossings.iter().find(|p| p.id == n).map(|p| p.id),
                    Selection::Node(n) => s.crossings.iter().find(|c| c.node == n).map(|c| c.id),
                    Selection::Edge(n) => {
                        s.crossings.iter().find(|c| c.edge == Some(n)).map(|c| c.id)
                    }
                    _ => None,
                };
                if let Some(project) = id {
                    self.selection = Some(Selection::Crossing(project));
                    self.draft = Some(Draft::Confirm(PlayerCommand::Commission {
                        office,
                        project,
                    }));
                } else {
                    self.notice = "Choose a known crossing site or its settlement.".into();
                }
            }
            Some(Draft::Unit(verb)) => {
                if let Some(target) = scene::selected_node(s, hit) {
                    self.draft = Some(Draft::Confirm(PlayerCommand::Unit { verb, target }));
                } else {
                    self.notice = "Choose a known graph location.".into();
                }
            }
            _ => {
                self.selection = Some(hit);
                self.tab = 0;
                self.scroll = 0;
            }
        }
    }
}
fn dispatch_text(d: DispatchState) -> &'static str {
    match d {
        DispatchState::DeliveredLocally => "handed over at physical contact",
        DispatchState::WithAgreedCourier => "with agreed courier; distant receipt unknown",
        DispatchState::HeldByIssuer => "held by ruler; no available local handoff",
    }
}
fn conf() -> Conf {
    Conf {
        window_title: "World of Nodes · First Kingdom".into(),
        window_width: 1440,
        window_height: 900,
        high_dpi: true,
        window_resizable: true,
        platform: macroquad::miniquad::conf::Platform {
            linux_backend: macroquad::miniquad::conf::LinuxBackend::X11WithWaylandFallback,
            ..Default::default()
        },
        ..Default::default()
    }
}
#[derive(Default)]
struct Options {
    scenario: ScenarioKind,
    seed: u64,
    debug: bool,
    run: bool,
    commission: bool,
    reports: bool,
    hour: Option<f64>,
    frames: Option<u64>,
    capture: Option<String>,
    evidence: Option<String>,
}
fn options() -> Options {
    let mut o = Options::default();
    let args: Vec<_> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--remote" => o.scenario = ScenarioKind::Remote,
            "--debug" => o.debug = true,
            "--run" => o.run = true,
            "--commission" => o.commission = true,
            "--reports" => o.reports = true,
            "--seed" | "--hour" | "--frames" | "--capture" | "--evidence" => {
                if i + 1 < args.len() {
                    i += 1;
                    match args[i - 1].as_str() {
                        "--seed" => o.seed = args[i].parse().unwrap_or(0),
                        "--hour" => o.hour = args[i].parse().ok(),
                        "--frames" => o.frames = args[i].parse().ok(),
                        "--capture" => o.capture = Some(args[i].clone()),
                        _ => o.evidence = Some(args[i].clone()),
                    }
                }
            }
            "--help" => {
                println!(
                    "World of Nodes: --remote --seed N --debug --run --commission --reports --hour H --frames N --capture PATH --evidence PATH\nQ/W/E/R verbs;1-4 options;LMB select;RMB context/cancel;Enter confirm;Space pause;F1 help;F2 debug;arrows/MMB/RMBdrag pan;wheel zoom."
                );
                std::process::exit(0);
            }
            _ => {}
        }
        i += 1;
    }
    o
}

#[macroquad::main(conf)]
async fn main() {
    install_font(
        load_ttf_font_from_bytes(include_bytes!("../assets/LiberationSans-Regular.ttf"))
            .expect("bundled UI font"),
    );
    let options = options();
    let mut session = Session::new(options.seed, options.scenario).expect("scenario");
    let mut ui = Ui::default();
    if options.reports {
        ui.tab = 2;
    }
    if options.debug {
        ui.mode = ViewMode::Debug;
    }
    let mut clock = Playback::default();
    clock.paused = !options.run;
    if options.commission {
        ui.issue(
            &mut session,
            PlayerCommand::Commission {
                office: 0,
                project: 0,
            },
        );
    }
    if let Some(hour) = options.hour {
        session
            .advance_to((hour * HOUR as f64) as i64)
            .expect("checkpoint");
    }
    let mut snap = session.snapshot(ui.mode);
    let mut previous: Option<Snapshot> = None;
    let mut metrics = Metrics::default();
    let mut hits: Vec<Hit> = vec![];
    loop {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        let sidebar_width = 390.0_f32.min(screen_width() * 0.36);
        let view = Rect::new(
            0.,
            148.,
            (screen_width() - sidebar_width).max(200.),
            (screen_height() - 326.).max(200.),
        );
        let capture_started = Instant::now();
        let known = session.snapshot(ViewMode::Play);
        metrics.snapshot_ms = capture_started.elapsed().as_secs_f64() * 1000.;
        let was_paused = clock.paused;
        metrics.frame(f64::from(get_frame_time()));
        if is_key_pressed(KeyCode::Escape) {
            ui.draft = None;
            ui.help = false;
        }
        if is_key_pressed(KeyCode::F1) {
            ui.help = !ui.help;
        }
        if is_key_pressed(KeyCode::F2) {
            ui.mode = if ui.mode == ViewMode::Play {
                ViewMode::Debug
            } else {
                ViewMode::Play
            };
            snap = session.snapshot(ui.mode);
            previous = None;
            ui.notice = if ui.mode == ViewMode::Debug {
                "DEBUG OMNISCIENCE · command authority unchanged."
            } else {
                "PLAY · local and dated personal knowledge only."
            }
            .into();
        }
        if is_key_pressed(KeyCode::Space) {
            clock.paused = !clock.paused;
        }
        if !ui.help {
            for (key, verb) in [
                (KeyCode::Q, Verb::Observe),
                (KeyCode::W, Verb::Order),
                (KeyCode::E, Verb::Govern),
                (KeyCode::R, Verb::Allocate),
            ] {
                if is_key_pressed(key) {
                    ui.verb = verb;
                    ui.draft = None;
                }
            }
            for (i, key) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4]
                .iter()
                .enumerate()
            {
                if is_key_pressed(*key) {
                    ui.choose(i, &known);
                }
            }
            if is_key_pressed(KeyCode::Enter)
                && let Some(Draft::Confirm(cmd)) = ui.draft.clone()
            {
                ui.issue(&mut session, cmd);
                snap = session.snapshot(ui.mode);
            }
            let wheel = mouse_wheel().1;
            if view.contains(mouse) {
                if wheel != 0. {
                    ui.camera.zoom_at(mouse, wheel, view);
                }
                if is_mouse_button_down(MouseButton::Middle) {
                    ui.camera.pan(mouse - ui.mouse);
                }
                if is_mouse_button_pressed(MouseButton::Right) {
                    ui.right_origin = Some(mouse);
                    ui.dragged = false;
                }
                if is_mouse_button_down(MouseButton::Right)
                    && ui.right_origin.is_some_and(|p| p.distance(mouse) > 5.)
                {
                    ui.dragged = true;
                    ui.camera.pan(mouse - ui.mouse);
                }
                if is_mouse_button_released(MouseButton::Right) {
                    if !ui.dragged {
                        if ui.draft.is_some() {
                            ui.draft = None;
                        } else if ui.selection == Some(Selection::Unit(0))
                            && let Some(hit) = scene::hit_test(&hits, mouse)
                            && let Some(target) = scene::selected_node(&known, hit)
                        {
                            let enemy = if let Selection::Person(id) = hit {
                                known
                                    .persons
                                    .iter()
                                    .find(|p| p.id == id)
                                    .is_some_and(|p| p.hostile == Some(true))
                            } else {
                                false
                            };
                            ui.issue(
                                &mut session,
                                PlayerCommand::Unit {
                                    verb: if enemy {
                                        MilitaryOrder::Attack
                                    } else {
                                        MilitaryOrder::Move
                                    },
                                    target,
                                },
                            );
                            snap = session.snapshot(ui.mode);
                        }
                    }
                    ui.right_origin = None;
                }
                if is_mouse_button_pressed(MouseButton::Left)
                    && let Some(hit) = scene::hit_test(&hits, mouse)
                {
                    ui.target(hit, &known, session.office());
                }
            } else if wheel > 0. {
                ui.scroll = ui.scroll.saturating_sub(2);
            } else if wheel < 0. {
                ui.scroll += 2;
            }
            let mut pan = Vec2::ZERO;
            if is_key_down(KeyCode::Left) {
                pan.x += 1.;
            }
            if is_key_down(KeyCode::Right) {
                pan.x -= 1.;
            }
            if is_key_down(KeyCode::Up) {
                pan.y += 1.;
            }
            if is_key_down(KeyCode::Down) {
                pan.y -= 1.;
            }
            ui.camera.pan(pan * get_frame_time() * 400.);
        }
        block_input(ui.help);
        clock.accumulate(f64::from(get_frame_time()));
        let t = Instant::now();
        let steps = if clock.paused { 0 } else { clock.take_steps(8) };
        for _ in 0..steps {
            previous = Some(snap.clone());
            if let Err(e) = session.advance_to(session.now() + Playback::STEP_MS) {
                ui.notice = format!("Simulation stopped: {e}");
                clock.paused = true;
                break;
            }
            metrics.simulated_steps += 1;
            let q = Instant::now();
            snap = session.snapshot(ui.mode);
            metrics.snapshot_ms += q.elapsed().as_secs_f64() * 1000.;
        }
        metrics.simulation_ms = t.elapsed().as_secs_f64() * 1000.;
        let t = Instant::now();
        clear_background(BG);
        hits = scene::draw(
            &snap,
            previous.as_ref(),
            if clock.paused { 1. } else { clock.fraction() },
            &ui.camera,
            view,
            ui.selection,
        );
        let capture_started = Instant::now();
        let known = session.snapshot(ViewMode::Play);
        metrics.snapshot_ms += capture_started.elapsed().as_secs_f64() * 1000.;
        header(&mut ui, &mut clock, &snap, &metrics, view);
        matters(&mut ui, &known, view);
        sidebar(&mut ui, &snap, &known, view);
        footer(&mut ui, &mut session, &mut snap, &known, view);
        if was_paused != clock.paused {
            previous = None;
        }
        if ui.help {
            help();
        }
        metrics.draw_ms = t.elapsed().as_secs_f64() * 1000.;
        ui.mouse = mouse;
        if is_key_pressed(KeyCode::F10) {
            let path = options.capture.as_deref().unwrap_or("world-of-nodes.png");
            get_screen_data().export_png(path);
            ui.notice = format!("Screenshot: {path}");
        }
        if is_key_pressed(KeyCode::F12)
            && let Some(path) = &options.evidence
        {
            write_evidence(path, &session, &snap, &metrics, &options, &ui);
        }
        if options.frames.is_some_and(|n| metrics.frame_count >= n) && metrics.frame_count > 2 {
            if let Some(path) = &options.capture {
                get_screen_data().export_png(path);
            }
            if let Some(path) = &options.evidence {
                write_evidence(path, &session, &snap, &metrics, &options, &ui);
            }
            break;
        }
        next_frame().await;
    }
}
fn help() {
    draw_rectangle(
        0.,
        0.,
        screen_width(),
        screen_height(),
        Color::new(0., 0., 0., 0.8),
    );
    let w = 720.0_f32.min(screen_width() - 80.);
    let x = (screen_width() - w) * 0.5;
    let y = 90.;
    draw_rectangle(x, y, w, 600., PANEL);
    draw_rectangle_lines(x, y, w, 600., 1., TEAL);
    text(
        "ONE COMMAND. A PHYSICAL WORLD.",
        x + 30.,
        y + 48.,
        28.,
        GOLD,
    );
    let mut at = y + 85.;
    for line in [
        "Start paused. W → 1 → click crossing site → Enter issues an order. Space resumes. A completed commission still needs voluntary materials and work.",
        "Select UNIT 0. W offers Move / Attack / Hold / Withdraw. Right-click a place to move. Only members beside the embodied ruler receive orders. Absent people retain their previous undertaking.",
        "Q opens observations and reports. Filled dots are observed now; hollow dots are last-seen people. Unknown is not zero. The graph is a schematic, not physical terrain coordinates.",
        "F2 enters DEBUG OMNISCIENCE. It reveals private state for research but grants no command rights. F1 or Esc closes help. E/R remain reserved; no new law or coercion is added.",
        "Initial conditions: accepted steward duties; four voluntary militia members including the ruler; a finite willing courier agreement; a seeded raid threat. These are not autonomous political emergence.",
        "The ruler remains hungry, tired and mortal. Local command reach follows their body. Orders, reports, food and people must travel.",
    ] {
        at = paragraph(line, x + 30., at, w - 60., 18., INK, 5) + 20.;
    }
}
fn write_evidence(path: &str, session: &Session, s: &Snapshot, m: &Metrics, o: &Options, ui: &Ui) {
    let replay = Session::replay(o.seed, o.scenario, session.command_log(), session.now());
    let equal = replay.as_ref().is_ok_and(|r| {
        r.world().state_hash() == session.world().state_hash()
            && r.world().event_hash() == session.world().event_hash()
    });
    let executable_hash = std::env::current_exe()
        .ok()
        .and_then(|p| std::fs::read(p).ok())
        .map(|bytes| {
            bytes.into_iter().fold(0xcbf29ce484222325u64, |h, b| {
                (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
            })
        });
    let mut output = format!(
        "compiled_executable_fnv1a64={executable_hash:x?}\nscenario={:?}\nseed={}\nsim_ms={}\nstate_hash={:016x}\nevent_hash={:016x}\nreplay_equal={}\nframes={}\nfps={:.3}\nlast_sim_batch_ms={:.6}\nlast_snapshot_ms={:.6}\nlast_draw_ms={:.6}\nrss_kib={:?}\nview_mode={:?}\nvisible_or_remembered_people={}\ncommands={:#?}\n",
        o.scenario,
        o.seed,
        session.now(),
        session.world().state_hash(),
        session.world().event_hash(),
        equal,
        m.frame_count,
        m.fps(),
        m.simulation_ms,
        m.snapshot_ms,
        m.draw_ms,
        m.rss_kib,
        s.mode,
        s.persons.len(),
        session.command_log()
    );
    // UI evidence is opt-in and read-only. Never used to make simulation decisions.
    let known = session.snapshot(ViewMode::Play);
    output.push_str(&format!(
        "ui_selection={:?}\nui_verb={:?}\nui_draft={:?}\nui_tab={}\nui_notice={}\n",
        ui.selection, ui.verb, ui.draft, ui.tab, ui.notice
    ));
    for matter in presentation::concerns(&known) {
        output.push_str(&format!(
            "known_matter={} | {}\n",
            matter.title, matter.evidence
        ));
    }
    for order in &known.orders {
        output.push_str(&format!(
            "known_order={} | {}\n",
            order.id,
            presentation::order_stage(&known, order).label
        ));
    }
    for action in presentation::contextual_actions(&known, ui.selection, ui.verb) {
        output.push_str(&format!(
            "ui_action={} | {} | enabled={} | {}\n",
            action.slot, action.label, action.enabled, action.reason
        ));
    }
    std::fs::write(path, output).expect("explicit research evidence path");
}

#[cfg(test)]
mod interface_tests {
    use super::*;
    #[test]
    fn debug_only_crossing_cannot_become_a_play_command_target() {
        let session = Session::new(0, ScenarioKind::Local).unwrap();
        let mut known = session.snapshot(ViewMode::Play);
        known.crossings.clear();
        let mut ui = Ui {
            draft: Some(Draft::Crossing),
            ..Default::default()
        };
        ui.target(Selection::Crossing(0), &known, session.office());
        assert!(matches!(ui.draft, Some(Draft::Crossing)));
        assert!(ui.notice.contains("known crossing"));
        assert!(session.command_log().is_empty());
    }
}
