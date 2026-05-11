use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct Wrapper<I: SoundNode> {
    source: I,
    last: [Option<f32>; 2],
}

impl<I: SoundNode> Wrapper<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self {
            source,
            last: [None; 2],
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Wrapper<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.last[channel as usize] = match (
            self.source.next(index, channel),
            self.last[channel as usize],
        ) {
            (Some(sample1), Some(sample2)) => {
                if sample1 + sample2 > 1.0 {
                    return Some(-1.0 + sample1 + sample2 - 1.0);
                }
                if sample1 + sample2 < -1.0 {
                    return Some(1.0 - sample1 + sample2 + 1.0);
                }
                Some(sample1 + sample2)
            }
            (None, Some(sample2)) => Some(sample2),
            (Some(sample1), None) => Some(sample1),
            _ => Some(0.0),
        };
        self.last[channel as usize]
    }
}
