use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    sound_graph::note::{Note, NoteSpeed, Octave, Pitch},
    sound_map::DawSource,
};

#[derive(Clone, Debug)]
pub struct Tracker<S: DawSource> {
    sample_rate: f32,
    source: S,
    speed: f32,
    notes: Vec<TrackerNote>,
    bpm: Arc<Mutex<f32>>,
    current_note: usize,
    last_idx: f32,
    table: Vec<f32>,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct TrackerNote {
    pub speed: NoteSpeed,
    pub note: Option<Note>, // option to signify silence.
}

impl TrackerNote {
    pub fn new(speed: NoteSpeed, note: Option<Note>) -> Self {
        Self { speed, note }
    }
}

impl<S: DawSource> Tracker<S> {
    #[inline]
    pub fn new(
        sample_rate: f32,
        bpm: Arc<Mutex<f32>>,
        notes: Vec<TrackerNote>,
        source: S,
        speed: f32,
        table: Vec<f32>,
    ) -> Self {
        Self {
            sample_rate,
            bpm,
            notes,
            source,
            speed,
            current_note: 0,
            last_idx: -1.0,
            table,
        }
    }
}

impl<S: DawSource + Clone> DawSource for Tracker<S> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        let tracker_note = &self.notes[self.current_note];
        let seconds_per_note =
            tracker_note.speed.get_beats() / (self.bpm.lock().map(|x| *x).unwrap_or(120.0) / 60.0);
        let samples_per_note = seconds_per_note * self.sample_rate;
        let note = match &tracker_note.note {
            None => {
                index /= self.speed;
                index %= samples_per_note;
                index /= samples_per_note;
                if self.last_idx > index {
                    self.current_note += 1;
                    if self.current_note >= self.notes.len() {
                        self.current_note = 0;
                    }
                }
                self.last_idx = index;
                return Some(0.0);
            }
            Some(x) => x,
        };
        self.source
            .next(
                index * Pitch(Octave::O4, note.clone()).match_freq()
                    / Pitch(Octave::O4, Note::C).match_freq(),
                channel,
            )
            .map(|f| {
                index /= self.speed;
                index %= samples_per_note;
                index /= samples_per_note;
                if self.last_idx > index {
                    self.current_note += 1;
                    if self.current_note >= self.notes.len() {
                        self.current_note = 0;
                    }
                }
                self.last_idx = index;
                let real_idx = index * self.table.len() as f32;
                let idx = real_idx.floor() as usize;
                let initial_weight = real_idx - idx as f32;
                let first = self.table[idx] * initial_weight;
                let second = if idx + 1 >= self.table.len() {
                    self.table[idx] + 0.001
                } else {
                    self.table[idx + 1]
                } * (1.0 - initial_weight);
                f * (first + second)
            })
    }
}
