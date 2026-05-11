use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{DataType, Output, ValueType};
use crate::sounds::input::InputChannel;
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn input_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
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
        op: Some(Arc::new(|mut props| {
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(InputChannel::new(
                        props.state.runtime_state.input.0.clone(),
                    ))),
                },
            )]))
        })),
    }
}
