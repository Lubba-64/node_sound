use std::sync::Arc;

use eframe::egui::ahash::HashMap;
use serde::{Deserialize, Serialize};

use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct WaveTableOscillator {
    pub left_table: Arc<Vec<f32>>,
    pub right_table: Arc<Vec<f32>>,
    pub sample_rate: f32,
    pub frequency: f32,
    pub base_frequency: f32,
    pub duration_seconds: f32,
    pub speed: f32,
    pub uses_speed: bool,
}

impl WaveTableOscillator {
    pub fn new_stereo(
        sample_rate: f32,
        base_frequency: f32,
        left: Arc<Vec<f32>>,
        right: Arc<Vec<f32>>,
        frequency: f32,
        uses_speed: bool,
        speed: f32,
    ) -> Self {
        let total_samples = left.len().max(right.len());
        let duration_seconds = total_samples as f32 / sample_rate;
        Self {
            left_table: left,
            right_table: right,
            sample_rate: sample_rate,
            frequency,
            base_frequency,
            duration_seconds,
            speed: if uses_speed { speed } else { 1.0 },
            uses_speed: uses_speed,
        }
    }

    pub fn playback_rate(&self) -> f32 {
        self.frequency / self.base_frequency
    }

    pub fn get_sample(&mut self, index: f32, channel: u8) -> Option<f32> {
        let speed_adjusted_index = index * self.playback_rate();

        if speed_adjusted_index >= self.duration_seconds * self.sample_rate {
            return None;
        }

        let table = match channel {
            0 => &self.left_table,
            1 => &self.right_table,
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

impl DawSource for WaveTableOscillator {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        self.get_sample(index, channel)
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

#[derive(Default, Serialize, Deserialize)]
pub struct WaveTableManager {
    table: HashMap<usize, (Arc<Vec<f32>>, Arc<Vec<f32>>)>,
    id: usize,
}

impl WaveTableManager {
    pub fn set_current_id(&mut self, id: usize) {
        self.id = id
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }

    pub fn make_wavetable_generic<S: DawSource>(
        &mut self,
        sample_rate: f32,
        base_frequency: f32,
        source: S,
        duration: f32,
        frequency: f32,
        uses_speed: bool,
        speed: f32,
    ) -> WaveTableOscillator {
        self.make_wavetable(
            sample_rate,
            base_frequency,
            source,
            duration,
            frequency,
            uses_speed,
            speed,
            Box::new(|source, total_samples| {
                let mut left = Vec::with_capacity(total_samples);
                let mut right = Vec::with_capacity(total_samples);
                for i in 0..total_samples {
                    let index = i as f32;
                    left.push(source.next(index, 0).unwrap_or(0.0));
                    right.push(source.next(index, 1).unwrap_or(0.0));
                }
                (left, right)
            }),
        )
    }

    pub fn make_wavetable<'a, S: DawSource>(
        &mut self,
        sample_rate: f32,
        base_frequency: f32,
        mut source: S,
        duration: f32,
        frequency: f32,
        uses_speed: bool,
        speed: f32,
        operator: Box<dyn Fn(&mut S, usize) -> (Vec<f32>, Vec<f32>) + 'a>,
    ) -> WaveTableOscillator {
        if !self.table.contains_key(&self.id) {
            let total_samples = (duration * sample_rate) as usize;
            let (left, right) = operator(&mut source, total_samples);
            self.table
                .insert(self.id, (Arc::new(left), Arc::new(right)));
        }
        WaveTableOscillator::new_stereo(
            sample_rate,
            base_frequency,
            self.table[&self.id].0.clone(),
            self.table[&self.id].1.clone(),
            frequency,
            uses_speed,
            speed,
        )
    }

    pub fn make_automated_wavetable_generic<'a, S: DawSource, F: DawSource>(
        &mut self,
        sample_rate: f32,
        base_frequency: f32,
        source: S,
        duration: f32,
        frequency: F,
        uses_speed: bool,
        speed: f32,
    ) -> AutomatedWaveTableOscillator<F> {
        self.make_automated_wavetable(
            sample_rate,
            base_frequency,
            source,
            duration,
            frequency,
            uses_speed,
            speed,
            Box::new(|source, total_samples| {
                let mut left = Vec::with_capacity(total_samples);
                let mut right = Vec::with_capacity(total_samples);
                for i in 0..total_samples {
                    let index = i as f32;
                    left.push(source.next(index, 0).unwrap_or(0.0));
                    right.push(source.next(index, 1).unwrap_or(0.0));
                }
                (left, right)
            }),
        )
    }

    pub fn make_automated_wavetable<'a, S: DawSource, F: DawSource>(
        &mut self,
        sample_rate: f32,
        base_frequency: f32,
        mut source: S,
        duration: f32,
        frequency: F,
        uses_speed: bool,
        speed: f32,
        operator: Box<dyn Fn(&mut S, usize) -> (Vec<f32>, Vec<f32>) + 'a>,
    ) -> AutomatedWaveTableOscillator<F> {
        if !self.table.contains_key(&self.id) {
            let total_samples = (duration * sample_rate) as usize;
            let (left, right) = operator(&mut source, total_samples);
            self.table
                .insert(self.id, (Arc::new(left), Arc::new(right)));
        }
        AutomatedWaveTableOscillator::new_stereo(
            sample_rate,
            base_frequency,
            self.table[&self.id].0.clone(),
            self.table[&self.id].1.clone(),
            frequency,
            uses_speed,
            speed,
        )
    }

    pub fn make_automated_wavetable_samples<'a, F: DawSource>(
        &mut self,
        sample_rate: f32,
        base_frequency: f32,
        frequency: F,
        uses_speed: bool,
        speed: f32,
        operator: Box<dyn Fn() -> (Vec<f32>, Vec<f32>) + 'a>,
    ) -> AutomatedWaveTableOscillator<F> {
        if !self.table.contains_key(&self.id) {
            let (left, right) = operator();
            self.table
                .insert(self.id, (Arc::new(left), Arc::new(right)));
        }
        AutomatedWaveTableOscillator::new_stereo(
            sample_rate,
            base_frequency,
            self.table[&self.id].0.clone(),
            self.table[&self.id].1.clone(),
            frequency,
            uses_speed,
            speed,
        )
    }

    pub fn make_wavetable_samples<'a>(
        &mut self,
        sample_rate: f32,
        base_frequency: f32,
        frequency: f32,
        uses_speed: bool,
        speed: f32,
        operator: Box<dyn Fn() -> (Vec<f32>, Vec<f32>) + 'a>,
    ) -> WaveTableOscillator {
        if !self.table.contains_key(&self.id) {
            let (left, right) = operator();
            self.table
                .insert(self.id, (Arc::new(left), Arc::new(right)));
        }
        WaveTableOscillator::new_stereo(
            sample_rate,
            base_frequency,
            self.table[&self.id].0.clone(),
            self.table[&self.id].1.clone(),
            frequency,
            uses_speed,
            speed,
        )
    }
}

// NEW NEW NEW
// NEW NEW NEW
// NEW NEW NEW
// NEW NEW NEW
// NEW NEW NEW
// NEW NEW NEW
// NEW NEW NEW
// NEW NEW NEW

#[derive(Clone)]
pub struct AutomatedWaveTableOscillator<F: DawSource> {
    pub left_table: Arc<Vec<f32>>,
    pub right_table: Arc<Vec<f32>>,
    pub sample_rate: f32,
    pub frequency: F,
    pub base_frequency: f32,
    pub duration_seconds: f32,
    pub speed: f32,
    pub uses_speed: bool,
}

impl<F: DawSource> AutomatedWaveTableOscillator<F> {
    pub fn new_stereo(
        sample_rate: f32,
        base_frequency: f32,
        left: Arc<Vec<f32>>,
        right: Arc<Vec<f32>>,
        frequency: F,
        uses_speed: bool,
        speed: f32,
    ) -> Self {
        let total_samples = left.len().max(right.len());
        let duration_seconds = total_samples as f32 / sample_rate;
        Self {
            left_table: left,
            right_table: right,
            sample_rate: sample_rate,
            frequency,
            base_frequency,
            duration_seconds,
            speed: if uses_speed { speed } else { 1.0 },
            uses_speed: uses_speed,
        }
    }

    fn playback_rate(&mut self, index: f32, channel: u8) -> f32 {
        self.frequency.next(index, channel).unwrap_or(0.0) / self.base_frequency
    }

    fn get_sample(&mut self, index: f32, channel: u8) -> Option<f32> {
        let speed_adjusted_index = index * self.playback_rate(index, channel);

        if speed_adjusted_index >= self.duration_seconds * self.sample_rate {
            return None;
        }

        let table = match channel {
            0 => &self.left_table,
            1 => &self.right_table,
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

impl<F: DawSource + Clone> DawSource for AutomatedWaveTableOscillator<F> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        self.get_sample(index, channel)
    }
    fn size_hint(&self) -> Option<f32> {
        None
    }
}
