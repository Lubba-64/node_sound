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
