use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::{AdvancedDelay, AsGenericSource};
use egui_node_graph_2::InputParamKind;
use std::collections::HashMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn advanced_delay_node() -> SoundNode {
    SoundNode {
        name: "Advanced Delay".to_string(),
        inputs: HashMap::from([
            (
                "delays".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "amplification".to_string(),
                    value: InputValueConfig::Float { value: 1.0 },
                },
            ),
            (
                "falloff".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "amplification".to_string(),
                    value: InputValueConfig::Float { value: 1.0 },
                },
            ),
            (
                "attenuation".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "amplification".to_string(),
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
pub fn advanced_delay_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(HashMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_queue::push_sound(
                AdvancedDelay::new(
                    sound_queue::clone_sound(props.get_source("audio 1")?)?,
                    props.get_float("delays")?.round() as u32,
                    props.get_float("attenuation")?,
                    props.get_float("falloff")?,
                )
                .as_generic(None),
            ),
        },
    )]))
}
