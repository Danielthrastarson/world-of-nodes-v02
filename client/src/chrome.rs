//! Client-only hierarchy and interaction chrome. All decisions use Play snapshots,
//! including when the map is displaying explicit debug truth.
use super::*;
use presentation::{StepState, Tone};

pub(super) fn header(ui: &mut Ui, clock: &mut Playback, s: &Snapshot, m: &Metrics, v: Rect) {
    draw_rectangle(0., 0., screen_width(), 80., BG);
    let x = (v.w - 385.).max(250.);
    fitted(
        "WORLD OF NODES",
        22.,
        31.,
        x - 44.,
        if v.w < 900. { 22. } else { 26. },
        INK,
    );
    fitted(
        "FIRST KINGDOM  /  living people, physical consequences",
        23.,
        54.,
        x - 45.,
        14.,
        MUTED,
    );
    text(
        &format!(
            "DAY {}   {:02}:{:02}",
            s.now / (24 * HOUR) + 1,
            (s.now / HOUR) % 24,
            (s.now / 60_000) % 60
        ),
        x,
        27.,
        22.,
        GOLD,
    );
    if button(
        Rect::new(x, 37., 106., 32.),
        if clock.paused { "RESUME" } else { "PAUSE" },
        !clock.paused,
        true,
    ) {
        clock.paused = !clock.paused;
    }
    for (i, speed) in [1, 4, 16].iter().enumerate() {
        if button(
            Rect::new(x + 113. + i as f32 * 58., 37., 53., 32.),
            &format!("{speed}x"),
            clock.speed == *speed,
            true,
        ) {
            clock.speed = *speed;
        }
    }
    let x = v.right() + 18.;
    let debug = s.mode == ViewMode::Debug;
    draw_rectangle(
        v.right(),
        0.,
        screen_width() - v.right(),
        80.,
        if debug {
            Color::new(0.24, 0.09, 0.075, 1.)
        } else {
            PANEL
        },
    );
    text(
        if debug {
            "DEBUG / ALL WORLD TRUTH"
        } else {
            "PLAY / RULER KNOWLEDGE"
        },
        x,
        26.,
        19.,
        if debug { RED } else { TEAL },
    );
    text(
        if debug {
            "Not ruler knowledge · commands still bounded"
        } else {
            "Solid = observed · hollow = remembered"
        },
        x,
        47.,
        14.,
        MUTED,
    );
    fitted(
        &format!(
            "F1 controls · F2 {} · {:.0} fps",
            if ui.mode == ViewMode::Play {
                "debug"
            } else {
                "return to play"
            },
            m.fps()
        ),
        x,
        68.,
        screen_width() - x - 15.,
        14.,
        if debug { RED } else { MUTED },
    );
    if clock.backlog_ms() > 600_000. {
        text("Simulation catching up", x - 380., 77., 12., GOLD);
    }
    rule(0., 79., screen_width());
}

pub(super) fn matters(ui: &mut Ui, s: &Snapshot, v: Rect) {
    draw_rectangle(0., 80., v.w, 68., BG);
    let concerns = presentation::concerns(s);
    let count = concerns.len().clamp(1, 4);
    let left = 112.;
    let width = (v.w - left - 18.) / count as f32;
    text("CURRENT", 20., 104., 13., MUTED);
    text("MATTERS", 20., 123., 13., MUTED);
    for (i, c) in concerns.iter().take(4).enumerate() {
        let r = Rect::new(left + i as f32 * width, 88., width - 8., 51.);
        let accent = match c.tone {
            Tone::Urgent => RED,
            Tone::Caution => GOLD,
            Tone::Information => TEAL,
        };
        draw_rectangle(r.x, r.y, r.w, r.h, PANEL);
        draw_rectangle(r.x, r.y, 3., r.h, accent);
        fitted(&c.title, r.x + 10., r.y + 20., r.w - 20., 16., INK);
        fitted(&c.evidence, r.x + 10., r.y + 39., r.w - 20., 12., MUTED);
        if clicked(r) {
            ui.notice = c.detail.clone();
            ui.draft = None;
            ui.tab = 0;
            ui.scroll = 0;
            if let Some(target) = c.target {
                ui.selection = Some(target);
            }
        }
    }
    rule(0., 147., v.w);
}

fn choose_selection(ui: &mut Ui, selected: Selection) {
    ui.selection = Some(selected);
    ui.tab = 0;
    ui.scroll = 0;
    ui.draft = None;
}
// Explicit text styling and clipping bounds keep this small draw helper transparent.
#[allow(clippy::too_many_arguments)]
fn content_line(
    value: &str,
    x: f32,
    y: &mut f32,
    width: f32,
    size: f32,
    color: Color,
    top: f32,
    bottom: f32,
) {
    for line in wrap(value, width, size) {
        if *y >= top && *y < bottom {
            text(&line, x, *y, size, color);
        }
        *y += size + 5.;
    }
}
fn stage_strip(
    s: &Snapshot,
    o: &world_of_nodes::provisioning::playable::snapshot::OrderSnapshot,
    x: f32,
    y: &mut f32,
    width: f32,
    top: f32,
    bottom: f32,
) {
    let stage = presentation::order_stage(s, o);
    content_line(&stage.label, x, y, width, 20., TEAL, top, bottom);
    let gap = width / stage.steps.len().max(1) as f32;
    for (i, step) in stage.steps.iter().enumerate() {
        let px = x + i as f32 * gap;
        let color = match step.state {
            StepState::Known => TEAL,
            StepState::Negative => RED,
            StepState::Pending => GOLD,
            StepState::Unknown => MUTED,
        };
        if *y >= top && *y + 35. < bottom {
            if i + 1 < stage.steps.len() {
                draw_line(px + 8., *y + 3., px + gap - 5., *y + 3., 1., LINE);
            }
            if step.state == StepState::Known {
                draw_circle(px + 5., *y + 3., 4., color);
            } else {
                draw_circle_lines(px + 5., *y + 3., 4., 1., color);
            }
            let label = ["Issued", "Handoff", "Received", "Obligation", "Result"]
                .get(i)
                .copied()
                .unwrap_or(&step.label);
            fitted(label, px, *y + 26., gap - 6., 12., color);
        }
    }
    *y += 48.;
    content_line(&stage.detail, x, y, width, 15., MUTED, top, bottom);
    *y += 12.;
}

pub(super) fn sidebar(ui: &mut Ui, s: &Snapshot, known: &Snapshot, v: Rect) {
    let x = v.right() + 20.;
    let width = screen_width() - x - 20.;
    draw_rectangle(
        v.right(),
        80.,
        screen_width() - v.right(),
        screen_height() - 80.,
        PANEL,
    );
    draw_line(v.right(), 80., v.right(), screen_height(), 1., LINE);
    for (i, label) in ["Inspect", "Orders", "Reports"].iter().enumerate() {
        if button(
            Rect::new(x + i as f32 * width / 3., 94., width / 3. - 5., 33.),
            label,
            ui.tab == i,
            true,
        ) {
            ui.tab = i;
            ui.scroll = 0;
        }
    }
    let panel = presentation::selected_panel(s, ui.selection);
    let selected_title = if let Some(Selection::Node(n)) = ui.selection {
        camera::node_name(n)
    } else {
        panel.title.clone()
    };
    let title = match ui.tab {
        1 => "Sovereign orders",
        2 => "Dated reports",
        _ => &selected_title,
    };
    draw_rectangle(x - 9., 149., 3., 52., GOLD);
    fitted(title, x + 4., 170., width - 8., 23., GOLD);
    fitted(
        match ui.tab {
            1 => "Progress requires evidence",
            2 => "Observation date stays with the report",
            _ => &panel.subtitle,
        },
        x + 4.,
        192.,
        width - 8.,
        13.,
        MUTED,
    );
    rule(x, 210., width);
    let top = 231.;
    let bottom = screen_height() - 34.;
    let mut y = top - ui.scroll as f32 * 22.;
    if ui.tab == 0 {
        if let Some(Selection::Order(id)) = ui.selection
            && let Some(o) = s.orders.iter().find(|o| o.id == id)
        {
            stage_strip(s, o, x, &mut y, width, top, bottom);
        }
        let actions = presentation::contextual_actions(known, ui.selection, Verb::Order);
        // Put the first decision beside its relevant context; the complete command
        // vocabulary remains in the persistent Q/W/E/R strip.
        for (i, section) in panel.sections.iter().enumerate() {
            if i == 1
                && let Some(Selection::Unit(_)) = ui.selection
                && let Some((verb, target, at, count)) = ui.last_unit_order
            {
                content_line(
                    "LAST ISSUED BY YOU",
                    x,
                    &mut y,
                    width,
                    13.,
                    TEAL,
                    top,
                    bottom,
                );
                content_line(
                    &format!(
                        "{verb:?} → {} · {} · {count} local recipients",
                        camera::node_name(target),
                        hours(at)
                    ),
                    x,
                    &mut y,
                    width,
                    15.,
                    INK,
                    top,
                    bottom,
                );
                content_line(
                    "Receipt is known; future execution is not guaranteed.",
                    x,
                    &mut y,
                    width,
                    14.,
                    MUTED,
                    top,
                    bottom,
                );
                y += 12.;
            }
            let actions_after = if matches!(ui.selection, Some(Selection::Unit(_))) {
                2
            } else {
                1
            };
            if i == actions_after
                && matches!(
                    ui.selection,
                    Some(
                        Selection::Node(_)
                            | Selection::Crossing(_)
                            | Selection::Unit(_)
                            | Selection::Order(_)
                            | Selection::Edge(_)
                    )
                )
            {
                content_line(
                    "AVAILABLE ACTIONS",
                    x,
                    &mut y,
                    width,
                    13.,
                    TEAL,
                    top,
                    bottom,
                );
                let n = if matches!(ui.selection, Some(Selection::Unit(_))) {
                    4
                } else {
                    2
                };
                for action in actions.iter().take(n) {
                    let h = 37.;
                    if y >= top
                        && y + h < bottom
                        && button(
                            Rect::new(x, y, width, h),
                            &format!(
                                "W · {}   {}{}",
                                action.slot,
                                action.label,
                                if action.enabled {
                                    ""
                                } else {
                                    " · unavailable"
                                }
                            ),
                            false,
                            action.enabled,
                        )
                    {
                        ui.verb = Verb::Order;
                        ui.choose(action.slot as usize - 1, known);
                    }
                    y += h + 6.;
                    if !action.enabled {
                        content_line(
                            &action.reason,
                            x + 8.,
                            &mut y,
                            width - 16.,
                            13.,
                            MUTED,
                            top,
                            bottom,
                        );
                        y += 4.;
                    }
                }
                y += 14.;
            }
            content_line(
                &section.heading.to_uppercase(),
                x,
                &mut y,
                width,
                13.,
                TEAL,
                top,
                bottom,
            );
            y += 4.;
            for line in &section.lines {
                content_line(
                    line,
                    x,
                    &mut y,
                    width,
                    16.,
                    if i == 0 { INK } else { MUTED },
                    top,
                    bottom,
                );
                y += 6.;
            }
            y += 14.;
        }
        if let Some(d) = &s.debug {
            content_line(
                "DEBUG / PRIVATE DIAGNOSTICS",
                x,
                &mut y,
                width,
                14.,
                RED,
                top,
                bottom,
            );
            for (k, value) in d.metrics.iter().take(7) {
                content_line(
                    &format!("{k}: {value}"),
                    x,
                    &mut y,
                    width,
                    14.,
                    MUTED,
                    top,
                    bottom,
                );
                y += 6.;
            }
        }
    } else if ui.tab == 1 {
        if s.orders.is_empty() {
            content_line("NO ORDER ISSUED", x, &mut y, width, 20., INK, top, bottom);
            y += 10.;
            content_line(
                "Select Hearth or a known crossing site. Choose Commission crossing to send an institutional objective.",
                x,
                &mut y,
                width,
                17.,
                MUTED,
                top,
                bottom,
            );
        }
        for o in s.orders.iter().rev() {
            if y >= top
                && y + 35. < bottom
                && button(
                    Rect::new(x, y, width, 35.),
                    &format!("Order {} · issued {}", o.id, hours(o.issued_at)),
                    ui.selection == Some(Selection::Order(o.id)),
                    true,
                )
            {
                choose_selection(ui, Selection::Order(o.id));
            }
            y += 57.;
            stage_strip(s, o, x + 5., &mut y, width - 10., top, bottom);
            y += 20.;
        }
    } else {
        if s.reports.is_empty() {
            content_line(
                "No report held. Q → 3 records a local observation. It cannot query distant places.",
                x,
                &mut y,
                width,
                17.,
                MUTED,
                top,
                bottom,
            );
        }
        for r in s.reports.iter().rev() {
            if y >= top
                && y + 35. < bottom
                && button(
                    Rect::new(x, y, width, 35.),
                    &format!("Report {} · {}", r.id, age(s.now, r.observed_at)),
                    ui.selection == Some(Selection::Report(r.id)),
                    true,
                )
            {
                choose_selection(ui, Selection::Report(r.id));
            }
            y += 57.;
            content_line(
                &format!("{} · food {}", camera::node_name(r.node), r.food_quantity),
                x + 6.,
                &mut y,
                width - 12.,
                17.,
                INK,
                top,
                bottom,
            );
            content_line(
                &format!(
                    "Source: Person {} · observed {}",
                    r.author,
                    hours(r.observed_at)
                ),
                x + 6.,
                &mut y,
                width - 12.,
                14.,
                MUTED,
                top,
                bottom,
            );
            content_line(
                &format!("Received {} · remembered evidence", hours(r.received_at)),
                x + 6.,
                &mut y,
                width - 12.,
                14.,
                GOLD,
                top,
                bottom,
            );
            y += 24.;
        }
    }
    if y > bottom || ui.scroll > 0 {
        text(
            "Scroll to inspect more · exact dates below",
            x,
            screen_height() - 14.,
            13.,
            MUTED,
        );
        if y < top && ui.scroll > 0 {
            ui.scroll = ui.scroll.saturating_sub(2);
        }
    }
}

pub(super) fn footer(
    ui: &mut Ui,
    session: &mut Session,
    s: &mut Snapshot,
    known: &Snapshot,
    v: Rect,
) {
    let top = v.bottom();
    let x = 20.;
    let width = v.w - 40.;
    draw_rectangle(0., top, v.w, screen_height() - top, BG);
    rule(0., top, v.w);
    for (i, (verb, label)) in [
        (Verb::Observe, "Q  OBSERVE"),
        (Verb::Order, "W  ORDER"),
        (Verb::Govern, "E  GOVERN"),
        (Verb::Allocate, "R  ALLOCATE"),
    ]
    .iter()
    .enumerate()
    {
        if button(
            Rect::new(x + i as f32 * width / 4., top + 9., width / 4. - 8., 31.),
            label,
            ui.verb == *verb,
            true,
        ) {
            ui.verb = *verb;
            ui.draft = None;
        }
    }
    let actions = presentation::contextual_actions(known, ui.selection, ui.verb);
    for (i, a) in actions.iter().enumerate() {
        if card(
            Rect::new(x + i as f32 * width / 4., top + 48., width / 4. - 8., 71.),
            &format!("{}  {}", a.slot, a.label),
            &a.reason,
            TEAL,
            a.enabled,
        ) {
            ui.choose(i, known);
        }
    }
    let y = top + 144.;
    match ui.draft.clone() {
        Some(Draft::Crossing) => text(
            "1  COMMISSION   →   2  CLICK CROSSING SITE   →   3  CONFIRM",
            x,
            y,
            17.,
            GOLD,
        ),
        Some(Draft::Unit(verb)) => text(
            &format!("{verb:?}   →   CLICK KNOWN LOCATION   →   CONFIRM  ·  RMB cancels"),
            x,
            y,
            17.,
            GOLD,
        ),
        Some(Draft::Confirm(cmd)) => {
            let label = match cmd {
                PlayerCommand::Commission { .. } => {
                    "READY TO ISSUE · resources and workers still required"
                }
                PlayerCommand::Revoke { .. } => {
                    "SEND REVOCATION · takes effect after physical receipt"
                }
                PlayerCommand::Unit { .. } => "ISSUE UNIT ORDER · only local members hear it",
                PlayerCommand::ObserveReport => "RECORD · ruler's local observation only",
            };
            fitted(label, x, y, width - 186., 16., GOLD);
            if button(
                Rect::new(x + width - 179., y - 23., 179., 32.),
                "ENTER · Confirm",
                true,
                true,
            ) {
                ui.issue(session, cmd);
                *s = session.snapshot(ui.mode);
            }
        }
        None => fitted(
            &ui.notice,
            x,
            y,
            width,
            15.,
            if ui.notice.contains("NOT ISSUED") {
                RED
            } else {
                INK
            },
        ),
    }
    text(
        "Space pause · wheel zoom · arrows / middle drag pan · RMB context / back · F9 world view · F1 controls",
        x,
        screen_height() - 10.,
        12.,
        MUTED,
    );
}
