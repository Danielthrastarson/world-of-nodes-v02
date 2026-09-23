mod camera;
mod clock;
mod metrics;
mod scene;
mod widgets;
use camera::MapCamera;
use clock::Playback;
use macroquad::prelude::*;
use metrics::Metrics;
use scene::{Hit, Selection};
use std::time::Instant;
use widgets::*;
use widgets::{GOLD, RED};
use world_of_nodes::provisioning::playable::session::{CommandResult, DispatchState};
use world_of_nodes::provisioning::playable::{
    MilitaryOrder, PlayerCommand, ScenarioKind, Session, Snapshot, ViewMode,
};
use world_of_nodes::provisioning::{HOUR, Location};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Verb {
    Observe,
    Order,
    Govern,
    Allocate,
}
#[derive(Clone)]
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
}
impl Default for Ui {
    fn default() -> Self {
        Self{selection:Some(Selection::Node(1)),verb:Verb::Observe,draft:None,camera:MapCamera::default(),mode:ViewMode::Play,help:false,tab:0,scroll:0,notice:"PAUSED · Accepted steward duties and voluntary militia are initial conditions. No order issued.".into(),mouse:Vec2::ZERO,right_origin:None,dragged:false}
    }
}
impl Ui {
    fn issue(&mut self, session: &mut Session, command: PlayerCommand) {
        let result = match session.dispatch(command) {
            Ok(CommandResult::Order { message, dispatch }) => {
                format!("ORDER {message} ISSUED · {}", dispatch_text(dispatch))
            }
            Ok(CommandResult::Revocation { message, dispatch }) => {
                format!("REVOCATION {message} ISSUED · {}", dispatch_text(dispatch))
            }
            Ok(CommandResult::Unit { receivers }) => format!(
                "Unit intention received by {receivers} local members. Execution remains physical."
            ),
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
        match self.verb {
            Verb::Observe => {
                self.tab = if index == 1 { 2 } else { 0 };
                if index == 2 {
                    self.draft = Some(Draft::Confirm(PlayerCommand::ObserveReport));
                }
                if index == 3 {
                    self.help = !self.help;
                }
            }
            Verb::Order if self.selection == Some(Selection::Unit(0)) => {
                let verb = [
                    MilitaryOrder::Move,
                    MilitaryOrder::Attack,
                    MilitaryOrder::Hold,
                    MilitaryOrder::Withdraw,
                ][index];
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
                    } else {
                        self.notice =
                            "Hold requires the embodied commander to arrive at a node.".into();
                    }
                } else {
                    self.draft = Some(Draft::Unit(verb));
                }
            }
            Verb::Order => {
                if index == 0 {
                    self.draft = Some(Draft::Crossing);
                } else if index == 1 {
                    if let Some(Selection::Order(command)) = self.selection {
                        self.draft = Some(Draft::Confirm(PlayerCommand::Revoke { command }));
                    } else {
                        self.notice =
                            "Select an order in the Orders panel before sending revocation.".into();
                    }
                } else {
                    self.notice = "This institutional power is not implemented.".into();
                }
            }
            _ => {
                self.notice =
                    "Reserved verb. No new coercive powers or allocation rights are introduced."
                        .into()
            }
        }
    }
    fn target(&mut self, hit: Selection, s: &Snapshot, office: usize) {
        match self.draft.clone() {
            Some(Draft::Crossing) => {
                let id = match hit {
                    Selection::Crossing(n) => Some(n),
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
        let sidebar_width = 340.0_f32.min(screen_width() * 0.36);
        let view = Rect::new(
            0.,
            104.,
            (screen_width() - sidebar_width).max(200.),
            (screen_height() - 270.).max(200.),
        );
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
                    ui.choose(i, &snap);
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
                            && let Some(target) = scene::selected_node(&snap, hit)
                        {
                            let enemy = if let Selection::Person(id) = hit {
                                snap.persons
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
                    ui.target(hit, &snap, session.office());
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
            metrics.snapshot_ms = q.elapsed().as_secs_f64() * 1000.;
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
        header(&mut ui, &mut clock, &snap, &metrics, view);
        sidebar(&mut ui, &snap, view);
        footer(&mut ui, &mut session, &mut snap, view);
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
            write_evidence(path, &session, &snap, &metrics, &options);
        }
        if options.frames.is_some_and(|n| metrics.frame_count >= n) && metrics.frame_count > 2 {
            if let Some(path) = &options.capture {
                get_screen_data().export_png(path);
            }
            if let Some(path) = &options.evidence {
                write_evidence(path, &session, &snap, &metrics, &options);
            }
            break;
        }
        next_frame().await;
    }
}
fn header(ui: &mut Ui, clock: &mut Playback, s: &Snapshot, m: &Metrics, v: Rect) {
    text("WORLD OF NODES", 24., 34., 27., INK);
    text("FIRST KINGDOM / causal interface", 25., 57., 15., MUTED);
    text(
        if s.mode == ViewMode::Play {
            "PLAY · RULER KNOWLEDGE"
        } else {
            "DEBUG · OMNISCIENT TRUTH"
        },
        25.,
        84.,
        17.,
        if s.mode == ViewMode::Play { TEAL } else { RED },
    );
    let x = (v.w - 390.).max(330.);
    text(
        &format!(
            "Day {}  {:02}:{:02}",
            s.now / (24 * HOUR) + 1,
            (s.now / HOUR) % 24,
            (s.now / 60_000) % 60
        ),
        x,
        31.,
        24.,
        GOLD,
    );
    if button(
        Rect::new(x, 43., 90., 32.),
        if clock.paused { "RESUME" } else { "PAUSE" },
        !clock.paused,
        true,
    ) {
        clock.paused = !clock.paused;
    }
    for (i, speed) in [1, 4, 16].iter().enumerate() {
        if button(
            Rect::new(x + 97. + i as f32 * 64., 43., 58., 32.),
            &format!("{speed}x"),
            clock.speed == *speed,
            true,
        ) {
            clock.speed = *speed;
        }
    }
    text(
        &format!(
            "1x = 2 sim min / sec · backlog {:.0} min",
            clock.backlog_ms() / 60_000.
        ),
        x,
        94.,
        14.,
        MUTED,
    );
    let x = v.right() + 18.;
    text(
        &format!("{} visible / remembered people", s.persons.len()),
        x,
        31.,
        17.,
        INK,
    );
    text(
        &format!(
            "{:.0} fps · sim {:.2}ms · draw {:.2}ms",
            m.fps(),
            m.simulation_ms,
            m.draw_ms
        ),
        x,
        55.,
        14.,
        MUTED,
    );
    text(
        &format!(
            "F1 help · F2 {}",
            if ui.mode == ViewMode::Play {
                "debug truth"
            } else {
                "return to play"
            }
        ),
        x,
        82.,
        15.,
        if ui.mode == ViewMode::Play {
            MUTED
        } else {
            RED
        },
    );
    rule(0., 103., screen_width());
}
fn sidebar(ui: &mut Ui, s: &Snapshot, v: Rect) {
    let x = v.right() + 18.;
    let width = screen_width() - x - 18.;
    draw_line(v.right(), 104., v.right(), screen_height(), 1., LINE);
    for (i, label) in ["Inspect", "Orders", "Reports"].iter().enumerate() {
        if button(
            Rect::new(x + i as f32 * width / 3., 119., width / 3. - 5., 31.),
            label,
            ui.tab == i,
            true,
        ) {
            ui.tab = i;
            ui.scroll = 0;
        }
    }
    let mut y = 180.;
    if ui.tab == 0 {
        let (title, lines) = scene::inspect(s, ui.selection);
        text(&title, x, y, 24., INK);
        y += 33.;
        for line in lines.iter().skip(ui.scroll) {
            if y > screen_height() - 45. {
                break;
            }
            y = paragraph(line, x, y, width, 17., MUTED, 5) + 12.;
        }
        if let Some(d) = &s.debug {
            y += 10.;
            rule(x, y, width);
            y += 28.;
            text("DEBUG STATE", x, y, 15., RED);
            y += 26.;
            for (k, value) in d.metrics.iter().take(7) {
                if y > screen_height() - 24. {
                    break;
                }
                y = paragraph(&format!("{k}: {value}"), x, y, width, 15., MUTED, 4) + 12.;
            }
        }
    } else if ui.tab == 1 {
        text("Scoped orders", x, y, 24., INK);
        y += 28.;
        y = paragraph(
            "Handoff does not prove distant completion. Select a record to inspect or revoke.",
            x,
            y,
            width,
            16.,
            MUTED,
            4,
        ) + 14.;
        if s.orders.is_empty() {
            paragraph(
                "No personally known orders. W → 1 to commission a crossing.",
                x,
                y,
                width,
                17.,
                MUTED,
                4,
            );
        }
        for o in s.orders.iter().rev().skip(ui.scroll) {
            if y > screen_height() - 95. {
                break;
            }
            if button(
                Rect::new(x, y, width, 34.),
                &format!("Order {} · {}", o.id, age(s.now, o.issued_at)),
                ui.selection == Some(Selection::Order(o.id)),
                true,
            ) {
                ui.selection = Some(Selection::Order(o.id));
                ui.tab = 0;
                ui.scroll = 0;
            }
            y += 57.;
            y = paragraph(&o.status, x + 8., y, width - 16., 15., TEAL, 3) + 10.;
        }
    } else {
        text("Dated reports", x, y, 24., INK);
        y += 28.;
        y = paragraph(
            "Old observations remain old even when reality changes.",
            x,
            y,
            width,
            16.,
            MUTED,
            4,
        ) + 14.;
        if s.reports.is_empty() {
            paragraph(
                "No reports held. Q → 3 records your local observation; it cannot query distant places.",
                x,
                y,
                width,
                17.,
                MUTED,
                5,
            );
        }
        for r in s.reports.iter().rev().skip(ui.scroll) {
            if y > screen_height() - 95. {
                break;
            }
            if button(
                Rect::new(x, y, width, 34.),
                &format!("Report {} · {}", r.id, age(s.now, r.observed_at)),
                ui.selection == Some(Selection::Report(r.id)),
                true,
            ) {
                ui.selection = Some(Selection::Report(r.id));
                ui.tab = 0;
                ui.scroll = 0;
            }
            y += 55.;
            text(
                &format!(
                    "Person {} / node {} / food {}",
                    r.author, r.node, r.food_quantity
                ),
                x + 8.,
                y,
                15.,
                MUTED,
            );
            y += 27.;
        }
    }
}
fn footer(ui: &mut Ui, session: &mut Session, s: &mut Snapshot, v: Rect) {
    let top = v.bottom() + 14.;
    let width = v.w - 42.;
    let x = 21.;
    for (i, (verb, label)) in [
        (Verb::Observe, "Q OBSERVE"),
        (Verb::Order, "W ORDER"),
        (Verb::Govern, "E GOVERN"),
        (Verb::Allocate, "R ALLOCATE"),
    ]
    .iter()
    .enumerate()
    {
        if button(
            Rect::new(x + i as f32 * width / 4., top, width / 4. - 8., 34.),
            label,
            ui.verb == *verb,
            true,
        ) {
            ui.verb = *verb;
            ui.draft = None;
        }
    }
    let labels = match ui.verb {
        Verb::Observe => ["1 Inspect", "2 Reports", "3 Record local", "4 Help"],
        Verb::Order if ui.selection == Some(Selection::Unit(0)) => {
            ["1 Move", "2 Attack", "3 Hold", "4 Withdraw"]
        }
        Verb::Order => [
            "1 Crossing",
            "2 Revoke order",
            "3 Unavailable",
            "4 Unavailable",
        ],
        _ => ["1 Deferred", "2 Deferred", "3 Deferred", "4 Deferred"],
    };
    for (i, label) in labels.iter().enumerate() {
        if button(
            Rect::new(x + i as f32 * width / 4., top + 41., width / 4. - 8., 29.),
            label,
            false,
            true,
        ) {
            ui.choose(i, s);
        }
    }
    let y = top + 98.;
    match ui.draft.clone() {
        Some(Draft::Crossing) => text(
            "CHOOSE CROSSING · LMB target · RMB cancels",
            x,
            y,
            18.,
            GOLD,
        ),
        Some(Draft::Unit(verb)) => text(
            &format!("{verb:?} · choose known location · RMB cancels"),
            x,
            y,
            18.,
            GOLD,
        ),
        Some(Draft::Confirm(cmd)) => {
            let label = match cmd {
                PlayerCommand::Commission { .. } => "ISSUE ORDER · no workers/resources assigned",
                PlayerCommand::Revoke { .. } => "SEND REVOCATION · effect waits for receipt",
                PlayerCommand::Unit { .. } => "ISSUE UNIT INTENT · local members only",
                PlayerCommand::ObserveReport => "RECORD LOCAL OBSERVATION",
            };
            text(label, x, y, 16., GOLD);
            if button(
                Rect::new(x + width - 143., y - 20., 143., 30.),
                "ENTER / CONFIRM",
                true,
                true,
            ) {
                ui.issue(session, cmd);
                *s = session.snapshot(ui.mode);
            }
        }
        None => {
            paragraph(
                &ui.notice,
                x,
                y,
                width,
                16.,
                if ui.notice.contains("NOT ISSUED") {
                    RED
                } else {
                    MUTED
                },
                2,
            );
        }
    }
    text(
        "Wheel zoom · arrows / middle drag pan · right drag pan · hollow = last seen",
        x,
        screen_height() - 13.,
        13.,
        MUTED,
    );
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
fn write_evidence(path: &str, session: &Session, s: &Snapshot, m: &Metrics, o: &Options) {
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
    let output = format!(
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
    std::fs::write(path, output).expect("explicit research evidence path");
}
