use std::io::Cursor;

use rodio::{Decoder, Source, source::UniformSourceIterator};

use crate::{
    sound_map::DawSource,
    sounds::wave_table::{WaveTableManager, WaveTableOscillator},
};

#[derive(Clone, Debug)]
pub struct CloneableDecoder {
    pub wavetable: WaveTableOscillator,
}

impl CloneableDecoder {
    pub fn new(
        data: Vec<u8>,
        uses_speed: bool,
        sample_rate: u32,
        speed: f32,
        manager: &mut WaveTableManager,
    ) -> Self {
        Self {
            wavetable: manager.make_wavetable_samples(
                sample_rate as f32,
                1.0,
                1.0,
                uses_speed,
                speed,
                Box::new(|| {
                    let data = data.clone();
                    let data: Vec<_> = UniformSourceIterator::new(
                        Decoder::new(Cursor::new(data))
                            .expect("expect valid wav data")
                            .convert_samples::<f32>()
                            .speed(1.0),
                        1,
                        sample_rate,
                    )
                    .collect();
                    (data.clone(), data)
                }),
            ),
        }
    }
}

impl DawSource for CloneableDecoder {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.next(index, channel)
    }
}
