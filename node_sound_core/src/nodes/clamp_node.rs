use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map;
use crate::sounds::Clamp;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn clamp_node() -> SoundNode {
    SoundNode {
        name: "Clamp".to_string(),
        inputs: BTreeMap::from([
            (
                "min".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "min".to_string(),
                    value: InputValueConfig::Float {
                        value: -1.0,
                        max: 1.0,
                        min: -1.0,
                    },
                },
            ),
            (
                "max".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "max".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        max: 1.0,
                        min: -1.0,
                    },
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
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}
pub fn clamp_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound(Box::new(Clamp::new(
                sound_map::clone_sound_ref(props.get_source("audio 1")?)?,
                Some(props.get_float("min")?),
                Some(props.get_float("max")?),
            ))),
        },
    )]))
}
