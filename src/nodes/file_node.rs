use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map;
use crate::sounds::CloneableDecoder;
use egui_node_graph_2::InputParamKind;
use rodio::{Decoder, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use super::{SoundNodeProps, SoundNodeResult};

pub fn file_node() -> SoundNode {
    SoundNode {
        name: "Audio File".to_string(),
        inputs: HashMap::from([(
            "file".to_string(),
            InputParameter {
                data_type: DataType::File,
                kind: InputParamKind::ConstantOnly,
                name: "file".to_string(),
                value: InputValueConfig::File { value: None },
            },
        )]),
        outputs: HashMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}

pub fn file_logic(props: SoundNodeProps) -> SoundNodeResult {
    let file = props.get_file("file")?;
    if file.is_none() {
        return Ok(HashMap::from([(
            "out".to_string(),
            ValueType::AudioSource { value: 0 },
        )]));
    }

    Ok(HashMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<CloneableDecoder>(Box::new(CloneableDecoder {
                path: file.clone().unwrap(),
                decoder: Decoder::new(BufReader::new(File::open(&file.unwrap())?))?
                    .convert_samples::<f32>(),
            })),
        },
    )]))
}
