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
pub struct DelayRepeat<I: SoundNode> {
    source: I,
    delay: f32,
    points: usize,
    sample_rate: f32,
    deque: [VecDeque<f32>; 2],
}

impl<I: SoundNode> DelayRepeat<I> {
    pub fn new(source: I, delay: f32, sample_rate: f32, points: usize) -> Self {
        Self {
            source,
            delay,
            points,
            sample_rate,
            deque: [
                vec![0.0; (sample_rate * delay) as usize].into(),
                vec![0.0; (sample_rate * delay) as usize].into(),
            ],
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for DelayRepeat<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let deque = &mut self.deque[channel as usize];
        deque.pop_back();
        deque.push_front(self.source.next(index, channel).unwrap_or_default());
        let total_size = (self.sample_rate * self.delay) as usize - 100;
        Some(
            (0..self.points)
                .into_iter()
                .map(|point| {
                    let read_index = if point == 0 {
                        0
                    } else {
                        (total_size / self.points) * point
                    }
                    .min(total_size - 1);
                    deque[read_index] / (self.points as f32 / (self.points - point) as f32)
                })
                .sum::<f32>(),
        )
    }
}

pub fn delay_repeat_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
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
        op: Some(Arc::new(|mut props| {
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            let delay = DelayRepeat::new(
                cloned,
                props.get_float("delay")?,
                props.sample_rate(),
                props.get_float("points")? as usize,
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(delay)),
                },
            )]))
        })),
    }
}
