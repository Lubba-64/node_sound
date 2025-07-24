use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map;
use crate::sounds::CloneableDecoder;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn file_node() -> SoundNode {
    SoundNode {
        name: "Audio File".to_string(),
        inputs: BTreeMap::from([(
            "file".to_string(),
            InputParameter {
                data_type: DataType::AudioFile,
                kind: InputParamKind::ConstantOnly,
                name: "file".to_string(),
                value: InputValueConfig::AudioFile {},
            },
        )]),
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
    let file = match props.get_file("file")? {
        None => {
            return Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource { value: 0 },
            )]));
        }
        Some(x) => x,
    };

    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(CloneableDecoder::new(file.1.clone()))),
        },
    )]))
}
