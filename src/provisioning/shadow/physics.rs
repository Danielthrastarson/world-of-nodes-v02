//! Independent action arithmetic. No World reference or truth query is permitted.
use super::super::{HOUR, MINUTE, SCALE, Time};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Body {
    pub elapsed: Time,
    pub nutrition: Time,
    pub fatigue: Time,
    pub starvation: Time,
}
impl Body {
    pub fn alive(&self) -> bool {
        self.starvation < 24 * HOUR
    }
    pub fn elapse(&mut self, dt: Time, sleep: bool) {
        let dt = dt.min(self.nutrition + 24 * HOUR - self.starvation).max(0);
        self.starvation += (dt - self.nutrition).max(0);
        self.nutrition = (self.nutrition - dt).max(0);
        self.fatigue = if sleep {
            (self.fatigue - 2 * dt).max(0)
        } else {
            (self.fatigue + dt).min(24 * HOUR)
        };
        self.elapsed += dt;
    }
    pub fn work(&mut self, nominal: Time) {
        let mut work = nominal as u128 * SCALE;
        while work > 0 && self.alive() {
            let h = if self.nutrition > 6 * HOUR {
                1000
            } else if self.nutrition > 0 {
                800
            } else {
                500
            };
            let f = if self.fatigue < 12 * HOUR {
                1000
            } else if self.fatigue < 16 * HOUR {
                900
            } else if self.fatigue < 20 * HOUR {
                750
            } else {
                500
            };
            let rate = h * f;
            let mut dt = work.div_ceil(rate) as Time;
            for boundary in [0, 6 * HOUR] {
                if self.nutrition > boundary {
                    dt = dt.min(self.nutrition - boundary);
                }
            }
            for boundary in [12, 16, 20, 22, 24].map(|h| h * HOUR) {
                if self.fatigue < boundary {
                    dt = dt.min(boundary - self.fatigue);
                }
            }
            dt = dt.min(self.nutrition + 24 * HOUR - self.starvation);
            work = work.saturating_sub(rate * dt as u128);
            self.elapse(dt, false);
        }
    }
    pub fn eat(&mut self, quantity: u64, communal: bool) -> bool {
        let ceiling = if communal { 12 } else { 24 } * HOUR;
        if quantity == 0 || quantity as Time * HOUR > ceiling - self.nutrition || !self.alive() {
            return false;
        }
        self.nutrition += quantity as Time * HOUR;
        self.starvation = 0;
        true
    }
    pub fn sleep_tick(&mut self) {
        self.elapse(MINUTE, true);
    }
}
