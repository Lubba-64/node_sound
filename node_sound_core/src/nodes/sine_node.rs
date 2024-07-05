use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map;
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
                    min: 17.0,
                    max: 4000.0,
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
pub fn sine_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<SineWave>(Box::new(SineWave::new(
                props.get_float("frequency")?,
            ))),
        },
    )]))
}
