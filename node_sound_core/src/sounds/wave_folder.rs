use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Wavefolder<I: DawSource> {
    source: I,
    gain: f32,
    offset: f32,
    folds: u8,
    last_sample: f32,
}

impl<I: DawSource> Wavefolder<I> {
    #[inline]
    pub fn new(source: I, gain: f32, offset: f32, folds: u8) -> Self {
        Self {
            source,
            gain,
            offset,
            folds,
            last_sample: 0.0,
        }
    }

    fn fold_wave(&mut self, mut sample: f32) -> f32 {
        sample = sample * self.gain + self.offset;

        for _ in 0..self.folds {
            sample += self.last_sample;
            sample = if sample > 1.0 {
                2.0 - sample
            } else if sample < -1.0 {
                -2.0 - sample
            } else {
                sample
            };
        }
        self.last_sample = sample;
        sample.clamp(-1.0, 1.0)
    }
}

impl<I: DawSource + Clone> DawSource for Wavefolder<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|sample| self.fold_wave(sample))
    }
}
