use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct Switch<I: SoundNode, I2: SoundNode, I3: SoundNode> {
    source1: I,
    source2: I2,
    switch: I3,
}

impl<I: SoundNode, I2: SoundNode, I3: SoundNode> Switch<I, I2, I3> {
    #[inline]
    pub fn new(source1: I, source2: I2, switch: I3) -> Self {
        Self {
            source1,
            source2,
            switch,
        }
    }
}

impl<I: SoundNode + Clone, I2: SoundNode + Clone, I3: SoundNode + Clone> SoundNode
    for Switch<I, I2, I3>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.switch.next(index, channel).map(|sample| {
            if sample > 0.0 {
                self.source1.next(index, channel).unwrap_or_default()
            } else {
                self.source2.next(index, channel).unwrap_or_default()
            }
        })
    }
}
