use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{DataType, Output, ValueType};
use crate::sounds::input::InputChannel;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn input_node() -> SoundNode {
    SoundNode {
        name: "Daw Input".to_string(),
        tooltip: r#"Input sound from DAW."#.to_string(),
        inputs: BTreeMap::from([]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}

pub fn input_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(InputChannel::new(
                props.state._unserializeable_state.input.0.clone(),
            ))),
        },
    )]))
}
