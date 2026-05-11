use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct AutomatedSpeed<I: SoundNode, I2: SoundNode> {
    source: I,
    base_freq: f32,
    freq: I2,
    last_index: f32,
    adjusted_index: f32,
}

impl<I: SoundNode, I2: SoundNode> AutomatedSpeed<I, I2> {
    pub fn new(source: I, base_freq: f32, freq: I2) -> Self {
        Self {
            source,
            base_freq,
            freq,
            last_index: 0.0,
            adjusted_index: 0.0,
        }
    }
}

impl<I: SoundNode + Clone, I2: SoundNode + Clone> SoundNode for AutomatedSpeed<I, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.adjusted_index += (index - self.last_index)
            * (self.freq.next(index, channel).unwrap_or(0.0) / self.base_freq);
        self.last_index = index;
        self.source.next(self.adjusted_index, channel)
    }
}

pub fn automated_speed_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Speed".to_string(),
        tooltip: r#"Changes the speed of the input waveform based off of the base frequency to the automation value."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "base frequency".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "base frequency".to_string(),
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
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "frequency".to_string(),
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
        op: Some(Arc::new(|mut props|{
            let cloned = AutomatedSpeed::new(
                props.clone_sound(props.get_source("audio 1")?)?,
                props.get_float("base frequency")?,
                props.clone_sound(props.get_source("frequency")?)?,
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(cloned)),
                },
            )]))
        }))
    }
}
