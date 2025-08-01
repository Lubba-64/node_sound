use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
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

pub fn midi_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let file = props.get_midi("file")?;
    if file.is_none() {
        return Ok(BTreeMap::from([(
            "out".to_string(),
            ValueType::AudioSource { value: 0 },
        )]));
    }
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(MidiRenderer::new(cloned, file.unwrap().1))),
        },
    )]))
}
