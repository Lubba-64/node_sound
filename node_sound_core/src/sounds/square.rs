use crate::sound_map::DawSource;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct SquareWave {
    frequency: f32,
    speed: f32,
    sample_rate: f32,
}

impl SquareWave {
    #[inline]
    pub fn new(frequency: f32, uses_speed: bool, sample_rate: f32, speed: f32) -> Self {
        Self {
            frequency,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl DawSource for SquareWave {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        let phase_increment = (2.0 * PI) * self.frequency / self.sample_rate / self.speed;
        let phase = (phase_increment * index) % (2.0 * PI);
        Some(if phase < PI { 1.0 } else { -1.0 })
    }

    fn size_hint(&self) -> Option<f32> {
        None
    }
}
