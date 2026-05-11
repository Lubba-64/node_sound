use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AutomatedDelayRepeat<I: SoundNode, B: SoundNode, P: SoundNode> {
    source: I,
    buffer_size_source: B,
    points_source: P,
    sample_rate: f32,
    deque: [VecDeque<f32>; 2],
}

impl<I: SoundNode, B: SoundNode, P: SoundNode> AutomatedDelayRepeat<I, B, P> {
    pub fn new(source: I, buffer_size_source: B, points_source: P, sample_rate: f32) -> Self {
        Self {
            source,
            buffer_size_source,
            points_source,
            sample_rate,
            deque: [VecDeque::new(), VecDeque::new()],
        }
    }

    fn update_buffer_size(&mut self, index: f32) -> usize {
        let buffer_size = self.buffer_size_source.next(index, 0).unwrap_or(0.0);
        (buffer_size.max(0.0) as usize).max(1) * self.sample_rate as usize
    }

    fn get_points(&mut self, index: f32) -> usize {
        let points = self.points_source.next(index, 0).unwrap_or(0.0);
        (points.max(0.0) as usize).max(1)
    }
}

impl<I: SoundNode + Clone, B: SoundNode + Clone, P: SoundNode + Clone> SoundNode
    for AutomatedDelayRepeat<I, B, P>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let buffer_size = self.update_buffer_size(index);
        let points = self.get_points(index);
        let deque = &mut self.deque[channel as usize];
        if deque.len() != buffer_size {
            deque.resize(buffer_size, 0.0);
        }
        if deque.len() > 0 {
            deque.pop_back();
            deque.push_front(self.source.next(index, channel).unwrap_or_default());
        }
        if deque.is_empty() {
            return Some(0.0);
        }
        let total_size = deque.len();
        Some(
            (0..points)
                .map(|point| {
                    let read_index = if point == 0 {
                        0
                    } else {
                        (total_size / points) * point
                    }
                    .min(total_size - 1);
                    deque[read_index] / (points as f32 / (points - point) as f32)
                })
                .sum::<f32>(),
        )
    }
}

pub fn automated_delay_repeat_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Delay Repeat".to_string(),
        tooltip: r#"Automated version of the delay repeat node which acts like a delay plugin."#
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
                "points".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "points".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "delay".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "delay".to_string(),
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
            let cloned2 = props.clone_sound(props.get_source("delay")?)?;
            let cloned3 = props.clone_sound(props.get_source("points")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(AutomatedDelayRepeat::new(
                        cloned1,
                        cloned2,
                        cloned3,
                        props.sample_rate(),
                    ))),
                },
            )]))
        })),
    }
}
