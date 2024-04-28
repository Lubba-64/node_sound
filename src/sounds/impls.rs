use super::{
    cloneable_decoder::CloneableDecoder, lfo::Lfo, source_wrapper::DynCloneIter, Abs,
    AsGenericSource, Clamp, GenericSource, Noise, SawToothWave, SquareWave, TriangleWave,
};

use rodio::{
    source::{
        Amplify, BltFilter, Delay, FadeIn, Mix, Repeat, SamplesConverter, SineWave, SkipDuration,
        Spatial, Speed, TakeDuration, Zero,
    },
    Sample, Source,
};
trait StaticSample: Sample + Send + Clone + 'static {}
trait StaticSource<S: StaticSample>: Source<Item = S> + Send + Clone + 'static {}
impl StaticSample for f32 {}
impl StaticSample for u16 {}
impl StaticSample for i16 {}

impl DynCloneIter<f32> for SineWave {}
impl AsGenericSource<f32> for SineWave {}
impl StaticSource<f32> for SineWave {}
impl DynCloneIter<f32> for SquareWave {}
impl AsGenericSource<f32> for SquareWave {}
impl StaticSource<f32> for SquareWave {}
impl DynCloneIter<f32> for TriangleWave {}
impl AsGenericSource<f32> for TriangleWave {}
impl StaticSource<f32> for TriangleWave {}
impl DynCloneIter<f32> for SawToothWave {}
impl AsGenericSource<f32> for SawToothWave {}
impl StaticSource<f32> for SawToothWave {}
impl<I> DynCloneIter<f32> for Amplify<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for Amplify<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for Amplify<I> where I: StaticSource<f32> {}
impl<I> DynCloneIter<f32> for BltFilter<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for BltFilter<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for BltFilter<I> where I: StaticSource<f32> {}
impl<I> DynCloneIter<f32> for Delay<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for Delay<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for Delay<I> where I: StaticSource<f32> {}
impl<D: StaticSample> DynCloneIter<D> for Zero<D> {}
impl<D: StaticSample> AsGenericSource<D> for Zero<D> {}
impl<D: StaticSample> StaticSource<D> for Zero<D> {}
impl<I> DynCloneIter<f32> for FadeIn<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for FadeIn<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for FadeIn<I> where I: StaticSource<f32> {}
impl<I1, I2> DynCloneIter<f32> for Mix<I1, I2>
where
    I1: StaticSource<f32>,
    I2: StaticSource<f32>,
{
}
impl<I1, I2> AsGenericSource<f32> for Mix<I1, I2>
where
    I1: StaticSource<f32>,
    I2: StaticSource<f32>,
{
}
impl<I1, I2> StaticSource<f32> for Mix<I1, I2>
where
    I1: StaticSource<f32>,
    I2: StaticSource<f32>,
{
}
impl<I1, I2> DynCloneIter<f32> for Lfo<I1, I2>
where
    I1: StaticSource<f32>,
    I2: StaticSource<f32>,
{
}
impl<I1, I2> AsGenericSource<f32> for Lfo<I1, I2>
where
    I1: StaticSource<f32>,
    I2: StaticSource<f32>,
{
}
impl<I1, I2> StaticSource<f32> for Lfo<I1, I2>
where
    I1: StaticSource<f32>,
    I2: StaticSource<f32>,
{
}
impl<I> DynCloneIter<f32> for Repeat<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for Repeat<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for Repeat<I> where I: StaticSource<f32> {}
impl<I> DynCloneIter<f32> for SkipDuration<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for SkipDuration<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for SkipDuration<I> where I: StaticSource<f32> {}
impl<I> DynCloneIter<f32> for Spatial<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for Spatial<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for Spatial<I> where I: StaticSource<f32> {}
impl<I> DynCloneIter<f32> for Speed<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for Speed<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for Speed<I> where I: StaticSource<f32> {}
impl StaticSource<f32> for GenericSource<f32> {}
impl<I> DynCloneIter<f32> for TakeDuration<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for TakeDuration<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for TakeDuration<I> where I: StaticSource<f32> {}
impl<I> DynCloneIter<f32> for SamplesConverter<I, f32> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for SamplesConverter<I, f32> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for SamplesConverter<I, f32> where I: StaticSource<f32> {}
impl DynCloneIter<f32> for CloneableDecoder {}
impl AsGenericSource<f32> for CloneableDecoder {}
impl StaticSource<f32> for CloneableDecoder {}
impl<I> DynCloneIter<f32> for Clamp<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for Clamp<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for Clamp<I> where I: StaticSource<f32> {}
impl<I> DynCloneIter<f32> for Abs<I> where I: StaticSource<f32> {}
impl<I> AsGenericSource<f32> for Abs<I> where I: StaticSource<f32> {}
impl<I> StaticSource<f32> for Abs<I> where I: StaticSource<f32> {}
impl DynCloneIter<f32> for Noise {}
impl AsGenericSource<f32> for Noise {}
impl StaticSource<f32> for Noise {}
