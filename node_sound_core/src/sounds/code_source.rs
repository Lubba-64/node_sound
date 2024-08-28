use rodio::Source;
use rune::runtime::{RuntimeContext, Shared};
use rune::Unit;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Sources, Vm};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct CodeSource<
    I1: Source<Item = f32>,
    I2: Source<Item = f32>,
    I3: Source<Item = f32>,
    I4: Source<Item = f32>,
    I5: Source<Item = f32>,
> {
    input1: UniformSourceIterator<I1, I1::Item>,
    input2: UniformSourceIterator<I2, I2::Item>,
    input3: UniformSourceIterator<I3, I3::Item>,
    input4: UniformSourceIterator<I4, I4::Item>,
    input5: UniformSourceIterator<I5, I5::Item>,
    code: Arc<Unit>,
    memory: Vec<f64>,
    runtime: Arc<RuntimeContext>,
}

impl<
        I1: Source<Item = f32>,
        I2: Source<Item = f32>,
        I3: Source<Item = f32>,
        I4: Source<Item = f32>,
        I5: Source<Item = f32>,
    > CodeSource<I1, I2, I3, I4, I5>
{
    #[inline]
    pub fn new(
        source1: I1,
        source2: I2,
        source3: I3,
        source4: I4,
        source5: I5,
        code: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let context = Context::with_default_modules()?;
        let runtime = Arc::new(context.runtime()?);
        Ok(Self {
            input1: UniformSourceIterator::new(source1, 2, DEFAULT_SAMPLE_RATE),
            input2: UniformSourceIterator::new(source2, 2, DEFAULT_SAMPLE_RATE),
            input3: UniformSourceIterator::new(source3, 2, DEFAULT_SAMPLE_RATE),
            input4: UniformSourceIterator::new(source4, 2, DEFAULT_SAMPLE_RATE),
            input5: UniformSourceIterator::new(source5, 2, DEFAULT_SAMPLE_RATE),
            code: Arc::new(setup(code, context)?),
            runtime,
            memory: vec![],
        })
    }
}

impl<
        I1: Source<Item = f32>,
        I2: Source<Item = f32>,
        I3: Source<Item = f32>,
        I4: Source<Item = f32>,
        I5: Source<Item = f32>,
    > Iterator for CodeSource<I1, I2, I3, I4, I5>
{
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let mut vm = Vm::new(self.runtime.clone(), self.code.clone());
        let result = rune::from_value::<Option<(f64, Vec<f64>)>>(
            vm.call(
                ["process"],
                (
                    self.input1.next(),
                    self.input2.next(),
                    self.input3.next(),
                    self.input4.next(),
                    self.input5.next(),
                    self.memory.clone(),
                ),
            )
            .unwrap_or(rune::Value::Option(Shared::new(None).unwrap())),
        )
        .unwrap_or(Some((0.0, vec![])));
        return match result {
            Some(x) => {
                self.memory = x.1;
                Some(x.0 as f32)
            }
            None => Some(0.0),
        };
    }
}

impl<
        I1: Source<Item = f32>,
        I2: Source<Item = f32>,
        I3: Source<Item = f32>,
        I4: Source<Item = f32>,
        I5: Source<Item = f32>,
    > Source for CodeSource<I1, I2, I3, I4, I5>
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        2
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        DEFAULT_SAMPLE_RATE
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn setup(code: String, context: Context) -> Result<Unit, Box<dyn std::error::Error>> {
    let mut sources = Sources::new();
    sources.insert(rune::Source::memory(code)?)?;

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build()?;

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }
    return Ok(result);
}
