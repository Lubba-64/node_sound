use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use rodio::Source;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn repeat_infinite_node() -> SoundNode {
    SoundNode {
        name: "Repeat Infinite".to_string(),
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

pub fn repeat_infinite_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props
        .clone_sound_ref(props.get_source("audio 1")?)?
        .repeat_infinite();

    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(cloned)),
        },
    )]))
}
