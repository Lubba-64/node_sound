use super::{
    lfo::{Lfo, SourceToLfo},
    source_wrapper::DynCloneIter,
    AsGenericSource, GenericSource, SawToothWave, SquareWave, TriangleWave,
};

use rodio::{
    source::{
        Amplify, BltFilter, Delay, FadeIn, Mix, Repeat, SineWave, SkipDuration, Spatial, Speed,
        TakeDuration, Zero,
    },
    Source,
};

trait StaticSource: Source<Item = f32> + Send + Clone + 'static {}

impl DynCloneIter<f32> for SineWave {}
impl AsGenericSource for SineWave {}
impl StaticSource for SineWave {}
impl DynCloneIter<f32> for SquareWave {}
impl AsGenericSource for SquareWave {}
impl StaticSource for SquareWave {}
impl DynCloneIter<f32> for TriangleWave {}
impl AsGenericSource for TriangleWave {}
impl StaticSource for TriangleWave {}
impl DynCloneIter<f32> for SawToothWave {}
impl AsGenericSource for SawToothWave {}
impl StaticSource for SawToothWave {}
impl<I> DynCloneIter<f32> for Amplify<I> where I: StaticSource {}
impl<I> AsGenericSource for Amplify<I> where I: StaticSource {}
impl<I> StaticSource for Amplify<I> where I: StaticSource {}
impl<I> DynCloneIter<f32> for BltFilter<I> where I: StaticSource {}
impl<I> AsGenericSource for BltFilter<I> where I: StaticSource {}
impl<I> StaticSource for BltFilter<I> where I: StaticSource {}
impl<I> DynCloneIter<f32> for Delay<I> where I: StaticSource {}
impl<I> AsGenericSource for Delay<I> where I: StaticSource {}
impl<I> StaticSource for Delay<I> where I: StaticSource {}
impl DynCloneIter<f32> for Zero<f32> {}
impl AsGenericSource for Zero<f32> {}
impl StaticSource for Zero<f32> {}
impl<I> DynCloneIter<f32> for FadeIn<I> where I: StaticSource {}
impl<I> AsGenericSource for FadeIn<I> where I: StaticSource {}
impl<I> StaticSource for FadeIn<I> where I: StaticSource {}
impl<I1, I2> DynCloneIter<f32> for Mix<I1, I2>
where
    I1: StaticSource,
    I2: StaticSource,
{
}
impl<I1, I2> AsGenericSource for Mix<I1, I2>
where
    I1: StaticSource,
    I2: StaticSource,
{
}
impl<I1, I2> StaticSource for Mix<I1, I2>
where
    I1: StaticSource,
    I2: StaticSource,
{
}
impl<I1, I2> DynCloneIter<f32> for Lfo<I1, I2>
where
    I1: StaticSource,
    I2: StaticSource,
{
}
impl<I1, I2> AsGenericSource for Lfo<I1, I2>
where
    I1: StaticSource,
    I2: StaticSource,
{
}
impl<I1, I2> StaticSource for Lfo<I1, I2>
where
    I1: StaticSource,
    I2: StaticSource,
{
}
impl<I> DynCloneIter<f32> for Repeat<I> where I: StaticSource {}
impl<I> AsGenericSource for Repeat<I> where I: StaticSource {}
impl<I> StaticSource for Repeat<I> where I: StaticSource {}
impl<I> DynCloneIter<f32> for SkipDuration<I> where I: StaticSource {}
impl<I> AsGenericSource for SkipDuration<I> where I: StaticSource {}
impl<I> StaticSource for SkipDuration<I> where I: StaticSource {}
impl<I> DynCloneIter<f32> for Spatial<I> where I: StaticSource {}
impl<I> AsGenericSource for Spatial<I> where I: StaticSource {}
impl<I> StaticSource for Spatial<I> where I: StaticSource {}
impl<I> DynCloneIter<f32> for Speed<I> where I: StaticSource {}
impl<I> AsGenericSource for Speed<I> where I: StaticSource {}
impl<I> StaticSource for Speed<I> where I: StaticSource {}
impl StaticSource for GenericSource<f32> {}
impl<I> DynCloneIter<f32> for TakeDuration<I> where I: StaticSource {}
impl<I> AsGenericSource for TakeDuration<I> where I: StaticSource {}
impl<I> StaticSource for TakeDuration<I> where I: StaticSource {}
