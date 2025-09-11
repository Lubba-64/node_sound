use crate::constants::{DEFAULT_SAMPLE_RATE, TWO_PI};
use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct SineWave {
    frequency: f32,
    speed: f32,
    sample_rate: f32,
    uses_speed: bool,
}

impl SineWave {
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

impl DawSource for SineWave {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        let phase_increment = TWO_PI * self.frequency / self.sample_rate / self.speed;
        let phase = (phase_increment * index) % TWO_PI;
        Some(phase.sin())
    }

    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.sample_rate = rate;
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
    }

    fn size_hint(&self) -> Option<f32> {
        None
    }
}
