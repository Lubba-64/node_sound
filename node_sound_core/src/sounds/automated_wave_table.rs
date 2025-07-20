use std::time::Duration;

use rodio::{source::UniformSourceIterator, Source};

use crate::constants::DEFAULT_SAMPLE_RATE;

#[derive(Clone)]
pub struct AutomatedWavetableOscillator<I: Source<Item = f32>> {
    sample_rate: u32,
    wave_table: Vec<f32>,
    freq: UniformSourceIterator<I, f32>,
    index: f32,
}

impl<I: Source<Item = f32>> AutomatedWavetableOscillator<I> {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>, freq: I) -> AutomatedWavetableOscillator<I> {
        AutomatedWavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            freq: UniformSourceIterator::new(freq, 1, DEFAULT_SAMPLE_RATE),
        }
    }

    fn get_sample(&mut self) -> Option<f32> {
        self.freq.next().map(|freq_value| {
            self.index += freq_value * self.wave_table.len() as f32 / self.sample_rate as f32;
            self.index %= self.wave_table.len() as f32;
            self.lerp()
        })
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}

impl<I: Source<Item = f32>> Source for AutomatedWavetableOscillator<I> {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

impl<I: Source<Item = f32>> Iterator for AutomatedWavetableOscillator<I> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return self.get_sample();
    }
}
