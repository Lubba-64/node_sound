use egui_node_graph_2::InputParamKind;
use std::time::Duration;

pub use self::data_types::*;
mod data_types {
    use std::fmt::Debug;

    use serde::{Deserialize, Serialize};
    use synthrs::midi::MidiSong;

    use super::*;
    #[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
    pub enum DataType {
        None,
        AudioSource,
        Float,
        Duration,
        AudioFile,
        MidiFile,
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
    }

    impl Default for &ValueType {
        fn default() -> Self {
            &ValueType::None
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum InputValueConfig {
        AudioSource {},
        Float { value: f32, min: f32, max: f32 },
        Duration { value: f32 },
        AudioFile {},
        MidiFile {},
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
                } => f.debug_struct("Float").field("value", value).finish(),
                Self::Duration { value } => {
                    f.debug_struct("Duration").field("value", value).finish()
                }
                Self::None => f.debug_struct("None").finish(),
                Self::AudioFile { value } => f
                    .debug_struct(&value.clone().unwrap_or(("None".to_string(), vec![])).0)
                    .finish(),
                Self::MidiFile { value: _ } => f
                    .debug_struct("Midi")
                    .field("value", &"Anonymous MidiFile")
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
    }
}

pub use self::input_output::*;

mod input_output {
    use serde::{Deserialize, Serialize};

    use super::*;
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
}
