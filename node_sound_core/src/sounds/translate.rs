use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct TranslateWave<I: DawSource> {
    source: I,
    start_min: f32,
    start_max: f32,
    end_min: f32,
    end_max: f32,
}

impl<I: DawSource> TranslateWave<I> {
    #[inline]
    pub fn new(
        source: I,
        mut start_min: f32,
        mut start_max: f32,
        mut end_min: f32,
        mut end_max: f32,
    ) -> Self {
        if start_min > start_max {
            let other = start_min;
            start_min = start_max;
            start_max = other;
        }
        if end_min > end_max {
            let other = end_min;
            end_min = end_max;
            end_max = other;
        }
        Self {
            source: source,
            start_min,
            start_max,
            end_min,
            end_max,
        }
    }
}

impl<I: DawSource + Clone> DawSource for TranslateWave<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        return match self.source.next(index, channel) {
            Some(p) => Some(
                self.end_min
                    + ((self.end_max - self.end_min) / (self.start_max - self.start_min))
                        * (p.clamp(self.start_min, self.start_max) - self.start_min),
            ),
            _ => None,
        };
    }
    fn note_speed(&mut self, _speed: f32) {}
    fn set_sample_rate(&mut self, _rate: f32) {}
}
