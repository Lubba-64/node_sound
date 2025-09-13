use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct SourceWavetableOscillator<S: DawSource> {
    pub left_table: Vec<f32>,
    pub right_table: Vec<f32>,
    pub sample_rate: f32,
    pub speed: f32,
    pub frequency: f32,
    pub base_frequency: f32,
    pub uses_speed: bool,
    pub duration_seconds: f32,
    pub source: S,
}

#[derive(Clone)]
pub struct WaveTableOscillator {
    pub left_table: Vec<f32>,
    pub right_table: Vec<f32>,
    pub sample_rate: f32,
    pub frequency: f32,
    pub base_frequency: f32,
    pub duration_seconds: f32,
    pub speed: f32,
    pub uses_speed: bool,
}

impl WaveTableOscillator {
    pub fn new_stereo(sample_rate: f32, base_frequency: f32) -> Self {
        Self {
            left_table: vec![],
            right_table: vec![],
            sample_rate: sample_rate,
            frequency: base_frequency,
            base_frequency,
            duration_seconds: 0.0,
            speed: 1.0,
            uses_speed: false,
        }
    }

    pub fn set_uses_speed(&mut self, uses_speed: bool) {
        self.uses_speed = uses_speed;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn rebuild_table(&mut self, new_sample_rate: f32, left: Vec<f32>, right: Vec<f32>) {
        self.left_table = left;
        self.right_table = right;
        self.rebuild_table_soft(new_sample_rate);
    }

    pub fn rebuild_table_soft(&mut self, new_sample_rate: f32) {
        let total_samples = self.left_table.len().max(self.right_table.len());
        self.duration_seconds = total_samples as f32 / new_sample_rate;
        self.sample_rate = new_sample_rate;
    }
}

impl DawSource for WaveTableOscillator {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        self.get_sample(index, channel)
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        if self.uses_speed {
            self.speed = speed;
        }
        self.sample_rate = rate;
        self.rebuild_table_soft(rate);
    }
    fn size_hint(&self) -> Option<f32> {
        if self.duration_seconds == 0.0 {
            return None;
        } else {
            if self.uses_speed {
                Some(self.duration_seconds * self.speed)
            } else {
                Some(self.duration_seconds / self.playback_rate())
            }
        }
    }
}

impl WaveTableTrait for WaveTableOscillator {
    fn frequency(&self) -> f32 {
        self.frequency
    }
    fn base_frequency(&self) -> f32 {
        self.base_frequency
    }
    fn duration_seconds(&self) -> f32 {
        self.duration_seconds
    }
    fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
    fn left_table(&self) -> &Vec<f32> {
        &self.left_table
    }
    fn right_table(&self) -> &Vec<f32> {
        &self.right_table
    }
}

impl<S: DawSource> SourceWavetableOscillator<S> {
    pub fn from_source(
        source: S,
        sample_rate: u32,
        duration_seconds: f32,
        base_frequency: f32,
        uses_speed: bool,
    ) -> Self {
        Self {
            left_table: vec![],
            right_table: vec![],
            sample_rate: sample_rate as f32,
            speed: 1.0,
            frequency: base_frequency,
            base_frequency,
            uses_speed,
            duration_seconds,
            source,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    fn rebuild_table(&mut self, new_sample_rate: f32) {
        let total_samples = (self.duration_seconds * new_sample_rate) as usize;
        self.left_table = Vec::with_capacity(total_samples);
        self.right_table = Vec::with_capacity(total_samples);

        for i in 0..total_samples {
            let index = i as f32;
            self.left_table
                .push(self.source.next(index, 0).unwrap_or(0.0));
            self.right_table
                .push(self.source.next(index, 1).unwrap_or(0.0));
        }

        self.sample_rate = new_sample_rate;
    }
}

impl<S: DawSource + Clone> DawSource for SourceWavetableOscillator<S> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        self.get_sample(index, channel)
    }

    fn note_speed(&mut self, speed: f32, rate: f32) {
        if self.uses_speed {
            self.speed = speed;
        }
        self.source.note_speed(speed, rate);
        self.rebuild_table(rate);
    }

    fn size_hint(&self) -> Option<f32> {
        if self.uses_speed {
            Some(self.duration_seconds * self.speed)
        } else {
            Some(self.duration_seconds / self.playback_rate())
        }
    }
}

impl<S: DawSource> WaveTableTrait for SourceWavetableOscillator<S> {
    fn frequency(&self) -> f32 {
        self.frequency
    }
    fn base_frequency(&self) -> f32 {
        self.base_frequency
    }
    fn duration_seconds(&self) -> f32 {
        self.duration_seconds
    }
    fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
    fn left_table(&self) -> &Vec<f32> {
        &self.left_table
    }
    fn right_table(&self) -> &Vec<f32> {
        &self.right_table
    }
}

pub trait WaveTableTrait {
    fn frequency(&self) -> f32;
    fn base_frequency(&self) -> f32;
    fn duration_seconds(&self) -> f32;
    fn sample_rate(&self) -> f32;
    fn left_table(&self) -> &Vec<f32>;
    fn right_table(&self) -> &Vec<f32>;
    fn playback_rate(&self) -> f32 {
        self.frequency() / self.base_frequency()
    }
    fn get_sample(&self, index: f32, channel: u8) -> Option<f32> {
        let speed_adjusted_index = index * self.playback_rate();

        if speed_adjusted_index >= self.duration_seconds() * self.sample_rate() {
            return None;
        }

        let table = match channel {
            0 => &self.left_table(),
            1 => &self.right_table(),
            _ => return None,
        };

        if table.is_empty() {
            return None;
        }

        let truncated_index = speed_adjusted_index as usize;

        if truncated_index >= table.len() - 1 {
            return Some(table[table.len() - 1]);
        }

        let next_index = truncated_index + 1;
        let next_index_weight = speed_adjusted_index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        Some(
            truncated_index_weight * table[truncated_index] + next_index_weight * table[next_index],
        )
    }
}
