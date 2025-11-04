use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::delay_repeat::DelayRepeat;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn delay_repeat_node() -> SoundNode {
    SoundNode {
        name: "DelayRepeat".to_string(),
        tooltip: r#"Acts more like a classic delay plugin"#.to_string(),
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
                "delay".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "delay".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 1.0,
                        max: 100.0,
                    },
                },
            ),
            (
                "points".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "points".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 1.0,
                        max: 10.0,
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
pub fn delay_repeat_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    let delay = DelayRepeat::new(
        cloned,
        props.get_float("delay")?,
        props.sample_rate(),
        props.get_float("points")? as usize,
        &mut props.state._unserializeable_state.buffers,
    );
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(delay)),
        },
    )]))
}
