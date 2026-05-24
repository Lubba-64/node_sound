use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Abs<I: SoundNode> {
    source: I,
}

impl<I: SoundNode> Abs<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self { source }
    }
}

impl<I: SoundNode + Clone> SoundNode for Abs<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel).map(|sample| sample.abs())
    }
}

pub fn abs_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Abs".to_string(),
        tooltip: r#"Applies absolute value to the waveform,
bringing everything on the bottom of the waveform to the top."#
            .to_string(),
        inputs: vec![Input {
            data_type: DataType::AudioSource,
            kind: InputParamKind::ConnectionOnly,
            name: "audio 1".to_string(),
            value: InputValueConfig::AudioSource {},
        }],
        outputs: get_default_outputs(),
        op: Some(Arc::new(|mut props| {
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(Abs::new(cloned))),
                },
            )]))
        })),
    }
}
