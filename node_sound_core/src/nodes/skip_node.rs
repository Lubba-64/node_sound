use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Skip<S: SoundNode> {
    duration: f32,
    source: S,
    sample_rate: f32,
    speed: f32,
}

impl<S: SoundNode> Skip<S> {
    pub fn new(duration: f32, source: S, uses_speed: bool, sample_rate: f32, speed: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<S: SoundNode + Clone> SoundNode for Skip<S> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index += self.duration * self.speed * self.sample_rate;
        self.source.next(index, channel)
    }
}

pub fn skip_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Skip".to_string(),
        tooltip: r#"Skips samples in the source for a given duration."#.to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConstantOnly,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
                },
            ),
            (
                "note independant".to_string(),
                InputParameter {
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
            let cloned = Skip::new(
                props.get_duration("duration")?.as_secs_f32(),
                props.clone_sound(props.get_source("audio 1")?)?,
                props.get_bool("note independant")?,
                props.sample_rate(),
                props.note_speed(),
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
