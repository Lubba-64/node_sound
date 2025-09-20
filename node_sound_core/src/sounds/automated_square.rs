use crate::sound_map::DawSource;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct AutomatedSquareWave<F: DawSource> {
    freq_source: F,
    speed: f32,
    sample_rate: f32,
    phase: f32,
}

impl<F: DawSource> AutomatedSquareWave<F> {
    #[inline]
    pub fn new(freq_source: F, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            freq_source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
            phase: 0.0,
        }
    }
}

impl<F: DawSource + Clone> DawSource for AutomatedSquareWave<F> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        let freq = self.freq_source.next(index, channel).unwrap_or(0.0);
        let phase_increment = 2.0 * PI * freq / self.sample_rate;
        self.phase = (self.phase + phase_increment * (index % (2.0 * PI))) % (2.0 * PI);
        Some(if self.phase < PI { 1.0 } else { -1.0 })
    }

    fn size_hint(&self) -> Option<f32> {
        None
    }
}
