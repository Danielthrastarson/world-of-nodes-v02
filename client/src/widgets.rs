use macroquad::prelude::*;
use std::cell::Cell;
use std::sync::OnceLock;
// Process-lifetime renderer asset. Avoid a thread-local GPU destructor running
// after Macroquad has destroyed the graphics context on native window exit.
static UI_FONT: OnceLock<Font> = OnceLock::new();
thread_local! {
    static BLOCK_INPUT: Cell<bool> = const { Cell::new(false) };
}
pub fn install_font(font: Font) {
    assert!(
        UI_FONT.set(font).is_ok(),
        "UI font installed once per process"
    );
}
pub fn block_input(blocked: bool) {
    BLOCK_INPUT.with(|b| b.set(blocked));
}

pub const BG: Color = Color::new(0.055, 0.087, 0.100, 1.0);
pub const PANEL: Color = Color::new(0.082, 0.122, 0.135, 1.0);
pub const INK: Color = Color::new(0.88, 0.91, 0.86, 1.0);
pub const MUTED: Color = Color::new(0.50, 0.61, 0.61, 1.0);
pub const LINE: Color = Color::new(0.19, 0.26, 0.27, 1.0);
pub const GOLD: Color = Color::new(0.88, 0.72, 0.43, 1.0);
pub const TEAL: Color = Color::new(0.35, 0.77, 0.69, 1.0);
pub const RED: Color = Color::new(0.90, 0.42, 0.35, 1.0);

pub fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(
        s,
        x,
        y,
        TextParams {
            font: UI_FONT.get(),
            font_size: size as u16,
            color,
            ..Default::default()
        },
    );
}
pub fn rule(x: f32, y: f32, width: f32) {
    draw_line(x, y, x + width, y, 1.0, LINE);
}
pub fn wrap(s: &str, width: f32, size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in s.lines() {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            let candidate = if line.is_empty() {
                word.to_string()
            } else {
                format!("{line} {word}")
            };
            if measure_text(&candidate, UI_FONT.get(), size as u16, 1.0).width > width
                && !line.is_empty()
            {
                lines.push(line);
                line = word.into();
            } else {
                line = candidate;
            }
        }
        lines.push(line);
    }
    lines
}
pub fn paragraph(
    s: &str,
    x: f32,
    y: f32,
    width: f32,
    size: f32,
    color: Color,
    max_lines: usize,
) -> f32 {
    let lines = wrap(s, width, size);
    for (i, line) in lines.iter().take(max_lines).enumerate() {
        text(line, x, y + i as f32 * (size + 5.0), size, color);
    }
    y + lines.len().min(max_lines) as f32 * (size + 5.0)
}
pub fn button(rect: Rect, label: &str, active: bool, enabled: bool) -> bool {
    let mouse = vec2(mouse_position().0, mouse_position().1);
    let hover = rect.contains(mouse);
    let fill = if active {
        Color::new(0.18, 0.29, 0.28, 1.0)
    } else if hover && enabled {
        Color::new(0.14, 0.20, 0.22, 1.0)
    } else {
        PANEL
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        if active { TEAL } else { LINE },
    );
    text(
        label,
        rect.x + 12.0,
        rect.y + rect.h * 0.5 + 5.0,
        18.0,
        if enabled { INK } else { MUTED },
    );
    enabled && !BLOCK_INPUT.with(Cell::get) && hover && is_mouse_button_pressed(MouseButton::Left)
}

pub fn hours(ms: i64) -> String {
    format!("{:.2}h", ms as f64 / 3_600_000.0)
}
pub fn age(now: i64, at: i64) -> String {
    let elapsed = (now - at).max(0);
    if elapsed < 60_000 {
        "just now".into()
    } else if elapsed < 3_600_000 {
        format!("{}m old", elapsed / 60_000)
    } else {
        format!("{:.1}h old", elapsed as f64 / 3_600_000.0)
    }
}
