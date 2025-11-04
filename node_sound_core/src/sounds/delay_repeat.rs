use crate::sound_map::DawSource;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
// this used to say ai shit but no this was actually all me because I disliked the ai version.
// I have no idea why putting the buffer in an arc made things better for the effect plugin.
#[derive(Clone, Debug)]
pub struct DelayRepeat<I: DawSource> {
    source: I,
    delay: f32,
    points: usize,
    sample_rate: f32,
    deque: [Arc<Mutex<VecDeque<f32>>>; 2],
}

impl<I: DawSource> DelayRepeat<I> {
    pub fn new(
        source: I,
        delay: f32,
        sample_rate: f32,
        points: usize,
        buffers: &mut Vec<Arc<Mutex<VecDeque<f32>>>>,
    ) -> Self {
        let l = Arc::new(Mutex::new(vec![0.0; (sample_rate * delay) as usize].into()));
        let r = Arc::new(Mutex::new(vec![0.0; (sample_rate * delay) as usize].into()));
        buffers.push(l.clone());
        buffers.push(r.clone());
        Self {
            source,
            delay,
            points,
            sample_rate,
            deque: [l, r],
        }
    }
}

impl<I: DawSource + Clone> DawSource for DelayRepeat<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let mut deque = match self.deque[channel as usize].lock() {
            Ok(x) => x,
            Err(_x) => return None,
        };
        (*deque).pop_back();
        (*deque).push_front(self.source.next(index, channel).unwrap_or_default());
        let total_size = (self.sample_rate * self.delay) as usize - 100;
        Some(
            (0..self.points)
                .into_iter()
                .map(|point| {
                    (*deque)[if point == 0 {
                        0
                    } else {
                        (total_size / self.points) * point
                    }] / (self.points as f32 / (self.points - point) as f32)
                })
                .sum::<f32>(),
        )
    }
}
