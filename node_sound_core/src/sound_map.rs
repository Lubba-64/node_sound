use dyn_clone::DynClone;
use rodio::source::{Source, Zero};
use rodio::{Decoder, Sample};
use std::cell::RefCell;
use std::io::ErrorKind;
use std::rc::Rc;

use crate::constants::DEFAULT_SAMPLE_RATE;

pub trait RefSourceIter<Item: Sample>:
    Source<Item = Item> + Iterator<Item = Item> + 'static
{
}
pub trait RefSourceIterDynClone<Item: Sample>: DynClone + RefSourceIter<Item> {}
impl<I> RefSourceIter<f32> for I where I: Iterator<Item = f32> + Source + 'static {}
impl<T: std::io::Read + std::io::Seek + 'static> RefSourceIter<i16> for Decoder<T> {}
impl<I> RefSourceIterDynClone<f32> for I where I: RefSourceIter<f32> + Clone {}

pub struct GenericSource<T>
where
    T: Sample,
{
    sound: Box<dyn RefSourceIterDynClone<T>>,
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
    pub fn new(sound: Box<dyn RefSourceIterDynClone<T>>) -> Self {
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
pub struct RepeatSource<I: RefSourceIterDynClone<f32>> {
    source: I,
    pub repeats: usize,
    repeat: usize,
    last: Option<f32>,
}

impl<I: RefSourceIterDynClone<f32>> RepeatSource<I> {
    pub fn new(source: I, repeats: usize) -> Self {
        RepeatSource {
            repeats: repeats,
            source: source,
            last: None,
            repeat: 0,
        }
    }
}

impl<I: RefSourceIterDynClone<f32>> Source for RepeatSource<I> {
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.total_duration()
    }
}

impl<I: RefSourceIterDynClone<f32>> Iterator for RepeatSource<I> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.repeat == 0 {
            self.repeat = self.repeats;
            self.last = self.source.next();
        }
        self.repeat -= 1;
        return self.last;
    }
}

pub struct RefSource {
    source: Rc<RefCell<dyn RefSourceIterDynClone<f32>>>,
}

impl Clone for RefSource {
    fn clone(&self) -> Self {
        return RefSource {
            source: Rc::new(RefCell::new(GenericSource::new(dyn_clone::clone_box(
                &*self.source.borrow(),
            )))),
        };
    }
}

unsafe impl Send for RefSource {}

impl RefSource {
    pub fn new<I: RefSourceIterDynClone<f32>>(source: Rc<RefCell<I>>) -> Self {
        Self { source: source }
    }
}

impl Iterator for RefSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.borrow_mut().next()
    }
}

impl Source for RefSource {
    fn current_frame_len(&self) -> Option<usize> {
        self.source.borrow().current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.borrow().channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.borrow().sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.borrow().total_duration()
    }
}

#[derive(Clone)]
pub struct SoundQueue {
    queue: Vec<Rc<RefCell<RepeatSource<GenericSource<f32>>>>>,
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
        self.queue[idx].borrow_mut().repeats += 1;
        return Ok(self.queue[idx].borrow().source.clone());
    }

    pub fn push_sound(&mut self, sound: Box<dyn RefSourceIterDynClone<f32>>) -> usize {
        self.queue.push(Rc::new(RefCell::new(RepeatSource::new(
            GenericSource::new(sound),
            0,
        ))));
        return self.queue.len() - 1;
    }

    pub fn clone_sound_ref(&mut self, idx: usize) -> Result<RefSource, Box<dyn std::error::Error>> {
        if idx >= self.queue.len() {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "Sound queue accessed an out of bounds element",
            )));
        }
        self.queue[idx].borrow_mut().repeats += 1;
        return Ok(RefSource::new(self.queue[idx].clone()));
    }

    pub fn sound_queue_len(&mut self) -> usize {
        self.queue.len()
    }

    pub fn clear(&mut self) {
        self.queue.clear()
    }

    pub fn set_repeats(&mut self, idx: usize, repeats: usize) {
        self.queue[idx].borrow_mut().repeats = repeats;
    }
}
