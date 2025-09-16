use super::{SoundNodeProps, SoundNodeResult};
use crate::constants::DEFAULT_SAMPLE_RATE;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::wave_table::SourceWavetableOscillator;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

pub fn duration_node() -> SoundNode {
    SoundNode {
        name: "Take Duration".to_string(),
        tooltip: r#"Takes a snapshot of the waveform for the amount of time you input."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "duration".to_string(),
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

pub fn duration_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let audio = SourceWavetableOscillator::from_source(
        props.clone_sound(props.get_source("audio 1")?)?,
        DEFAULT_SAMPLE_RATE,
        props.get_duration("duration")?.as_secs_f32(),
        1.0,
        props.get_bool("note independant")?,
    );
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(audio)),
        },
    )]))
}
