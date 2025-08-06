use std::time::Duration;

use rodio::{Source, source::UniformSourceIterator};

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};

#[derive(Clone)]
pub struct AutomatedWavetableOscillator<I: Source<Item = f32>> {
    sample_rate: u32,
    wave_table: Vec<f32>,
    freq: UniformSourceIterator<I>,
    index: f32,
    uses_speed: bool,
    speed: f32,
}

impl<I: Source<Item = f32>> AutomatedWavetableOscillator<I> {
    pub fn new(
        sample_rate: u32,
        wave_table: Vec<f32>,
        freq: I,
        uses_speed: bool,
    ) -> AutomatedWavetableOscillator<I> {
        AutomatedWavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            freq: UniformSourceIterator::new(freq, 1, DEFAULT_SAMPLE_RATE),
            speed: 1.0,
            uses_speed,
        }
    }

    fn get_sample(&mut self) -> Option<f32> {
        self.freq.next().map(|freq_value| {
            self.index +=
                freq_value * self.wave_table.len() as f32 / self.sample_rate as f32 / self.speed;
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

    fn current_span_len(&self) -> Option<usize> {
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

impl<I: Source<Item = f32>> SetSpeed for AutomatedWavetableOscillator<I> {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
    }
}
