use crate::{
    sound_map::DawSource,
    sounds::{automated_speed::AutomatedSpeed, sine::SineWave},
};

#[derive(Clone, Debug)]
pub struct AutomatedSineWave<F: DawSource> {
    freq_source: AutomatedSpeed<SineWave, F>,
    speed: f32,
}

impl<F: DawSource> AutomatedSineWave<F> {
    #[inline]
    pub fn new(freq_source: F, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            freq_source: AutomatedSpeed::new(
                SineWave::new(1.0, false, sample_rate, 1.0),
                1.0,
                freq_source,
            ),
            speed: if uses_speed { speed } else { 1.0 },
        }
    }
}

impl<F: DawSource + Clone> DawSource for AutomatedSineWave<F> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        self.freq_source.next(index, channel)
    }
}
