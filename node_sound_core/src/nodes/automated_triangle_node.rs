use crate::nodes::SoundNodeMetadata;
use crate::nodes::automated_speed_node::AutomatedSpeed;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::{node::SoundNode, nodes::triangle_node::TriangleWave};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AutomatedTriangleWave<F: SoundNode> {
    freq_source: AutomatedSpeed<TriangleWave, F>,
    speed: f32,
}

impl<F: SoundNode> AutomatedTriangleWave<F> {
    #[inline]
    pub fn new(freq_source: F, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            freq_source: AutomatedSpeed::new(
                TriangleWave::new(1.0, false, sample_rate, 1.0),
                1.0,
                freq_source,
            ),
            speed: if uses_speed { speed } else { 1.0 },
        }
    }
}

impl<F: SoundNode + Clone> SoundNode for AutomatedTriangleWave<F> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index /= self.speed;
        self.freq_source.next(index, channel)
    }
}

pub fn automated_triangle_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Triangle Wave".to_string(),
        tooltip: r#"Automated version of the Triangle node.
Automates the frequency with a given waveform.
Use TranslateWave to set the frequency values of the automation,
by setting the end min and end max to your desired frequency values."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "freq".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "freq".to_string(),
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
            let cloned = props.clone_sound(props.get_source("freq")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(AutomatedTriangleWave::new(
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
