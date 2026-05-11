use rand::Rng;

use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct Weird<I: SoundNode> {
    source: I,
    rules: Vec<fn(f32) -> f32>,
    current_rule: [usize; 2],
    rule_change_counter: [usize; 2],
}

impl<I: SoundNode> Weird<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        let rule1 = |sample: f32| sample.abs().sin() * 0.7;
        let rule2 = |sample: f32| (sample * 3.0).fract() * 2.0 - 1.0;
        let rule3 = |sample: f32| {
            if sample > 0.0 {
                sample.sqrt()
            } else {
                -(-sample).sqrt()
            }
        };
        Self {
            source,
            rules: vec![rule1, rule2, rule3],
            current_rule: [0, 0],
            rule_change_counter: [0, 0],
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Weird<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let channel_idx = channel as usize;
        self.rule_change_counter[channel_idx] += rand::thread_rng().gen_range(1..4);
        if self.rule_change_counter[channel_idx] > 4410 {
            self.current_rule[channel_idx] = (self.current_rule[channel_idx]
                + rand::thread_rng().gen_range(1..4))
                % self.rules.len();
            self.rule_change_counter[channel_idx] = 0;
        }
        if let Some(sample) = self.source.next(index, channel) {
            let rule = self.rules[self.current_rule[channel_idx]];
            Some(rule(sample))
        } else {
            None
        }
    }
}
