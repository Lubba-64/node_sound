use std::time::Duration;

use rodio::{
    source::{
        Amplify, BltFilter, Buffered, Delay, FadeIn, Mix, Repeat, SamplesConverter, SineWave,
        SkipDuration, Spatial, Speed, UniformSourceIterator, Zero,
    },
    Sample, Source,
};

use super::{SawToothWave, SquareWave, TriangleWave};
#[derive(Clone)]
pub struct FiniteSource<T>
where
    T: Sample,
{
    sound: Vec<T>,
    sample_rate: u32,
    channles: u16,
    index: usize,
}

impl<T> FiniteSource<T>
where
    T: Sample,
{
    fn new(sound: Vec<T>, sample_rate: u32, channels: u16) -> Self {
        Self {
            sound: sound,
            sample_rate: sample_rate,
            channles: channels,
            index: 0,
        }
    }
}

impl<T> Iterator for FiniteSource<T>
where
    T: Sample,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + 1 >= self.sound.len() {
            return None;
        }
        self.index += 1;
        Some(self.sound[self.index])
    }
}

impl<T> Source for FiniteSource<T>
where
    T: Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        let diff = self.sound.len() - self.index;
        if diff == 0 {
            return None;
        }
        Some(diff)
    }

    fn channels(&self) -> u16 {
        self.channles
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(Duration::from_secs_f32(
            self.sound.len() as f32 / self.sample_rate as f32,
        ))
    }
}

pub use as_finite_source::*;
pub mod as_finite_source {
    use std::fmt::Debug;

    use super::*;
    pub trait AsFiniteSource<T>: Source<Item = T> + Sized
    where
        T: Sample,
    {
        fn as_finite(self, duration: Duration) -> FiniteSource<T> {
            let channels = self.channels();
            let sample_rate = self.sample_rate();
            let sound: Vec<T> = self.take_duration(duration).collect();
            FiniteSource::new(sound, sample_rate, channels)
        }
    }
    impl AsFiniteSource<f32> for SineWave {}

    impl AsFiniteSource<f32> for SquareWave {}

    impl AsFiniteSource<f32> for TriangleWave {}

    impl AsFiniteSource<f32> for SawToothWave {}

    impl<I> AsFiniteSource<I::Item> for Amplify<I>
    where
        I: Source,
        I::Item: Sample,
    {
    }

    impl<I> AsFiniteSource<I::Item> for BltFilter<I> where I: Source<Item = f32> {}

    impl<I> AsFiniteSource<I::Item> for Buffered<I>
    where
        I: Source,
        I::Item: Sample,
    {
    }

    impl<I> AsFiniteSource<I::Item> for Delay<I>
    where
        I: Source,
        I::Item: Sample,
    {
    }

    impl<S> AsFiniteSource<S> for Zero<S> where S: Sample {}
    impl<I> AsFiniteSource<I::Item> for FadeIn<I>
    where
        I: Source,
        I::Item: Sample,
    {
    }
    impl<I1, I2> AsFiniteSource<I1::Item> for Mix<I1, I2>
    where
        I1: Source,
        I1::Item: Sample,
        I2: Source,
        I2::Item: Sample,
    {
    }

    impl<I> AsFiniteSource<I::Item> for Repeat<I>
    where
        I: Source,
        I::Item: Sample,
    {
    }

    impl<I, D> AsFiniteSource<D> for SamplesConverter<I, D>
    where
        I: Source,
        I::Item: Sample,
        D: Sample,
    {
    }
    impl<I> AsFiniteSource<I::Item> for SkipDuration<I>
    where
        I: Source,
        I::Item: Sample,
    {
    }
    impl<I> AsFiniteSource<I::Item> for Spatial<I>
    where
        I: Source,
        I::Item: Sample + Debug,
    {
    }
    impl<I> AsFiniteSource<I::Item> for Speed<I>
    where
        I: Source,
        I::Item: Sample + Debug,
    {
    }
    impl<I, D> AsFiniteSource<D> for UniformSourceIterator<I, D>
    where
        I: Source,
        I::Item: Sample,
        D: Sample,
    {
    }
}
