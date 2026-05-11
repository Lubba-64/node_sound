pub mod abs_node;
pub mod after_node;
pub mod amplify_node;
pub mod automated_bpm_sync_node;
pub mod automated_clamp_node;
pub mod automated_delay_repeat_node;
pub mod automated_duration_node;
pub mod automated_hold_node;
pub mod automated_mod_node;
pub mod automated_mod_raw_node;
pub mod automated_sawtooth_node;
pub mod automated_sine_node;
pub mod automated_skip_node;
pub mod automated_speed_node;
pub mod automated_square_node;
pub mod automated_translate_node;
pub mod automated_triangle_node;
pub mod automated_wave_shaper_node;
pub mod automated_wave_table_node;
pub mod avg_node;
pub mod bit_crush_node;
pub mod bpm_sync_node;
pub mod bpm_sync_source_node;
pub mod clamp_node;
pub mod clamp_to_note_node;
pub mod const_node;
pub mod daw_automation_mix_node;
pub mod daw_automation_source_node;
pub mod delay_node;
pub mod delay_repeat_node;
pub mod duration_node;
pub mod eq_node;
pub mod file_node;
pub mod flip_node;
pub mod glitch_node;
pub mod grain_node;
pub mod hold_node;
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
pub mod switch_node;
pub mod tracker_node;
pub mod translate_node;
pub mod triangle_node;
pub mod unison_node;
pub mod vertical_wave_shaper_node;
pub mod wave_folder_node;
pub mod wave_shaper_node;
pub mod wave_table_node;
pub mod weird_node;
pub mod wrapper_node;

use crate::error::Result;
use crate::{
    sound_graph::{
        graph::SoundNodeGraphState,
        graph_types::{InputParameter, Output, ValueType},
    },
    sound_map::{GenericSoundNode, SoundNode},
    sounds::{tracker::TrackerNote, wave_table::WaveTableManager},
};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
    time::Duration,
};
use synthrs::midi::MidiSong;

pub struct SoundNodeProps<'a> {
    pub inputs: HashMap<String, ValueType>,
    pub state: &'a mut SoundNodeGraphState,
}

impl<'a> SoundNodeProps<'a> {
    fn push_sound(&mut self, sound: Box<dyn SoundNode>) -> usize {
        self.state.runtime_state.queue.push_sound(sound)
    }

    fn clone_sound(&mut self, idx: usize) -> Result<GenericSoundNode> {
        self.state.runtime_state.queue.clone_sound(idx)
    }

    fn get_node_idx(&self) -> usize {
        self.state.runtime_state.queue.sound_queue_len()
    }

    fn wavetables(&mut self) -> &mut WaveTableManager {
        &mut self.state.user_state.wavetables
    }

    fn update_wavetables_node_idx(&mut self) {
        let idx = self.get_node_idx();
        self.wavetables().set_current_id(idx);
    }

    fn sample_rate(&self) -> f32 {
        self.state.runtime_state.queue.get_sample_rate()
    }

    fn note_speed(&self) -> f32 {
        self.state.runtime_state.queue.get_note_speed()
    }

    fn bpm(&self) -> Arc<Mutex<f32>> {
        self.state.runtime_state.queue.get_bpm()
    }

    fn get_float(&self, name: &str) -> Result<f32> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_float()?)
    }
    fn get_bool(&self, name: &str) -> Result<bool> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_bool()?)
    }
    fn get_source(&self, name: &str) -> Result<usize> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_source()?)
    }
    fn get_duration(&self, name: &str) -> Result<Duration> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_duration()?)
    }
    fn get_file(&self, name: &str) -> Result<Option<(String, Vec<u8>)>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_file()?)
    }
    fn get_midi(&self, name: &str) -> Result<Option<(String, MidiSong)>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_midi()?)
    }
    fn get_graph(&self, name: &str) -> Result<Option<Vec<f32>>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_graph()?)
    }
    fn get_dropdown(&self, name: &str) -> Result<String> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_dropdown()?)
    }
    fn get_tracker(&self, name: &str) -> Result<Vec<TrackerNote>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_tracker()?)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SoundNodeMetadata {
    pub name: String,
    pub tooltip: String,
    pub inputs: BTreeMap<String, InputParameter>,
    pub outputs: BTreeMap<String, Output>,
    #[serde(skip)]
    pub op: Option<Arc<dyn Fn(SoundNodeProps) -> Result<BTreeMap<String, ValueType>>>>,
}

impl SoundNodeMetadata {
    pub fn run(&self, props: SoundNodeProps) -> SoundNodeResult {
        self.op
            .as_ref()
            .and_then(|op| Some(op(props)))
            .ok_or(anyhow!("run function not precent"))?
    }
}

type SoundNodeResult = Result<BTreeMap<String, ValueType>>;

#[derive(Clone)]
pub struct NodeDefinitions(pub Vec<SoundNodeMetadata>);

impl NodeDefinitions {
    pub fn get_node(&self, name: String) -> Option<&SoundNodeMetadata> {
        self.0.iter().filter(|node| node.name == name).nth(0)
    }
}

impl Default for NodeDefinitions {
    fn default() -> Self {
        get_nodes()
    }
}

pub fn get_nodes() -> NodeDefinitions {
    let nodes: Vec<SoundNodeMetadata> = vec![
        sawtooth_node::sawtooth_node(),
        sine_node::sine_node(),
        square_node::square_node(),
        triangle_node::triangle_node(),
        mix_node::mix_node(),
        minus_node::minus_node(),
        const_node::const_node(),
        speed_node::speed_node(),
        lfo_node::lfo_node(),
        flip_node::flip_node(),
        output_node::output_node(),
        wrapper_node::wrapper_node(),
        wave_table_node::wave_table_node(),
        wave_shaper_node::wave_shaper_node(),
        translate_node::translate_node(),
        automated_triangle_node::automated_triangle_node(),
        automated_sawtooth_node::automated_sawtooth_node(),
        automated_sine_node::automated_sine_node(),
        automated_square_node::automated_square_node(),
        midi_node::midi_node(),
        split_channels_node::split_channels_node(),
        merge_channels_node::merge_channels_node(),
        reverse_node::reverse_node(),
        repeat_infinite::repeat_infinite_node(),
        repeat_n_node::repeat_n_node(),
        file_node::file_node(),
        skip_node::skip_node(),
        delay_node::delay_node(),
        amplify_node::amplify_node(),
        reverb_node::reverb_node(),
        noise_node::noise_node(),
        mod_node::mod_node(),
        mod_raw_node::mod_raw_node(),
        daw_automation_source_node::daw_automation_source_node(),
        clamp_node::clamp_node(),
        abs_node::abs_node(),
        automated_clamp_node::automated_clamp_node(),
        automated_mod_node::automated_mod_node(),
        automated_mod_raw_node::automated_mod_raw_node(),
        automated_translate_node::automated_translate_node(),
        duration_node::duration_node(),
        bit_crush_node::bit_crush_node(),
        automated_wave_shaper_node::automated_wave_shaper_node(),
        automated_wave_table_node::automated_wave_table_node(),
        weird_node::weird_node(),
        no_op_node::no_op_node(),
        signum_node::signum_node(),
        vertical_wave_shaper_node::vertical_wave_shaper_node(),
        random_duration_node::random_duration_node(),
        avg_node::avg_node(),
        input_node::input_node(),
        ref_node::ref_node(),
        bpm_sync_node::bpm_sync_node(),
        bpm_sync_source_node::bpm_sync_source_node(),
        tracker_node::tracker_node(),
        eq_node::eq_node(),
        unison_node::unison_node(),
        daw_automation_mix_node::daw_automation_mix_node(),
        automated_speed_node::automated_speed_node(),
        after_node::after_node(),
        hold_node::hold_node(),
        switch_node::switch_node(),
        automated_hold_node::automated_hold_node(),
        delay_repeat_node::delay_repeat_node(),
        wave_folder_node::wave_folder_node(),
        automated_delay_repeat_node::automated_delay_repeat_node(),
        automated_duration_node::automated_duration_node(),
        automated_skip_node::automated_skip_node(),
        grain_node::grain_node(),
        glitch_node::glitch_node(),
        clamp_to_note_node::clamp_to_note_node(),
        automated_bpm_sync_node::automated_bpm_sync_node(),
    ];
    NodeDefinitions(nodes)
}
