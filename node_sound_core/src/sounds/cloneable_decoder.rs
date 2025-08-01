use std::io::Cursor;

use rodio::{
    Decoder, Source,
    source::{SamplesConverter, Speed},
};

use crate::sound_map::SetSpeed;

pub struct CloneableDecoder {
    pub data: Vec<u8>,
    pub decoder: Speed<SamplesConverter<Decoder<Cursor<Vec<u8>>>, f32>>,
    uses_speed: bool,
    speed: f32,
}

impl CloneableDecoder {
    pub fn new(data: Vec<u8>, uses_speed: bool) -> Self {
        Self {
            data: data.clone(),
            decoder: Decoder::new(Cursor::new(data))
                .expect("expect valid wav data")
                .convert_samples()
                .speed(1.0),
            speed: 1.0,
            uses_speed,
        }
    }
}

impl Clone for CloneableDecoder {
    fn clone(&self) -> Self {
        CloneableDecoder {
            data: self.data.clone(),
            decoder: Decoder::new(Cursor::new(self.data.clone()))
                .expect("expect valid wav data")
                .convert_samples()
                .speed(1.0 / self.speed),
            speed: self.speed,
            uses_speed: self.uses_speed,
        }
    }
}

impl Iterator for CloneableDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.decoder.next()
    }
}

impl Source for CloneableDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        self.decoder.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.decoder.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.decoder.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.decoder.total_duration()
    }
}

impl SetSpeed<f32> for CloneableDecoder {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
        self.decoder = Decoder::new(Cursor::new(self.data.clone()))
            .expect("expect valid wav data")
            .convert_samples()
            .speed(1.0 / self.speed);
    }
}
