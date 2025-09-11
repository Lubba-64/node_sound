use super::{SoundNodeProps, SoundNodeResult};
use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::amplify::Amplify;
use crate::sounds::delay::Delay;
use crate::sounds::mix::Mix;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

pub fn reverb_node() -> SoundNode {
    SoundNode {
        name: "Reverb".to_string(),
        inputs: BTreeMap::from([
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
                },
            ),
            (
                "amplification".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "amplification".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: MAX_FREQ,
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

pub fn reverb_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = Delay::new(
        props.get_duration("duration")?.as_secs_f32(),
        Amplify::new(
            props.clone_sound(props.get_source("audio 1")?)?,
            props.get_float("amplification")?,
        ),
        props.get_bool("note independant")?,
    );
    let mixed = Mix::new(props.clone_sound(props.get_source("audio 1")?)?, cloned);
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(mixed)),
        },
    )]))
}
