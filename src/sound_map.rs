use rodio::source::Source;
use rodio::Sample;
use std::cell::RefCell;
use std::io::ErrorKind;
use std::rc::Rc;

pub struct AutomationChannelF32(Box<dyn Iterator<Item = f32> + 'static>);

impl Iterator for AutomationChannelF32 {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return self.0.next();
    }
}

impl AutomationChannelF32 {
    fn from_source(source: impl RefSourceIter<f32>) -> Self {
        Self(Box::new(source))
    }
}

pub trait RefSourceIter<Item: Sample>:
    Source<Item = Item> + Iterator<Item = Item> + 'static
{
}

pub struct RepeatN<I: Iterator<Item = f32>> {
    iter: I,
    repeats: usize,
    repeat: usize,
    last: Option<f32>,
}

impl<I: Iterator<Item = f32>> RepeatN<I> {
    fn new(iter: I, repeats: usize) -> Self {
        Self {
            repeats,
            iter,
            repeat: 0,
            last: None,
        }
    }
}

impl<I: Iterator<Item = f32>> Iterator for RepeatN<I> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.repeat == 0 {
            self.repeat = self.repeats;
            self.last = self.iter.next();
        }
        self.repeat -= 1;
        return self.last;
    }
}

pub struct GenericSource<T>
where
    T: Sample,
{
    sound: Box<dyn RefSourceIter<T>>,
}

impl RefSourceIter<f32> for GenericSource<f32> {}

unsafe impl<T: Sample> Send for GenericSource<T> {}

impl<T> GenericSource<T>
where
    T: Sample,
{
    fn new(sound: Box<dyn RefSourceIter<T>>) -> Self {
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

pub struct RepeatSource<I: RefSourceIter<f32>> {
    source: I,
    pub repeats: usize,
    repeat: usize,
    last: Option<f32>,
}

impl<I: RefSourceIter<f32>> RefSourceIter<f32> for RepeatSource<I> {}

impl<I: RefSourceIter<f32>> RepeatSource<I> {
    pub fn new(source: I, repeats: usize) -> Self {
        RepeatSource {
            repeats: repeats,
            source: source,
            last: None,
            repeat: 0,
        }
    }
}

impl<I: RefSourceIter<f32>> Source for RepeatSource<I> {
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

impl<I: RefSourceIter<f32>> Iterator for RepeatSource<I> {
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

#[derive(Clone)]
pub struct RefSource {
    source: Rc<RefCell<dyn RefSourceIter<f32>>>,
}

unsafe impl Send for RefSource {}

impl RefSourceIter<f32> for RefSource {}

impl RefSource {
    pub fn new<I: RefSourceIter<f32>>(source: Rc<RefCell<I>>) -> Self {
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
        self.source.borrow_mut().current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.borrow_mut().channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.borrow_mut().sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.borrow_mut().total_duration()
    }
}

static mut SOUND_QUEUE: Vec<Rc<RefCell<RepeatSource<GenericSource<f32>>>>> = vec![];

pub fn push_sound<I: RefSourceIter<f32>>(sound: Box<dyn RefSourceIter<f32>>) -> usize {
    unsafe {
        SOUND_QUEUE.push(Rc::new(RefCell::new(RepeatSource::new(
            GenericSource::new(sound),
            0,
        ))));
        return SOUND_QUEUE.len() - 1;
    }
}

pub fn clone_sound(idx: usize) -> Result<RefSource, Box<dyn std::error::Error>> {
    if idx >= unsafe { SOUND_QUEUE.len() } {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "Sound queue accessed an out of bounds element",
        )));
    }
    unsafe {
        SOUND_QUEUE[idx].borrow_mut().repeats += 1;
    }
    return unsafe { Ok(RefSource::new(SOUND_QUEUE[idx].clone())) };
}

pub fn clear() {
    unsafe { SOUND_QUEUE.clear() }
}

pub fn set_repeats(idx: usize, repeats: usize) {
    unsafe {
        SOUND_QUEUE[idx].borrow_mut().repeats = repeats;
    }
}

static mut AUTOMATION_QUEUE: Vec<Rc<RefCell<RepeatN<AutomationChannelF32>>>> = vec![];

pub fn push_automation(automation: AutomationChannelF32) -> usize {
    unsafe {
        AUTOMATION_QUEUE.push(Rc::new(RefCell::new(RepeatN::new(automation, 0))));
        return AUTOMATION_QUEUE.len() - 1;
    }
}

pub fn clone_automation(
    idx: usize,
) -> Result<Rc<RefCell<RepeatN<AutomationChannelF32>>>, Box<dyn std::error::Error>> {
    if idx >= unsafe { AUTOMATION_QUEUE.len() } {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "Sound queue accessed an out of bounds element",
        )));
    }
    unsafe {
        AUTOMATION_QUEUE[idx].borrow_mut().repeats += 1;
    }
    return unsafe { Ok(AUTOMATION_QUEUE[idx].clone()) };
}

pub fn automation_clear() {
    unsafe { AUTOMATION_QUEUE.clear() }
}

pub fn set_automation_repeats(idx: usize, repeats: usize) {
    unsafe {
        AUTOMATION_QUEUE[idx].borrow_mut().repeats = repeats;
    }
}
