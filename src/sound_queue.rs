use std::io::ErrorKind;

use crate::sounds::GenericSource;
static mut SOUND_QUEUE: Vec<GenericSource<f32>> = vec![];

pub fn push_sound(v: GenericSource<f32>) -> usize {
    unsafe {
        SOUND_QUEUE.push(v);
        return SOUND_QUEUE.len() - 1;
    }
}

pub fn clone_sound(idx: usize) -> Result<GenericSource<f32>, Box<dyn std::error::Error>> {
    if idx >= unsafe { SOUND_QUEUE.len() } {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "Sound queue accessed an out of bounds element",
        )));
    }
    return unsafe { Ok(SOUND_QUEUE[idx].clone()) };
}

pub fn clear() {
    unsafe { SOUND_QUEUE.clear() }
}
