use dyn_clone::DynClone;
use std::io::ErrorKind;

use crate::sounds::const_wave::ConstWave;

pub trait DawSource: DynClone {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32>;
    fn note_speed(&mut self, speed: f32, rate: f32);
    fn size_hint(&self) -> Option<f32>;
}

pub struct GenericSource {
    sound: Box<dyn DawSource>,
}

impl Clone for GenericSource {
    fn clone(&self) -> Self {
        Self {
            sound: dyn_clone::clone_box(&*self.sound),
        }
    }
}
unsafe impl Send for GenericSource {}

impl GenericSource {
    pub fn new(sound: Box<dyn DawSource>) -> Self {
        Self { sound: sound }
    }
}

impl DawSource for GenericSource {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.sound.next(index, channel)
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.sound.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        self.sound.size_hint()
    }
}

#[derive(Clone)]
pub struct SoundQueue {
    queue: Vec<GenericSource>,
}

impl Default for SoundQueue {
    fn default() -> Self {
        SoundQueue::new()
    }
}

impl SoundQueue {
    pub fn new() -> Self {
        let mut queue = SoundQueue { queue: vec![] };
        queue.push_sound(Box::new(ConstWave::new(0.0)));
        return queue;
    }

    pub fn clone_sound(&mut self, idx: usize) -> Result<GenericSource, Box<dyn std::error::Error>> {
        if idx >= self.queue.len() {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "Sound queue accessed an out of bounds element",
            )));
        }
        return Ok(self.queue[idx].clone());
    }

    pub fn push_sound(&mut self, sound: Box<dyn DawSource>) -> usize {
        self.queue.push(GenericSource::new(sound));
        return self.queue.len() - 1;
    }

    pub fn sound_queue_len(&mut self) -> usize {
        self.queue.len()
    }

    pub fn clear(&mut self) {
        self.queue.clear()
    }

    pub fn note_speed(&mut self, speed: f32, sample_rate: f32) {
        if self.queue.len() == 0 {
            return;
        }
        let last = self.queue.len() - 1;
        self.queue[last].sound.note_speed(speed, sample_rate);
    }
}
