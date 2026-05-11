use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AutomatedHold<I: SoundNode, I2: SoundNode> {
    source: I,
    hold_length: I2,
    speed: f32,
    counter: [u32; 2],
    sample_rate: f32,
    held_value: [f32; 2],
}

impl<I: SoundNode, I2: SoundNode> AutomatedHold<I, I2> {
    pub fn new(source: I, hold_length: I2, sample_rate: f32, speed: f32, uses_speed: bool) -> Self {
        Self {
            source,
            hold_length,
            counter: [0; 2],
            sample_rate,
            speed: if uses_speed { speed } else { 1.0 },
            held_value: [0.0; 2],
        }
    }
}

impl<I: SoundNode + Clone, I2: SoundNode + Clone> SoundNode for AutomatedHold<I, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let next = self.source.next(index, channel)?;
        let ch = channel as usize;
        self.counter[ch] += 1;
        if self.counter[ch]
            >= (self.hold_length.next(index, channel).unwrap_or_default() / 100.0
                * self.sample_rate
                * self.speed)
                .round() as u32
        {
            self.counter[ch] = 0;
            self.held_value[ch] = next;
        }
        Some(self.held_value[ch])
    }
}

pub fn automated_hold_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Hold".to_string(),
        tooltip: r#"Automated hold node, Holds a sample and repeats it for n seconds"#.to_string(),
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
                "hold".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "hold".to_string(),
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
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            let hold = props.clone_sound(props.get_source("hold")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(AutomatedHold::new(
                        cloned,
                        hold,
                        props.sample_rate(),
                        props.note_speed(),
                        props.get_bool("note independant")?,
                    ))),
                },
            )]))
        })),
    }
}
