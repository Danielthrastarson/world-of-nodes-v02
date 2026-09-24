use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct MapCamera {
    pub center: Vec2,
    pub zoom: f32,
}
impl Default for MapCamera {
    fn default() -> Self {
        Self {
            center: vec2(625.0, 340.0),
            zoom: 1.05,
        }
    }
}
impl MapCamera {
    pub fn project(&self, world: Vec2, viewport: Rect) -> Vec2 {
        viewport.center() + (world - self.center) * self.zoom
    }
    pub fn unproject(&self, screen: Vec2, viewport: Rect) -> Vec2 {
        self.center + (screen - viewport.center()) / self.zoom
    }
    pub fn zoom_at(&mut self, screen: Vec2, amount: f32, viewport: Rect) {
        let before = self.unproject(screen, viewport);
        self.zoom = (self.zoom * 1.14_f32.powf(amount)).clamp(0.25, 4.0);
        self.center += before - self.unproject(screen, viewport);
    }
    pub fn pan(&mut self, delta: Vec2) {
        self.center -= delta / self.zoom;
    }
}

/// Display layout only. The graph's travel times/capacities remain authoritative.
pub fn node_position(id: usize) -> Vec2 {
    match id {
        0 => vec2(90.0, 385.0),
        1 => vec2(430.0, 395.0),
        2 => vec2(960.0, 405.0),
        3 => vec2(710.0, 90.0),
        4 => vec2(1200.0, 160.0),
        n => vec2(
            80.0 + (n % 6) as f32 * 155.0,
            520.0 + (n / 6) as f32 * 130.0,
        ),
    }
}
pub fn node_name(id: usize) -> String {
    match id {
        0 => "Crown outpost".into(),
        1 => "Hearth settlement".into(),
        2 => "East fields".into(),
        3 => "Old ridge track".into(),
        4 => "Outer approach".into(),
        n => format!("Place {n}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zoom_keeps_target_under_cursor_and_pan_is_reversible() {
        let area = Rect::new(50., 100., 800., 600.);
        let cursor = vec2(207., 409.);
        let mut c = MapCamera::default();
        let target = c.unproject(cursor, area);
        c.zoom_at(cursor, 2., area);
        assert!(c.project(target, area).distance(cursor) < 0.001);
        let before = c.center;
        c.pan(vec2(17., -20.));
        c.pan(vec2(-17., 20.));
        assert!(c.center.distance(before) < 0.001);
    }
}
