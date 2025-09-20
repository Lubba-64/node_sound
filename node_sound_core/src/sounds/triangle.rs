use crate::sound_map::DawSource;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct TriangleWave {
    frequency: f32,
    speed: f32,
    sample_rate: f32,
    uses_speed: bool,
}

impl TriangleWave {
    #[inline]
    pub fn new(frequency: f32, uses_speed: bool, sample_rate: f32, speed: f32) -> Self {
        Self {
            frequency,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
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

    fn size_hint(&self) -> Option<f32> {
        None
    }
}
