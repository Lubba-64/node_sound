use crate::{
    constants::MIDDLE_C_FREQ,
    sound_map::DawSource,
    sounds::wave_table::{WaveTableManager, WaveTableOscillator},
};
use synthrs::{midi::MidiSong, synthesizer::make_samples_from_midi, wave};

#[derive(Clone, Debug)]
pub struct MidiRenderer {
    wavetable: WaveTableOscillator,
}

impl MidiRenderer {
    #[inline]
    pub fn new<S: DawSource>(
        source: S,
        song: MidiSong,
        uses_speed: bool,
        speed: f32,
        sample_rate: f32,
        cache: &mut WaveTableManager,
    ) -> Self {
        Self {
            wavetable: cache.make_wavetable(
                sample_rate,
                MIDDLE_C_FREQ as f32,
                source,
                1.0,
                MIDDLE_C_FREQ as f32,
                uses_speed,
                speed.clone(),
                Box::new(|source, total_samples| {
                    let mut source_samples: Vec<f64> = Vec::with_capacity(total_samples as usize);
                    for i in 0..total_samples as usize {
                        let sample = source.next(i as f32, 0).unwrap_or(0.0);
                        source_samples.push(sample.into());
                    }
                    let sampler = |frequency: f64| {
                        wave::sampler(
                            frequency * speed as f64,
                            &source_samples,
                            source_samples.len(),
                            MIDDLE_C_FREQ as f64,
                            total_samples as usize,
                        )
                    };
                    let midi_samples =
                        make_samples_from_midi(sampler, total_samples as usize, true, song.clone())
                            .expect("midi play failed");
                    let left: Vec<_> = midi_samples.iter().map(|&x| x as f32).collect();
                    (left.clone(), left)
                }),
            ),
        }
    }
}

impl DawSource for MidiRenderer {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.next(index, channel)
    }
}
