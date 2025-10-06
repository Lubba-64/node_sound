use std::sync::{Arc, Mutex};

use crate::{sound_graph::note::NoteSpeed, sound_map::DawSource};

#[derive(Clone, Debug)]
pub struct BPMSyncSource {
    sample_rate: f32,
    table: Vec<f32>,
    speed: f32,
    note_speed: NoteSpeed,
    bpm: Arc<Mutex<f32>>,
}

impl BPMSyncSource {
    #[inline]
    pub fn new(
        sample_rate: f32,
        bpm: Arc<Mutex<f32>>,
        note_speed: NoteSpeed,
        table: Vec<f32>,
        speed: f32,
    ) -> Self {
        Self {
            sample_rate,
            bpm,
            note_speed,
            table,
            speed,
        }
    }
}

impl DawSource for BPMSyncSource {
    fn next(&mut self, mut index: f32, _channel: u8) -> Option<f32> {
        let seconds_per_note =
            self.note_speed.get_beats() / (self.bpm.lock().map(|x| *x).unwrap_or(120.0) / 60.0);
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
