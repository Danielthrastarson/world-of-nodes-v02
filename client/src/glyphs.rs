//! Small schematic marks for actual snapshot objects. The marks depict known
//! state, never query the world, and have no effect on simulation geometry.
use macroquad::prelude::*;
use world_of_nodes::provisioning::playable::snapshot::{
    CartSnapshot, CrossingSnapshot, Evidence, FieldSnapshot, GoodsSnapshot, KnowledgeSource,
    PersonSnapshot,
};

const OUTLINE: Color = Color::new(0.16, 0.20, 0.17, 1.0);
const CLOTH: Color = Color::new(0.90, 0.86, 0.68, 1.0);
const SKIN: Color = Color::new(0.91, 0.73, 0.51, 1.0);
const TIMBER: Color = Color::new(0.53, 0.35, 0.20, 1.0);
const TIMBER_LIGHT: Color = Color::new(0.72, 0.52, 0.31, 1.0);
const SOIL: Color = Color::new(0.39, 0.31, 0.20, 1.0);
const FURROW: Color = Color::new(0.25, 0.23, 0.15, 1.0);
const CROP: Color = Color::new(0.57, 0.67, 0.29, 1.0);
const GRAIN: Color = Color::new(0.88, 0.70, 0.32, 1.0);
const GOLD: Color = Color::new(1.0, 0.84, 0.34, 1.0);
const MILITARY: Color = Color::new(0.35, 0.70, 0.72, 1.0);
const HOSTILE: Color = Color::new(0.88, 0.36, 0.28, 1.0);
const PAPER: Color = Color::new(0.82, 0.88, 0.87, 1.0);

fn current(e: &Evidence) -> bool {
    matches!(
        e.source,
        KnowledgeSource::Local | KnowledgeSource::SelfKnowledge | KnowledgeSource::Debug
    )
}
fn tint(mut color: Color, evidence: &Evidence) -> Color {
    if !current(evidence) {
        // Muted historical marks do not pretend to be presently observed objects.
        let grey = (color.r + color.g + color.b) / 3.0;
        color.r = color.r * 0.65 + grey * 0.35;
        color.g = color.g * 0.65 + grey * 0.35;
        color.b = color.b * 0.65 + grey * 0.35;
        color.a *= if evidence.source == KnowledgeSource::Report {
            0.72
        } else {
            0.52
        };
    }
    color
}
fn line(a: Vec2, b: Vec2, width: f32, color: Color) {
    draw_line(a.x, a.y, b.x, b.y, width.max(0.7), color);
}
fn selected_box(at: Vec2, half: Vec2, scale: f32) {
    draw_rectangle_lines(
        at.x - half.x - 4.0 * scale,
        at.y - half.y - 4.0 * scale,
        2.0 * half.x + 8.0 * scale,
        2.0 * half.y + 8.0 * scale,
        (2.0 * scale).max(1.3),
        GOLD,
    );
}
fn report_mark(e: &Evidence, at: Vec2, scale: f32) {
    if e.source != KnowledgeSource::Report {
        return;
    }
    draw_rectangle(at.x, at.y, 6.0 * scale, 8.0 * scale, tint(PAPER, e));
    draw_rectangle_lines(
        at.x,
        at.y,
        6.0 * scale,
        8.0 * scale,
        scale.max(0.7),
        tint(OUTLINE, e),
    );
    line(
        at + vec2(1.5, 3.0) * scale,
        at + vec2(4.5, 3.0) * scale,
        scale,
        tint(OUTLINE, e),
    );
    line(
        at + vec2(1.5, 5.0) * scale,
        at + vec2(4.0, 5.0) * scale,
        scale,
        tint(OUTLINE, e),
    );
}

/// The same embodied Person mark gains role badges; it is never an army counter.
/// No courier or incapacitation badge is guessed from hidden/private body state.
pub fn person(p: &PersonSnapshot, at: Vec2, scale: f32, viewer: bool, selected: bool) {
    let s = scale.max(0.25);
    let e = &p.evidence;
    let outline = tint(OUTLINE, e);
    if selected {
        draw_circle_lines(at.x, at.y, 12.5 * s, (2.2 * s).max(1.3), GOLD);
    }
    if p.action.as_deref() == Some("dead") {
        // A fallen body is the same existing Person, not a spawned grave.
        line(
            at + vec2(-5.0, 2.0) * s,
            at + vec2(3.0, -1.0) * s,
            4.0 * s,
            outline,
        );
        draw_circle(at.x + 5.0 * s, at.y - 2.0 * s, 2.6 * s, tint(SKIN, e));
        line(
            at + vec2(-5.0, 2.0) * s,
            at + vec2(-8.0, 0.0) * s,
            1.5 * s,
            outline,
        );
        line(
            at + vec2(-5.0, 2.0) * s,
            at + vec2(-7.0, 5.0) * s,
            1.5 * s,
            outline,
        );
        report_mark(e, at + vec2(7.0, 4.0) * s, s);
        return;
    }
    let cloth = if viewer {
        GOLD
    } else if p.hostile == Some(true) {
        HOSTILE
    } else if p.unit.is_some() {
        MILITARY
    } else {
        CLOTH
    };
    if current(e) {
        draw_ellipse(at.x, at.y + 1.8 * s, 4.9 * s, 5.8 * s, 0.0, outline);
        draw_ellipse(at.x, at.y + 1.7 * s, 3.5 * s, 4.5 * s, 0.0, tint(cloth, e));
        draw_circle(at.x, at.y - 4.7 * s, 3.7 * s, outline);
        draw_circle(at.x, at.y - 4.7 * s, 2.6 * s, tint(SKIN, e));
    } else {
        draw_ellipse_lines(
            at.x,
            at.y + 1.8 * s,
            4.6 * s,
            5.6 * s,
            0.0,
            (1.3 * s).max(0.8),
            tint(cloth, e),
        );
        draw_circle_lines(
            at.x,
            at.y - 4.7 * s,
            3.2 * s,
            (1.2 * s).max(0.8),
            tint(SKIN, e),
        );
    }
    if p.unit.is_some() {
        let badge = tint(
            if p.hostile == Some(true) {
                HOSTILE
            } else {
                MILITARY
            },
            e,
        );
        // A small shield beside the common Person body means known membership.
        let b = at + vec2(5.0, 0.5) * s;
        draw_rectangle(b.x - 1.0 * s, b.y - 3.0 * s, 5.0 * s, 5.0 * s, outline);
        draw_triangle(
            b + vec2(-1.0, 2.0) * s,
            b + vec2(4.0, 2.0) * s,
            b + vec2(1.5, 5.0) * s,
            outline,
        );
        draw_rectangle(b.x, b.y - 2.0 * s, 3.0 * s, 3.8 * s, badge);
        draw_triangle(
            b + vec2(0.0, 1.5) * s,
            b + vec2(3.0, 1.5) * s,
            b + vec2(1.5, 3.6) * s,
            badge,
        );
    } else if p.hostile == Some(true) {
        // Known hostile behavior does not imply a military role.
        draw_circle_lines(at.x, at.y, 9.0 * s, (1.7 * s).max(1.0), tint(HOSTILE, e));
    }
    if p.role.as_deref().is_some_and(|r| r.contains("Steward")) {
        let b = at + vec2(-7.0, 2.0) * s;
        draw_rectangle(
            b.x - 2.5 * s,
            b.y - 3.5 * s,
            5.0 * s,
            7.0 * s,
            tint(PAPER, e),
        );
        draw_circle(b.x, b.y + 1.0 * s, 1.5 * s, tint(GOLD, e));
    }
    if p.role.as_deref().is_some_and(|r| r.contains("Sovereign")) {
        let b = at + vec2(0.0, -9.0) * s;
        line(
            b + vec2(-3.0, 0.0) * s,
            b + vec2(3.0, 0.0) * s,
            1.6 * s,
            tint(GOLD, e),
        );
        for x in [-3.0, 0.0, 3.0] {
            line(
                b + vec2(x, 0.0) * s,
                b + vec2(x, -2.5) * s,
                1.2 * s,
                tint(GOLD, e),
            );
        }
    }
    report_mark(e, at + vec2(7.0, 5.0) * s, s);
}

/// A plot stays bare when no crop exists. A harvesting marker describes an
/// undertaking, not an invented completion percentage or a second worker.
pub fn field(f: &FieldSnapshot, at: Vec2, scale: f32, selected: bool) {
    let s = scale.max(0.25);
    let e = &f.evidence;
    let half = vec2(27.0, 18.0) * s;
    let has_crop = f.crop.is_some();
    if !has_crop {
        // An available cultivation unit is suitable ground, not proof that any
        // Person has ploughed it or previously harvested a crop here.
        let arable = Color::new(0.57, 0.61, 0.38, 1.0);
        let ground = Color::new(0.48, 0.53, 0.33, 1.0);
        draw_rectangle(
            at.x - half.x,
            at.y - half.y,
            2.0 * half.x,
            2.0 * half.y,
            tint(arable, e),
        );
        for (x, y) in [(-17.0, -8.0), (-5.0, 6.0), (9.0, -6.0), (15.0, 9.0)] {
            line(
                at + vec2(x, y) * s,
                at + vec2(x + 5.0, y - 1.0) * s,
                1.0 * s,
                tint(ground, e),
            );
        }
        // Corner brackets denote the known plot boundary, not physical fencing.
        for side_x in [-1.0_f32, 1.0] {
            for side_y in [-1.0_f32, 1.0] {
                let corner = at + vec2(side_x * 27.0, side_y * 18.0) * s;
                line(
                    corner,
                    corner - vec2(side_x * 7.0, 0.0) * s,
                    1.3 * s,
                    tint(TIMBER_LIGHT, e),
                );
                line(
                    corner,
                    corner - vec2(0.0, side_y * 7.0) * s,
                    1.3 * s,
                    tint(TIMBER_LIGHT, e),
                );
            }
        }
        if selected {
            selected_box(at, half, s);
        }
        report_mark(e, at + vec2(23.0, 12.0) * s, s);
        return;
    }
    draw_rectangle(
        at.x - half.x,
        at.y - half.y,
        half.x * 2.0,
        half.y * 2.0,
        tint(SOIL, e),
    );
    draw_rectangle_lines(
        at.x - half.x,
        at.y - half.y,
        half.x * 2.0,
        half.y * 2.0,
        (1.5 * s).max(0.8),
        tint(TIMBER_LIGHT, e),
    );
    let mature = matches!(f.stage.as_str(), "mature" | "harvesting");
    let sowing = f.stage == "sowing";
    for row in 0..5 {
        let y = -12.0 + row as f32 * 6.0;
        line(
            at + vec2(-23.0, y + 1.5) * s,
            at + vec2(23.0, y + 1.5) * s,
            2.0 * s,
            tint(FURROW, e),
        );
        if has_crop {
            for plant in 0..8 {
                let x = -21.0 + plant as f32 * 6.0;
                let base = at + vec2(x, y) * s;
                if sowing {
                    draw_circle(base.x, base.y, 0.9 * s, tint(GRAIN, e));
                } else {
                    let color = tint(if mature { GRAIN } else { CROP }, e);
                    line(base, base - vec2(0.0, 3.5) * s, 1.2 * s, color);
                    line(
                        base - vec2(0.0, 1.0) * s,
                        base - vec2(2.0, 3.0) * s,
                        1.1 * s,
                        color,
                    );
                    line(
                        base - vec2(0.0, 1.0) * s,
                        base + vec2(2.0, -3.0) * s,
                        1.1 * s,
                        color,
                    );
                    if mature {
                        draw_circle(base.x, base.y - 4.0 * s, 1.2 * s, color);
                    }
                }
            }
        }
    }
    if f.stage == "harvesting" && has_crop {
        let b = at + vec2(22.0, -14.0) * s;
        draw_circle(b.x, b.y, 5.5 * s, tint(OUTLINE, e));
        line(
            b + vec2(-2.0, 3.0) * s,
            b + vec2(2.0, -3.0) * s,
            1.4 * s,
            tint(PAPER, e),
        );
        line(
            b + vec2(-2.0, -2.0) * s,
            b + vec2(2.0, -3.0) * s,
            1.4 * s,
            tint(PAPER, e),
        );
    }
    if selected {
        selected_box(at, half, s);
    }
    report_mark(e, at + vec2(23.0, 12.0) * s, s);
}

fn stock_level(quantity: u64) -> f32 {
    // Each batch remains one object. Six symbolic marks encode its quantity on
    // a bounded logarithmic scale, with the last mark's area showing the
    // fractional level. A mark is not a simulated container or a fixed quantum.
    // The display saturates at 256; inspection retains the exact quantity.
    (6.0 * (quantity.min(256) as f64).ln_1p() / 257.0_f64.ln()) as f32
}
fn stock_marks(quantity: u64) -> usize {
    stock_level(quantity).ceil().min(6.0) as usize
}

pub fn goods(g: &GoodsSnapshot, at: Vec2, scale: f32, selected: bool) {
    let s = scale.max(0.25);
    let e = &g.evidence;
    let level = stock_level(g.quantity);
    let count = stock_marks(g.quantity);
    let wood = g.material.contains("Wood");
    let food = g.material.contains("Food");
    for item in 0..count {
        let b = at + vec2((item % 3) as f32 * 9.0 - 9.0, 4.0 - (item / 3) as f32 * 9.0) * s;
        // Scaling area instead of merely rounding the count keeps changes such
        // as 24 -> 20 or 100 -> 80 visible without adding floating numbers.
        let piece = s * (level - item as f32).clamp(0.0, 1.0).sqrt();
        if wood {
            line(
                b + vec2(-3.0, 2.0) * piece,
                b + vec2(3.0, -3.0) * piece,
                6.0 * piece,
                tint(OUTLINE, e),
            );
            line(
                b + vec2(-3.0, 2.0) * piece,
                b + vec2(3.0, -3.0) * piece,
                4.0 * piece,
                tint(TIMBER, e),
            );
            draw_circle(
                b.x - 3.0 * piece,
                b.y + 2.0 * piece,
                2.0 * piece,
                tint(TIMBER_LIGHT, e),
            );
        } else if food {
            draw_ellipse(b.x, b.y, 4.7 * piece, 5.6 * piece, 0.0, tint(OUTLINE, e));
            draw_ellipse(b.x, b.y, 3.6 * piece, 4.7 * piece, 0.0, tint(GRAIN, e));
            line(
                b + vec2(-2.0, -3.0) * piece,
                b + vec2(2.0, -3.0) * piece,
                1.1 * piece,
                tint(TIMBER, e),
            );
        } else {
            draw_poly(b.x, b.y, 5, 5.0 * piece, 18.0, tint(OUTLINE, e));
            draw_poly(b.x, b.y, 5, 3.8 * piece, 18.0, tint(PAPER, e));
        }
    }
    if count == 0 {
        draw_ellipse_lines(
            at.x,
            at.y,
            11.0 * s,
            6.0 * s,
            0.0,
            s.max(0.7),
            tint(OUTLINE, e),
        );
    }
    if selected {
        selected_box(at, vec2(15.0, 12.0) * s, s);
    }
    report_mark(e, at + vec2(12.0, 6.0) * s, s);
}

pub fn cart(c: &CartSnapshot, at: Vec2, scale: f32, selected: bool) {
    let s = scale.max(0.25);
    let e = &c.evidence;
    for y in [-9.0, 9.0] {
        draw_rectangle(
            at.x - 9.0 * s,
            at.y + (y - 2.0) * s,
            15.0 * s,
            4.0 * s,
            tint(OUTLINE, e),
        );
        line(
            at + vec2(-7.0, y) * s,
            at + vec2(4.0, y) * s,
            1.1 * s,
            tint(TIMBER_LIGHT, e),
        );
    }
    draw_rectangle(
        at.x - 13.0 * s,
        at.y - 8.0 * s,
        23.0 * s,
        16.0 * s,
        tint(OUTLINE, e),
    );
    draw_rectangle(
        at.x - 11.5 * s,
        at.y - 6.5 * s,
        20.0 * s,
        13.0 * s,
        tint(TIMBER, e),
    );
    for y in [-4.0, 0.0, 4.0] {
        line(
            at + vec2(-10.0, y) * s,
            at + vec2(7.0, y) * s,
            s,
            tint(TIMBER_LIGHT, e),
        );
    }
    // An empty cart depiction never invents cargo; goods use their own snapshot.
    for y in [-4.0, 4.0] {
        line(
            at + vec2(9.0, y) * s,
            at + vec2(19.0, y) * s,
            2.0 * s,
            tint(TIMBER_LIGHT, e),
        );
    }
    if selected {
        selected_box(at + vec2(3.0, 0.0) * s, vec2(17.0, 12.0) * s, s);
    }
    report_mark(e, at + vec2(12.0, 7.0) * s, s);
}

fn crossing_planks(c: &CrossingSnapshot) -> usize {
    if c.completed == Some(true) {
        return 9;
    }
    // This is committed material, not fabricated labor progress. The last gap
    // stays visibly open until the completed flag is actually known true.
    match (c.wood_committed, c.wood_required) {
        (Some(wood), Some(required)) if required > 0 => {
            ((wood.min(required) as u128 * 7) / required as u128) as usize
        }
        _ => 0,
    }
}

pub fn crossing(c: &CrossingSnapshot, at: Vec2, scale: f32, selected: bool) {
    let s = scale.max(0.25);
    let e = &c.evidence;
    let completed = c.completed == Some(true);
    let planks = crossing_planks(c);
    for side in [-1.0, 1.0] {
        let b = at + vec2(side * 28.0, 0.0) * s;
        if completed {
            draw_rectangle(
                b.x - 3.0 * s,
                b.y - 13.0 * s,
                6.0 * s,
                26.0 * s,
                tint(TIMBER_LIGHT, e),
            );
        } else {
            // Site corner marks are a blueprint, not pre-existing foundations.
            for y in [-12.0, 12.0] {
                line(
                    b + vec2(0.0, y) * s,
                    b + vec2(-side * 5.0, y) * s,
                    1.3 * s,
                    tint(PAPER, e),
                );
                line(
                    b + vec2(0.0, y) * s,
                    b + vec2(0.0, y - y.signum() * 5.0) * s,
                    1.3 * s,
                    tint(PAPER, e),
                );
            }
        }
    }
    for plank in 0..planks {
        let x = -23.0 + plank as f32 * 5.75;
        draw_rectangle(
            at.x + (x - 2.3) * s,
            at.y - 10.0 * s,
            4.6 * s,
            20.0 * s,
            tint(TIMBER, e),
        );
        line(
            at + vec2(x - 1.0, -8.0) * s,
            at + vec2(x - 1.0, 8.0) * s,
            s,
            tint(TIMBER_LIGHT, e),
        );
    }
    if completed {
        for y in [-11.0, 11.0] {
            line(
                at + vec2(-28.0, y) * s,
                at + vec2(28.0, y) * s,
                2.5 * s,
                tint(TIMBER_LIGHT, e),
            );
        }
    } else {
        // Dashed preview preserves the visual difference between an available
        // crossing location and a connection that has actually been built.
        for x in [-20.0, -8.0, 4.0, 16.0] {
            line(
                at + vec2(x, 0.0) * s,
                at + vec2(x + 5.0, 0.0) * s,
                1.1 * s,
                tint(PAPER, e),
            );
        }
    }
    if selected {
        selected_box(at, vec2(32.0, 15.0) * s, s);
    }
    report_mark(e, at + vec2(27.0, 9.0) * s, s);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> CrossingSnapshot {
        CrossingSnapshot {
            id: 0,
            node: 0,
            edge: Some(0),
            owner: None,
            wood_committed: Some(0),
            wood_required: Some(20),
            work_remaining: Some(100),
            completed: Some(false),
            authorized: Some(true),
            evidence: Evidence {
                source: KnowledgeSource::Local,
                observed_at: 0,
                received_at: None,
                author: None,
                sender: None,
            },
        }
    }

    #[test]
    fn crossing_never_shows_completed_from_material_alone() {
        let mut c = site();
        assert_eq!(crossing_planks(&c), 0);
        c.wood_committed = Some(20);
        c.work_remaining = Some(0);
        assert_eq!(crossing_planks(&c), 7);
        c.completed = None;
        assert_eq!(crossing_planks(&c), 7);
        c.completed = Some(true);
        assert_eq!(crossing_planks(&c), 9);
    }

    #[test]
    fn unknown_crossing_materials_do_not_create_timbers() {
        let mut c = site();
        c.wood_committed = None;
        assert_eq!(crossing_planks(&c), 0);
        c.wood_required = Some(0);
        assert_eq!(crossing_planks(&c), 0);
    }

    #[test]
    fn stock_marks_are_bounded_and_zero_stays_empty() {
        assert_eq!(stock_level(0), 0.0);
        assert_eq!(stock_marks(0), 0);
        assert_eq!(stock_marks(1), 1);
        assert_eq!(stock_marks(6), 3);
        assert_eq!(stock_marks(200), 6);
        assert_eq!(stock_level(256), 6.0);
        assert_eq!(stock_level(u64::MAX), 6.0);
        assert_eq!(stock_marks(u64::MAX), 6);
    }

    #[test]
    fn stock_quantity_changes_remain_visible_before_saturation() {
        assert!(stock_level(24) > stock_level(20));
        assert!(stock_level(100) > stock_level(80));
        assert!(stock_level(200) > stock_level(6));
        // These pairs share a mark count, so the partial mark must carry the
        // quantity difference instead of rounding both stocks to identical piles.
        assert_eq!(stock_marks(24), stock_marks(20));
        assert_eq!(stock_marks(100), stock_marks(80));
        for quantity in 1..=256 {
            assert!(stock_level(quantity) > stock_level(quantity - 1));
        }
        for quantity in 0..1024 {
            assert!(stock_level(quantity) <= stock_level(quantity + 1));
            assert!(stock_marks(quantity) <= 6);
        }
    }
}
