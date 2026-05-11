use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct AutomatedSpeed<I: SoundNode, I2: SoundNode> {
    source: I,
    base_freq: f32,
    freq: I2,
    last_index: f32,
    adjusted_index: f32,
}

impl<I: SoundNode, I2: SoundNode> AutomatedSpeed<I, I2> {
    pub fn new(source: I, base_freq: f32, freq: I2) -> Self {
        Self {
            source,
            base_freq,
            freq,
            last_index: 0.0,
            adjusted_index: 0.0,
        }
    }
}

impl<I: SoundNode + Clone, I2: SoundNode + Clone> SoundNode for AutomatedSpeed<I, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.adjusted_index += (index - self.last_index)
            * (self.freq.next(index, channel).unwrap_or(0.0) / self.base_freq);
        self.last_index = index;
        self.source.next(self.adjusted_index, channel)
    }
}
