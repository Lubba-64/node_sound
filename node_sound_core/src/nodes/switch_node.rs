use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::switch::Switch;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn switch_node() -> SoundNode {
    SoundNode {
        name: "Switch".to_string(),
        tooltip: r#"Uses the switch channel to switch between two sources."#.to_string(),
        inputs: BTreeMap::from([
            (
                "switch".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "switch".to_string(),
                    value: InputValueConfig::AudioSource {},
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
            (
                "audio 2".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 2".to_string(),
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
pub fn switch_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned1 = props.clone_sound(props.get_source("audio 1")?)?;
    let cloned2 = props.clone_sound(props.get_source("audio 2")?)?;
    let switch = props.clone_sound(props.get_source("switch")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(Switch::new(cloned1, cloned2, switch))),
        },
    )]))
}
