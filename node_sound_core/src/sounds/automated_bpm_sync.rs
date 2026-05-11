use std::sync::{Arc, Mutex};

use crate::{node::SoundNode, sound_graph::note::NoteSpeedType};

#[derive(Clone, Debug)]
pub struct AutomatedBPMSync<S: SoundNode> {
    sample_rate: f32,
    table: Vec<f32>,
    speed: f32,
    note_speed_type: NoteSpeedType,
    note_speed: S,
    bpm: Arc<Mutex<f32>>,
}

impl<S: SoundNode> AutomatedBPMSync<S> {
    #[inline]
    pub fn new(
        sample_rate: f32,
        bpm: Arc<Mutex<f32>>,
        note_speed: S,
        note_speed_type: NoteSpeedType,
        table: Vec<f32>,
        speed: f32,
    ) -> Self {
        Self {
            sample_rate,
            bpm,
            note_speed,
            note_speed_type,
            table,
            speed,
        }
    }
}

impl<S: SoundNode + Clone> SoundNode for AutomatedBPMSync<S> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        let seconds_per_note = self
            .note_speed_type
            .get_beats_type()
            .iter()
            .nth(self.note_speed.next(index, channel).unwrap_or_default() as usize)
            .cloned()
            .unwrap_or_default()
            .get_beats()
            / (self.bpm.lock().map(|bpm| *bpm).unwrap_or(120.0) / 60.0);
        let samples_per_note = seconds_per_note * self.sample_rate;
        index /= self.speed;
        index %= samples_per_note;
        index /= samples_per_note;
        let real_idx = index * self.table.len() as f32;
        let idx = real_idx.floor() as usize;
        let initial_weight = real_idx - idx as f32;
        let first = self.table[idx] * initial_weight;
        let second = if idx + 1 >= self.table.len() {
            self.table[idx] + 0.001
        } else {
            self.table[idx + 1]
        } * (1.0 - initial_weight);
        Some(first + second)
    }
}
