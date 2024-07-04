use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map;
use crate::sounds::MidiRenderer;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn midi_node() -> SoundNode {
    SoundNode {
        name: "Midi File".to_string(),
        inputs: BTreeMap::from([
            (
                "file".to_string(),
                InputParameter {
                    data_type: DataType::MidiFile,
                    kind: InputParamKind::ConstantOnly,
                    name: "file".to_string(),
                    value: InputValueConfig::MidiFile {},
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

pub fn midi_logic(props: SoundNodeProps) -> SoundNodeResult {
    let file = props.get_midi("file")?;
    if file.is_none() {
        return Ok(BTreeMap::from([(
            "out".to_string(),
            ValueType::AudioSource { value: 0 },
        )]));
    }

    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<MidiRenderer>(Box::new(MidiRenderer::new(
                sound_map::clone_sound(props.get_source("audio 1")?)?,
                file.unwrap().1,
            ))),
        },
    )]))
}
