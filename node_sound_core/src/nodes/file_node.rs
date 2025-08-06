use super::{SoundNodeProps, SoundNodeResult};
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::CloneableDecoder;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

pub fn file_node() -> SoundNode {
    SoundNode {
        name: "Audio File".to_string(),
        inputs: BTreeMap::from([
            (
                "file".to_string(),
                InputParameter {
                    data_type: DataType::AudioFile,
                    kind: InputParamKind::ConstantOnly,
                    name: "file".to_string(),
                    value: InputValueConfig::AudioFile {},
                },
            ),
            (
                "note independant".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "note independant".to_string(),
                    value: InputValueConfig::Bool { value: false },
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

pub fn file_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let (name, data) = match props.get_file("file")? {
        None => {
            return Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource { value: 0 },
            )]));
        }
        Some(x) => x,
    };

    props
        .state
        .user_state
        .file_database
        .add_sample(name.clone(), data);

    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(
                match CloneableDecoder::new(
                    &props.state.user_state.file_database,
                    name,
                    props.get_float("note independant")? != 0.0,
                ) {
                    None => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "Sample not found in database after adding.",
                        )));
                    }
                    Some(x) => x,
                },
            )),
        },
    )]))
}
