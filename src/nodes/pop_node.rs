use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::{AsGenericSource, Noise, Pop};
use egui_node_graph_2::InputParamKind;
use std::collections::HashMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn pop_node() -> SoundNode {
    SoundNode {
        name: "Pop".to_string(),
        inputs: HashMap::from([(
            "amplitude".to_string(),
            InputParameter {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "amplitude".to_string(),
                value: InputValueConfig::Float { value: 1.0 },
            },
        )]),
        outputs: HashMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}
pub fn pop_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(HashMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_queue::push_sound(
                Pop::new(props.get_float("amplitude")?).as_generic(None),
            ),
        },
    )]))
}
