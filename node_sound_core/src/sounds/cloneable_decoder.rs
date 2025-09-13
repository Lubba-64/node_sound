use std::io::Cursor;

use rodio::{Decoder, Source};

use crate::{
    constants::DEFAULT_SAMPLE_RATE, sound_map::DawSource, sounds::wave_table::WaveTableOscillator,
};

#[derive(Clone)]
pub struct CloneableDecoder {
    pub wavetable: WaveTableOscillator,
}

impl CloneableDecoder {
    pub fn new(data: Vec<u8>, uses_speed: bool) -> Self {
        let data: Vec<_> = Decoder::new(Cursor::new(data))
            .expect("expect valid wav data")
            .convert_samples()
            .speed(1.0)
            .collect();
        let mut wavetable = WaveTableOscillator::new_stereo(DEFAULT_SAMPLE_RATE as f32, 1.0);
        wavetable.rebuild_table(DEFAULT_SAMPLE_RATE as f32, data.clone(), data);
        wavetable.set_uses_speed(uses_speed);
        Self { wavetable }
    }
}

impl DawSource for CloneableDecoder {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.next(index, channel)
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.wavetable.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        self.wavetable.size_hint()
    }
}
