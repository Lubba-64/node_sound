use super::{SoundNodeProps, SoundNodeResult};
use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::ConstWave;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

pub fn const_node() -> SoundNode {
    SoundNode {
        name: "Const".to_string(),
        inputs: BTreeMap::from([(
            "amplitude".to_string(),
            InputParameter {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "amplitude".to_string(),
                value: InputValueConfig::Float {
                    value: 1.0,
                    min: -MAX_FREQ,
                    max: MAX_FREQ,
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
pub fn const_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(ConstWave::new(props.get_float("amplitude")?))),
        },
    )]))
}
