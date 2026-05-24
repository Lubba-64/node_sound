// instead of sample being from -1.0 to 1.0, it gets transformed to from end_min to end_max
pub fn scale_param(sample: f32, end_min: f32, end_max: f32) -> f32 {
    end_min + (end_max - end_min) * (sample + 1.0) / 2.0
}
