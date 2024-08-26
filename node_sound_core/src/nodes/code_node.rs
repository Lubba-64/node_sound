use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map;
use crate::sounds::CodeSource;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn code_node() -> SoundNode {
    SoundNode {
        name: "Code".to_string(),
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
                "audio 2".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 2".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "audio 3".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 3".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "audio 4".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 4".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "audio 5".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 5".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "code".to_string(),
                InputParameter {
                    data_type: DataType::Code,
                    kind: InputParamKind::ConnectionOnly,
                    name: "code".to_string(),
                    value: InputValueConfig::Code {
                        value: "
pub fn process(input1, input2, input3, input4, input5){
    (
    input1.unwrap_or(0.0) + 
    input2.unwrap_or(0.0) + 
    input3.unwrap_or(0.0) + 
    input4.unwrap_or(0.0) + 
    input5.unwrap_or(0.0)
    ) 
    / 5.0
}"
                        .to_string(),
                    },
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
pub fn code_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound(Box::new(CodeSource::new(
                sound_map::clone_sound_ref(props.get_source("audio 1")?)?,
                sound_map::clone_sound_ref(props.get_source("audio 2")?)?,
                sound_map::clone_sound_ref(props.get_source("audio 3")?)?,
                sound_map::clone_sound_ref(props.get_source("audio 4")?)?,
                sound_map::clone_sound_ref(props.get_source("audio 5")?)?,
                props.get_code("code")?,
            )?)),
        },
    )]))
}
