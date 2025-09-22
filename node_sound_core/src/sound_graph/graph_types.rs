use egui_node_graph_2::InputParamKind;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use synthrs::midi::MidiSong;

use crate::sound_graph::note::NoteValue;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum DataType {
    None,
    AudioSource,
    Float,
    Duration,
    AudioFile,
    MidiFile,
    Graph,
    Bool,
    Dropdown,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum ValueType {
    #[default]
    None,
    AudioSource {
        value: usize,
    },
    Float {
        value: f32,
        min: f32,
        max: f32,
        note: NoteValue,
    },
    Duration {
        value: Duration,
    },
    AudioFile {
        value: Option<(String, Vec<u8>)>,
    },
    MidiFile {
        value: Option<(String, MidiSong)>,
    },
    Graph {
        value: Option<Vec<f32>>,
        width: f32,
        height: f32,
        id: usize,
    },
    Bool {
        value: bool,
    },
    Dropdown {
        value: String,
        values: Vec<String>,
    },
}

impl Default for &ValueType {
    fn default() -> Self {
        &ValueType::None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InputValueConfig {
    AudioSource {},
    Float {
        value: f32,
        min: f32,
        max: f32,
    },
    Duration {
        value: f32,
    },
    AudioFile {},
    MidiFile {},
    Graph {
        value: Vec<f32>,
        width: f32,
        height: f32,
    },
    Bool {
        value: bool,
    },
    Dropdown {
        value: String,
        values: Vec<String>,
    },
}

impl Debug for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AudioSource { value: _ } => f
                .debug_struct("Source")
                .field("value", &"Anonymous AudioSource")
                .finish(),
            Self::Float {
                value,
                min: _,
                max: _,
                note: _,
            } => f.debug_struct("Float").field("value", value).finish(),
            Self::Duration { value } => f.debug_struct("Duration").field("value", value).finish(),
            Self::None => f.debug_struct("None").finish(),
            Self::AudioFile { value } => f
                .debug_struct(&value.clone().unwrap_or(("None".to_string(), vec![])).0)
                .finish(),
            Self::MidiFile { value: _ } => f
                .debug_struct("Midi")
                .field("value", &"Anonymous MidiFile")
                .finish(),
            Self::Graph {
                value: _,
                id: _,
                width: _,
                height: _,
            } => f
                .debug_struct("Graph")
                .field("value", &"Anonymous Graph")
                .finish(),
            Self::Bool { value } => {
                if *value {
                    f.debug_struct("true").finish()
                } else {
                    f.debug_struct("false").finish()
                }
            }
            Self::Dropdown {
                value: _,
                values: _,
            } => f
                .debug_struct("Dropdown")
                .field("value", &"Anonymous Dropdown")
                .finish(),
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
            ValueType::Float {
                value,
                min: _,
                max: _,
                note: _,
            } => Ok(value),
            _ => Err("invalid cast".to_string()),
        }
    }

    pub fn try_to_duration(self) -> Result<Duration, String> {
        match self {
            ValueType::Duration { value } => Ok(value),
            _ => Err("invalid cast".to_string()),
        }
    }

    pub fn try_to_bool(self) -> Result<bool, String> {
        match self {
            ValueType::Bool { value } => Ok(value),
            _ => Err("invalid cast".to_string()),
        }
    }

    pub fn try_to_file(self) -> Result<Option<(String, Vec<u8>)>, String> {
        match self {
            ValueType::AudioFile { value } => Ok(value),
            _ => Err("invalid cast".to_string()),
        }
    }
    pub fn try_to_midi(self) -> Result<Option<(String, MidiSong)>, String> {
        match self {
            ValueType::MidiFile { value } => Ok(value),
            _ => Err("invalid cast".to_string()),
        }
    }
    pub fn try_to_graph(self) -> Result<Option<Vec<f32>>, String> {
        match self {
            ValueType::Graph {
                value,
                id: _,
                width: _,
                height: _,
            } => Ok(value),
            _ => Err("invalid cast".to_string()),
        }
    }

    pub fn try_to_dropdown(self) -> Result<String, String> {
        match self {
            ValueType::Dropdown { value, values: _ } => Ok(value),
            _ => Err("invalid cast".to_string()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputParameter {
    pub name: String,
    pub data_type: DataType,
    pub kind: InputParamKind,
    pub value: InputValueConfig,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub name: String,
    pub data_type: DataType,
}
