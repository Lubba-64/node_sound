use crate::{
    node::GenericSoundNode,
    node_prelude::*,
    nodes::{
        const_node::ConstWave, daw_automation_source_node::DawAutomationChannel, lfo_node::Lfo,
        minus_node::Minus, mix_node::Mix,
    },
};

#[derive(Clone, Debug)]
pub struct DawAutomationMix {
    source: GenericSoundNode,
}

impl DawAutomationMix {
    #[inline]
    pub fn new<S: SoundNode + Clone + 'static, S2: SoundNode + Clone + 'static>(
        channel: Arc<Mutex<f32>>,
        audio1: S,
        audio2: S2,
    ) -> Self {
        let channel = DawAutomationChannel::new(channel);
        Self {
            source: GenericSoundNode::new(Box::new(Mix::new(
                Lfo::new(Minus::new(channel.clone(), ConstWave::new(1.0)), audio1),
                Lfo::new(channel, audio2),
            ))),
        }
    }
}

impl SoundNode for DawAutomationMix {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel)
    }
}

pub fn daw_automation_mix_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Daw Automations Mix".to_string(),
        tooltip: r#"Mixes between audio 1 and audio 2 based on the daw parameter."#.to_string(),
        inputs: BTreeMap::from([
            (
                "channel".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOnly,
                    name: "channel".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: 17.0,
                    },
                },
            ),
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "audio 2".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 2".to_string(),
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
            let cloned1 = props.clone_sound(props.get_source("audio 2")?)?;
            let cloned2 = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(DawAutomationMix::new(
                        props.state.runtime_state.automations.0
                            [(props.get_float("channel")?.round() as usize).clamp(0, 17)]
                        .clone(),
                        cloned1,
                        cloned2,
                    ))),
                },
            )]))
        })),
    }
}
