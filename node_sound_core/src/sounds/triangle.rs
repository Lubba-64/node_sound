use std::f32::consts::PI;

use crate::constants::DEFAULT_SAMPLE_RATE;
use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct TriangleWave {
    frequency: f32,
    speed: f32,
    sample_rate: f32,
    uses_speed: bool,
}

impl TriangleWave {
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

impl DawSource for TriangleWave {
    fn next(&mut self, mut index: f32, _channel: u8) -> Option<f32> {
        index /= self.speed;
        let phase_increment = (2.0 * PI) * self.frequency / self.sample_rate;
        let phase = (phase_increment * index) % (2.0 * PI);
        Some(if phase < PI {
            -1.0 + (2.0 * phase / PI)
        } else {
            3.0 - (2.0 * phase / PI)
        })
    }

    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.sample_rate = rate;
        if self.uses_speed {
            self.speed = speed;
        }
    }

    fn size_hint(&self) -> Option<f32> {
        None
    }
}
