use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct MergeChannels<I1: SoundNode, I2: SoundNode> {
    source1: I1,
    source2: I2,
}

impl<I1: SoundNode, I2: SoundNode> MergeChannels<I1, I2> {
    #[inline]
    pub fn new(source1: I1, source2: I2) -> Self {
        Self { source1, source2 }
    }
}

impl<I1: SoundNode + Clone, I2: SoundNode + Clone> SoundNode for MergeChannels<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if channel == 0 {
            self.source1.next(index, 0)
        } else if channel == 1 {
            self.source2.next(index, 0)
        } else {
            None
        }
    }
}

pub fn merge_channels_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Merge Channels".to_string(),
        tooltip:
            r#"Merges the left audio of the two sources into a left and right singular source."#
                .to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                Input {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "audio 2".to_string(),
                Input {
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
            let cloned1 = props.clone_sound(props.get_source("audio 1")?)?;
            let cloned2 = props.clone_sound(props.get_source("audio 2")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(MergeChannels::new(cloned1, cloned2))),
                },
            )]))
        })),
    }
}
