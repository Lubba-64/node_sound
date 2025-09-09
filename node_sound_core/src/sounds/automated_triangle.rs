use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::DawSource};

#[derive(Clone)]
pub struct AutomatedTriangleWave<F: DawSource> {
    freq_source: F,
    speed: f32,
    sample_rate: f32,
    uses_speed: bool,
}

impl<F: DawSource> AutomatedTriangleWave<F> {
    #[inline]
    pub fn new(freq_source: F, uses_speed: bool) -> Self {
        Self {
            freq_source,
            speed: 1.0,
            sample_rate: DEFAULT_SAMPLE_RATE as f32,
            uses_speed,
        }
    }
}

impl<F: DawSource + Clone> DawSource for AutomatedTriangleWave<F> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        let freq = self.freq_source.next(index, channel).unwrap_or(0.0);
        let phase_increment = freq / self.sample_rate;
        let phase = (phase_increment * index) % 1.0;
        let triangle = (((phase * 2.0) - 1.0).abs() - 0.5) * 2.0;
        Some(triangle)
    }

    fn note_speed(&mut self, speed: f32, rate: f32) {
        if self.uses_speed {
            self.speed = speed;
            self.freq_source.note_speed(1.0, rate);
        }
        self.sample_rate = rate;
    }
}
