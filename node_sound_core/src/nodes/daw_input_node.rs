use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{DataType, Output, ValueType};
use crate::sounds::DawInputChannel;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn daw_input_node() -> SoundNode {
    SoundNode {
        name: "Input".to_string(),
        inputs: BTreeMap::from([]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}

pub fn daw_input_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(DawInputChannel::new())),
        },
    )]))
}
