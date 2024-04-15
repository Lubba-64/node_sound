use crate::sounds::GenericSource;
static mut SOUND_QUEUE: Vec<GenericSource<f32>> = vec![];

pub fn push_sound(v: GenericSource<f32>) -> usize {
    unsafe {
        SOUND_QUEUE.push(v);
        return SOUND_QUEUE.len() - 1;
    }
}

pub fn pop_sound(idx: usize) -> GenericSource<f32> {
    return unsafe { SOUND_QUEUE.remove(idx) };
}

pub fn clear() {
    unsafe { SOUND_QUEUE.clear() }
}
