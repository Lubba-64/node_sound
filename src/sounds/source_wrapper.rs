use std::time::Duration;

use dyn_clone::DynClone;
use rodio::{Sample, Source};

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

impl<'a, S: Sample> Iterator for GenericSource<S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        self.sound.next()
    }
}

impl<'a, S: Sample> Source for GenericSource<S> {
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
    impl DynCloneIter<u16> for std::vec::IntoIter<u16> {}
    impl DynCloneIter<i16> for std::vec::IntoIter<i16> {}
    use super::*;
    pub trait AsGenericSource<S: Sample>:
        Source + DynCloneIter<S> + Sized + Send + Clone + 'static
    {
        fn as_generic(&self, duration: Option<Duration>) -> GenericSource<S>
        where
            Self: Sized,
        {
            let channels = self.channels();
            let sample_rate = self.sample_rate();
            GenericSource::new(Box::new(self.clone()), sample_rate, channels, duration)
        }
    }
}
