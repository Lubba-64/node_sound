use core::time::Duration;
use rodio::source::Source;

#[derive(Clone, Debug)]
pub struct DefaultSource {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    duration: Option<Duration>,
}

impl DefaultSource {
    pub fn new(
        sample_rate: u32,
        wave_table: Vec<f32>,
        duration: Option<Duration>,
    ) -> DefaultSource {
        DefaultSource {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
            duration: duration,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    pub fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }

    pub fn reset(&mut self) {
        self.index = 0.0;
        self.index_increment = 0.0;
    }
}

impl Source for DefaultSource {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        self.duration
    }
}

impl Iterator for DefaultSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}
