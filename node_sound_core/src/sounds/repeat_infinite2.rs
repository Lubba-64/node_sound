use std::time::Duration;

use rodio::{source::Buffered, Sample, Source};

use rodio::source::SeekError;

/// Internal function that builds a `Repeat` object.
pub fn repeat<I>(input: I) -> Repeat2<I>
where
    I: Source,
    I::Item: Sample,
{
    let input = input.buffered();
    Repeat2 {
        inner: input.clone(),
        next: input,
    }
}

/// A source that repeats the given source.
pub struct Repeat2<I>
where
    I: Source,
    I::Item: Sample,
{
    inner: Buffered<I>,
    next: Buffered<I>,
}

impl<I> Iterator for Repeat2<I>
where
    I: Source,
    I::Item: Sample,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if let Some(value) = self.inner.next() {
            return Some(value);
        }

        self.inner = self.next.clone();
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // infinite
        (0, None)
    }
}

impl<I> Source for Repeat2<I>
where
    I: Iterator + Source,
    I::Item: Sample,
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        match self.inner.current_frame_len() {
            Some(0) => self.next.current_frame_len(),
            a => a,
        }
    }

    #[inline]
    fn channels(&self) -> u16 {
        match self.inner.current_frame_len() {
            Some(0) => self.next.channels(),
            _ => self.inner.channels(),
        }
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        match self.inner.current_frame_len() {
            Some(0) => self.next.sample_rate(),
            _ => self.inner.sample_rate(),
        }
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }

    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.inner.try_seek(pos)
    }
}

impl<I> Clone for Repeat2<I>
where
    I: Source,
    I::Item: Sample,
{
    #[inline]
    fn clone(&self) -> Repeat2<I> {
        Repeat2 {
            inner: self.inner.clone(),
            next: self.next.clone(),
        }
    }
}
