use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{DataType, Output, ValueType};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct InputChannel {
    channel: Arc<Mutex<(f32, f32)>>,
}

impl InputChannel {
    #[inline]
    pub fn new(channel: Arc<Mutex<(f32, f32)>>) -> Self {
        Self { channel }
    }
}

impl SoundNode for InputChannel {
    fn next(&mut self, _index: f32, channel: u8) -> Option<f32> {
        match self.channel.lock() {
            Err(_) => None,
            Ok(sample) => Some(if channel == 0 { sample.0 } else { sample.1 }),
        }
    }
}

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
