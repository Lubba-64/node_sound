use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Lfo<I1, I2> {
    input1: I1,
    input2: I2,
}

impl<I1: Source<Item = f32>, I2: Source<Item = f32>> Lfo<I1, I2> {
    #[inline]
    pub fn new(source1: I1, source2: I2) -> Self {
        Self {
            input1: source1,
            input2: source2,
        }
    }
}

impl<I1: Source<Item = f32>, I2: Source<Item = f32>> Iterator for Lfo<I1, I2> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let x = match self.input1.next() {
            Some(x) => x,
            None => 0.0,
        };
        let y = match self.input2.next() {
            Some(x) => x,
            None => 0.0,
        };
        Some(x * y)
    }
}

impl<I1: Source<Item = f32>, I2: Source<Item = f32>> Source for Lfo<I1, I2> {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        DEFAULT_SAMPLE_RATE
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub trait SourceToLfo {
    fn lfo<T2>(self, source2: T2) -> Lfo<Self, T2>
    where
        Self: Source<Item = f32> + Sized,
        T2: Source<Item = f32> + Sized,
    {
        Lfo::new(self, source2)
    }
}
