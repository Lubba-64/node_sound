use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct AutomatedDuration<I: SoundNode, D: SoundNode> {
    source: I,
    duration: D,
    sample_rate: f32,
    speed: f32,
}

impl<S: SoundNode, D: SoundNode> AutomatedDuration<S, D> {
    pub fn new(duration: D, source: S, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<I: SoundNode + Clone, D: SoundNode + Clone> SoundNode for AutomatedDuration<I, D> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index / self.speed
            > self.sample_rate * self.duration.next(index, channel).unwrap_or_default()
        {
            None
        } else {
            self.source.next(index, channel)
        }
    }
}

pub fn automated_duration_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Take Duration".to_string(),
        tooltip: r#"Takes a snapshot of the waveform for the amount of time you input."#
            .to_string(),
        inputs: vec![
            Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "duration".to_string(),
                value: InputValueConfig::AudioSource {},
            },
            Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 1".to_string(),
                value: InputValueConfig::AudioSource {},
            },
            Input {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "note independant".to_string(),
                value: InputValueConfig::Bool { value: false },
            },
        ],
        outputs: get_default_outputs(),
        op: Some(Arc::new(|mut props| {
            let duration = AutomatedDuration::new(
                props.clone_sound(props.get_source("duration")?)?,
                props.clone_sound(props.get_source("audio 1")?)?,
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
