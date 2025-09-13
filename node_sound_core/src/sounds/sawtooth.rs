use std::f32::consts::PI;

use crate::constants::DEFAULT_SAMPLE_RATE;
use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct SawtoothWave {
    frequency: f32,
    speed: f32,
    sample_rate: f32,
    uses_speed: bool,
}

impl SawtoothWave {
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

impl DawSource for SawtoothWave {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        let phase_increment = 2.0 * PI * self.frequency / self.sample_rate;
        let phase = (phase_increment * index) % (2.0 * PI);
        Some((phase / PI) - 1.0)
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
