use super::super::sound_map::RefSourceIter;
use super::{
    cloneable_decoder::CloneableDecoder, lfo::Lfo, Abs, Clamp, MergeChannels, Mod, Noise, Pop,
    SawToothWave, SplitChannels, SquareWave, TriangleWave,
};

use rodio::source::{
    Amplify, BltFilter, ChannelVolume, Delay, FadeIn, Mix, Repeat, SamplesConverter, SineWave,
    SkipDuration, Spatial, Speed, TakeDuration, Zero,
};

impl RefSourceIter<f32> for SineWave {}
impl RefSourceIter<f32> for SquareWave {}
impl RefSourceIter<f32> for TriangleWave {}
impl RefSourceIter<f32> for SawToothWave {}
impl<I> RefSourceIter<f32> for Amplify<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for BltFilter<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for Delay<I> where I: RefSourceIter<f32> {}
impl RefSourceIter<f32> for Zero<f32> {}
impl<I> RefSourceIter<f32> for FadeIn<I> where I: RefSourceIter<f32> {}
impl<I1, I2> RefSourceIter<f32> for Mix<I1, I2>
where
    I1: RefSourceIter<f32>,
    I2: RefSourceIter<f32>,
{
}
impl<I1, I2> RefSourceIter<f32> for Lfo<I1, I2>
where
    I1: RefSourceIter<f32>,
    I2: RefSourceIter<f32>,
{
}
impl<I1, I2> RefSourceIter<f32> for MergeChannels<I1, I2>
where
    I1: RefSourceIter<f32>,
    I2: RefSourceIter<f32>,
{
}
impl<I> RefSourceIter<f32> for SplitChannels<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for Repeat<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for SkipDuration<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for Spatial<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for Speed<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for TakeDuration<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for SamplesConverter<I, f32> where I: RefSourceIter<f32> {}
impl RefSourceIter<f32> for CloneableDecoder {}
impl<I> RefSourceIter<f32> for Clamp<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for Abs<I> where I: RefSourceIter<f32> {}
impl RefSourceIter<f32> for Noise {}
impl RefSourceIter<f32> for Pop {}
impl<I> RefSourceIter<f32> for ChannelVolume<I> where I: RefSourceIter<f32> {}
impl<I> RefSourceIter<f32> for Mod<I> where I: RefSourceIter<f32> {}
