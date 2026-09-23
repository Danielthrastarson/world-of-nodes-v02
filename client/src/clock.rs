/// A wall-clock playback accumulator, never a second simulation clock.
/// 1x is explicitly two simulated minutes per wall second in this accelerated slice.
#[derive(Debug)]
pub struct Playback {
    pub paused: bool,
    pub speed: u32,
    pending_ms: f64,
}
impl Default for Playback {
    fn default() -> Self {
        Self {
            paused: true,
            speed: 1,
            pending_ms: 0.0,
        }
    }
}
impl Playback {
    pub const STEP_MS: i64 = 60_000;
    pub fn accumulate(&mut self, wall_seconds: f64) {
        if !self.paused && wall_seconds.is_finite() && wall_seconds > 0.0 {
            self.pending_ms += wall_seconds * 120_000.0 * f64::from(self.speed);
        }
    }
    pub fn take_steps(&mut self, budget: u32) -> u32 {
        let steps = (self.pending_ms / Self::STEP_MS as f64)
            .floor()
            .min(f64::from(budget)) as u32;
        self.pending_ms -= f64::from(steps) * Self::STEP_MS as f64;
        steps
    }
    pub fn fraction(&self) -> f32 {
        (self.pending_ms / Self::STEP_MS as f64).clamp(0.0, 1.0) as f32
    }
    pub fn backlog_ms(&self) -> f64 {
        self.pending_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn drawing_cadence_does_not_change_elapsed_simulation_time() {
        let mut a = Playback {
            paused: false,
            ..Default::default()
        };
        let mut b = Playback {
            paused: false,
            ..Default::default()
        };
        let mut ticks = 0;
        for _ in 0..120 {
            a.accumulate(1.0 / 60.0);
            ticks += a.take_steps(8);
        }
        b.accumulate(2.0);
        assert_eq!(ticks, b.take_steps(8));
    }
    #[test]
    fn overload_retains_backlog_and_pause_never_adds_time() {
        let mut c = Playback {
            paused: false,
            ..Default::default()
        };
        c.accumulate(100.0);
        assert_eq!(c.take_steps(8), 8);
        assert!(c.backlog_ms() > 0.0);
        let before = c.backlog_ms();
        c.paused = true;
        c.accumulate(50.0);
        assert_eq!(c.backlog_ms(), before);
    }
}
