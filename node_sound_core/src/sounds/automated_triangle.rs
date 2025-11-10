use crate::{
    sound_map::DawSource,
    sounds::{automated_speed::AutomatedSpeed, triangle::TriangleWave},
};

#[derive(Clone, Debug)]
pub struct AutomatedTriangleWave<F: DawSource> {
    freq_source: AutomatedSpeed<TriangleWave, F>,
    speed: f32,
}

impl<F: DawSource> AutomatedTriangleWave<F> {
    #[inline]
    pub fn new(freq_source: F, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            freq_source: AutomatedSpeed::new(
                TriangleWave::new(1.0, false, sample_rate, 1.0),
                1.0,
                freq_source,
            ),
            speed: if uses_speed { speed } else { 1.0 },
        }
    }
}

impl<F: DawSource + Clone> DawSource for AutomatedTriangleWave<F> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        self.freq_source.next(index, channel)
    }
}
