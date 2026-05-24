use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Duration<I: SoundNode> {
    source: I,
    duration: f32,
    sample_rate: f32,
    speed: f32,
}

impl<S: SoundNode> Duration<S> {
    pub fn new(duration: f32, source: S, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Duration<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index / self.speed > self.sample_rate * self.duration {
            None
        } else {
            self.source.next(index, channel)
        }
    }
}

pub fn duration_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Take Duration".to_string(),
        tooltip: r#"Takes a snapshot of the waveform for the amount of time you input."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "duration".to_string(),
                Input {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
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
            let duration = Duration::new(
                props.get_duration("duration")?.as_secs_f32(),
                cloned,
                props.get_bool("note independant")?,
                props.note_speed(),
                props.sample_rate(),
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(duration)),
                },
            )]))
        })),
    }
}
