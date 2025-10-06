use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct AutomatedSpeed<I: DawSource, I2: DawSource> {
    source: I,
    base_freq: f32,
    freq: I2,
    last_index: f32,
    adjusted_index: f32,
    sample_rate: f32,
}

impl<I: DawSource, I2: DawSource> AutomatedSpeed<I, I2> {
    pub fn new(source: I, base_freq: f32, freq: I2, sample_rate: f32) -> Self {
        Self {
            source,
            base_freq,
            freq,
            last_index: 0.0,
            adjusted_index: 0.0,
            sample_rate,
        }
    }
}

impl<I: DawSource + Clone, I2: DawSource + Clone> DawSource for AutomatedSpeed<I, I2> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index %= self.sample_rate;
        if self.last_index > index {
            self.last_index = 0.0;
        }
        self.adjusted_index += (index - self.last_index)
            * (self.freq.next(index, channel).unwrap_or(0.0) / self.base_freq);
        self.last_index = index;
        self.source.next(self.adjusted_index, channel)
    }
}
