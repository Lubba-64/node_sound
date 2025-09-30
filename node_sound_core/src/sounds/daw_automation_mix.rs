use crate::sound_map::DawSource;
use crate::sound_map::GenericSource;
use crate::sounds::const_wave::ConstWave;
use crate::sounds::daw_automation_source::DawAutomationChannel;
use crate::sounds::lfo::Lfo;
use crate::sounds::minus::Minus;
use crate::sounds::mix::Mix;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct DawAutomationMix {
    source: GenericSource,
}

impl DawAutomationMix {
    #[inline]
    pub fn new<S: DawSource + Clone + 'static, S2: DawSource + Clone + 'static>(
        channel: Arc<Mutex<f32>>,
        audio1: S,
        audio2: S2,
    ) -> Self {
        let channel = DawAutomationChannel::new(channel);
        Self {
            source: GenericSource::new(Box::new(Mix::new(
                Lfo::new(Minus::new(channel.clone(), ConstWave::new(1.0)), audio1),
                Lfo::new(channel, audio2),
            ))),
        }
    }
}

impl DawSource for DawAutomationMix {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel)
    }
    fn size_hint(&self) -> Option<f32> {
        self.source.size_hint()
    }
}
