use noise::{NoiseFn, Perlin, Seedable};

pub fn sin_wave_table(samples: usize) -> Vec<f32> {
    (0..samples)
        .map(|n| (2.0 * std::f32::consts::PI * n as f32 / samples as f32).sin())
        .collect()
}

pub fn square_wave_table(samples: usize) -> Vec<f32> {
    sin_wave_table(samples).iter().map(|f| f.signum()).collect()
}

pub fn sawtooth_wave_table(samples: usize) -> Vec<f32> {
    (0..samples)
        .map(|n| (n as f32 / samples as f32) % 1.0)
        .collect()
}

pub fn triangle_wave_table(samples: usize) -> Vec<f32> {
    sawtooth_wave_table(samples)
        .iter()
        .map(|f| (0.5 * f).abs())
        .collect()
}

pub fn perlin_wave_table(wave_table_size: usize, seed: u32) -> Vec<f32> {
    let per = Perlin::new();
    per.set_seed(seed);
    (0..wave_table_size)
        .map(|n| per.get([(n as f64 / wave_table_size as f64), 0.0]) as f32)
        .collect()
}
