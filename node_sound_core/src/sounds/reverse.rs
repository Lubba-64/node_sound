use crate::{
    constants::DEFAULT_SAMPLE_RATE,
    sound_map::DawSource,
    sounds::wave_table::{SourceWavetableOscillator, WaveTableTrait},
};

#[derive(Clone)]
pub struct ReverseSource<S: DawSource> {
    wavetable: SourceWavetableOscillator<S>,
}

impl<S: DawSource> ReverseSource<S> {
    #[inline]
    pub fn new(source: S, duration: f32) -> Self {
        let sample_rate = DEFAULT_SAMPLE_RATE;
        let table =
            SourceWavetableOscillator::from_source(source, sample_rate, duration, 1.0, false);

        Self { wavetable: table }
    }
}

impl<S: DawSource + Clone> DawSource for ReverseSource<S> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.get_sample(index, channel)
    }

    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.wavetable.note_speed(speed, rate);
        self.wavetable.left_table.reverse();
        self.wavetable.right_table.reverse();
    }

    fn size_hint(&self) -> Option<f32> {
        self.wavetable.size_hint()
    }
}
