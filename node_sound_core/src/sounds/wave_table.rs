use std::time::Duration;

use rodio::Source;

use crate::sound_map::SetSpeed;

#[derive(Clone)]
pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    speed: f32,
    uses_speed: bool,
    freq: f32,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>, uses_speed: bool) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
            speed: 1.0,
            uses_speed,
            freq: 1.0,
        };
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.freq = frequency;
        self.index_increment =
            frequency * self.wave_table.len() as f32 / self.sample_rate as f32 / self.speed;
    }

    fn get_sample(&mut self) -> Option<f32> {
        self.index += self.index_increment;
        if self.index > self.wave_table.len() as f32 {
            return None;
        }
        return Some(self.lerp());
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }
}

impl Source for WavetableOscillator {
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

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return self.get_sample();
    }
}

impl SetSpeed for WavetableOscillator {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
        self.set_frequency(self.freq);
    }
}
