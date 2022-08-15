use egui_node_graph::InputParamKind;
use egui_node_graph::NodeId;
use rodio::Source;
use slotmap::{self, SecondaryMap, SlotMap};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::Duration;

pub use self::data_types::*;
mod data_types {
    use std::fmt::Debug;

    use rodio::Sample;

    use super::*;
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum DataType {
        AudioSource,
        Float,
        Duration,
    }

    #[derive(Clone)]
    pub enum ValueType {
        AudioSource { value: usize },
        Float { value: f32 },
        Duration { value: Duration },
    }

    #[derive(Clone, Debug)]
    pub enum InputValueConfig {
        AudioSource,
        Float { value: f32 },
        Duration { value: f32 },
    }

    impl Debug for ValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::AudioSource { value } => f
                    .debug_struct("Source")
                    .field("value", &"Anonymous AudioSource")
                    .finish(),
                Self::Float { value } => f.debug_struct("Float").field("value", value).finish(),
                Self::Duration { value } => {
                    f.debug_struct("Duration").field("value", value).finish()
                }
            }
        }
    }

    impl ValueType {
        /// Tries to downcast this value type to a vector
        pub fn try_to_source(self) -> Result<usize, String> {
            match self {
                ValueType::AudioSource { value } => Ok(value),
                _ => Err("invalid cast".to_string()),
            }
        }

        /// Tries to downcast this value type to a scalar
        pub fn try_to_float(self) -> Result<f32, String> {
            match self {
                ValueType::Float { value } => Ok(value),
                _ => Err("invalid cast".to_string()),
            }
        }

        pub fn try_to_duration(self) -> Result<Duration, String> {
            match self {
                ValueType::Duration { value } => Ok(value),
                _ => Err("invalid cast".to_string()),
            }
        }
    }
}

pub use self::input_output::*;

use super::nodes::FiniteSource;
mod input_output {
    use super::*;
    #[derive(Clone, Debug)]
    pub struct InputParameter {
        pub name: String,
        pub data_type: DataType,
        pub kind: InputParamKind,
        pub value: InputValueConfig,
    }
    #[derive(Clone, Debug)]
    pub struct Output {
        pub name: String,
        pub data_type: DataType,
    }
}

#[derive(Clone)]
pub struct SoundNode {
    pub name: String,
    pub inputs: HashMap<String, InputParameter>,
    pub outputs: HashMap<String, Output>,
    pub operation:
        fn(HashMap<String, ValueType>, &mut Vec<FiniteSource<f32>>) -> HashMap<String, ValueType>,
}

pub struct NodeDefinitions(pub BTreeMap<String, SoundNode>);

slotmap::new_key_type! { pub struct SoundNodeId; }
impl SoundNodeId {
    pub fn display_id(self) -> String {
        format!("{:?}", self.0)
    }
}

pub struct NodeSoundGraph {
    pub nodes: SlotMap<SoundNodeId, SoundNode>,
}

#[derive(Clone, Debug)]
pub struct NodeMapping(
    SecondaryMap<SoundNodeId, NodeId>,
    SecondaryMap<NodeId, SoundNodeId>,
);
