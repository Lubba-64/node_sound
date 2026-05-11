use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::cloneable_decoder::CloneableDecoder;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn file_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Audio File".to_string(),
        tooltip: r#"Imports a wav, flac, or mp3 file as a waveform. Mono audio preferred."#
            .to_string(),
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
        op: Some(Arc::new(|mut props| {
            props.update_wavetables_node_idx();
            let file = match props.get_file("file")? {
                None => {
                    return Ok(BTreeMap::from([(
                        "out".to_string(),
                        ValueType::AudioSource { value: 0 },
                    )]));
                }
                Some(x) => x,
            };
            let decoder = CloneableDecoder::new(
                file.1.clone(),
                props.get_bool("note independant")?,
                props.sample_rate() as u32,
                props.note_speed(),
                &mut props.state.user_state.wavetables,
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(decoder)),
                },
            )]))
        })),
    }
}
