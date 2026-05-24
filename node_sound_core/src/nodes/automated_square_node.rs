use crate::{
    node_prelude::*,
    nodes::{automated_speed_node::AutomatedSpeed, square_node::SquareWave},
};

#[derive(Clone, Debug)]
pub struct AutomatedSquareWave<F: SoundNode> {
    freq_source: AutomatedSpeed<SquareWave, F>,
    speed: f32,
}

impl<F: SoundNode> AutomatedSquareWave<F> {
    #[inline]
    pub fn new(freq_source: F, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            freq_source: AutomatedSpeed::new(
                SquareWave::new(1.0, false, sample_rate, 1.0),
                1.0,
                freq_source,
            ),
            speed: if uses_speed { speed } else { 1.0 },
        }
    }
}

impl<F: SoundNode + Clone> SoundNode for AutomatedSquareWave<F> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        self.freq_source.next(index, channel)
    }
}

pub fn automated_square_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Square Wave".to_string(),
        tooltip: r#"Automated version of the Square node.
Automates the frequency with a given waveform.
Use TranslateWave to set the frequency values of the automation,
by setting the end min and end max to your desired frequency values."#
            .to_string(),
        inputs: vec![
            Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "freq".to_string(),
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
            let cloned = props.clone_sound(props.get_source("freq")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(AutomatedSquareWave::new(
                        cloned,
                        props.get_bool("note independant")?,
                        props.note_speed(),
                        props.sample_rate(),
                    ))),
                },
            )]))
        })),
    }
}
