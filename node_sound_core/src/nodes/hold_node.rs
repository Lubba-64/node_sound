use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Hold<I: SoundNode> {
    source: I,
    hold_length: u32,
    counter: [u32; 2],
    held_value: [f32; 2],
}

impl<I: SoundNode> Hold<I> {
    pub fn new(
        source: I,
        hold_length: f32,
        sample_rate: f32,
        speed: f32,
        uses_speed: bool,
    ) -> Self {
        Self {
            source,
            hold_length: (hold_length / 100.0 * sample_rate * if uses_speed { speed } else { 1.0 })
                .round() as u32,
            counter: [0; 2],
            held_value: [0.0; 2],
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Hold<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let next = self.source.next(index, channel)?;
        let ch = channel as usize;
        self.counter[ch] += 1;
        if self.counter[ch] >= self.hold_length {
            self.counter[ch] = 0;
            self.held_value[ch] = next;
        }
        Some(self.held_value[ch])
    }
}

pub fn hold_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Hold".to_string(),
        tooltip: r#"Holds a sample and repeats it for n seconds"#.to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                Input {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "hold".to_string(),
                Input {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "hold".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: 100.0,
                    },
                },
            ),
            (
                "note independant".to_string(),
                Input {
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
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(Hold::new(
                        cloned,
                        props.get_float("hold")?,
                        props.sample_rate(),
                        props.note_speed(),
                        props.get_bool("note independant")?,
                    ))),
                },
            )]))
        })),
    }
}
