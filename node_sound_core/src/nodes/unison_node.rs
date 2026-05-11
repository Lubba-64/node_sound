use crate::{
    node::GenericSoundNode,
    node_prelude::*,
    nodes::{skip_node::Skip, speed_node::Speed},
};

#[derive(Clone, Debug)]
pub struct UnisonVoice {
    source: MixVec,
}

#[derive(Clone, Debug)]
pub struct MixVec {
    vec: Vec<GenericSoundNode>,
}

impl MixVec {
    fn new(vec: Vec<GenericSoundNode>) -> Self {
        Self { vec }
    }
}

impl SoundNode for MixVec {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let mut samples = vec![0.0; self.vec.len()];
        for i in 0..self.vec.len() {
            samples[i] = self.vec[i].next(index, channel).unwrap_or_default();
        }
        Some(samples.iter().sum::<f32>() / (self.vec.len() as f32).sqrt())
    }
}

impl UnisonVoice {
    pub fn new<S: SoundNode + 'static + Clone>(
        source: S,
        mut phase_sep: f32,
        voices: u8,
        sample_rate: f32,
        note_speed: f32,
        base_frequency: f32,
        detune_amount: f32,
    ) -> Self {
        phase_sep = phase_sep.abs().clamp(0.0, 100.0) / 100.0;
        let time_offset = phase_sep * (1.0 / base_frequency);
        let mut mix_vec = Vec::new();
        for i in 0..voices {
            let detune_factor = if voices > 1 {
                let position = (i as f32) / ((voices - 1) as f32) - 0.5;
                1.0 + detune_amount * position * 0.01
            } else {
                1.0
            };
            let detuned_speed = note_speed * detune_factor;
            let voice_source = if i == 0 {
                if (detune_factor - 1.0).abs() > 0.0001 {
                    GenericSoundNode::new(Box::new(Speed::new(source.clone(), detuned_speed)))
                } else {
                    GenericSoundNode::new(Box::new(source.clone()))
                }
            } else {
                let phase_offset = time_offset * i as f32;
                GenericSoundNode::new(Box::new(Speed::new(
                    Skip::new(phase_offset, source.clone(), true, sample_rate, note_speed),
                    detuned_speed,
                )))
            };
            mix_vec.push(voice_source);
        }
        Self {
            source: MixVec::new(mix_vec),
        }
    }
}

impl SoundNode for UnisonVoice {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel)
    }
}

pub fn unison_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Unison".to_string(),
        tooltip: r#"Unison with multiple voices.
voices is the number of voices.
unison is the amount of unison the voices have with them being identical at 0 and
totally phase separated at 100.0"#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "unison".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "unison".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: 100.0,
                    },
                },
            ),
            (
                "voices".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "voices".to_string(),
                    value: InputValueConfig::Float {
                        value: 5.0,
                        min: 0.0,
                        max: 25.0,
                    },
                },
            ),
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "frequency".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: MAX_FREQ,
                    },
                },
            ),
            (
                "detune".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "detune".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: 20.0,
                    },
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
        ]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
        op: Some(Arc::new(|mut props| {
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(UnisonVoice::new(
                        cloned,
                        props.get_float("unison")?,
                        props.get_float("voices")? as u8,
                        props.sample_rate(),
                        props.note_speed(),
                        props.get_float("frequency")?,
                        props.get_float("detune")?,
                    ))),
                },
            )]))
        })),
    }
}
