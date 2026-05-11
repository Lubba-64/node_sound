use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Delay<S: SoundNode> {
    duration: f32,
    source: S,
    sample_rate: f32,
    speed: f32,
}

impl<S: SoundNode> Delay<S> {
    pub fn new(duration: f32, source: S, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<S: SoundNode + Clone> SoundNode for Delay<S> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index > self.duration * self.speed * self.sample_rate {
            self.source.next(index, channel)
        } else {
            Some(0.0)
        }
    }
}

pub fn delay_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Delay".to_string(),
        tooltip: r#"Delays the given waveform by an amount of time."#.to_string(),
        inputs: BTreeMap::from([
            (
                "delay".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
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
            let cloned = Delay::new(
                props.get_duration("delay")?.as_secs_f32(),
                props.clone_sound(props.get_source("audio 1")?)?,
                props.get_bool("note independant")?,
                props.note_speed(),
                props.sample_rate(),
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
