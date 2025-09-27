use crate::constants::WAVE_TABLE_SIZE;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::tracker::{Tracker, TrackerNote};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn tracker_node() -> SoundNode {
    SoundNode {
        name: "Tracker".to_string(),
        tooltip: r#"Allows you to play notes at given speeds."#.to_string(),
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
                "tracker".to_string(),
                InputParameter {
                    data_type: DataType::TrackerNotes,
                    kind: InputParamKind::ConstantOnly,
                    name: "tracker".to_string(),
                    value: InputValueConfig::TrackerNotes {
                        notes: vec![TrackerNote::default()],
                    },
                },
            ),
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

pub fn tracker_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(Tracker::new(
                props.sample_rate(),
                props.bpm(),
                props.get_tracker("tracker")?,
                cloned,
                props.note_speed(),
                props
                    .get_graph("graph")?
                    .unwrap_or(vec![0.0; WAVE_TABLE_SIZE]),
            ))),
        },
    )]))
}
