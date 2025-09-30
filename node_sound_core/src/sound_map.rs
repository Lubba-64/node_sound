use dyn_clone::DynClone;
use eframe::egui::ahash::{HashMap, HashMapExt};
use itertools::Itertools;
use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    io::ErrorKind,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::sounds::const_wave::ConstWave;

pub trait DawSource: DynClone + Debug {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32>;
    fn size_hint(&self) -> Option<f32>;
}

#[derive(Debug)]
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
    fn size_hint(&self) -> Option<f32> {
        self.sound.size_hint()
    }
}

#[derive(Debug)]
pub struct RefSource {
    sound: Rc<RefCell<dyn DawSource>>,
    val: Rc<RefCell<HashMap<usize, Option<f32>>>>,
    size: Rc<Cell<usize>>,
    count: Rc<Cell<usize>>,
    id: usize,
}

impl Clone for RefSource {
    fn clone(&self) -> Self {
        self.size.set(self.size.get() + 1);
        Self {
            sound: self.sound.clone(),
            val: self.val.clone(),
            size: self.size.clone(),
            count: self.count.clone(),
            id: self.id + 1,
        }
    }
}

unsafe impl Send for RefSource {}

impl RefSource {
    pub fn new(sound: Rc<RefCell<dyn DawSource>>) -> Self {
        Self {
            sound: sound,
            size: Rc::new(Cell::new(1)),
            val: Rc::new(RefCell::new(HashMap::new())),
            count: Rc::new(Cell::new(0)),
            id: 0,
        }
    }
}

impl DawSource for RefSource {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let count = self.count.get();
        let size = self.size.get();
        let mut z = self.val.borrow_mut();
        if count + 1 >= size {
            z.clear();
            self.count.set(0);
        } else {
            self.count.set(count + 1);
        }
        if !z.contains_key(&(index as usize)) {
            z.insert(index as usize, self.sound.borrow_mut().next(index, channel));
        }
        z[&(index as usize)]
    }
    fn size_hint(&self) -> Option<f32> {
        None
    }
}

pub struct SoundQueue {
    queue: Vec<GenericSource>,
    sample_rate: f32,
    speed: f32,
    bpm: Arc<Mutex<f32>>,
}

impl Default for SoundQueue {
    fn default() -> Self {
        SoundQueue::new(48000.0)
    }
}

impl SoundQueue {
    pub fn new(sample_rate: f32) -> Self {
        let mut queue = SoundQueue {
            queue: vec![],
            speed: 1.0,
            sample_rate: sample_rate,
            bpm: Arc::new(Mutex::new(120.0)),
        };
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

    pub fn arc_clone_sound(&mut self, idx: usize) -> Result<RefSource, Box<dyn std::error::Error>> {
        if idx >= self.queue.len() {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "Sound queue accessed an out of bounds element",
            )));
        }
        return Ok(RefSource::new(Rc::new(RefCell::new(
            self.queue[idx].clone(),
        ))));
    }

    pub fn push_sound(&mut self, sound: Box<dyn DawSource>) -> usize {
        self.queue.push(GenericSource::new(sound));
        return self.queue.len() - 1;
    }

    pub fn sound_queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
        self.push_sound(Box::new(ConstWave::new(0.0)));
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn set_note_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn get_note_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_bpm(&self) -> Arc<Mutex<f32>> {
        self.bpm.clone()
    }
    pub fn set_bpm(&mut self, bpm: Arc<Mutex<f32>>) {
        self.bpm = bpm
    }
}
