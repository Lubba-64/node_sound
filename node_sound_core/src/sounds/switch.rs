use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Switch<I: DawSource, I2: DawSource, I3: DawSource> {
    source1: I,
    source2: I2,
    switch: I3,
}

impl<I: DawSource, I2: DawSource, I3: DawSource> Switch<I, I2, I3> {
    #[inline]
    pub fn new(source1: I, source2: I2, switch: I3) -> Self {
        Self {
            source1,
            source2,
            switch,
        }
    }
}

impl<I: DawSource + Clone, I2: DawSource + Clone, I3: DawSource + Clone> DawSource
    for Switch<I, I2, I3>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.switch.next(index, channel).map(|x| {
            if x > 0.0 {
                self.source1.next(index, channel).unwrap_or_default()
            } else {
                self.source2.next(index, channel).unwrap_or_default()
            }
        })
    }
}
