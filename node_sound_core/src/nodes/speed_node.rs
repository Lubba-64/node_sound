use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Speed<I: SoundNode> {
    source: I,
    speed: f32,
}

impl<I: SoundNode> Speed<I> {
    pub fn new(source: I, speed: f32) -> Self {
        Self { source, speed }
    }
}

impl<I: SoundNode + Clone> SoundNode for Speed<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let scaled_index = index * self.speed;
        self.source.next(scaled_index, channel)
    }
}

pub fn speed_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Speed".to_string(),
        tooltip: r#"Changes the speed of the input waveform."#.to_string(),
        inputs: BTreeMap::from([
            (
                "speed".to_string(),
                Input {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "speed".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: MAX_FREQ,
                    },
                },
            ),
            (
                "audio 1".to_string(),
                Input {
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
            let cloned = Speed::new(
                props.clone_sound(props.get_source("audio 1")?)?,
                props.get_float("speed")?,
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
