use egui_node_graph_2::InputParamKind;
use std::time::Duration;

pub use self::data_types::*;
mod data_types {
    #[derive(Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
    pub enum Note {
        #[default]
        C,
        CS,
        D,
        DS,
        E,
        F,
        FS,
        G,
        GS,
        A,
        AS,
        B,
    }

    impl Note {
        pub const ALL: [Note; 12] = [
            Note::C,
            Note::CS,
            Note::D,
            Note::DS,
            Note::E,
            Note::F,
            Note::FS,
            Note::G,
            Note::GS,
            Note::A,
            Note::AS,
            Note::B,
        ];
    }

    #[derive(Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
    pub enum Octave {
        O0,
        O1,
        O2,
        O3,
        O4,
        #[default]
        O5,
        O6,
        O7,
        O8,
    }

    impl Octave {
        pub const ALL: [Octave; 9] = [
            Octave::O0,
            Octave::O1,
            Octave::O2,
            Octave::O3,
            Octave::O4,
            Octave::O5,
            Octave::O6,
            Octave::O7,
            Octave::O8,
        ];
    }

    impl ToString for Octave {
        fn to_string(&self) -> String {
            match self {
                Octave::O0 => 0.to_string(),
                Octave::O1 => 1.to_string(),
                Octave::O2 => 2.to_string(),
                Octave::O3 => 3.to_string(),
                Octave::O4 => 4.to_string(),
                Octave::O5 => 5.to_string(),
                Octave::O6 => 6.to_string(),
                Octave::O7 => 7.to_string(),
                Octave::O8 => 8.to_string(),
            }
        }
    }

    impl ToString for Note {
        fn to_string(&self) -> String {
            match self {
                Note::C => "C".to_string(),
                Note::CS => "C#".to_string(),
                Note::D => "D".to_string(),
                Note::DS => "D#".to_string(),
                Note::E => "E".to_string(),
                Note::F => "F".to_string(),
                Note::FS => "F#".to_string(),
                Note::G => "G".to_string(),
                Note::GS => "G#".to_string(),
                Note::A => "A".to_string(),
                Note::AS => "A#".to_string(),
                Note::B => "B".to_string(),
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
    pub struct NoteValue(pub Octave, pub Note);

    impl NoteValue {
        pub fn match_freq(&self) -> f32 {
            match self.clone() {
                NoteValue(Octave::O0, Note::C) => 16.35,
                NoteValue(Octave::O0, Note::CS) => 17.32,
                NoteValue(Octave::O0, Note::D) => 18.35,
                NoteValue(Octave::O0, Note::DS) => 19.45,
                NoteValue(Octave::O0, Note::E) => 20.6,
                NoteValue(Octave::O0, Note::F) => 21.83,
                NoteValue(Octave::O0, Note::FS) => 23.12,
                NoteValue(Octave::O0, Note::G) => 24.5,
                NoteValue(Octave::O0, Note::GS) => 25.96,
                NoteValue(Octave::O0, Note::A) => 27.5,
                NoteValue(Octave::O0, Note::AS) => 29.14,
                NoteValue(Octave::O0, Note::B) => 30.87,
                NoteValue(Octave::O1, Note::C) => 32.7,
                NoteValue(Octave::O1, Note::CS) => 34.65,
                NoteValue(Octave::O1, Note::D) => 36.71,
                NoteValue(Octave::O1, Note::DS) => 38.89,
                NoteValue(Octave::O1, Note::E) => 41.2,
                NoteValue(Octave::O1, Note::F) => 43.65,
                NoteValue(Octave::O1, Note::FS) => 46.25,
                NoteValue(Octave::O1, Note::G) => 49.0,
                NoteValue(Octave::O1, Note::GS) => 51.91,
                NoteValue(Octave::O1, Note::A) => 55.0,
                NoteValue(Octave::O1, Note::AS) => 58.27,
                NoteValue(Octave::O1, Note::B) => 61.74,
                NoteValue(Octave::O2, Note::C) => 65.41,
                NoteValue(Octave::O2, Note::CS) => 69.3,
                NoteValue(Octave::O2, Note::D) => 73.42,
                NoteValue(Octave::O2, Note::DS) => 77.78,
                NoteValue(Octave::O2, Note::E) => 82.41,
                NoteValue(Octave::O2, Note::F) => 87.31,
                NoteValue(Octave::O2, Note::FS) => 92.5,
                NoteValue(Octave::O2, Note::G) => 98.0,
                NoteValue(Octave::O2, Note::GS) => 103.83,
                NoteValue(Octave::O2, Note::A) => 110.0,
                NoteValue(Octave::O2, Note::AS) => 116.54,
                NoteValue(Octave::O2, Note::B) => 123.47,
                NoteValue(Octave::O3, Note::C) => 130.81,
                NoteValue(Octave::O3, Note::CS) => 138.59,
                NoteValue(Octave::O3, Note::D) => 146.83,
                NoteValue(Octave::O3, Note::DS) => 155.56,
                NoteValue(Octave::O3, Note::E) => 164.81,
                NoteValue(Octave::O3, Note::F) => 174.61,
                NoteValue(Octave::O3, Note::FS) => 185.0,
                NoteValue(Octave::O3, Note::G) => 196.0,
                NoteValue(Octave::O3, Note::GS) => 207.65,
                NoteValue(Octave::O3, Note::A) => 220.0,
                NoteValue(Octave::O3, Note::AS) => 233.08,
                NoteValue(Octave::O3, Note::B) => 246.94,
                NoteValue(Octave::O4, Note::C) => 261.63,
                NoteValue(Octave::O4, Note::CS) => 277.18,
                NoteValue(Octave::O4, Note::D) => 293.66,
                NoteValue(Octave::O4, Note::DS) => 311.13,
                NoteValue(Octave::O4, Note::E) => 329.63,
                NoteValue(Octave::O4, Note::F) => 349.23,
                NoteValue(Octave::O4, Note::FS) => 369.99,
                NoteValue(Octave::O4, Note::G) => 392.0,
                NoteValue(Octave::O4, Note::GS) => 415.3,
                NoteValue(Octave::O4, Note::A) => 440.0,
                NoteValue(Octave::O4, Note::AS) => 466.16,
                NoteValue(Octave::O4, Note::B) => 493.88,
                NoteValue(Octave::O5, Note::C) => 523.25,
                NoteValue(Octave::O5, Note::CS) => 554.37,
                NoteValue(Octave::O5, Note::D) => 587.33,
                NoteValue(Octave::O5, Note::DS) => 622.25,
                NoteValue(Octave::O5, Note::E) => 659.25,
                NoteValue(Octave::O5, Note::F) => 698.46,
                NoteValue(Octave::O5, Note::FS) => 739.99,
                NoteValue(Octave::O5, Note::G) => 783.99,
                NoteValue(Octave::O5, Note::GS) => 830.61,
                NoteValue(Octave::O5, Note::A) => 880.0,
                NoteValue(Octave::O5, Note::AS) => 932.33,
                NoteValue(Octave::O5, Note::B) => 987.77,
                NoteValue(Octave::O6, Note::C) => 1046.5,
                NoteValue(Octave::O6, Note::CS) => 1108.73,
                NoteValue(Octave::O6, Note::D) => 1174.66,
                NoteValue(Octave::O6, Note::DS) => 1244.51,
                NoteValue(Octave::O6, Note::E) => 1318.51,
                NoteValue(Octave::O6, Note::F) => 1396.91,
                NoteValue(Octave::O6, Note::FS) => 1479.98,
                NoteValue(Octave::O6, Note::G) => 1567.98,
                NoteValue(Octave::O6, Note::GS) => 1661.22,
                NoteValue(Octave::O6, Note::A) => 1760.0,
                NoteValue(Octave::O6, Note::AS) => 1864.66,
                NoteValue(Octave::O6, Note::B) => 1975.53,
                NoteValue(Octave::O7, Note::C) => 2093.0,
                NoteValue(Octave::O7, Note::CS) => 2217.46,
                NoteValue(Octave::O7, Note::D) => 2349.32,
                NoteValue(Octave::O7, Note::DS) => 2489.0,
                NoteValue(Octave::O7, Note::E) => 2637.0,
                NoteValue(Octave::O7, Note::F) => 2793.83,
                NoteValue(Octave::O7, Note::FS) => 2959.96,
                NoteValue(Octave::O7, Note::G) => 3135.96,
                NoteValue(Octave::O7, Note::GS) => 3322.44,
                NoteValue(Octave::O7, Note::A) => 3520.0,
                NoteValue(Octave::O7, Note::AS) => 3729.31,
                NoteValue(Octave::O7, Note::B) => 3951.0,
                NoteValue(Octave::O8, Note::C) => 4186.0,
                NoteValue(Octave::O8, Note::CS) => 4434.92,
                NoteValue(Octave::O8, Note::D) => 4698.63,
                NoteValue(Octave::O8, Note::DS) => 4978.0,
                NoteValue(Octave::O8, Note::E) => 5274.0,
                NoteValue(Octave::O8, Note::F) => 5587.65,
                NoteValue(Octave::O8, Note::FS) => 5919.91,
                NoteValue(Octave::O8, Note::G) => 6271.93,
                NoteValue(Octave::O8, Note::GS) => 6644.88,
                NoteValue(Octave::O8, Note::A) => 7040.0,
                NoteValue(Octave::O8, Note::AS) => 7458.62,
                NoteValue(Octave::O8, Note::B) => 7902.13,
            }
        }
    }

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
        Graph,
        Code,
    }

    #[derive(Clone, Default, Serialize, Deserialize)]
    pub enum ValueType {
        Code {
            value: String,
        },
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
            id: usize,
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
        Graph { value: Vec<f32> },
        Code { value: String },
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
                Self::Graph { value: _, id: _ } => f
                    .debug_struct("Graph")
                    .field("value", &"Anonymous Graph")
                    .finish(),
                Self::Code { value: _ } => f
                    .debug_struct("Code")
                    .field("value", &"Anonymous Code")
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
                ValueType::Graph { value, id: _ } => Ok(value),
                _ => Err("invalid cast".to_string()),
            }
        }
        pub fn try_to_code(self) -> Result<String, String> {
            match self {
                ValueType::Code { value } => Ok(value),
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
