use std::f32::consts::PI;

use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct SineWave {
    frequency: f32,
    speed: f32,
    sample_rate: f32,
}

impl SineWave {
    #[inline]
    pub fn new(frequency: f32, uses_speed: bool, sample_rate: f32, speed: f32) -> Self {
        Self {
            frequency,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl DawSource for SineWave {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        let phase_increment = (2.0 * PI) * self.frequency / self.sample_rate / self.speed;
        let phase = (phase_increment * index) % (2.0 * PI);
        Some(phase.sin())
    }

    fn size_hint(&self) -> Option<f32> {
        None
    }
}
