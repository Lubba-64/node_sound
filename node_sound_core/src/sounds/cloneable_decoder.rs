use std::io::Cursor;

use rodio::{Decoder, Source, source::SamplesConverter};

pub struct CloneableDecoder {
    pub data: Vec<u8>,
    pub decoder: SamplesConverter<Decoder<Cursor<Vec<u8>>>, f32>,
}

impl CloneableDecoder {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data.clone(),
            decoder: Decoder::new(Cursor::new(data))
                .expect("expect valid wav data")
                .convert_samples(),
        }
    }
}

impl Clone for CloneableDecoder {
    fn clone(&self) -> Self {
        CloneableDecoder {
            data: self.data.clone(),
            decoder: Decoder::new(Cursor::new(self.data.clone()))
                .expect("expect valid wav data")
                .convert_samples(),
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
