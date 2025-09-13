use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct AutomatedClamp<I1: DawSource, I2: DawSource, I3: DawSource> {
    source: I1,
    min: I2,
    max: I3,
}

impl<I1: DawSource, I2: DawSource, I3: DawSource> AutomatedClamp<I1, I2, I3> {
    #[inline]
    pub fn new(source: I1, min: I2, max: I3) -> Self {
        Self { source, max, min }
    }
}

impl<I1: DawSource + Clone, I2: DawSource + Clone, I3: DawSource + Clone> DawSource
    for AutomatedClamp<I1, I2, I3>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source.next(index, channel),
            self.min.next(index, channel),
            self.max.next(index, channel),
        ) {
            (Some(source), Some(mut min), Some(mut max)) => {
                if min > max {
                    std::mem::swap(&mut min, &mut max);
                }
                Some(source.clamp(min, max))
            }
            _ => None,
        }
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.min.note_speed(speed, rate);
        self.max.note_speed(speed, rate);
        self.source.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        let max = self.max.size_hint()?;
        let min = self.min.size_hint()?;
        let source = self.source.size_hint()?;
        Some(max.max(min.max(source)))
    }
}
