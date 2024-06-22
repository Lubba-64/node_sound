use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map::{self, RefSource};
use crate::sounds::AutomatedSawToothWave;
use egui_node_graph_2::InputParamKind;
use std::collections::HashMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_sawtooth_node() -> SoundNode {
    SoundNode {
        name: "Automated Sawtooth Wave".to_string(),
        inputs: HashMap::from([(
            "freq".to_string(),
            InputParameter {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "freq".to_string(),
                value: InputValueConfig::AudioSource {},
            },
        )]),
        outputs: HashMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}
pub fn automated_sawtooth_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(HashMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<AutomatedSawToothWave<RefSource>>(Box::new(
                AutomatedSawToothWave::new(sound_map::clone_sound(props.get_source("freq")?)?),
            )),
        },
    )]))
}
