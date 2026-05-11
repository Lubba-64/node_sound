use crate::constants::MAX_FREQ;
use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ConstWave {
    val: f32,
}

impl ConstWave {
    #[inline]
    pub fn new(val: f32) -> Self {
        Self { val }
    }
}

impl SoundNode for ConstWave {
    fn next(&mut self, _index: f32, _channel: u8) -> Option<f32> {
        Some(self.val)
    }
}

pub fn const_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Const".to_string(),
        tooltip: r#"A waveform that just sits at a given value forever."#.to_string(),
        inputs: BTreeMap::from([(
            "amplitude".to_string(),
            InputParameter {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "amplitude".to_string(),
                value: InputValueConfig::Float {
                    value: 1.0,
                    min: -MAX_FREQ,
                    max: MAX_FREQ,
                },
            },
        )]),
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
                    value: props
                        .push_sound(Box::new(ConstWave::new(props.get_float("amplitude")?))),
                },
            )]))
        })),
    }
}
