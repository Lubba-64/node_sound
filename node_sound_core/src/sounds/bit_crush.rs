use rodio::Source;
use std::time::Duration;

#[derive(Clone)]
pub struct BitCrusher<I: Source<Item = f32>> {
    source: I,
    step_size: f32,
}

impl<I: Source<Item = f32>> BitCrusher<I> {
    #[inline]
    pub fn new(source: I, bits: u32) -> Self {
        let bits = bits.clamp(1, 16); // Practical bit depth range
        let step_size = 2.0 / ((1u32 << bits) - 1) as f32; // Properly using bits
        Self { source, step_size }
    }
}

impl<I: Source<Item = f32>> Iterator for BitCrusher<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.source.next().map(|sample| {
            // Proper quantization using the bit depth
            ((sample / self.step_size).round() * self.step_size).clamp(-1.0, 1.0)
        })
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
