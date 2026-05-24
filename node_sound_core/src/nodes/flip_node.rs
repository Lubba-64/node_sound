use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Flip<I1: SoundNode> {
    source: I1,
}

impl<I1: SoundNode> Flip<I1> {
    #[inline]
    pub fn new(source: I1) -> Self {
        Self { source }
    }
}

impl<I1: SoundNode + Clone> SoundNode for Flip<I1> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel).map(|sample| -sample)
    }
}

pub fn flip_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Flip".to_string(),
        tooltip: r#"Flips the waveform vertically."#.to_string(),
        inputs: BTreeMap::from([(
            "audio 1".to_string(),
            Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 1".to_string(),
                value: InputValueConfig::AudioSource {},
            },
        )]),
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
                    value: props.push_sound(Box::new(Flip::new(cloned))),
                },
            )]))
        })),
    }
}
