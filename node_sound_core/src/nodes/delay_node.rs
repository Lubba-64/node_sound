use super::{SoundNodeProps, SoundNodeResult};
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::delay::Delay;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

pub fn delay_node() -> SoundNode {
    SoundNode {
        name: "Delay".to_string(),
        tooltip: r#"Delays the given waveform by an amount of time."#.to_string(),
        inputs: BTreeMap::from([
            (
                "delay".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
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

pub fn delay_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = Delay::new(
        props.get_duration("delay")?.as_secs_f32(),
        props.clone_sound(props.get_source("audio 1")?)?,
        props.get_bool("note independant")?,
        props.note_speed(),
        props.sample_rate(),
    );
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(cloned)),
        },
    )]))
}
