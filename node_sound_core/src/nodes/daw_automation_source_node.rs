use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::daw_automation_source::DawAutomationChannel;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn daw_automations_node() -> SoundNode {
    SoundNode {
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
    }
}

pub fn daw_automations_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(DawAutomationChannel::new(
                props.state._unserializeable_state.automations.0
                    [(props.get_float("channel")?.round() as usize).clamp(0, 17)]
                .clone(),
            ))),
        },
    )]))
}
