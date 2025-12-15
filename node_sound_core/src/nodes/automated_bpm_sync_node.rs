use crate::constants::WAVE_TABLE_SIZE;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_graph::note::NoteSpeedType;
use crate::sounds::automated_bpm_sync::AutomatedBPMSync;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::str::FromStr;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_bpm_sync_node() -> SoundNode {
    SoundNode {
        name: "Automated BPM Sync".to_string(),
        tooltip: r#"Syncs a wavetable to each note. 
The automation values for Any is 0-21, 0-7 for the rest which corresponds to the automation input value."#
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
                "note speed".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "note speed".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "note speed type".to_string(),
                InputParameter {
                    data_type: DataType::Dropdown,
                    kind: InputParamKind::ConstantOnly,
                    name: "note speeds".to_string(),
                    value: InputValueConfig::Dropdown {
                        value: NoteSpeedType::Normal.to_string(),
                        values: NoteSpeedType::ALL.map(|x| x.to_string()).to_vec(),
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
pub fn automated_bpm_sync_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let speed = NoteSpeedType::from_str(&props.get_dropdown("note speed type")?)?;
    let cloned = props.clone_sound(props.get_source("note speed")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(AutomatedBPMSync::new(
                props.sample_rate(),
                props.bpm(),
                cloned,
                speed,
                props
                    .get_graph("graph")?
                    .unwrap_or(vec![0.0; WAVE_TABLE_SIZE]),
                props.note_speed(),
            ))),
        },
    )]))
}
