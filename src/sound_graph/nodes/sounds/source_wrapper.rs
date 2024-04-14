use std::time::Duration;

use super::{SawToothWave, SquareWave, TriangleWave};
use rodio::{
    source::{
        Amplify, BltFilter, Buffered, Delay, FadeIn, Mix, Repeat, SamplesConverter, SineWave,
        SkipDuration, Spatial, Speed, UniformSourceIterator, Zero,
    },
    Sample, Source,
};

pub struct SourceWrapper<T>
where
    T: Sample,
{
    sound: Box<dyn Iterator<Item = T>>,
    sample_rate: u32,
    channles: u16,
    index: usize,
    duration: Option<std::time::Duration>,
}

impl<T> SourceWrapper<T>
where
    T: Sample,
{
    fn new(
        sound: Box<dyn Iterator<Item = T>>,
        sample_rate: u32,
        channels: u16,
        duration: Option<std::time::Duration>,
    ) -> Self {
        Self {
            sound: sound,
            sample_rate: sample_rate,
            channles: channels,
            index: 0,
            duration: duration,
        }
    }
}

impl<T> Iterator for SourceWrapper<T>
where
    T: Sample,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.sound.next()
    }
}

impl<T> Source for SourceWrapper<T>
where
    T: Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channles
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.duration
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
        fn as_generic(
            self,
            duration: Option<Duration>,
            repeats: Option<usize>,
        ) -> SourceWrapper<T> {
            let channels = self.channels();
            let sample_rate = self.sample_rate();
            match repeats {
                Some(x) => {
                    let x = Box::new(self.flat_map(|n| std::iter::repeat(n).take(x)));
                    SourceWrapper::new(x, sample_rate, channels, duration)
                }
                None => SourceWrapper::new(Box::new(self), sample_rate, channels, duration),
            }
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
