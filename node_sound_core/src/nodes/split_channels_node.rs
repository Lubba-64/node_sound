use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct SplitChannels<I: SoundNode> {
    source: I,
    channel: u8,
}

impl<I: SoundNode> SplitChannels<I> {
    #[inline]
    pub fn new(source: I, channel: u8) -> Self {
        Self { source, channel }
    }
}

impl<I: SoundNode + Clone> SoundNode for SplitChannels<I> {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        self.source.next(index, self.channel)
    }
}

pub fn split_channels_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Split Channels".to_string(),
        tooltip: r#"Takes only the left or right channel and puts it on both channels."#
            .to_string(),
        inputs: BTreeMap::from([
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
                "channel".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOnly,
                    name: "channel".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: 1.0,
                    },
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
                    value: props.push_sound(Box::new(SplitChannels::new(
                        cloned,
                        props.get_float("channel")?.round() as u8,
                    ))),
                },
            )]))
        })),
    }
}
