use crate::{
    sound_graph::{
        graph::SoundNodeGraphState,
        graph_types::{InputParameter, Output, ValueType},
    },
    sound_map::{DawSource, GenericSource},
    sounds::wave_table::WaveTableManager,
};
use eframe::egui::ahash::HashMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use synthrs::midi::MidiSong;
pub mod abs_node;
pub mod amplify_node;
pub mod automated_clamp_node;
pub mod automated_mod_node;
pub mod automated_mod_raw_node;
pub mod automated_sawtooth_node;
pub mod automated_sine_node;
pub mod automated_square_node;
pub mod automated_translate_node;
pub mod automated_triangle_node;
pub mod automated_wave_shaper_node;
pub mod automated_wave_table_node;
pub mod avg_node;
pub mod bit_crush_node;
pub mod bpm_sync_node;
pub mod clamp_node;
pub mod const_node;
pub mod daw_automation_source_node;
pub mod delay_node;
pub mod duration_node;
pub mod eq_node;
pub mod file_node;
pub mod flip_node;
pub mod input_node;
pub mod lfo_node;
pub mod merge_channels_node;
pub mod midi_node;
pub mod minus_node;
pub mod mix_node;
pub mod mod_node;
pub mod mod_raw_node;
pub mod no_op_node;
pub mod noise_node;
pub mod output_node;
pub mod random_duration_node;
pub mod ref_node;
pub mod repeat_infinite;
pub mod repeat_n_node;
pub mod reverb_node;
pub mod reverse_node;
pub mod sawtooth_node;
pub mod signum_node;
pub mod sine_node;
pub mod skip_node;
pub mod speed_node;
pub mod split_channels_node;
pub mod square_node;
pub mod translate_node;
pub mod triangle_node;
pub mod unison_node;
pub mod vertical_wave_shaper_node;
pub mod wave_shaper_node;
pub mod wave_table_node;
pub mod weird_node;
pub mod wrapper_node;

pub struct SoundNodeProps<'a> {
    pub inputs: HashMap<String, ValueType>,
    pub state: &'a mut SoundNodeGraphState,
}

impl<'a> SoundNodeProps<'a> {
    fn push_sound(&mut self, sound: Box<dyn DawSource>) -> usize {
        self.state._unserializeable_state.queue.push_sound(sound)
    }

    fn clone_sound(&mut self, idx: usize) -> Result<GenericSource, Box<dyn std::error::Error>> {
        self.state._unserializeable_state.queue.clone_sound(idx)
    }

    fn get_node_idx(&self) -> usize {
        self.state._unserializeable_state.queue.sound_queue_len()
    }

    fn wavetables(&mut self) -> &mut WaveTableManager {
        &mut self.state.user_state.wavetables
    }

    fn update_wavetables_node_idx(&mut self) {
        let idx = self.get_node_idx();
        self.wavetables().set_current_id(idx);
    }

    fn sample_rate(&self) -> f32 {
        self.state._unserializeable_state.queue.get_sample_rate()
    }

    fn note_speed(&self) -> f32 {
        self.state._unserializeable_state.queue.get_note_speed()
    }

    fn bpm(&self) -> Arc<Mutex<f32>> {
        self.state._unserializeable_state.queue.get_bpm()
    }

    fn get_float(&self, name: &str) -> Result<f32, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_float()?)
    }
    fn get_bool(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_bool()?)
    }
    fn get_source(&self, name: &str) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_source()?)
    }
    fn get_duration(&self, name: &str) -> Result<Duration, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_duration()?)
    }
    fn get_file(
        &self,
        name: &str,
    ) -> Result<Option<(String, Vec<u8>)>, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_file()?)
    }
    fn get_midi(
        &self,
        name: &str,
    ) -> Result<Option<(String, MidiSong)>, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_midi()?)
    }
    fn get_graph(&self, name: &str) -> Result<Option<Vec<f32>>, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_graph()?)
    }
    fn get_dropdown(&self, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_dropdown()?)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SoundNode {
    pub name: String,
    pub tooltip: String,
    pub inputs: BTreeMap<String, InputParameter>,
    pub outputs: BTreeMap<String, Output>,
}
type SoundNodeOp =
    fn(SoundNodeProps) -> Result<BTreeMap<String, ValueType>, Box<dyn std::error::Error>>;
type SoundNodeResult = Result<BTreeMap<String, ValueType>, Box<dyn std::error::Error>>;

#[derive(Clone)]
pub struct NodeDefinitions(pub BTreeMap<String, (SoundNode, Box<SoundNodeOp>)>);

impl Default for NodeDefinitions {
    fn default() -> Self {
        get_nodes()
    }
}

pub fn get_nodes() -> NodeDefinitions {
    let nodes: Vec<(SoundNode, Box<SoundNodeOp>)> = vec![
        (
            sawtooth_node::sawtooth_node(),
            Box::new(sawtooth_node::sawtooth_logic),
        ),
        (sine_node::sine_node(), Box::new(sine_node::sine_logic)),
        (
            square_node::square_node(),
            Box::new(square_node::square_logic),
        ),
        (
            triangle_node::triangle_node(),
            Box::new(triangle_node::triangle_logic),
        ),
        (mix_node::mix_node(), Box::new(mix_node::mix_logic)),
        (minus_node::minus_node(), Box::new(minus_node::minus_logic)),
        (const_node::const_node(), Box::new(const_node::const_logic)),
        (speed_node::speed_node(), Box::new(speed_node::speed_logic)),
        (lfo_node::lfo_node(), Box::new(lfo_node::lfo_logic)),
        (flip_node::flip_node(), Box::new(flip_node::flip_logic)),
        (
            output_node::output_node(),
            Box::new(output_node::output_logic),
        ),
        (
            wrapper_node::wrapper_node(),
            Box::new(wrapper_node::wrapper_logic),
        ),
        (
            wave_table_node::wave_table_node(),
            Box::new(wave_table_node::wave_table_logic),
        ),
        (
            wave_shaper_node::wave_shaper_node(),
            Box::new(wave_shaper_node::wave_shaper_logic),
        ),
        (
            translate_node::translate_node(),
            Box::new(translate_node::translate_logic),
        ),
        (
            automated_triangle_node::automated_triangle_node(),
            Box::new(automated_triangle_node::automated_triangle_logic),
        ),
        (
            automated_sawtooth_node::automated_sawtooth_node(),
            Box::new(automated_sawtooth_node::automated_sawtooth_logic),
        ),
        (
            automated_sine_node::automated_sine_node(),
            Box::new(automated_sine_node::automated_sine_logic),
        ),
        (
            automated_square_node::automated_square_node(),
            Box::new(automated_square_node::automated_square_logic),
        ),
        (midi_node::midi_node(), Box::new(midi_node::midi_logic)),
        (
            split_channels_node::split_channels_node(),
            Box::new(split_channels_node::split_channels_logic),
        ),
        (
            merge_channels_node::merge_channels_node(),
            Box::new(merge_channels_node::merge_channels_logic),
        ),
        (
            reverse_node::reverse_node(),
            Box::new(reverse_node::reverse_logic),
        ),
        (
            repeat_infinite::repeat_infinite_node(),
            Box::new(repeat_infinite::repeat_infinite_logic),
        ),
        (
            repeat_n_node::repeat_n_node(),
            Box::new(repeat_n_node::repeat_n_logic),
        ),
        (file_node::file_node(), Box::new(file_node::file_logic)),
        (skip_node::skip_node(), Box::new(skip_node::skip_logic)),
        (delay_node::delay_node(), Box::new(delay_node::delay_logic)),
        (
            amplify_node::amplify_node(),
            Box::new(amplify_node::amplify_logic),
        ),
        (
            reverb_node::reverb_node(),
            Box::new(reverb_node::reverb_logic),
        ),
        (noise_node::noise_node(), Box::new(noise_node::noise_logic)),
        (mod_node::mod_node(), Box::new(mod_node::mod_logic)),
        (
            mod_raw_node::mod_raw_node(),
            Box::new(mod_raw_node::mod_raw_logic),
        ),
        (
            daw_automation_source_node::daw_automations_node(),
            Box::new(daw_automation_source_node::daw_automations_logic),
        ),
        (clamp_node::clamp_node(), Box::new(clamp_node::clamp_logic)),
        (abs_node::abs_node(), Box::new(abs_node::abs_logic)),
        (
            automated_clamp_node::automated_clamp_node(),
            Box::new(automated_clamp_node::automated_clamp_logic),
        ),
        (
            automated_mod_node::automated_mod_node(),
            Box::new(automated_mod_node::automated_mod_logic),
        ),
        (
            automated_mod_raw_node::automated_mod_raw_node(),
            Box::new(automated_mod_raw_node::automated_mod_raw_logic),
        ),
        (
            automated_translate_node::automated_translate_node(),
            Box::new(automated_translate_node::automated_translate_logic),
        ),
        (
            duration_node::duration_node(),
            Box::new(duration_node::duration_logic),
        ),
        (
            bit_crush_node::bit_crush_node(),
            Box::new(bit_crush_node::bit_crush_logic),
        ),
        (
            automated_wave_shaper_node::automated_wave_shaper_node(),
            Box::new(automated_wave_shaper_node::automated_wave_shaper_logic),
        ),
        (
            automated_wave_table_node::automated_wave_table_node(),
            Box::new(automated_wave_table_node::automated_wave_table_logic),
        ),
        (weird_node::weird_node(), Box::new(weird_node::weird_logic)),
        (no_op_node::no_op_node(), Box::new(no_op_node::no_op_logic)),
        (
            signum_node::signum_node(),
            Box::new(signum_node::signum_logic),
        ),
        (
            vertical_wave_shaper_node::vertical_wave_shaper_node(),
            Box::new(vertical_wave_shaper_node::vertical_wave_shaper_logic),
        ),
        (
            random_duration_node::random_duration_node(),
            Box::new(random_duration_node::random_duration_logic),
        ),
        (avg_node::avg_node(), Box::new(avg_node::avg_logic)),
        (input_node::input_node(), Box::new(input_node::input_logic)),
        (ref_node::ref_node(), Box::new(ref_node::ref_logic)),
        (
            bpm_sync_node::bpm_sync_node(),
            Box::new(bpm_sync_node::bpm_sync_logic),
        ),
        (eq_node::eq_node(), Box::new(eq_node::eq_logic)),
        (
            unison_node::unison_node(),
            Box::new(unison_node::unison_logic),
        ),
    ];
    NodeDefinitions(BTreeMap::from_iter(
        nodes.iter().map(|n| (n.0.name.clone(), n.clone())),
    ))
}
