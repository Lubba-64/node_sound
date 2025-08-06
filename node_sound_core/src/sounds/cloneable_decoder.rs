use crate::sound_map::SetSpeed;
use rodio::Source;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AudioSampleDatabase {
    samples: HashMap<String, (Rc<[f32]>, u16, u32)>,
}

impl AudioSampleDatabase {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            samples: HashMap::new(),
        }))
    }

    pub fn cleanup_unused_samples(&mut self, active_samples: &HashSet<String>) {
        self.samples.retain(|key, _| active_samples.contains(key));
    }

    pub fn add_sample(&mut self, name: String, data: Vec<u8>) {
        let (samples, channels, sample_rate) = decode_samples(data);
        self.samples
            .insert(name, (samples.into(), channels, sample_rate));
    }
}

fn decode_samples(data: Vec<u8>) -> (Vec<f32>, u16, u32) {
    use rodio::{Decoder, Source};
    use std::io::Cursor;

    let decoder = Decoder::new(Cursor::new(data)).expect("Expect valid audio data");
    let channels = decoder.channels();
    let sample_rate = decoder.sample_rate();
    let samples: Vec<f32> = decoder.collect();
    (samples, channels, sample_rate)
}

pub struct CloneableDecoder {
    samples: Rc<[f32]>,
    index: usize,
    channels: u16,
    sample_rate: u32,
    uses_speed: bool,
    speed: f32,
}

impl CloneableDecoder {
    pub fn new(
        database: &AudioSampleDatabase,
        sample_key: String,
        uses_speed: bool,
    ) -> Option<Self> {
        let (samples, channels, sample_rate) = {
            let (samples, channels, sample_rate) = database.samples.get(&sample_key)?;
            (Rc::clone(samples), *channels, *sample_rate)
        };

        Some(Self {
            samples,
            index: 0,
            channels,
            sample_rate,
            speed: 1.0,
            uses_speed,
        })
    }
}

impl Clone for CloneableDecoder {
    fn clone(&self) -> Self {
        Self {
            samples: Rc::clone(&self.samples),
            index: 0, // Reset index for new iterator
            channels: self.channels,
            sample_rate: self.sample_rate,
            speed: self.speed,
            uses_speed: self.uses_speed,
        }
    }
}

impl Iterator for CloneableDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.samples.len() {
            return None;
        }

        let sample = self.samples[self.index];
        self.index += 1;

        if self.uses_speed && self.speed != 1.0 {
            let skip = (1.0 / self.speed).max(0.1);
            self.index = (self.index as f32 * skip) as usize;
        }

        Some(sample)
    }
}

impl Source for CloneableDecoder {
    fn current_span_len(&self) -> Option<usize> {
        Some(self.samples.len().saturating_sub(self.index))
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        (self.sample_rate as f32 * self.speed) as u32
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        let samples_per_channel = self.samples.len() / self.channels as usize;
        let duration_secs = samples_per_channel as f32 / self.sample_rate as f32;
        Some(std::time::Duration::from_secs_f32(duration_secs))
    }
}

impl SetSpeed for CloneableDecoder {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
    }
}
