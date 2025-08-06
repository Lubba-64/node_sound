use dyn_clone::DynClone;
use rodio::source::{Amplify, Delay, Mix, Repeat, SkipDuration, Source, Speed, TakeDuration, Zero};
use std::io::ErrorKind;

use crate::constants::DEFAULT_SAMPLE_RATE;

pub trait SourceIter: Source + Iterator<Item = f32> + 'static {}
pub trait SourceIterDynClone: DynClone + SourceIter<Item = f32> + SetSpeed {}
impl<I> SourceIter for I where I: Iterator<Item = f32> + Source + 'static {}
impl<I> SourceIterDynClone for I where I: SourceIter + Clone + SetSpeed {}
pub trait SetSpeed: Source + Iterator<Item = f32> {
    fn set_speed(&mut self, _speed: f32) {}
}
impl SetSpeed for Zero {}
impl<I: Iterator<Item = f32> + Source> SetSpeed for Speed<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed for Delay<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed for SkipDuration<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed for Repeat<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed for TakeDuration<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed for Amplify<I> {}
impl<I: Iterator<Item = f32> + Source, I2: Iterator<Item = f32> + Source> SetSpeed for Mix<I, I2> {}

pub struct GenericSource {
    sound: Box<dyn SourceIterDynClone>,
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
    pub fn new(sound: Box<dyn SourceIterDynClone>) -> Self {
        Self { sound: sound }
    }
}

impl Iterator for GenericSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.sound.next()
    }
}

impl Source for GenericSource {
    fn current_span_len(&self) -> Option<usize> {
        self.sound.current_span_len()
    }

    fn channels(&self) -> u16 {
        self.sound.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.sound.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.sound.total_duration()
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
        queue.push_sound(Box::new(Zero::new(1, DEFAULT_SAMPLE_RATE)));
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

    pub fn push_sound(&mut self, sound: Box<dyn SourceIterDynClone>) -> usize {
        self.queue.push(GenericSource::new(sound));
        return self.queue.len() - 1;
    }

    pub fn sound_queue_len(&mut self) -> usize {
        self.queue.len()
    }

    pub fn clear(&mut self) {
        self.queue.clear()
    }

    pub fn set_speed(&mut self, speed: f32) {
        for n in 0..self.queue.len() {
            self.queue[n].sound.set_speed(speed);
        }
    }
}
