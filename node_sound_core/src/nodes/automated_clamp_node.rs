use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AutomatedClamp<I1: SoundNode, I2: SoundNode, I3: SoundNode> {
    source: I1,
    min: I2,
    max: I3,
}

impl<I1: SoundNode, I2: SoundNode, I3: SoundNode> AutomatedClamp<I1, I2, I3> {
    #[inline]
    pub fn new(source: I1, min: I2, max: I3) -> Self {
        Self { source, max, min }
    }
}

impl<I1: SoundNode + Clone, I2: SoundNode + Clone, I3: SoundNode + Clone> SoundNode
    for AutomatedClamp<I1, I2, I3>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source.next(index, channel),
            self.min.next(index, channel),
            self.max.next(index, channel),
        ) {
            (Some(source), Some(mut min), Some(mut max)) => {
                if min > max {
                    std::mem::swap(&mut min, &mut max);
                }
                Some(source.clamp(min, max))
            }
            _ => None,
        }
    }
}

pub fn automated_clamp_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Clamp Wave".to_string(),
        tooltip: r#"Automated version of the Clamp node.
Clamp will make sure no values go above the maximum or below the minimum.
min and max are waveforms going from -1.0 to 1.0."#
            .to_string(),
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
                "max".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "max".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "min".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "min".to_string(),
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
            let cloned1 = props.clone_sound(props.get_source("min")?)?;
            let cloned2 = props.clone_sound(props.get_source("max")?)?;
            let cloned3 = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props
                        .push_sound(Box::new(AutomatedClamp::new(cloned1, cloned2, cloned3))),
                },
            )]))
        })),
    }
}
