use rodio::Source;

pub fn samples_from_source(mut source: impl Source<Item = f32>, limit: usize) -> Vec<f32> {
    let mut output = vec![];
    let mut next = source.next();
    let mut idx = 0;
    while next.is_some() && idx < limit {
        idx += 1;
        output.push(next.unwrap_or(0.0));
        next = source.next();
    }
    return output;
}
pub fn to_semitones(f1: f32, f2: f32) -> f32 {
    12.0 * f32::log2(f2 / f1)
}
pub fn from_semitones(f2: f32, n: f32) -> f32 {
    f2 / 2.0_f32.powf(n / 12.0)
}
