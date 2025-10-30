use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct AutomatedHold<I: DawSource, I2: DawSource> {
    source: I,
    hold_length: I2,
    speed: f32,
    counter: u32,
    sample_rate: f32,
    held_value: [f32; 2],
}

impl<I: DawSource, I2: DawSource> AutomatedHold<I, I2> {
    pub fn new(source: I, hold_length: I2, sample_rate: f32, speed: f32, uses_speed: bool) -> Self {
        Self {
            source,
            hold_length,
            counter: 0,
            sample_rate,
            speed: if uses_speed { speed } else { 1.0 },
            held_value: [0.0; 2],
        }
    }
}

impl<I: DawSource + Clone, I2: DawSource + Clone> DawSource for AutomatedHold<I, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let next = self.source.next(index, channel)?;
        let ch = channel as usize;
        self.counter += 1;
        if self.counter
            >= (self.hold_length.next(index, channel).unwrap_or_default() / 100.0
                * self.sample_rate
                * self.speed)
                .round() as u32
        {
            self.counter = 0;
            self.held_value[ch] = next;
        }
        Some(self.held_value[ch])
    }
}
