use crate::{
    constants::{DEFAULT_SAMPLE_RATE, MIDDLE_C_FREQ},
    sound_map::DawSource,
    sounds::wave_table::WavetableOscillator,
};
use synthrs::{midi::MidiSong, synthesizer::make_samples_from_midi, wave};

#[derive(Clone)]
pub struct MidiRenderer {
    wavetable: WavetableOscillator,
    source_samples: Vec<f64>,
    song: MidiSong,
    sample_rate: f32,
    uses_speed: bool,
    speed: f32,
}

impl MidiRenderer {
    #[inline]
    pub fn new<S: DawSource>(mut source: S, song: MidiSong, uses_speed: bool) -> Self {
        let sample_rate = DEFAULT_SAMPLE_RATE as f32;
        let num_samples = sample_rate as usize;
        let mut source_samples: Vec<f64> = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let sample = source.next(i as f32, 0).unwrap_or(0.0);
            source_samples.push(sample.into());
        }
        Self {
            wavetable: Self::render_midi_samples(
                &source_samples,
                &song,
                sample_rate,
                1.0,
                uses_speed,
            ),
            source_samples,
            song,
            sample_rate,
            uses_speed,
            speed: 1.0,
        }
    }

    fn render_midi_samples(
        source_samples: &Vec<f64>,
        song: &MidiSong,
        sample_rate: f32,
        speed: f32,
        uses_speed: bool,
    ) -> WavetableOscillator {
        let sampler = |frequency: f64| {
            wave::sampler(
                frequency * speed as f64,
                source_samples,
                source_samples.len(),
                MIDDLE_C_FREQ as f64,
                sample_rate as usize,
            )
        };
        let midi_samples =
            make_samples_from_midi(sampler, sample_rate as usize, true, song.clone())
                .expect("midi play failed");
        let left: Vec<_> = midi_samples.iter().map(|&x| x as f32).collect();
        WavetableOscillator::new_stereo(
            left.clone(),
            left,
            sample_rate as u32,
            MIDDLE_C_FREQ,
            uses_speed,
        )
    }
}

impl DawSource for MidiRenderer {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        self.wavetable.next(index, channel)
    }

    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.sample_rate = rate;
        if self.uses_speed {
            self.speed = speed;
        }
        self.wavetable = Self::render_midi_samples(
            &self.source_samples,
            &self.song,
            self.sample_rate,
            self.speed,
            self.uses_speed,
        );
    }
}
