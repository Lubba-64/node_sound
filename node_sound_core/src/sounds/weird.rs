use rand::Rng;

use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct Weird<I: DawSource> {
    source: I,
    rules: Vec<fn(f32) -> f32>,
    current_rule: [usize; 2],
    rule_change_counter: [usize; 2],
}

impl<I: DawSource> Weird<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        let rule1 = |x: f32| x.abs().sin() * 0.7;
        let rule2 = |x: f32| (x * 3.0).fract() * 2.0 - 1.0;
        let rule3 = |x: f32| if x > 0.0 { x.sqrt() } else { -(-x).sqrt() };
        Self {
            source,
            rules: vec![rule1, rule2, rule3],
            current_rule: [0, 0],
            rule_change_counter: [0, 0],
        }
    }
}

impl<I: DawSource + Clone> DawSource for Weird<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let channel_idx = channel as usize;
        self.rule_change_counter[channel_idx] += rand::thread_rng().gen_range(1..4);
        if self.rule_change_counter[channel_idx] > 4410 {
            self.current_rule[channel_idx] = (self.current_rule[channel_idx]
                + rand::thread_rng().gen_range(1..4))
                % self.rules.len();
            self.rule_change_counter[channel_idx] = 0;
        }
        if let Some(x) = self.source.next(index, channel) {
            let rule = self.rules[self.current_rule[channel_idx]];
            Some(rule(x))
        } else {
            None
        }
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.source.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        self.source.size_hint()
    }
}
