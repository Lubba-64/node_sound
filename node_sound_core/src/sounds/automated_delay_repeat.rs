use crate::sound_map::DawSource;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct AutomatedDelayRepeat<I: DawSource, B: DawSource, P: DawSource> {
    source: I,
    buffer_size_source: B,
    points_source: P,
    sample_rate: f32,
    deque: [VecDeque<f32>; 2],
}

impl<I: DawSource, B: DawSource, P: DawSource> AutomatedDelayRepeat<I, B, P> {
    pub fn new(source: I, buffer_size_source: B, points_source: P, sample_rate: f32) -> Self {
        Self {
            source,
            buffer_size_source,
            points_source,
            sample_rate,
            deque: [VecDeque::new(), VecDeque::new()],
        }
    }

    fn update_buffer_size(&mut self, index: f32) -> usize {
        let buffer_size = self.buffer_size_source.next(index, 0).unwrap_or(0.0);
        (buffer_size.max(0.0) as usize).max(1) * self.sample_rate as usize
    }

    fn get_points(&mut self, index: f32) -> usize {
        let points = self.points_source.next(index, 0).unwrap_or(0.0);
        (points.max(0.0) as usize).max(1)
    }
}

impl<I: DawSource + Clone, B: DawSource + Clone, P: DawSource + Clone> DawSource
    for AutomatedDelayRepeat<I, B, P>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let buffer_size = self.update_buffer_size(index);
        let points = self.get_points(index);
        let deque = &mut self.deque[channel as usize];
        if deque.len() != buffer_size {
            deque.resize(buffer_size, 0.0);
        }
        if deque.len() > 0 {
            deque.pop_back();
            deque.push_front(self.source.next(index, channel).unwrap_or_default());
        }
        if deque.is_empty() {
            return Some(0.0);
        }
        let total_size = deque.len();
        Some(
            (0..points)
                .map(|point| {
                    let read_index = if point == 0 {
                        0
                    } else {
                        (total_size / points) * point
                    }
                    .min(total_size - 1);
                    deque[read_index] / (points as f32 / (points - point) as f32)
                })
                .sum::<f32>(),
        )
    }
}
