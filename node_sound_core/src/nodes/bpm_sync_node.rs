use crate::constants::WAVE_TABLE_SIZE;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_graph::note::NoteSpeed;
use crate::sounds::bpm_sync::BPMSync;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::str::FromStr;

use super::{SoundNodeProps, SoundNodeResult};

pub fn bpm_sync_node() -> SoundNode {
    SoundNode {
        name: "BPM Sync".to_string(),
        tooltip:
            r#"Syncs with the BPM to modify a waveform's amplitude with a wavetable every note.
        Note speed can be controlled with a dropdown of options."#
                .to_string(),
        inputs: BTreeMap::from([
            (
                "graph".to_string(),
                InputParameter {
                    data_type: DataType::Graph,
                    kind: InputParamKind::ConstantOnly,
                    name: "graph".to_string(),
                    value: InputValueConfig::Graph {
                        value: vec![0.01; WAVE_TABLE_SIZE],
                        height: 100.0,
                        width: 300.0,
                    },
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
                "note speed".to_string(),
                InputParameter {
                    data_type: DataType::Dropdown,
                    kind: InputParamKind::ConstantOnly,
                    name: "note speed".to_string(),
                    value: InputValueConfig::Dropdown {
                        value: NoteSpeed::Quarter.to_string(),
                        values: NoteSpeed::ALL.map(|x| x.to_string()).to_vec(),
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
pub fn bpm_sync_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    let speed = NoteSpeed::from_str(&props.get_dropdown("note speed")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(BPMSync::new(
                cloned,
                props.sample_rate(),
                props.bpm(),
                speed,
                props
                    .get_graph("graph")?
                    .unwrap_or(vec![0.0; WAVE_TABLE_SIZE]),
                props.note_speed(),
            ))),
        },
    )]))
}
