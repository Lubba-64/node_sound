use dyn_clone::DynClone;
use std::{
    io::ErrorKind,
    sync::{Arc, Mutex},
};

use crate::sounds::const_wave::ConstWave;

pub trait RefClone: Sized + Clone {
    fn clone_soft(&self) -> Self {
        self.clone()
    }
}

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

pub struct RefSource {
    sound: Arc<Mutex<Box<dyn DawSource>>>,
    val: Arc<Mutex<Option<f32>>>,
    size: Arc<Mutex<usize>>,
    count: Arc<Mutex<usize>>,
    id: usize,
}

impl Clone for RefSource {
    fn clone(&self) -> Self {
        *self.size.lock().expect("expected refsource lock for size") += 1;
        Self {
            sound: Arc::new(Mutex::new(dyn_clone::clone_box(
                &**self.sound.lock().expect("expected ok"),
            ))),
            size: Arc::new(Mutex::new(1)),
            val: Arc::new(Mutex::new(Some(0.0))),
            count: Arc::new(Mutex::new(0)),
            id: self.id + 1,
        }
    }
}

unsafe impl Send for RefSource {}

impl RefSource {
    pub fn new(sound: Arc<Mutex<Box<dyn DawSource>>>) -> Self {
        Self {
            sound: sound,
            size: Arc::new(Mutex::new(1)),
            val: Arc::new(Mutex::new(Some(0.0))),
            count: Arc::new(Mutex::new(0)),
            id: 0,
        }
    }

    pub fn soft_clone(&self) -> Self {
        *self.size.lock().expect("expected refsource lock for size") += 1;
        Self {
            sound: self.sound.clone(),
            val: self.val.clone(),
            size: self.size.clone(),
            count: self.count.clone(),
            id: self.id + 1,
        }
    }
}

impl RefClone for RefSource {
    fn clone_soft(&self) -> Self {
        self.soft_clone()
    }
}

impl DawSource for RefSource {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let mut count = self.count.lock().expect("expected lock on refsource");
        let size = self.size.lock().expect("expected size lock");
        *count += 1;
        if *count >= *size {
            *count = 0;
            let val = self
                .sound
                .lock()
                .expect("source id is zero, should never fail")
                .next(index, channel);
            *self.val.lock().expect("expected lock for refsource") = val;
            return val;
        }
        *self.val.lock().expect("expected lock for refsource")
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        if self.id == 0 {
            self.sound
                .lock()
                .expect("source id is zero, should never fail")
                .note_speed(speed, rate);
        }
    }
    fn size_hint(&self) -> Option<f32> {
        None
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
        return Ok(self.queue[idx].clone_soft());
    }

    pub fn arc_clone_sound(&mut self, idx: usize) -> Result<RefSource, Box<dyn std::error::Error>> {
        if idx >= self.queue.len() {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "Sound queue accessed an out of bounds element",
            )));
        }
        return Ok(RefSource::new(Arc::new(Mutex::new(Box::new(
            self.queue[idx].clone(),
        )))));
    }

    pub fn push_sound(&mut self, sound: Box<dyn DawSource>) -> usize {
        self.queue.push(GenericSource::new(sound));
        return self.queue.len() - 1;
    }

    pub fn sound_queue_len(&mut self) -> usize {
        self.queue.len()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
        self.push_sound(Box::new(ConstWave::new(0.0)));
    }

    pub fn note_speed(&mut self, speed: f32, sample_rate: f32) {
        if self.queue.len() == 0 {
            return;
        }
        let last = self.queue.len() - 1;
        self.queue[last].sound.note_speed(speed, sample_rate);
    }
}
