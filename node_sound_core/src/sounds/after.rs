use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct After<T: DawSource, U: DawSource> {
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

impl<T: DawSource, U: DawSource> After<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Self {
            first,
            second,
            current: AfterState::PlayingFirst,
        }
    }
}

impl<T: DawSource + Clone, U: DawSource + Clone> DawSource for After<T, U> {
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

    fn size_hint(&self) -> Option<f32> {
        None
    }
}
