use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::AutomatedTranslateWave;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_translate_node() -> SoundNode {
    SoundNode {
        name: "Automated Translate Wave".to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "start_max".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "start_max".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "start_min".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "start_min".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "end_max".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "end_max".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "end_min".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "end_min".to_string(),
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

pub fn automated_translate_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned1 = props.clone_sound(props.get_source("start_min")?)?;
    let cloned2 = props.clone_sound(props.get_source("start_max")?)?;
    let cloned3 = props.clone_sound(props.get_source("end_min")?)?;
    let cloned4 = props.clone_sound(props.get_source("end_max")?)?;
    let cloned5 = props.clone_sound(props.get_source("audio 1")?)?;

    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(AutomatedTranslateWave::new(
                cloned1, cloned2, cloned3, cloned4, cloned5,
            ))),
        },
    )]))
}
