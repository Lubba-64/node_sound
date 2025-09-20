use super::{SoundNodeProps, SoundNodeResult};
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::random_duration::RandomDuration;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

pub fn random_duration_node() -> SoundNode {
    SoundNode {
        name: "Random Take Duration".to_string(),
        tooltip: r#"Takes a snapshot of the waveform for the amount of time you input.
        The Random Take Duration node does this as a random number from min duration to max duration."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "min duration".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "min duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
                },
            ),
                        (
                "max duration".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "max duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
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

pub fn random_duration_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(RandomDuration::new(
                cloned,
                props.get_duration("min duration")?.as_secs_f32(),
                props.get_duration("max duration")?.as_secs_f32(),
                props.get_bool("note independant")?,
                props.sample_rate(),
                props.note_speed(),
            ))),
        },
    )]))
}
