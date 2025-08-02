use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
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
                        max: MAX_FREQ,
                        min: -MAX_FREQ,
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
                        max: MAX_FREQ,
                        min: -MAX_FREQ,
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
                        max: MAX_FREQ,
                        min: -MAX_FREQ,
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
                        max: MAX_FREQ,
                        min: -MAX_FREQ,
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

pub fn translate_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(TranslateWave::new(
                cloned,
                props.get_float("start_min")?,
                props.get_float("start_max")?,
                props.get_float("end_min")?,
                props.get_float("end_max")?,
            ))),
        },
    )]))
}
