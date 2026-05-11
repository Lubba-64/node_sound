use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct DawAutomationChannel {
    channel: Arc<Mutex<f32>>,
}

impl DawAutomationChannel {
    #[inline]
    pub fn new(channel: Arc<Mutex<f32>>) -> Self {
        Self { channel }
    }
}

impl SoundNode for DawAutomationChannel {
    fn next(&mut self, _index: f32, _channel: u8) -> Option<f32> {
        Some(*self.channel.lock().ok()?)
    }
}

pub fn daw_automation_source_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Daw Automations".to_string(),
        tooltip: r#"Daw automation parameters 1-18 can be accessed through this node."#.to_string(),
        inputs: BTreeMap::from([(
            "channel".to_string(),
            InputParameter {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOnly,
                name: "channel".to_string(),
                value: InputValueConfig::Float {
                    value: 0.0,
                    min: 0.0,
                    max: 17.0,
                },
            },
        )]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
        op: Some(Arc::new(|mut props| {
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(DawAutomationChannel::new(
                        props.state.runtime_state.automations.0
                            [(props.get_float("channel")?.round() as usize).clamp(0, 17)]
                        .clone(),
                    ))),
                },
            )]))
        })),
    }
}
