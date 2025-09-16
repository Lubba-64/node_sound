use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::amplify::Amplify;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn amplify_node() -> SoundNode {
    SoundNode {
        name: "Amplify".to_string(),
        tooltip: r#"Amplifies the waveform making sounds louder."#.to_string(),
        inputs: BTreeMap::from([
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
pub fn amplify_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = Amplify::new(
        props.clone_sound(props.get_source("audio 1")?)?,
        props.get_float("amplification")?,
    );
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(cloned)),
        },
    )]))
}
