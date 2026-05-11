use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct After<T: SoundNode, U: SoundNode> {
    first: T,
    second: U,
    current: AfterState,
}

#[derive(Clone, Debug)]
enum AfterState {
    PlayingFirst,
    PlayingSecond,
    Finished,
}

impl<T: SoundNode, U: SoundNode> After<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Self {
            first,
            second,
            current: AfterState::PlayingFirst,
        }
    }
}

impl<T: SoundNode + Clone, U: SoundNode + Clone> SoundNode for After<T, U> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match self.current {
            AfterState::PlayingFirst => {
                if let Some(sample) = self.first.next(index, channel) {
                    Some(sample)
                } else {
                    self.current = AfterState::PlayingSecond;
                    self.second.next(index, channel)
                }
            }
            AfterState::PlayingSecond => {
                if let Some(sample) = self.second.next(index, channel) {
                    Some(sample)
                } else {
                    self.current = AfterState::Finished;
                    None
                }
            }
            AfterState::Finished => None,
        }
    }
}
