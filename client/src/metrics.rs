use std::collections::VecDeque;

#[derive(Default)]
pub struct Metrics {
    frames: VecDeque<f64>,
    pub simulation_ms: f64,
    pub snapshot_ms: f64,
    pub projection_ms: f64,
    pub ambient: f32,
    pub solar_phase: f32,
    pub visible_persons: usize,
    pub effect_cues: usize,
    pub visible_effects: usize,
    pub layer_ms: [f64; 10],
    pub draw_ms: f64,
    pub simulated_steps: u64,
    pub frame_count: u64,
    pub rss_kib: Option<u64>,
}
impl Metrics {
    pub fn frame(&mut self, seconds: f64) {
        if seconds.is_finite() && seconds > 0.0 {
            self.frames.push_back(seconds);
            if self.frames.len() > 120 {
                self.frames.pop_front();
            }
        }
        self.frame_count += 1;
        if self.frame_count % 120 == 1 {
            self.rss_kib = memory_kib();
        }
    }
    pub fn fps(&self) -> f64 {
        let sum: f64 = self.frames.iter().sum();
        if sum > 0.0 {
            self.frames.len() as f64 / sum
        } else {
            0.0
        }
    }
}
#[cfg(target_os = "linux")]
pub fn memory_kib() -> Option<u64> {
    std::fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find(|line| line.starts_with("VmRSS:"))?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}
#[cfg(not(target_os = "linux"))]
pub fn memory_kib() -> Option<u64> {
    None
}
