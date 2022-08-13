use rodio::Sample;
use noise::{Perlin, Seedable, NoiseFn};

pub fn sin_wave_table(wave_table_size: usize) -> Vec<f32>{
    (0..wave_table_size).map(|n| (2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin()).collect()
}

pub fn square_wave_table(wave_table_size: usize) -> Vec<f32> {
    sin_wave_table(wave_table_size).iter().map(|f| f.signum()).collect()
}

pub fn sawtooth_wave_table(wave_table_size: usize) -> Vec<f32>{
    (0..wave_table_size).map(|n| (n as f32 / wave_table_size as f32) % 1.0).collect()
}

pub fn triangle_wave_table(wave_table_size: usize) -> Vec<f32>{
    add(sawtooth_wave_table(wave_table_size), -0.5).iter().map(|f| f.abs()).collect()
}

pub fn change_amplitude(sample: Vec<f32>, amplitude: f32) -> Vec<f32> {
    sample.iter().map(|f| f.amplify(amplitude)).collect()
}

pub fn combine(sample: Vec<f32>, sample2: Vec<f32>) -> Vec<f32>{
    sample.iter().zip(sample2).map(|z| z.0 + z.1).collect()
}

pub fn add(sample: Vec<f32>, value: f32) -> Vec<f32>{
    sample.iter().map(|f| f + value).collect()
}

pub fn perlin_wave_table(wave_table_size: usize, seed: u32)-> Vec<f32>{
    let per = Perlin::new();
    per.set_seed(seed);
    (0..wave_table_size).map(|n| per.get([(n as f64 / wave_table_size as f64), 0.0]) as f32).collect()
}