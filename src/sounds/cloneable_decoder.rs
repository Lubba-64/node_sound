use std::{fs::File, io::BufReader};

use rodio::{source::SamplesConverter, Decoder, Source};

pub struct CloneableDecoder {
    pub path: String,
    pub decoder: SamplesConverter<Decoder<BufReader<File>>, f32>,
}

impl Clone for CloneableDecoder {
    fn clone(&self) -> Self {
        CloneableDecoder {
            path: self.path.clone(),
            decoder: Decoder::new(BufReader::new(File::open(self.path.clone()).unwrap()))
                .unwrap()
                .convert_samples::<f32>(),
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
