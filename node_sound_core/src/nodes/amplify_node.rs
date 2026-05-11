use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Amplify<I: SoundNode> {
    source: I,
    amplification: f32,
}

impl<I: SoundNode> Amplify<I> {
    #[inline]
    pub fn new(source: I, amplification: f32) -> Self {
        Self {
            source,
            amplification,
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Amplify<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|sample| sample * self.amplification)
    }
}

pub fn amplify_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Amplify".to_string(),
        tooltip: r#"Amplifies the waveform making sounds louder."#.to_string(),
        inputs: BTreeMap::from([
            (
                "amplification".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "amplification".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: MAX_FREQ,
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
            let cloned = Amplify::new(
                props.clone_sound(props.get_source("audio 1")?)?,
                props.get_float("amplification")?,
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(cloned)),
                },
            )]))
        })),
    }
}
