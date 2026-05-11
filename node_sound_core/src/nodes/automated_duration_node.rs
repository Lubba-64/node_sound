use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::automated_duration::AutomatedDuration;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn automated_duration_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Take Duration".to_string(),
        tooltip: r#"Takes a snapshot of the waveform for the amount of time you input."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "duration".to_string(),
                    value: InputValueConfig::AudioSource {},
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
        op: Some(Arc::new(|mut props| {
            let duration = AutomatedDuration::new(
                props.clone_sound(props.get_source("duration")?)?,
                props.clone_sound(props.get_source("audio 1")?)?,
                props.get_bool("note independant")?,
                props.note_speed(),
                props.sample_rate(),
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(duration)),
                },
            )]))
        })),
    }
}
