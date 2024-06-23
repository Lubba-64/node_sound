use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct AutomatedTranslateWave<
    I: Source<Item = f32>,
    I2: Source<Item = f32>,
    I3: Source<Item = f32>,
    I4: Source<Item = f32>,
    I5: Source<Item = f32>,
> {
    source: UniformSourceIterator<I, I::Item>,
    start_min: UniformSourceIterator<I2, I::Item>,
    start_max: UniformSourceIterator<I3, I::Item>,
    end_min: UniformSourceIterator<I4, I::Item>,
    end_max: UniformSourceIterator<I5, I::Item>,
}

impl<
        I: Source<Item = f32>,
        I2: Source<Item = f32>,
        I3: Source<Item = f32>,
        I4: Source<Item = f32>,
        I5: Source<Item = f32>,
    > AutomatedTranslateWave<I, I2, I3, I4, I5>
{
    #[inline]
    pub fn new(source: I, start_min: I2, start_max: I3, end_min: I4, end_max: I5) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            start_min: UniformSourceIterator::new(start_min, 2, DEFAULT_SAMPLE_RATE),
            start_max: UniformSourceIterator::new(start_max, 2, DEFAULT_SAMPLE_RATE),
            end_min: UniformSourceIterator::new(end_min, 2, DEFAULT_SAMPLE_RATE),
            end_max: UniformSourceIterator::new(end_max, 2, DEFAULT_SAMPLE_RATE),
        }
    }
}

impl<
        I: Source<Item = f32>,
        I2: Source<Item = f32>,
        I3: Source<Item = f32>,
        I4: Source<Item = f32>,
        I5: Source<Item = f32>,
    > Iterator for AutomatedTranslateWave<I, I2, I3, I4, I5>
{
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let p = self.source.next().unwrap_or(0.0);
        let mut start_min = self.start_min.next().unwrap_or(0.0);
        let mut start_max = self.start_max.next().unwrap_or(0.0);
        let mut end_min = self.end_min.next().unwrap_or(0.0);
        let mut end_max = self.end_max.next().unwrap_or(0.0);
        if start_min > start_max {
            let other = start_min;
            start_min = start_max;
            start_max = other;
        }
        if end_min > end_max {
            let other = end_min;
            end_min = end_max;
            end_max = other;
        }
        return Some(
            end_min
                + ((end_max - end_min) / (start_max - start_min))
                    * (p.clamp(start_min, start_max) - start_min),
        );
    }
}

impl<
        I: Source<Item = f32>,
        I2: Source<Item = f32>,
        I3: Source<Item = f32>,
        I4: Source<Item = f32>,
        I5: Source<Item = f32>,
    > Source for AutomatedTranslateWave<I, I2, I3, I4, I5>
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        2
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
