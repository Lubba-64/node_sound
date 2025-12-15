use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
pub struct Pitch(pub Octave, pub Note);

impl Pitch {
    pub const ALL: [Pitch; 108] = [
        Pitch(Octave::O0, Note::C),
        Pitch(Octave::O0, Note::CS),
        Pitch(Octave::O0, Note::D),
        Pitch(Octave::O0, Note::DS),
        Pitch(Octave::O0, Note::E),
        Pitch(Octave::O0, Note::F),
        Pitch(Octave::O0, Note::FS),
        Pitch(Octave::O0, Note::G),
        Pitch(Octave::O0, Note::GS),
        Pitch(Octave::O0, Note::A),
        Pitch(Octave::O0, Note::AS),
        Pitch(Octave::O0, Note::B),
        Pitch(Octave::O1, Note::C),
        Pitch(Octave::O1, Note::CS),
        Pitch(Octave::O1, Note::D),
        Pitch(Octave::O1, Note::DS),
        Pitch(Octave::O1, Note::E),
        Pitch(Octave::O1, Note::F),
        Pitch(Octave::O1, Note::FS),
        Pitch(Octave::O1, Note::G),
        Pitch(Octave::O1, Note::GS),
        Pitch(Octave::O1, Note::A),
        Pitch(Octave::O1, Note::AS),
        Pitch(Octave::O1, Note::B),
        Pitch(Octave::O2, Note::C),
        Pitch(Octave::O2, Note::CS),
        Pitch(Octave::O2, Note::D),
        Pitch(Octave::O2, Note::DS),
        Pitch(Octave::O2, Note::E),
        Pitch(Octave::O2, Note::F),
        Pitch(Octave::O2, Note::FS),
        Pitch(Octave::O2, Note::G),
        Pitch(Octave::O2, Note::GS),
        Pitch(Octave::O2, Note::A),
        Pitch(Octave::O2, Note::AS),
        Pitch(Octave::O2, Note::B),
        Pitch(Octave::O3, Note::C),
        Pitch(Octave::O3, Note::CS),
        Pitch(Octave::O3, Note::D),
        Pitch(Octave::O3, Note::DS),
        Pitch(Octave::O3, Note::E),
        Pitch(Octave::O3, Note::F),
        Pitch(Octave::O3, Note::FS),
        Pitch(Octave::O3, Note::G),
        Pitch(Octave::O3, Note::GS),
        Pitch(Octave::O3, Note::A),
        Pitch(Octave::O3, Note::AS),
        Pitch(Octave::O3, Note::B),
        Pitch(Octave::O4, Note::C),
        Pitch(Octave::O4, Note::CS),
        Pitch(Octave::O4, Note::D),
        Pitch(Octave::O4, Note::DS),
        Pitch(Octave::O4, Note::E),
        Pitch(Octave::O4, Note::F),
        Pitch(Octave::O4, Note::FS),
        Pitch(Octave::O4, Note::G),
        Pitch(Octave::O4, Note::GS),
        Pitch(Octave::O4, Note::A),
        Pitch(Octave::O4, Note::AS),
        Pitch(Octave::O4, Note::B),
        Pitch(Octave::O5, Note::C),
        Pitch(Octave::O5, Note::CS),
        Pitch(Octave::O5, Note::D),
        Pitch(Octave::O5, Note::DS),
        Pitch(Octave::O5, Note::E),
        Pitch(Octave::O5, Note::F),
        Pitch(Octave::O5, Note::FS),
        Pitch(Octave::O5, Note::G),
        Pitch(Octave::O5, Note::GS),
        Pitch(Octave::O5, Note::A),
        Pitch(Octave::O5, Note::AS),
        Pitch(Octave::O5, Note::B),
        Pitch(Octave::O6, Note::C),
        Pitch(Octave::O6, Note::CS),
        Pitch(Octave::O6, Note::D),
        Pitch(Octave::O6, Note::DS),
        Pitch(Octave::O6, Note::E),
        Pitch(Octave::O6, Note::F),
        Pitch(Octave::O6, Note::FS),
        Pitch(Octave::O6, Note::G),
        Pitch(Octave::O6, Note::GS),
        Pitch(Octave::O6, Note::A),
        Pitch(Octave::O6, Note::AS),
        Pitch(Octave::O6, Note::B),
        Pitch(Octave::O7, Note::C),
        Pitch(Octave::O7, Note::CS),
        Pitch(Octave::O7, Note::D),
        Pitch(Octave::O7, Note::DS),
        Pitch(Octave::O7, Note::E),
        Pitch(Octave::O7, Note::F),
        Pitch(Octave::O7, Note::FS),
        Pitch(Octave::O7, Note::G),
        Pitch(Octave::O7, Note::GS),
        Pitch(Octave::O7, Note::A),
        Pitch(Octave::O7, Note::AS),
        Pitch(Octave::O7, Note::B),
        Pitch(Octave::O8, Note::C),
        Pitch(Octave::O8, Note::CS),
        Pitch(Octave::O8, Note::D),
        Pitch(Octave::O8, Note::DS),
        Pitch(Octave::O8, Note::E),
        Pitch(Octave::O8, Note::F),
        Pitch(Octave::O8, Note::FS),
        Pitch(Octave::O8, Note::G),
        Pitch(Octave::O8, Note::GS),
        Pitch(Octave::O8, Note::A),
        Pitch(Octave::O8, Note::AS),
        Pitch(Octave::O8, Note::B),
    ];

    pub const ALL_FREQ: [f32; 108] = [
        16.35, 17.32, 18.35, 19.45, 20.6, 21.83, 23.12, 24.5, 25.96, 27.5, 29.14, 30.87, 32.7,
        34.65, 36.71, 38.89, 41.2, 43.65, 46.25, 49.0, 51.91, 55.0, 58.27, 61.74, 65.41, 69.3,
        73.42, 77.78, 82.41, 87.31, 92.5, 98.0, 103.83, 110.0, 116.54, 123.47, 130.81, 138.59,
        146.83, 155.56, 164.81, 174.61, 185.0, 196.0, 207.65, 220.0, 233.08, 246.94, 261.63,
        277.18, 293.66, 311.13, 329.63, 349.23, 369.99, 392.0, 415.3, 440.0, 466.16, 493.88,
        523.25, 554.37, 587.33, 622.25, 659.25, 698.46, 739.99, 783.99, 830.61, 880.0, 932.33,
        987.77, 1046.5, 1108.73, 1174.66, 1244.51, 1318.51, 1396.91, 1479.98, 1567.98, 1661.22,
        1760.0, 1864.66, 1975.53, 2093.0, 2217.46, 2349.32, 2489.0, 2637.0, 2793.83, 2959.96,
        3135.96, 3322.44, 3520.0, 3729.31, 3951.0, 4186.0, 4434.92, 4698.63, 4978.0, 5274.0,
        5587.65, 5919.91, 6271.93, 6644.88, 7040.0, 7458.62, 7902.13,
    ];

    pub fn match_freq(&self) -> f32 {
        match self.clone() {
            Pitch(Octave::O0, Note::C) => 16.35,
            Pitch(Octave::O0, Note::CS) => 17.32,
            Pitch(Octave::O0, Note::D) => 18.35,
            Pitch(Octave::O0, Note::DS) => 19.45,
            Pitch(Octave::O0, Note::E) => 20.6,
            Pitch(Octave::O0, Note::F) => 21.83,
            Pitch(Octave::O0, Note::FS) => 23.12,
            Pitch(Octave::O0, Note::G) => 24.5,
            Pitch(Octave::O0, Note::GS) => 25.96,
            Pitch(Octave::O0, Note::A) => 27.5,
            Pitch(Octave::O0, Note::AS) => 29.14,
            Pitch(Octave::O0, Note::B) => 30.87,
            Pitch(Octave::O1, Note::C) => 32.7,
            Pitch(Octave::O1, Note::CS) => 34.65,
            Pitch(Octave::O1, Note::D) => 36.71,
            Pitch(Octave::O1, Note::DS) => 38.89,
            Pitch(Octave::O1, Note::E) => 41.2,
            Pitch(Octave::O1, Note::F) => 43.65,
            Pitch(Octave::O1, Note::FS) => 46.25,
            Pitch(Octave::O1, Note::G) => 49.0,
            Pitch(Octave::O1, Note::GS) => 51.91,
            Pitch(Octave::O1, Note::A) => 55.0,
            Pitch(Octave::O1, Note::AS) => 58.27,
            Pitch(Octave::O1, Note::B) => 61.74,
            Pitch(Octave::O2, Note::C) => 65.41,
            Pitch(Octave::O2, Note::CS) => 69.3,
            Pitch(Octave::O2, Note::D) => 73.42,
            Pitch(Octave::O2, Note::DS) => 77.78,
            Pitch(Octave::O2, Note::E) => 82.41,
            Pitch(Octave::O2, Note::F) => 87.31,
            Pitch(Octave::O2, Note::FS) => 92.5,
            Pitch(Octave::O2, Note::G) => 98.0,
            Pitch(Octave::O2, Note::GS) => 103.83,
            Pitch(Octave::O2, Note::A) => 110.0,
            Pitch(Octave::O2, Note::AS) => 116.54,
            Pitch(Octave::O2, Note::B) => 123.47,
            Pitch(Octave::O3, Note::C) => 130.81,
            Pitch(Octave::O3, Note::CS) => 138.59,
            Pitch(Octave::O3, Note::D) => 146.83,
            Pitch(Octave::O3, Note::DS) => 155.56,
            Pitch(Octave::O3, Note::E) => 164.81,
            Pitch(Octave::O3, Note::F) => 174.61,
            Pitch(Octave::O3, Note::FS) => 185.0,
            Pitch(Octave::O3, Note::G) => 196.0,
            Pitch(Octave::O3, Note::GS) => 207.65,
            Pitch(Octave::O3, Note::A) => 220.0,
            Pitch(Octave::O3, Note::AS) => 233.08,
            Pitch(Octave::O3, Note::B) => 246.94,
            Pitch(Octave::O4, Note::C) => 261.63,
            Pitch(Octave::O4, Note::CS) => 277.18,
            Pitch(Octave::O4, Note::D) => 293.66,
            Pitch(Octave::O4, Note::DS) => 311.13,
            Pitch(Octave::O4, Note::E) => 329.63,
            Pitch(Octave::O4, Note::F) => 349.23,
            Pitch(Octave::O4, Note::FS) => 369.99,
            Pitch(Octave::O4, Note::G) => 392.0,
            Pitch(Octave::O4, Note::GS) => 415.3,
            Pitch(Octave::O4, Note::A) => 440.0,
            Pitch(Octave::O4, Note::AS) => 466.16,
            Pitch(Octave::O4, Note::B) => 493.88,
            Pitch(Octave::O5, Note::C) => 523.25,
            Pitch(Octave::O5, Note::CS) => 554.37,
            Pitch(Octave::O5, Note::D) => 587.33,
            Pitch(Octave::O5, Note::DS) => 622.25,
            Pitch(Octave::O5, Note::E) => 659.25,
            Pitch(Octave::O5, Note::F) => 698.46,
            Pitch(Octave::O5, Note::FS) => 739.99,
            Pitch(Octave::O5, Note::G) => 783.99,
            Pitch(Octave::O5, Note::GS) => 830.61,
            Pitch(Octave::O5, Note::A) => 880.0,
            Pitch(Octave::O5, Note::AS) => 932.33,
            Pitch(Octave::O5, Note::B) => 987.77,
            Pitch(Octave::O6, Note::C) => 1046.5,
            Pitch(Octave::O6, Note::CS) => 1108.73,
            Pitch(Octave::O6, Note::D) => 1174.66,
            Pitch(Octave::O6, Note::DS) => 1244.51,
            Pitch(Octave::O6, Note::E) => 1318.51,
            Pitch(Octave::O6, Note::F) => 1396.91,
            Pitch(Octave::O6, Note::FS) => 1479.98,
            Pitch(Octave::O6, Note::G) => 1567.98,
            Pitch(Octave::O6, Note::GS) => 1661.22,
            Pitch(Octave::O6, Note::A) => 1760.0,
            Pitch(Octave::O6, Note::AS) => 1864.66,
            Pitch(Octave::O6, Note::B) => 1975.53,
            Pitch(Octave::O7, Note::C) => 2093.0,
            Pitch(Octave::O7, Note::CS) => 2217.46,
            Pitch(Octave::O7, Note::D) => 2349.32,
            Pitch(Octave::O7, Note::DS) => 2489.0,
            Pitch(Octave::O7, Note::E) => 2637.0,
            Pitch(Octave::O7, Note::F) => 2793.83,
            Pitch(Octave::O7, Note::FS) => 2959.96,
            Pitch(Octave::O7, Note::G) => 3135.96,
            Pitch(Octave::O7, Note::GS) => 3322.44,
            Pitch(Octave::O7, Note::A) => 3520.0,
            Pitch(Octave::O7, Note::AS) => 3729.31,
            Pitch(Octave::O7, Note::B) => 3951.0,
            Pitch(Octave::O8, Note::C) => 4186.0,
            Pitch(Octave::O8, Note::CS) => 4434.92,
            Pitch(Octave::O8, Note::D) => 4698.63,
            Pitch(Octave::O8, Note::DS) => 4978.0,
            Pitch(Octave::O8, Note::E) => 5274.0,
            Pitch(Octave::O8, Note::F) => 5587.65,
            Pitch(Octave::O8, Note::FS) => 5919.91,
            Pitch(Octave::O8, Note::G) => 6271.93,
            Pitch(Octave::O8, Note::GS) => 6644.88,
            Pitch(Octave::O8, Note::A) => 7040.0,
            Pitch(Octave::O8, Note::AS) => 7458.62,
            Pitch(Octave::O8, Note::B) => 7902.13,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum NoteSpeed {
    WholeTriplet,
    Whole,
    WholeDotted,
    HalfTriplet,
    Half,
    HalfDotted,
    #[default]
    Quarter,
    QuarterTriplet,
    QuarterDotted,
    Eighth,
    EighthTriplet,
    EighthDotted,
    SixteenthTriplet,
    Sixteenth,
    SixteenthDotted,
    ThirtySecondTriplet,
    ThirtySecond,
    ThirtySecondDotted,
    SixtyFourthTriplet,
    SixtyFourth,
    SixtyFourthDotted,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum NoteSpeedType {
    Any,
    #[default]
    Normal,
    Dotted,
    Triplet,
}

impl ToString for NoteSpeedType {
    fn to_string(&self) -> String {
        match self {
            NoteSpeedType::Any => "Any".to_string(),
            NoteSpeedType::Normal => "Normal".to_string(),
            NoteSpeedType::Dotted => "Dotted".to_string(),
            NoteSpeedType::Triplet => "Triplet".to_string(),
        }
    }
}

impl FromStr for NoteSpeedType {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Any" => NoteSpeedType::Any,
            "Normal" => NoteSpeedType::Normal,
            "Dotted" => NoteSpeedType::Dotted,
            "Triplet" => NoteSpeedType::Triplet,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Incorrect from_str value",
                ));
            }
        })
    }
}

impl NoteSpeedType {
    pub const ALL: [NoteSpeedType; 4] = [
        NoteSpeedType::Any,
        NoteSpeedType::Normal,
        NoteSpeedType::Dotted,
        NoteSpeedType::Triplet,
    ];

    pub fn get_beats_type(&self) -> Vec<NoteSpeed> {
        match self {
            NoteSpeedType::Any => NoteSpeed::ALL.to_vec(),
            NoteSpeedType::Normal => NoteSpeed::ALL_NORMAL.to_vec(),
            NoteSpeedType::Dotted => NoteSpeed::ALL_DOTTED.to_vec(),
            NoteSpeedType::Triplet => NoteSpeed::ALL_TRIPLET.to_vec(),
        }
    }
}

impl ToString for NoteSpeed {
    fn to_string(&self) -> String {
        match self {
            NoteSpeed::WholeTriplet => "1 triplet".to_string(),
            NoteSpeed::Whole => "1".to_string(),
            NoteSpeed::WholeDotted => "1.".to_string(),
            NoteSpeed::HalfTriplet => "1/2 triplet".to_string(),
            NoteSpeed::Half => "1/2".to_string(),
            NoteSpeed::HalfDotted => "1/2.".to_string(),
            NoteSpeed::QuarterTriplet => "1/4 triplet".to_string(),
            NoteSpeed::Quarter => "1/4".to_string(),
            NoteSpeed::QuarterDotted => "1/4.".to_string(),
            NoteSpeed::EighthTriplet => "1/8 triplet".to_string(),
            NoteSpeed::Eighth => "1/8".to_string(),
            NoteSpeed::EighthDotted => "1/8.".to_string(),
            NoteSpeed::SixteenthTriplet => "1/16 triplet".to_string(),
            NoteSpeed::Sixteenth => "1/16".to_string(),
            NoteSpeed::SixteenthDotted => "1/16.".to_string(),
            NoteSpeed::ThirtySecondTriplet => "1/32 triplet".to_string(),
            NoteSpeed::ThirtySecond => "1/32".to_string(),
            NoteSpeed::ThirtySecondDotted => "1/32.".to_string(),
            NoteSpeed::SixtyFourthTriplet => "1/64 triplet".to_string(),
            NoteSpeed::SixtyFourth => "1/64".to_string(),
            NoteSpeed::SixtyFourthDotted => "1/64.".to_string(),
        }
    }
}

impl FromStr for NoteSpeed {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1 triplet" => NoteSpeed::WholeTriplet,
            "1" => NoteSpeed::Whole,
            "1." => NoteSpeed::WholeDotted,
            "1/2 triplet" => NoteSpeed::HalfTriplet,
            "1/2" => NoteSpeed::Half,
            "1/2." => NoteSpeed::HalfDotted,
            "1/4 triplet" => NoteSpeed::QuarterTriplet,
            "1/4" => NoteSpeed::Quarter,
            "1/4." => NoteSpeed::QuarterDotted,
            "1/8 triplet" => NoteSpeed::EighthTriplet,
            "1/8" => NoteSpeed::Eighth,
            "1/8." => NoteSpeed::EighthDotted,
            "1/16 triplet" => NoteSpeed::SixteenthTriplet,
            "1/16" => NoteSpeed::Sixteenth,
            "1/16." => NoteSpeed::SixteenthDotted,
            "1/32 triplet" => NoteSpeed::ThirtySecondTriplet,
            "1/32" => NoteSpeed::ThirtySecond,
            "1/32." => NoteSpeed::ThirtySecondDotted,
            "1/64 triplet" => NoteSpeed::SixtyFourthTriplet,
            "1/64" => NoteSpeed::SixtyFourth,
            "1/64." => NoteSpeed::SixtyFourthDotted,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Incorrect from_str value",
                ));
            }
        })
    }
}

impl NoteSpeed {
    pub const ALL: [NoteSpeed; 21] = [
        NoteSpeed::WholeTriplet,
        NoteSpeed::Whole,
        NoteSpeed::WholeDotted,
        NoteSpeed::HalfTriplet,
        NoteSpeed::Half,
        NoteSpeed::HalfDotted,
        NoteSpeed::QuarterTriplet,
        NoteSpeed::Quarter,
        NoteSpeed::QuarterDotted,
        NoteSpeed::EighthTriplet,
        NoteSpeed::Eighth,
        NoteSpeed::EighthDotted,
        NoteSpeed::SixteenthTriplet,
        NoteSpeed::Sixteenth,
        NoteSpeed::SixteenthDotted,
        NoteSpeed::ThirtySecondTriplet,
        NoteSpeed::ThirtySecond,
        NoteSpeed::ThirtySecondDotted,
        NoteSpeed::SixtyFourthTriplet,
        NoteSpeed::SixtyFourth,
        NoteSpeed::SixtyFourthDotted,
    ];
    pub const ALL_NORMAL: [NoteSpeed; 7] = [
        NoteSpeed::Whole,
        NoteSpeed::Half,
        NoteSpeed::Quarter,
        NoteSpeed::Eighth,
        NoteSpeed::Sixteenth,
        NoteSpeed::ThirtySecond,
        NoteSpeed::SixtyFourth,
    ];
    pub const ALL_DOTTED: [NoteSpeed; 7] = [
        NoteSpeed::WholeDotted,
        NoteSpeed::HalfDotted,
        NoteSpeed::QuarterDotted,
        NoteSpeed::EighthDotted,
        NoteSpeed::SixteenthDotted,
        NoteSpeed::ThirtySecondDotted,
        NoteSpeed::SixtyFourthDotted,
    ];
    pub const ALL_TRIPLET: [NoteSpeed; 7] = [
        NoteSpeed::WholeTriplet,
        NoteSpeed::HalfTriplet,
        NoteSpeed::QuarterTriplet,
        NoteSpeed::EighthTriplet,
        NoteSpeed::SixteenthTriplet,
        NoteSpeed::ThirtySecondTriplet,
        NoteSpeed::SixtyFourthTriplet,
    ];

    pub fn get_beats(&self) -> f32 {
        match self {
            NoteSpeed::WholeTriplet => 8.0 / 3.0,
            NoteSpeed::Whole => 4.0,
            NoteSpeed::WholeDotted => 6.0,
            NoteSpeed::HalfTriplet => 4.0 / 3.0,
            NoteSpeed::Half => 2.0,
            NoteSpeed::HalfDotted => 3.0,
            NoteSpeed::QuarterTriplet => 2.0 / 3.0,
            NoteSpeed::Quarter => 1.0,
            NoteSpeed::QuarterDotted => 1.5,
            NoteSpeed::EighthTriplet => 1.0 / 3.0,
            NoteSpeed::Eighth => 0.5,
            NoteSpeed::EighthDotted => 0.75,
            NoteSpeed::SixteenthTriplet => 0.5 / 3.0,
            NoteSpeed::Sixteenth => 0.25,
            NoteSpeed::SixteenthDotted => 0.375,
            NoteSpeed::ThirtySecondTriplet => 0.25 / 3.0,
            NoteSpeed::ThirtySecond => 0.125,
            NoteSpeed::ThirtySecondDotted => 0.1875,
            NoteSpeed::SixtyFourthTriplet => 0.125 / 3.0,
            NoteSpeed::SixtyFourth => 0.0625,
            NoteSpeed::SixtyFourthDotted => 0.09375,
        }
    }
}
