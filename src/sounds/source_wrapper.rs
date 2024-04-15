use std::time::Duration;

use super::{SawToothWave, SquareWave, TriangleWave};
use dyn_clone::DynClone;
use rodio::{
    source::{
        Amplify, BltFilter, Delay, FadeIn, Mix, Repeat, SineWave, SkipDuration, Spatial, Speed,
        TakeDuration, Zero,
    },
    Sample, Source,
};

pub trait DynCloneIter<T>: Iterator<Item = T> + Send + DynClone {}

type SendIterDyn<T> = dyn DynCloneIter<T>;

pub struct GenericSource<T>
where
    T: Sample,
{
    sound: Box<SendIterDyn<T>>,
    sample_rate: u32,
    channles: u16,
    duration: Option<std::time::Duration>,
}

impl<T: Sample> Clone for GenericSource<T> {
    fn clone(&self) -> Self {
        GenericSource {
            channles: self.channles,
            duration: self.duration,
            sample_rate: self.sample_rate,
            sound: dyn_clone::clone_box(&(*self.sound)),
        }
    }
}

unsafe impl<T: Sample> Send for GenericSource<T> {}

impl<T> GenericSource<T>
where
    T: Sample,
{
    fn new(
        sound: Box<SendIterDyn<T>>,
        sample_rate: u32,
        channels: u16,
        duration: Option<std::time::Duration>,
    ) -> Self {
        Self {
            sound: sound,
            sample_rate: sample_rate,
            channles: channels,
            duration: duration,
        }
    }
}

impl<'a> Iterator for GenericSource<f32> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.sound.next()
    }
}

impl<'a> Source for GenericSource<f32> {
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
    impl DynCloneIter<f32> for std::vec::IntoIter<f32> {}
    use super::*;
    pub trait AsGenericSource: Source + DynCloneIter<f32> + Sized + Send + Clone + 'static {
        fn as_generic(&self, duration: Option<Duration>) -> GenericSource<f32>
        where
            Self: Sized,
        {
            let channels = self.channels();
            let sample_rate = self.sample_rate();
            GenericSource::new(Box::new(self.clone()), sample_rate, channels, duration)
        }
    }

    trait StaticSource: Source<Item = f32> + Send + Clone + 'static {}

    impl DynCloneIter<f32> for SineWave {}
    impl AsGenericSource for SineWave {}
    impl StaticSource for SineWave {}
    impl DynCloneIter<f32> for SquareWave {}
    impl AsGenericSource for SquareWave {}
    impl StaticSource for SquareWave {}
    impl DynCloneIter<f32> for TriangleWave {}
    impl AsGenericSource for TriangleWave {}
    impl StaticSource for TriangleWave {}
    impl DynCloneIter<f32> for SawToothWave {}
    impl AsGenericSource for SawToothWave {}
    impl StaticSource for SawToothWave {}
    impl<I> DynCloneIter<f32> for Amplify<I> where I: StaticSource {}
    impl<I> AsGenericSource for Amplify<I> where I: StaticSource {}
    impl<I> StaticSource for Amplify<I> where I: StaticSource {}
    impl<I> DynCloneIter<f32> for BltFilter<I> where I: StaticSource {}
    impl<I> AsGenericSource for BltFilter<I> where I: StaticSource {}
    impl<I> StaticSource for BltFilter<I> where I: StaticSource {}
    impl<I> DynCloneIter<f32> for Delay<I> where I: StaticSource {}
    impl<I> AsGenericSource for Delay<I> where I: StaticSource {}
    impl<I> StaticSource for Delay<I> where I: StaticSource {}
    impl DynCloneIter<f32> for Zero<f32> {}
    impl AsGenericSource for Zero<f32> {}
    impl StaticSource for Zero<f32> {}
    impl<I> DynCloneIter<f32> for FadeIn<I> where I: StaticSource {}
    impl<I> AsGenericSource for FadeIn<I> where I: StaticSource {}
    impl<I> StaticSource for FadeIn<I> where I: StaticSource {}
    impl<I1, I2> DynCloneIter<f32> for Mix<I1, I2>
    where
        I1: StaticSource,
        I2: StaticSource,
    {
    }
    impl<I1, I2> AsGenericSource for Mix<I1, I2>
    where
        I1: StaticSource,
        I2: StaticSource,
    {
    }
    impl<I1, I2> StaticSource for Mix<I1, I2>
    where
        I1: StaticSource,
        I2: StaticSource,
    {
    }
    impl<I> DynCloneIter<f32> for Repeat<I> where I: StaticSource {}
    impl<I> AsGenericSource for Repeat<I> where I: StaticSource {}
    impl<I> StaticSource for Repeat<I> where I: StaticSource {}
    impl<I> DynCloneIter<f32> for SkipDuration<I> where I: StaticSource {}
    impl<I> AsGenericSource for SkipDuration<I> where I: StaticSource {}
    impl<I> StaticSource for SkipDuration<I> where I: StaticSource {}
    impl<I> DynCloneIter<f32> for Spatial<I> where I: StaticSource {}
    impl<I> AsGenericSource for Spatial<I> where I: StaticSource {}
    impl<I> StaticSource for Spatial<I> where I: StaticSource {}
    impl<I> DynCloneIter<f32> for Speed<I> where I: StaticSource {}
    impl<I> AsGenericSource for Speed<I> where I: StaticSource {}
    impl<I> StaticSource for Speed<I> where I: StaticSource {}
    impl StaticSource for GenericSource<f32> {}
    impl<I> DynCloneIter<f32> for TakeDuration<I> where I: StaticSource {}
    impl<I> AsGenericSource for TakeDuration<I> where I: StaticSource {}
    impl<I> StaticSource for TakeDuration<I> where I: StaticSource {}
}
