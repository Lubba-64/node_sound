use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::reverse::ReverseSource;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn reverse_node() -> SoundNode {
    SoundNode {
        name: "Reverse".to_string(),
        tooltip: r#"Reverses a waveform over a certain duration."#.to_string(),
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
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
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
pub fn reverse_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    props.update_wavetables_node_idx();
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    let source = ReverseSource::new(
        cloned,
        props.get_duration("duration")?.as_secs_f32(),
        props.sample_rate(),
        &mut props.state.user_state.wavetables,
    );
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(source)),
        },
    )]))
}
