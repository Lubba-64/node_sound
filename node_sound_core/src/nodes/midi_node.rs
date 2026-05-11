use crate::{
    error::NodeSoundError,
    node_prelude::*,
    nodes::wave_table::{WaveTableManager, WaveTableOscillator},
};
use synthrs::{midi::MidiSong, synthesizer::make_samples_from_midi, wave};

#[derive(Clone, Debug)]
pub struct MidiRenderer {
    wavetable: WaveTableOscillator,
}

impl MidiRenderer {
    #[inline]
    pub fn new<S: SoundNode>(
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
                    let left: Vec<_> = midi_samples.iter().map(|&sample| sample as f32).collect();
                    (left.clone(), left)
                }),
            ),
        }
    }
}

impl SoundNode for MidiRenderer {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.next(index, channel)
    }
}

pub fn midi_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Midi File".to_string(),
        tooltip: r#"Imports and plays a midi file with the given waveform."#.to_string(),
        inputs: BTreeMap::from([
            (
                "file".to_string(),
                InputParameter {
                    data_type: DataType::MidiFile,
                    kind: InputParamKind::ConstantOnly,
                    name: "file".to_string(),
                    value: InputValueConfig::MidiFile {},
                },
            ),
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "note independant".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "note independant".to_string(),
                    value: InputValueConfig::Bool { value: false },
                },
            ),
        ]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
        op: Some(Arc::new(|mut props| {
            props.update_wavetables_node_idx();
            let file = props.get_midi("file")?;
            if file.is_none() {
                return Ok(BTreeMap::from([(
                    "out".to_string(),
                    ValueType::AudioSource { value: 0 },
                )]));
            }
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            let midi = MidiRenderer::new(
                cloned,
                file.ok_or::<NodeSoundError>(anyhow!("midi file is missing").into())?
                    .1,
                props.get_bool("note independant")?,
                props.note_speed(),
                props.sample_rate(),
                &mut props.state.user_state.wavetables,
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(midi)),
                },
            )]))
        })),
    }
}
