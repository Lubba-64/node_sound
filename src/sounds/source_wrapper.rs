use std::time::Duration;

use super::{SawToothWave, SquareWave, TriangleWave};
use rodio::{
    source::{
        Amplify, BltFilter, Buffered, Delay, FadeIn, Mix, Repeat, SamplesConverter, SineWave,
        SkipDuration, Spatial, Speed, UniformSourceIterator, Zero,
    },
    Sample, Source,
};

type SendIterDyn<T> = Box<dyn Iterator<Item = T> + Send>;

pub struct GenericSource<T>
where
    T: Sample,
{
    sound: SendIterDyn<T>,
    sample_rate: u32,
    channles: u16,
    index: usize,
    duration: Option<std::time::Duration>,
}

unsafe impl<T: Sample> Send for GenericSource<T> {}

impl<T> GenericSource<T>
where
    T: Sample,
{
    fn new(
        sound: SendIterDyn<T>,
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

impl<T> Iterator for GenericSource<T>
where
    T: Sample,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.sound.next()
    }
}

impl<T> Source for GenericSource<T>
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

pub use as_finite_source_impls::*;
pub mod as_finite_source_impls {
    use std::fmt::Debug;

    use super::*;
    pub trait AsGenericSource<T>: Source<Item = T> + Sized + Send
    where
        T: Sample + Send,
    {
        fn as_generic(
            self,
            duration: Option<Duration>,
            repeats: Option<usize>,
        ) -> GenericSource<T> {
            let channels = self.channels();
            let sample_rate = self.sample_rate();
            match repeats {
                Some(x) => {
                    let x = Box::new(self.flat_map(|n| std::iter::repeat(n).take(x)));
                    GenericSource::new(x as SendIterDyn<T>, sample_rate, channels, duration)
                }
                None => GenericSource::new(
                    Box::new(self) as SendIterDyn<T>,
                    sample_rate,
                    channels,
                    duration,
                ),
            }
        }
    }
    impl AsGenericSource<f32> for SineWave {}

    impl AsGenericSource<f32> for SquareWave {}

    impl AsGenericSource<f32> for TriangleWave {}

    impl AsGenericSource<f32> for SawToothWave {}

    impl<I> AsGenericSource<I::Item> for Amplify<I> where I: Source<Item = f32> + Send {}

    impl<I> AsGenericSource<I::Item> for BltFilter<I> where I: Source<Item = f32> + Send {}

    impl<I> AsGenericSource<I::Item> for Delay<I> where I: Source<Item = f32> + Send {}

    impl AsGenericSource<f32> for Zero<f32> {}
    impl<I> AsGenericSource<I::Item> for FadeIn<I> where I: Source<Item = f32> + Send {}
    impl<I1, I2> AsGenericSource<I1::Item> for Mix<I1, I2>
    where
        I1: Source<Item = f32> + Send,
        I2: Source<Item = f32> + Send,
    {
    }

    impl<I> AsGenericSource<I::Item> for Repeat<I> where I: Source<Item = f32> + Send {}
    impl<I> AsGenericSource<I::Item> for SkipDuration<I> where I: Source<Item = f32> + Send {}
    impl<I> AsGenericSource<I::Item> for Spatial<I> where I: Source<Item = f32> + Send {}
    impl<I> AsGenericSource<I::Item> for Speed<I> where I: Source<Item = f32> + Send {}
}
