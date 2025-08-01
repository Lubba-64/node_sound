use rodio::Source;
use std::time::Duration;

use crate::sound_map::SetSpeed;

#[derive(Clone)]
pub struct BitCrusher<I: Source<Item = f32>> {
    source: I,
    step_size: f32,
}

impl<I: Source<Item = f32>> BitCrusher<I> {
    #[inline]
    pub fn new(source: I, bits: u32) -> Self {
        let bits = bits.clamp(1, 16);
        let step_size = 1.0 / bits as f32;
        Self { source, step_size }
    }
}

impl<I: Source<Item = f32>> Iterator for BitCrusher<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.source
            .next()
            .map(|sample| ((sample / self.step_size).rem_euclid(self.step_size)).clamp(-1.0, 1.0))
    }
}

impl<I: Source<Item = f32>> Source for BitCrusher<I> {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }
    #[inline]
    fn channels(&self) -> u16 {
        self.source.channels()
    }
    #[inline]
    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }
    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.source.total_duration()
    }
}

impl<I: Source<Item = f32>> SetSpeed<f32> for BitCrusher<I> {
    fn set_speed(&mut self, _speed: f32) {}
}
