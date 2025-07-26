use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::SawToothWave;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn sawtooth_node() -> SoundNode {
    SoundNode {
        name: "Sawtooth Wave".to_string(),
        inputs: BTreeMap::from([(
            "frequency".to_string(),
            InputParameter {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "frequency".to_string(),
                value: InputValueConfig::Float {
                    value: 0.0,
                    min: 0.0,
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
pub fn sawtooth_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(SawToothWave::new(props.get_float("frequency")?))),
        },
    )]))
}
