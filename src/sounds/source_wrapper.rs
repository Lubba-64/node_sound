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
}
