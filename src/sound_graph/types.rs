use egui_node_graph::InputParamKind;
use egui_node_graph::NodeId;
use slotmap::{self, SecondaryMap, SlotMap};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::Duration;

pub use self::data_types::*;
mod data_types {
    use std::fmt::Debug;

    use super::*;
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum DataType {
        AudioSource,
        Float,
        Duration,
    }
    
    #[derive(Clone)]
    pub enum ValueType {
        AudioSource { value: usize},
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
        pub fn try_to_source(self) -> anyhow::Result<usize> {
            if let ValueType::AudioSource { value } = self {
                Ok(value)
            } else {
                anyhow::bail!("Invalid cast from {:?} to vec2", self)
            }
        }

        /// Tries to downcast this value type to a scalar
        pub fn try_to_float(self) -> anyhow::Result<f32> {
            if let ValueType::Float { value } = self {
                Ok(value)
            } else {
                anyhow::bail!("Invalid cast from {:?} to scalar", self)
            }
        }

        pub fn try_to_duration(self) -> anyhow::Result<Duration> {
            if let ValueType::Duration { value } = self {
                Ok(value)
            } else {
                anyhow::bail!("Invalid cast from {:?} to scalar", self)
            }
        }
    }
}

pub use self::input_output::*;
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

#[derive(Clone, Debug)]
pub struct SoundNode {
    pub name: String,
    pub inputs: Vec<InputParameter>,
    pub outputs: Vec<Output>,
    pub operation : fn(HashMap<String, ValueType>) -> HashMap<String, ValueType>
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
