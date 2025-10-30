use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Hold<I: DawSource> {
    source: I,
    hold_length: u32,
    counter: u32,
    held_value: [f32; 2],
}

impl<I: DawSource> Hold<I> {
    pub fn new(
        source: I,
        hold_length: f32,
        sample_rate: f32,
        speed: f32,
        uses_speed: bool,
    ) -> Self {
        Self {
            source,
            hold_length: (hold_length / 100.0 * sample_rate * if uses_speed { speed } else { 1.0 })
                .round() as u32,
            counter: 0,
            held_value: [0.0; 2],
        }
    }
}

impl<I: DawSource + Clone> DawSource for Hold<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let next = self.source.next(index, channel)?;
        let ch = channel as usize;
        self.counter += 1;
        if self.counter >= self.hold_length {
            self.counter = 0;
            self.held_value[ch] = next;
        }
        Some(self.held_value[ch])
    }
}
