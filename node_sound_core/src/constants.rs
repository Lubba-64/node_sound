pub type UnitType = f32;

pub const DEFAULT_SAMPLE_RATE: u32 = 44100;
pub const MIDDLE_C_FREQ: f32 = 261.63;
pub const WAVE_TABLE_SIZE: usize = 100;
pub const MAX_FREQ: f32 = 8000.0; // NoteValue(Octave::O8, Note::B) => 7902.13,
