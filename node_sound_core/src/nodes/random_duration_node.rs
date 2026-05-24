use crate::node_prelude::*;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct RandomDuration<I: SoundNode> {
    source: I,
    duration_min: f32,
    duration_max: f32,
    duration: f32,
    sample_rate: f32,
    speed: f32,
    last_index: f32,
}

impl<I: SoundNode + Clone> RandomDuration<I> {
    #[inline]
    pub fn new(
        source: I,
        duration_min: f32,
        duration_max: f32,
        uses_speed: bool,
        sample_rate: f32,
        speed: f32,
    ) -> Self {
        let mut _self = Self {
            source,
            duration_min,
            duration_max,
            sample_rate,
            speed: if uses_speed { speed } else { 1.0 },
            duration: duration_min,
            last_index: 0.0,
        };
        _self.next_duration();
        _self
    }

    fn next_duration(&mut self) {
        self.duration = if self.duration_min == self.duration_max {
            self.duration_min
        } else {
            rand::thread_rng().gen_range(self.duration_min..self.duration_max)
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for RandomDuration<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index / self.speed < self.last_index {
            self.next_duration();
        }
        self.last_index = index;
        if index / self.speed > self.sample_rate * self.duration * self.speed {
            None
        } else {
            self.source.next(index, channel)
        }
    }
}

pub fn random_duration_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Random Take Duration".to_string(),
        tooltip: r#"Takes a snapshot of the waveform for the amount of time you input.
        The Random Take Duration node does this as a random number from min duration to max duration."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "min duration".to_string(),
                Input {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "min duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
                },
            ),
                        (
                "max duration".to_string(),
                Input {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "max duration".to_string(),
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
        op: Some(Arc::new(|mut props|{
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(RandomDuration::new(
                        cloned,
                        props.get_duration("min duration")?.as_secs_f32(),
                        props.get_duration("max duration")?.as_secs_f32(),
                        props.get_bool("note independant")?,
                        props.sample_rate(),
                        props.note_speed(),
                    ))),
                },
            )]))
        }))
    }
}
