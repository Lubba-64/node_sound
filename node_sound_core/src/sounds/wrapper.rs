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
            (Some(x), Some(y)) => {
                if x + y > 1.0 {
                    return Some(-1.0 + x + y - 1.0);
                }
                if x + y < -1.0 {
                    return Some(1.0 - x + y + 1.0);
                }
                Some(x + y)
            }
            (None, Some(y)) => Some(y),
            (Some(x), None) => Some(x),
            _ => Some(0.0),
        };
        self.last[channel as usize]
    }
}
