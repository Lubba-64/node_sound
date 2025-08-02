use dyn_clone::DynClone;
use rodio::source::{Amplify, Delay, Mix, Repeat, SkipDuration, Source, Speed, TakeDuration, Zero};
use rodio::{Decoder, Sample};
use std::io::ErrorKind;

use crate::constants::DEFAULT_SAMPLE_RATE;

pub trait SourceIter<Item: Sample>: Source<Item = Item> + Iterator<Item = Item> + 'static {}
pub trait SourceIterDynClone<Item: Sample>: DynClone + SourceIter<Item> + SetSpeed<Item> {}
impl<I> SourceIter<f32> for I where I: Iterator<Item = f32> + Source + 'static {}
impl<T: std::io::Read + std::io::Seek + 'static> SourceIter<i16> for Decoder<T> {}
impl<I> SourceIterDynClone<f32> for I where I: SourceIter<f32> + Clone + SetSpeed<f32> {}
pub trait SetSpeed<Item: Sample>: Source<Item = Item> + Iterator<Item = Item> {
    fn set_speed(&mut self, _speed: f32) {}
}
impl SetSpeed<f32> for Zero<f32> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed<f32> for Speed<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed<f32> for Delay<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed<f32> for SkipDuration<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed<f32> for Repeat<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed<f32> for TakeDuration<I> {}
impl<I: Iterator<Item = f32> + Source> SetSpeed<f32> for Amplify<I> {}
impl<I: Iterator<Item = f32> + Source, I2: Iterator<Item = f32> + Source> SetSpeed<f32>
    for Mix<I, I2>
{
}

pub struct GenericSource<T>
where
    T: Sample,
{
    sound: Box<dyn SourceIterDynClone<T>>,
}

impl Clone for GenericSource<f32> {
    fn clone(&self) -> Self {
        Self {
            sound: dyn_clone::clone_box(&*self.sound),
        }
    }
}
unsafe impl<T: Sample> Send for GenericSource<T> {}

impl<T> GenericSource<T>
where
    T: Sample,
{
    pub fn new(sound: Box<dyn SourceIterDynClone<T>>) -> Self {
        Self { sound: sound }
    }
}

impl<'a, S: Sample> Iterator for GenericSource<S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        self.sound.next()
    }
}

impl<'a, S: Sample> Source for GenericSource<S> {
    fn current_frame_len(&self) -> Option<usize> {
        self.sound.current_frame_len()
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
    queue: Vec<GenericSource<f32>>,
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

    pub fn clone_sound(
        &mut self,
        idx: usize,
    ) -> Result<GenericSource<f32>, Box<dyn std::error::Error>> {
        if idx >= self.queue.len() {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "Sound queue accessed an out of bounds element",
            )));
        }
        return Ok(self.queue[idx].clone());
    }

    pub fn push_sound(&mut self, sound: Box<dyn SourceIterDynClone<f32>>) -> usize {
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
