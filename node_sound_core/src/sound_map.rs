use crate::sounds::const_wave::ConstWave;
use dyn_clone::DynClone;
use eframe::egui::ahash::{HashMap, HashMapExt};
use ordered_float::OrderedFloat;
use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    io::ErrorKind,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub trait Oscillator: DynClone + Debug {
    fn set_phase(&mut self, phase: f32);
    fn get_phase(&self) -> f32;
    fn get_frequency(&self) -> f32;
    fn set_frequency(&mut self, freq: f32);
    fn calculate_output(&self) -> f32;
}

#[derive(Debug)]
pub struct GenericOsc {
    osc: Box<dyn Oscillator>,
}

impl Clone for GenericOsc {
    fn clone(&self) -> Self {
        Self {
            osc: dyn_clone::clone_box(&*self.osc),
        }
    }
}

impl GenericOsc {
    pub fn new(osc: Box<dyn Oscillator>) -> Self {
        Self { osc }
    }
}

impl Oscillator for GenericOsc {
    fn calculate_output(&self) -> f32 {
        self.osc.calculate_output()
    }
    fn get_frequency(&self) -> f32 {
        self.osc.get_frequency()
    }
    fn get_phase(&self) -> f32 {
        self.osc.get_phase()
    }
    fn set_frequency(&mut self, freq: f32) {
        self.osc.set_frequency(freq);
    }
    fn set_phase(&mut self, phase: f32) {
        self.osc.set_phase(phase);
    }
}

pub trait DawSource: DynClone + Debug {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32>;
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
unsafe impl Sync for GenericSource {}

impl GenericSource {
    pub fn new(sound: Box<dyn DawSource>) -> Self {
        Self { sound: sound }
    }
}

impl DawSource for GenericSource {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.sound.next(index, channel)
    }
}

#[derive(Debug)]
pub struct RefSource {
    sound: Rc<RefCell<dyn DawSource>>,
    val: Rc<RefCell<HashMap<OrderedFloat<f32>, Option<f32>>>>,
    size: Rc<Cell<usize>>,
    count: Rc<Cell<usize>>,
    last_index: f32,
}

impl Clone for RefSource {
    fn clone(&self) -> Self {
        Self {
            sound: self.sound.clone(),
            val: self.val.clone(),
            size: self.size.clone(),
            count: self.count.clone(),
            last_index: self.last_index,
        }
    }
}

unsafe impl Send for RefSource {}

impl RefSource {
    pub fn new(sound: Rc<RefCell<dyn DawSource>>) -> Self {
        Self {
            sound: sound,
            size: Rc::new(Cell::new(0)),
            val: Rc::new(RefCell::new(HashMap::new())),
            count: Rc::new(Cell::new(0)),
            last_index: 0.0,
        }
    }
}

impl DawSource for RefSource {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let mut val = self.val.borrow_mut();
        if self.last_index > index {
            val.clear();
        }
        self.last_index = index;
        if !val.contains_key(&OrderedFloat(index)) {
            val.insert(
                OrderedFloat(index),
                self.sound.borrow_mut().next(index, channel),
            );
        }
        val[&OrderedFloat(index)]
    }
}

pub struct SoundQueue {
    osc_queue: Vec<GenericOsc>,
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
            osc_queue: vec![],
            queue: vec![],
            speed: 1.0,
            sample_rate: sample_rate,
            bpm: Arc::new(Mutex::new(120.0)),
        };
        queue.push_sound(Box::new(ConstWave::new(0.0)));
        queue.push_osc(Box::new(ConstWave::new(0.0)));
        return queue;
    }

    pub fn clone_osc(&mut self, idx: usize) -> Result<GenericOsc, Box<dyn std::error::Error>> {
        if idx >= self.osc_queue.len() {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "Sound queue accessed an out of bounds element",
            )));
        }
        return Ok(self.osc_queue[idx].clone());
    }

    pub fn push_osc(&mut self, osc: Box<dyn Oscillator>) -> usize {
        self.osc_queue.push(GenericOsc::new(osc));
        return self.osc_queue.len() - 1;
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
        self.osc_queue.clear();
        self.push_sound(Box::new(ConstWave::new(0.0)));
        self.push_osc(Box::new(ConstWave::new(0.0)));
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
