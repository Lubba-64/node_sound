use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map;
use crate::sounds::TranslateWave;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn translate_node() -> SoundNode {
    SoundNode {
        name: "Translate Wave".to_string(),
        inputs: BTreeMap::from([
            (
                "start_min".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "start_min".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        max: 4000.0,
                        min: -4000.0,
                    },
                },
            ),
            (
                "start_max".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "start_max".to_string(),
                    value: InputValueConfig::Float {
                        value: -1.0,
                        max: 4000.0,
                        min: -4000.0,
                    },
                },
            ),
            (
                "end_min".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "end_min".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        max: 4000.0,
                        min: -4000.0,
                    },
                },
            ),
            (
                "end_max".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "end_max".to_string(),
                    value: InputValueConfig::Float {
                        value: -1.0,
                        max: 4000.0,
                        min: -4000.0,
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
pub fn translate_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound(Box::new(TranslateWave::new(
                sound_map::clone_sound_ref(props.get_source("audio 1")?)?,
                props.get_float("start_min")?,
                props.get_float("start_max")?,
                props.get_float("end_min")?,
                props.get_float("end_max")?,
            ))),
        },
    )]))
}
