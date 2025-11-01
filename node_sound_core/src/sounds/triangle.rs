use crate::sound_map::{DawSource, Oscillator};
use std::f32::consts::PI;

#[derive(Clone, Debug)]
pub struct TriangleWave {
    frequency: f32,
    speed: f32,
    sample_rate: f32,
    phase: f32,
}

impl TriangleWave {
    #[inline]
    pub fn new(frequency: f32, uses_speed: bool, sample_rate: f32, speed: f32) -> Self {
        Self {
            frequency,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
            phase: 0.0,
        }
    }

    fn calculate(&self) -> f32 {
        if self.phase < PI {
            -1.0 + (2.0 * self.phase / PI)
        } else {
            3.0 - (2.0 * self.phase / PI)
        }
    }
}

impl DawSource for TriangleWave {
    fn next(&mut self, mut index: f32, _channel: u8) -> Option<f32> {
        index /= self.speed;
        let phase_increment = (2.0 * PI) * self.frequency / self.sample_rate;
        self.phase = (phase_increment * index) % (2.0 * PI);
        Some(self.calculate())
    }
}

impl Oscillator for TriangleWave {
    fn set_phase(&mut self, phase: f32) {
        self.phase = phase % (2.0 * PI);
    }
    fn get_phase(&self) -> f32 {
        self.phase
    }
    fn get_frequency(&self) -> f32 {
        self.frequency
    }
    fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }
    fn calculate_output(&self) -> f32 {
        self.calculate()
    }
}
