use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Signum<I: SoundNode> {
    source: I,
}

impl<I: SoundNode> Signum<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self { source }
    }
}

impl<I: SoundNode + Clone> SoundNode for Signum<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|sample| sample.signum())
    }
}

pub fn signum_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Signum".to_string(),
        tooltip: r#"if the wave is above 0, it becomes 1. if it's below zero, it becomes -1.
This results in a weird square wave type effect."#
            .to_string(),
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
                    value: props.push_sound(Box::new(Signum::new(cloned))),
                },
            )]))
        })),
    }
}
