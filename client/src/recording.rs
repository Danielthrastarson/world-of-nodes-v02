//! Optional local diagnostics. Receives immutable Session references only.
//! Input polling is non-consuming and the request channel cannot issue commands.
use crate::{
    Ui,
    clock::Playback,
    journal::{Journal, json_string as quote},
    metrics::Metrics,
    presentation,
    scene::{self, Hit},
    widgets,
};
use macroquad::prelude::*;
use std::{
    collections::BTreeMap,
    path::Path,
    time::{Duration, Instant},
};
use world_of_nodes::provisioning::playable::{ScenarioKind, Session, Snapshot, ViewMode};

#[derive(Default)]
struct FocusEvents(Vec<bool>);
impl macroquad::miniquad::EventHandler for FocusEvents {
    fn update(&mut self) {}
    fn draw(&mut self) {}
    fn window_minimized_event(&mut self) {
        self.0.push(false);
    }
    fn window_restored_event(&mut self) {
        self.0.push(true);
    }
}

pub struct Recording {
    journal: Journal,
    subscriber: usize,
    ui_state: String,
    camera: String,
    last_camera: Instant,
    last_publish: Instant,
    last_poll: Instant,
    checkpoint_at: i64,
    commands: usize,
    reports: BTreeMap<u64, String>,
    orders: BTreeMap<u64, String>,
    error_shown: bool,
}
impl Recording {
    pub fn open(path: &Path, seed: u64, scenario: ScenarioKind) -> std::io::Result<Self> {
        let mut journal = Journal::open(path, seed, scenario)?;
        journal.event("client", &[
            ("pid", std::process::id().to_string()),
            ("executable", std::env::current_exe()?.display().to_string()),
            ("arguments", format!("{:?}", std::env::args().collect::<Vec<_>>())),
            ("privacy", "game-window input only; no text/clipboard/desktop capture; snapshots always Play knowledge".into()),
        ]);
        let now = Instant::now();
        journal.publish("status.json", "{\"running\":true,\"clean_exit\":false}\n");
        Ok(Self {
            journal,
            subscriber: macroquad::input::utils::register_input_subscriber(),
            ui_state: String::new(),
            camera: String::new(),
            last_camera: now,
            last_publish: now,
            last_poll: now,
            checkpoint_at: -1,
            commands: 0,
            reports: BTreeMap::new(),
            orders: BTreeMap::new(),
            error_shown: false,
        })
    }
    pub fn inputs(&mut self, ui: &Ui, session: &Session, frame: u64, view: Rect, hits: &[Hit]) {
        self.journal.begin_frame(frame, session.now());
        let mut focus = FocusEvents::default();
        macroquad::input::utils::repeat_all_miniquad_input(&mut focus, self.subscriber);
        for focused in focus.0 {
            self.journal
                .event("window_focus", &[("focused", focused.to_string())]);
        }
        let mut keys: Vec<_> = get_keys_pressed()
            .into_iter()
            .map(|k| format!("{k:?}"))
            .collect();
        keys.sort();
        let mut released: Vec<_> = get_keys_released()
            .into_iter()
            .map(|k| format!("{k:?}"))
            .collect();
        released.sort();
        let mut buttons = Vec::new();
        for button in [MouseButton::Left, MouseButton::Middle, MouseButton::Right] {
            if is_mouse_button_pressed(button) {
                buttons.push(format!("{button:?}:pressed"));
            }
            if is_mouse_button_released(button) {
                buttons.push(format!("{button:?}:released"));
            }
        }
        let wheel = mouse_wheel();
        if !keys.is_empty() || !released.is_empty() || !buttons.is_empty() || wheel != (0., 0.) {
            let mouse = vec2(mouse_position().0, mouse_position().1);
            let zone = if view.contains(mouse) {
                "map"
            } else if mouse.y < view.y {
                "header"
            } else if mouse.y > view.y + view.h {
                "footer"
            } else {
                "sidebar"
            };
            self.journal.event(
                "input",
                &[
                    ("keys_pressed", format!("{keys:?}")),
                    ("keys_released", format!("{released:?}")),
                    ("buttons", format!("{buttons:?}")),
                    ("wheel", format!("{wheel:?}")),
                    ("mouse", format!("{},{}", mouse.x, mouse.y)),
                    ("zone", zone.into()),
                    (
                        "hit",
                        if view.contains(mouse) {
                            format!("{:?}", scene::hit_test(hits, mouse))
                        } else {
                            "None".into()
                        },
                    ),
                    ("help_open", ui.help.to_string()),
                    ("selection_before", format!("{:?}", ui.selection)),
                    ("verb_before", format!("{:?}", ui.verb)),
                    ("draft_before", format!("{:?}", ui.draft)),
                ],
            );
        }
    }
    pub fn after_frame(&mut self, ui: &Ui, clock: &Playback, session: &Session, metrics: &Metrics) {
        self.journal.begin_frame(metrics.frame_count, session.now());
        self.journal.record_commands(session.command_log());
        let new_commands = self.commands != session.command_log().len();
        self.commands = session.command_log().len();
        let interactions = widgets::take_interactions();
        for item in &interactions {
            self.journal.event(
                "widget",
                &[
                    ("label", item.label.clone()),
                    ("enabled", item.enabled.to_string()),
                    ("blocked_help", item.blocked_help.to_string()),
                ],
            );
        }
        let state = ui_json(ui, clock);
        let changed = state != self.ui_state;
        if changed {
            self.journal.event("ui_state", &[("state", state.clone())]);
            self.ui_state = state;
        }
        let camera = camera_json(ui);
        if camera != self.camera && self.last_camera.elapsed() >= Duration::from_millis(250) {
            self.journal.event("camera", &[("state", camera.clone())]);
            self.camera = camera;
            self.last_camera = Instant::now();
        }
        let due = self.checkpoint_at < 0 || session.now() - self.checkpoint_at >= 300_000;
        let refresh = changed
            || new_commands
            || due
            || !interactions.is_empty()
            || self.last_publish.elapsed() >= Duration::from_millis(500);
        if refresh {
            // Always refresh after footer dispatches; never publish debug truth as ruler knowledge.
            let known = session.snapshot(ViewMode::Play);
            let mut information_changed = false;
            for report in &known.reports {
                let value = format!("{report:?}");
                if self.reports.get(&report.id) != Some(&value) {
                    self.reports.insert(report.id, value.clone());
                    self.journal.event("report_observed", &[("report", value)]);
                    information_changed = true;
                }
            }
            for order in &known.orders {
                let value = format!(
                    "{} | {} | {:?}",
                    order.status,
                    presentation::order_stage(&known, order).label,
                    order.evidence.source
                );
                if self.orders.get(&order.id) != Some(&value) {
                    self.orders.insert(order.id, value.clone());
                    self.journal.event(
                        "order_observed",
                        &[("order_id", order.id.to_string()), ("state", value)],
                    );
                    information_changed = true;
                }
            }
            let contents = self.current(ui, clock, &known, metrics);
            self.journal.publish("current.json", &contents);
            if changed || new_commands || due || information_changed {
                self.journal
                    .event("known_snapshot", &[("snapshot", contents)]);
            }
            self.last_publish = Instant::now();
        }
        if due || new_commands {
            self.checkpoint(session);
        }
        if self.last_poll.elapsed() >= Duration::from_millis(250) {
            self.last_poll = Instant::now();
            if let Some((id, action)) = self.journal.poll_request() {
                let start = Instant::now();
                let mut succeeded = true;
                let filename = match action.as_str() {
                    "snapshot" => {
                        let contents =
                            self.current(ui, clock, &session.snapshot(ViewMode::Play), metrics);
                        self.journal.publish("snapshot.json", &contents);
                        "snapshot.json".to_string()
                    }
                    "checkpoint" => {
                        self.checkpoint(session);
                        "checkpoint.json".into()
                    }
                    "screenshot" => {
                        let filename = format!("{id}.png");
                        let path = self.journal.directory().join(&filename);
                        // Macroquad's PNG export panics on I/O errors. A failed
                        // diagnostic must not terminate the human's game.
                        succeeded = std::panic::catch_unwind(|| {
                            get_screen_data().export_png(path.to_str().expect("CLI path UTF-8"));
                        })
                        .is_ok();
                        if !succeeded {
                            self.journal.event(
                                "request_failed",
                                &[
                                    ("request_id", id.clone()),
                                    ("action", action.clone()),
                                    ("error", "screenshot export failed; see process log".into()),
                                ],
                            );
                            self.journal.mark_error(
                                "screenshot export",
                                std::io::Error::other("export failed; see process log"),
                            );
                        }
                        filename
                    }
                    _ => unreachable!("Journal validates diagnostic-only actions"),
                };
                if succeeded && self.journal.error().is_none() {
                    self.journal.event(
                        "request_completed",
                        &[
                            ("request_id", id),
                            ("action", action),
                            ("path", filename),
                            ("view_mode", format!("{:?}", ui.mode)),
                            ("duration_ms", start.elapsed().as_millis().to_string()),
                        ],
                    );
                }
            }
        }
        if let Some(error) = self.journal.error() {
            if !self.error_shown {
                eprintln!("PLAYTEST RECORDING FAILED: {error}");
                self.error_shown = true;
            }
            widgets::text(
                "RECORDING FAILED — see client log",
                12.,
                screen_height() - 6.,
                13.,
                widgets::RED,
            );
        } else {
            widgets::text(
                "LOCAL PLAYTEST RECORDING",
                12.,
                screen_height() - 6.,
                11.,
                widgets::MUTED,
            );
        }
    }
    fn checkpoint(&mut self, session: &Session) {
        let state = session.world().state_hash();
        let events = session.world().event_hash();
        self.journal.checkpoint(session.now(), state, events);
        self.journal.publish("checkpoint.json", &format!(
            "{{\"sim_ms\":{},\"state_hash\":\"{state:016x}\",\"event_hash\":\"{events:016x}\",\"command_count\":{}}}\n",
            session.now(), session.command_log().len()));
        self.checkpoint_at = session.now();
    }
    pub fn finish(&mut self, ui: &Ui, clock: &Playback, session: &Session, metrics: &Metrics) {
        self.journal.begin_frame(metrics.frame_count, session.now());
        self.journal.record_commands(session.command_log());
        let contents = self.current(ui, clock, &session.snapshot(ViewMode::Play), metrics);
        self.journal.publish("current.json", &contents);
        self.journal
            .event("final_visible_state", &[("snapshot", contents)]);
        self.checkpoint(session);
        self.journal.finish(session.now());
        if let Some(error) = self.journal.error() {
            eprintln!("PLAYTEST RECORDING INCOMPLETE: {error}");
        }
    }
    fn current(&self, ui: &Ui, clock: &Playback, s: &Snapshot, metrics: &Metrics) -> String {
        format!(
            "{{\"schema\":1,\"sim_ms\":{},\"frame\":{},\"knowledge_mode\":\"Play\",{},\"camera\":{},\"known\":{},\"performance\":{{\"fps\":{},\"last_sim_batch_ms\":{},\"last_draw_ms\":{},\"rss_kib\":{}}},\"logger_error\":{}}}\n",
            s.now,
            metrics.frame_count,
            ui_fields(ui, clock),
            camera_json(ui),
            known_json(s, ui),
            metrics.fps(),
            metrics.simulation_ms,
            metrics.draw_ms,
            metrics
                .rss_kib
                .map(|v| v.to_string())
                .unwrap_or_else(|| "null".into()),
            self.journal
                .error()
                .map(quote)
                .unwrap_or_else(|| "null".into())
        )
    }
}
fn camera_json(ui: &Ui) -> String {
    format!(
        "{{\"x\":{},\"y\":{},\"zoom\":{}}}",
        ui.camera.center.x, ui.camera.center.y, ui.camera.zoom
    )
}
fn ui_fields(ui: &Ui, clock: &Playback) -> String {
    format!(
        "\"view_mode\":{},\"paused\":{},\"speed\":{},\"selection\":{},\"verb\":{},\"draft\":{},\"tab\":{},\"scroll\":{},\"help\":{},\"notice\":{}",
        quote(&format!("{:?}", ui.mode)),
        clock.paused,
        clock.speed,
        quote(&format!("{:?}", ui.selection)),
        quote(&format!("{:?}", ui.verb)),
        quote(&format!("{:?}", ui.draft)),
        ui.tab,
        ui.scroll,
        ui.help,
        quote(&ui.notice)
    )
}
fn ui_json(ui: &Ui, clock: &Playback) -> String {
    format!("{{{}}}", ui_fields(ui, clock))
}
fn known_json(s: &Snapshot, ui: &Ui) -> String {
    assert_eq!(
        s.mode,
        ViewMode::Play,
        "diagnostic knowledge exports must be Play projections"
    );
    let nodes = s.nodes.iter().map(|n| format!("{{\"id\":{},\"source\":{},\"observed_at\":{},\"food_request\":{},\"people_seen\":{},\"contents_known\":{}}}",
        n.id, quote(&format!("{:?}", n.evidence.source)), n.evidence.observed_at,
        n.board_quantity.map(|q| q.to_string()).unwrap_or_else(|| "null".into()), n.people_seen, n.contents_known)).collect::<Vec<_>>().join(",");
    let orders = s
        .orders
        .iter()
        .map(|o| {
            format!(
                "{{\"id\":{},\"stage\":{},\"status\":{},\"issued_at\":{},\"source\":{}}}",
                o.id,
                quote(&presentation::order_stage(s, o).label),
                quote(&o.status),
                o.issued_at,
                quote(&format!("{:?}", o.evidence.source))
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let reports = s.reports.iter().map(|r| format!("{{\"id\":{},\"node\":{},\"author\":{},\"sender\":{},\"observed_at\":{},\"received_at\":{},\"food_quantity\":{}}}",
        r.id, r.node, r.author, r.sender, r.observed_at, r.received_at, r.food_quantity)).collect::<Vec<_>>().join(",");
    let matters = presentation::concerns(s)
        .iter()
        .map(|c| {
            format!(
                "{{\"title\":{},\"detail\":{},\"evidence\":{}}}",
                quote(&c.title),
                quote(&c.detail),
                quote(&c.evidence)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let actions = presentation::contextual_actions(s, ui.selection, ui.verb)
        .iter()
        .map(|a| {
            format!(
                "{{\"slot\":{},\"label\":{},\"enabled\":{},\"reason\":{}}}",
                a.slot,
                quote(&a.label),
                a.enabled,
                quote(&a.reason)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    // Complete read-only DTO text preserves details needed for diagnosis without
    // adding a second world serializer or ever walking authoritative internals.
    format!(
        "{{\"nodes\":[{nodes}],\"orders\":[{orders}],\"reports\":[{reports}],\"matters\":[{matters}],\"actions\":[{actions}],\"selected_panel\":{},\"snapshot_details\":{}}}",
        quote(&format!(
            "{:?}",
            presentation::selected_panel(s, ui.selection)
        )),
        quote(&format!("{s:?}"))
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exported_knowledge_stays_play_when_interface_is_debug() {
        let session = Session::new(0, ScenarioKind::Remote).unwrap();
        let ui = Ui {
            mode: ViewMode::Debug,
            ..Default::default()
        };
        let known = session.snapshot(ViewMode::Play);
        let text = known_json(&known, &ui);
        assert!(text.contains("mode: Play"));
        assert!(!text.contains("mode: Debug"));
        assert!(text.contains("debug: None"));
        assert!(!text.contains("source: Debug"));
    }
}
