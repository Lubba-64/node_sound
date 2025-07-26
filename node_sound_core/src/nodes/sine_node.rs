use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use rodio::source::SineWave;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn sine_node() -> SoundNode {
    SoundNode {
        name: "Sine Wave".to_string(),
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

pub fn sine_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(SineWave::new(props.get_float("frequency")?))),
        },
    )]))
}
