use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn no_op_node() -> SoundNode {
    SoundNode {
        name: "No Op".to_string(),
        tooltip: r#"Does nothing. Connect multiple nodes
to this node to avoid reconnecting a bunch of stuff when you change a node in your graph."#
            .to_string(),
        inputs: BTreeMap::from([(
            "audio 1".to_string(),
            InputParameter {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 1".to_string(),
                value: InputValueConfig::AudioSource {},
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

pub fn no_op_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.get_source("audio 1")?,
        },
    )]))
}
