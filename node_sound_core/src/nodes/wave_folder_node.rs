use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::wave_folder::Wavefolder;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn wave_folder_node() -> SoundNode {
    SoundNode {
        name: "Wave Folder".to_string(),
        tooltip: r#"Distortion effect for folding a wave into itself repeatedly."#.to_string(),
        inputs: BTreeMap::from([
            (
                "gain".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "gain".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: 8.0,
                    },
                },
            ),
            (
                "offset".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "offset".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: -2.0,
                        max: 2.0,
                    },
                },
            ),
            (
                "folds".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "folds".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 1.0,
                        max: 8.0,
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

pub fn wave_folder_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(Wavefolder::new(
                cloned,
                props.get_float("gain")?,
                props.get_float("offset")?,
                props.get_float("folds")? as u8,
            ))),
        },
    )]))
}
