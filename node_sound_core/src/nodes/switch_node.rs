use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Switch<I: SoundNode, I2: SoundNode, I3: SoundNode> {
    source1: I,
    source2: I2,
    switch: I3,
}

impl<I: SoundNode, I2: SoundNode, I3: SoundNode> Switch<I, I2, I3> {
    #[inline]
    pub fn new(source1: I, source2: I2, switch: I3) -> Self {
        Self {
            source1,
            source2,
            switch,
        }
    }
}

impl<I: SoundNode + Clone, I2: SoundNode + Clone, I3: SoundNode + Clone> SoundNode
    for Switch<I, I2, I3>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.switch.next(index, channel).map(|sample| {
            if sample > 0.0 {
                self.source1.next(index, channel).unwrap_or_default()
            } else {
                self.source2.next(index, channel).unwrap_or_default()
            }
        })
    }
}

pub fn switch_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Switch".to_string(),
        tooltip: r#"Uses the switch channel to switch between two sources."#.to_string(),
        inputs: BTreeMap::from([
            (
                "switch".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "switch".to_string(),
                    value: InputValueConfig::AudioSource {},
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
                "audio 2".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 2".to_string(),
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
        op: Some(Arc::new(|mut props| {
            let cloned1 = props.clone_sound(props.get_source("audio 1")?)?;
            let cloned2 = props.clone_sound(props.get_source("audio 2")?)?;
            let switch = props.clone_sound(props.get_source("switch")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(Switch::new(cloned1, cloned2, switch))),
                },
            )]))
        })),
    }
}
