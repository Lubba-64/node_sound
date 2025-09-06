use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct WavetableOscillator {
    left_table: Vec<f32>,
    right_table: Vec<f32>,
    sample_rate: f32,
    speed: f32,
    frequency: f32,
    base_frequency: f32,
    uses_speed: bool,
    duration_samples: f32,
}

impl WavetableOscillator {
    pub fn from_source<S: DawSource>(
        source: &mut S,
        sample_rate: u32,
        duration_seconds: f32,
        base_frequency: f32,
        uses_speed: bool,
    ) -> Self {
        let total_samples = (duration_seconds * sample_rate as f32) as usize;
        let mut left_table = Vec::with_capacity(total_samples);
        let mut right_table = Vec::with_capacity(total_samples);

        for i in 0..total_samples {
            let index = i as f32;
            left_table.push(source.next(index, 0).unwrap_or(0.0));
            right_table.push(source.next(index, 1).unwrap_or(0.0));
        }

        Self {
            left_table,
            right_table,
            sample_rate: sample_rate as f32,
            speed: 1.0,
            frequency: base_frequency,
            base_frequency,
            uses_speed,
            duration_samples: total_samples as f32,
        }
    }

    pub fn new_stereo(
        left_table: Vec<f32>,
        right_table: Vec<f32>,
        sample_rate: u32,
        base_frequency: f32,
        uses_speed: bool,
    ) -> Self {
        let duration_samples = left_table.len().max(right_table.len()) as f32;
        Self {
            left_table,
            right_table,
            sample_rate: sample_rate as f32,
            speed: 1.0,
            frequency: base_frequency,
            base_frequency,
            uses_speed,
            duration_samples,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn duration_seconds(&self) -> f32 {
        self.duration_samples / self.sample_rate
    }

    fn get_sample(&self, index: f32, channel: u8) -> f32 {
        let playback_rate = (self.frequency / self.base_frequency) * self.speed;
        let speed_adjusted_index = index * playback_rate;
        let wrapped_index = speed_adjusted_index % self.duration_samples;

        let table = match channel {
            0 => &self.left_table,
            1 => &self.right_table,
            _ => return 0.0,
        };

        if table.is_empty() {
            return 0.0;
        }

        let truncated_index = wrapped_index as usize % table.len();
        let next_index = (truncated_index + 1) % table.len();

        let next_index_weight = wrapped_index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * table[truncated_index] + next_index_weight * table[next_index]
    }
}

impl DawSource for WavetableOscillator {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        Some(self.get_sample(index, channel))
    }

    fn note_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }
}
