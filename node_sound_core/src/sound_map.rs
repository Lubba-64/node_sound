use dyn_clone::DynClone;
use rodio::source::Source;
use rodio::Sample;
use std::io::ErrorKind;

pub trait RefSourceIter<Item: Sample>:
    Source<Item = Item> + Iterator<Item = Item> + 'static
{
}
pub trait RefSourceIterDynClone<Item: Sample>: DynClone + RefSourceIter<Item> {}

pub struct RepeatN<I: Iterator<Item = f32>> {
    iter: I,
    repeats: usize,
    repeat: usize,
    last: Option<f32>,
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
    sound: Box<dyn RefSourceIterDynClone<T>>,
}

impl RefSourceIter<f32> for GenericSource<f32> {}
impl RefSourceIterDynClone<f32> for GenericSource<f32> {}

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
    fn new(sound: Box<dyn RefSourceIterDynClone<T>>) -> Self {
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

impl<I: RefSourceIterDynClone<f32>> RefSourceIter<f32> for RepeatSource<I> {}
impl<I: RefSourceIterDynClone<f32> + Clone> RefSourceIterDynClone<f32> for RepeatSource<I> {}

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

pub struct RefSource<'a> {
    idx: usize,
    source: &'a mut dyn RefSourceIterDynClone<f32>,
}

impl Clone for RefSource<'static> {
    fn clone(&self) -> Self {
        return unsafe { RefSource::new(&mut SOUND_QUEUE[self.idx], self.idx) };
    }
}

unsafe impl<'a> Send for RefSource<'a> {}

impl RefSourceIter<f32> for RefSource<'static> {}
impl RefSourceIterDynClone<f32> for RefSource<'static> {}

impl<'a> RefSource<'a> {
    pub fn new<I: RefSourceIterDynClone<f32>>(source: &'a mut I, idx: usize) -> Self {
        Self {
            source: source,
            idx,
        }
    }
    pub fn clone_inner(&self) -> Box<dyn RefSourceIterDynClone<f32>> {
        dyn_clone::clone_box(&*self.source)
    }
}

impl<'a> Iterator for RefSource<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next()
    }
}

impl<'a> Source for RefSource<'a> {
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

static mut SOUND_QUEUE: Vec<RepeatSource<GenericSource<f32>>> = vec![];

pub fn push_sound<I: RefSourceIterDynClone<f32>>(
    sound: Box<dyn RefSourceIterDynClone<f32>>,
) -> usize {
    unsafe {
        SOUND_QUEUE.push(RepeatSource::new(GenericSource::new(sound), 0));
        return SOUND_QUEUE.len() - 1;
    }
}

pub fn clone_sound_ref(idx: usize) -> Result<RefSource<'static>, Box<dyn std::error::Error>> {
    if idx >= unsafe { SOUND_QUEUE.len() } {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "Sound queue accessed an out of bounds element",
        )));
    }
    unsafe {
        SOUND_QUEUE[idx].repeats += 1;
    }
    return unsafe { Ok(RefSource::new(&mut SOUND_QUEUE[idx], idx)) };
}

pub fn sound_queue_len() -> usize {
    unsafe { SOUND_QUEUE.len() }
}

pub fn clear() {
    unsafe { SOUND_QUEUE.clear() }
}

pub fn set_repeats(idx: usize, repeats: usize) {
    unsafe {
        SOUND_QUEUE[idx].repeats = repeats;
    }
}

pub fn clone_sound(idx: usize) -> Result<GenericSource<f32>, Box<dyn std::error::Error>> {
    if idx >= unsafe { SOUND_QUEUE.len() } {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "Sound queue accessed an out of bounds element",
        )));
    }
    unsafe {
        SOUND_QUEUE[idx].repeats += 1;
    }
    return unsafe { Ok(SOUND_QUEUE[idx].source.clone()) };
}
