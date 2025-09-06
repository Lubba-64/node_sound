use crate::constants::{DEFAULT_SAMPLE_RATE, TWO_PI};
use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct SquareWave {
    frequency: f32,
    speed: f32,
    sample_rate: f32,
    uses_speed: bool,
}

impl SquareWave {
    #[inline]
    pub fn new(frequency: f32, uses_speed: bool) -> Self {
        Self {
            frequency,
            speed: 1.0,
            sample_rate: DEFAULT_SAMPLE_RATE as f32,
            uses_speed,
        }
    }
}

impl DawSource for SquareWave {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        let phase_increment = TWO_PI * self.frequency / self.sample_rate / self.speed;
        let phase = (phase_increment * index) % TWO_PI;
        Some(phase.sin().signum())
    }

    fn note_speed(&mut self, speed: f32) {
        if self.uses_speed {
            self.speed = speed;
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate
    }
}
