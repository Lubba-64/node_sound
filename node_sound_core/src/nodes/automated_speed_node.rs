use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::automated_speed::AutomatedSpeed;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_speed_node() -> SoundNode {
    SoundNode {
        name: "Automated Speed".to_string(),
        tooltip: r#"Changes the speed of the input waveform based off of the base frequency to the automation value."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "base frequency".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "base frequency".to_string(),
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
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "frequency".to_string(),
                    value: InputValueConfig::AudioSource {},
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

pub fn automated_speed_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = AutomatedSpeed::new(
        props.clone_sound(props.get_source("audio 1")?)?,
        props.get_float("base frequency")?,
        props.clone_sound(props.get_source("frequency")?)?,
        props.sample_rate(),
    );
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(cloned)),
        },
    )]))
}
