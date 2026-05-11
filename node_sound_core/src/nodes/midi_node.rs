use crate::error::NodeSoundError;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::midi::MidiRenderer;
use anyhow::anyhow;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn midi_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Midi File".to_string(),
        tooltip: r#"Imports and plays a midi file with the given waveform."#.to_string(),
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

pub fn midi_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    props.update_wavetables_node_idx();
    let file = props.get_midi("file")?;
    if file.is_none() {
        return Ok(BTreeMap::from([(
            "out".to_string(),
            ValueType::AudioSource { value: 0 },
        )]));
    }
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    let midi = MidiRenderer::new(
        cloned,
        file.ok_or::<NodeSoundError>(anyhow!("midi file is missing").into())?
            .1,
        props.get_bool("note independant")?,
        props.note_speed(),
        props.sample_rate(),
        &mut props.state.user_state.wavetables,
    );
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(midi)),
        },
    )]))
}
