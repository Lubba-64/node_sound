use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::AsGenericSource;
use egui_node_graph_2::InputParamKind;
use rodio::Source;
use std::collections::HashMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn speed_node() -> SoundNode {
    SoundNode {
        name: "Speed".to_string(),
        inputs: HashMap::from([
            (
                "speed".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "speed".to_string(),
                    value: InputValueConfig::Float { value: 1.0 },
                },
            ),
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
        ]),
        outputs: HashMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}
pub fn speed_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(HashMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_queue::push_sound(
                sound_queue::clone_sound(props.get_source("audio 1")?)?
                    .speed(props.get_float("speed")?)
                    .as_generic(None),
            ),
        },
    )]))
}
