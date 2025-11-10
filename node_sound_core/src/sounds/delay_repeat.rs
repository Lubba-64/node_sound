use crate::sound_map::DawSource;
use std::collections::VecDeque;
// this used to say ai shit but no this was actually all me because I disliked the ai version.
// needs to be in an owned arc because otherwise
#[derive(Clone, Debug)]
pub struct DelayRepeat<I: DawSource> {
    source: I,
    delay: f32,
    points: usize,
    sample_rate: f32,
    deque: [VecDeque<f32>; 2],
}

impl<I: DawSource> DelayRepeat<I> {
    pub fn new(source: I, delay: f32, sample_rate: f32, points: usize) -> Self {
        Self {
            source,
            delay,
            points,
            sample_rate,
            deque: [
                vec![0.0; (sample_rate * delay) as usize].into(),
                vec![0.0; (sample_rate * delay) as usize].into(),
            ],
        }
    }
}

impl<I: DawSource + Clone> DawSource for DelayRepeat<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let deque = &mut self.deque[channel as usize];
        deque.pop_back();
        deque.push_front(self.source.next(index, channel).unwrap_or_default());
        let total_size = (self.sample_rate * self.delay) as usize - 100;
        Some(
            (0..self.points)
                .into_iter()
                .map(|point| {
                    let read_index = if point == 0 {
                        0
                    } else {
                        (total_size / self.points) * point
                    }
                    .min(total_size - 1);
                    deque[read_index] / (self.points as f32 / (self.points - point) as f32)
                })
                .sum::<f32>(),
        )
    }
}
