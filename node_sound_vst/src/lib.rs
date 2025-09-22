use futures::executor;
use nih_plug::{params::persist::PersistentField, prelude::*};
use nih_plug_egui::{EguiState, create_egui_editor};
use node_sound_core::sound_map::DawSource;
use node_sound_core::{
    constants::MIDDLE_C_FREQ,
    nodes::get_nodes,
    sound_graph::{
        self,
        copy_paste_del_helpers::{copy, delete_nodes, paste},
        graph::{ActiveNodeState, FileManager, SoundNodeGraph, evaluate_node},
        graph_types::ValueType,
    },
    sound_map::GenericSource,
    sounds::{const_wave::ConstWave, speed::Speed},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use util::midi_note_to_freq;

const NUM_VOICES: u32 = 16;
const GAIN_POLY_MOD_ID: u32 = 0;
const MAX_BLOCK_SIZE: usize = 64;
const MIDI_NOTES_LEN: u8 = 128;

/// Data for a single synth voice. In a real synth where performance matter, you may want to use a
/// struct of arrays instead of having a struct for each voice.
#[derive(Debug, Clone)]
struct Voice {
    /// The identifier for this voice. Polyphonic modulation events are linked to a voice based on
    /// these IDs. If the host doesn't provide these IDs, then this is computed through
    /// `compute_fallback_voice_id()`. In that case polyphonic modulation will not work, but the
    /// basic note events will still have an effect.
    voice_id: i32,
    /// The note's channel, in `0..16`. Only used for the voice terminated event.
    channel: u8,
    /// The note's key/note, in `0..128`. Only used for the voice terminated event.
    note: u8,
    /// The voices internal ID. Each voice has an internal voice ID one higher than the previous
    /// voice. This is used to steal the last voice in case all 16 voices are in use.
    internal_voice_id: u64,
    /// The square root of the note's velocity. This is used as a gain multiplier.
    _velocity_sqrt: f32,
    /// Whether the key has been released and the voice is in its release stage. The voice will be
    /// terminated when the amplitude envelope hits 0 while the note is releasing.
    releasing: bool,
    /// Fades between 0 and 1 with timings based on the global attack and release settings.
    amp_envelope: Smoother<f32>,

    /// If this voice has polyphonic gain modulation applied, then this contains the normalized
    /// offset and a smoother.
    voice_gain: Option<(f32, Smoother<f32>)>,
}

pub struct NodeSound {
    params: Arc<NodeSoundParams>,
    voices: [Option<Voice>; NUM_VOICES as usize],
    next_internal_voice_id: u64,
    sample_rate: Arc<Mutex<f32>>,
    bpm: Arc<Mutex<f32>>,
    source_sound_buffers: Arc<Mutex<[Option<GenericSource>; MIDI_NOTES_LEN as usize]>>,
    voice_sound_buffers: Arc<Mutex<[(Option<GenericSource>, usize); MIDI_NOTES_LEN as usize]>>,
}

pub struct PluginPresetState {
    graph: Arc<Mutex<SoundNodeGraph>>,
}

#[derive(Params)]
pub struct NodeSoundParams {
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,
    #[persist = "editor-preset"]
    plugin_state: PluginPresetState,
    /// A voice's gain. This can be polyphonically modulated.
    #[id = "gain"]
    gain: FloatParam,
    /// The amplitude envelope attack time. This is the same for every voice.
    #[id = "amp_atk"]
    amp_attack_ms: FloatParam,
    /// The amplitude envelope release time. This is the same for every voice.
    #[id = "amp_rel"]
    amp_release_ms: FloatParam,
    #[id = "a1"]
    pub a1: FloatParam,
    #[id = "a2"]
    pub a2: FloatParam,
    #[id = "a3"]
    pub a3: FloatParam,
    #[id = "a4"]
    pub a4: FloatParam,
    #[id = "a5"]
    pub a5: FloatParam,
    #[id = "a6"]
    pub a6: FloatParam,
    #[id = "a7"]
    pub a7: FloatParam,
    #[id = "a8"]
    pub a8: FloatParam,
    #[id = "a9"]
    pub a9: FloatParam,
    #[id = "a10"]
    pub a10: FloatParam,
    #[id = "a11"]
    pub a11: FloatParam,
    #[id = "a12"]
    pub a12: FloatParam,
    #[id = "a13"]
    pub a13: FloatParam,
    #[id = "a14"]
    pub a14: FloatParam,
    #[id = "a15"]
    pub a15: FloatParam,
    #[id = "a16"]
    pub a16: FloatParam,
    #[id = "a17"]
    pub a17: FloatParam,
    #[id = "a18"]
    pub a18: FloatParam,
    root_sound_id: Arc<Mutex<Option<usize>>>,
}

impl<'a> PersistentField<'a, String> for PluginPresetState {
    fn set(&self, new_value: String) {
        *self
            .graph
            .lock()
            .expect("expected to lock graph state on deserialize") =
            ron::de::from_str(&new_value).expect("Graph state is ok");
    }

    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&String) -> R,
    {
        f(&ron::ser::to_string(&self.graph).expect("Graph state is ok"))
    }
}

impl Default for NodeSound {
    fn default() -> Self {
        let params = NodeSoundParams::default();
        Self {
            params: Arc::new(params),
            voices: [0; NUM_VOICES as usize].map(|_| None),
            next_internal_voice_id: 0,
            sample_rate: Arc::new(Mutex::new(48000.0)),
            bpm: Arc::new(Mutex::new(120.0)),
            source_sound_buffers: Arc::new(Mutex::new([0; MIDI_NOTES_LEN as usize].map(|_| None))),
            voice_sound_buffers: Arc::new(Mutex::new(
                [0; MIDI_NOTES_LEN as usize].map(|_| (None, 0)),
            )),
        }
    }
}

macro_rules! mkparam {
    ($field: ident, $name: literal) => {
        let $field = FloatParam::new(
            $name,
            0.0,
            FloatRange::Linear {
                min: -1.0,
                max: 1.0,
            },
        )
        .with_smoother(SmoothingStyle::None)
        .with_step_size(0.01);
    };
}

impl Default for NodeSoundParams {
    fn default() -> Self {
        mkparam! {a1, "A1"}
        mkparam! {a2, "A2"}
        mkparam! {a3, "A3"}
        mkparam! {a4, "A4"}
        mkparam! {a5, "A5"}
        mkparam! {a6, "A6"}
        mkparam! {a7, "A7"}
        mkparam! {a8, "A8"}
        mkparam! {a9, "A9"}
        mkparam! {a10, "A10"}
        mkparam! {a11, "A11"}
        mkparam! {a12, "A12"}
        mkparam! {a13, "A13"}
        mkparam! {a14, "A14"}
        mkparam! {a15, "A15"}
        mkparam! {a16, "A16"}
        mkparam! {a17, "A17"}
        mkparam! {a18, "A18"}

        Self {
            editor_state: EguiState::from_size(1280, 720),
            plugin_state: PluginPresetState {
                graph: Arc::new(Mutex::new(
                    sound_graph::graph::SoundNodeGraph::new_vst_synth(),
                )),
            },
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-12.0),
                // Because we're representing gain as decibels the range is already logarithmic
                FloatRange::Linear {
                    min: util::db_to_gain(-36.0),
                    max: util::db_to_gain(0.0),
                },
            )
            .with_poly_modulation_id(GAIN_POLY_MOD_ID)
            .with_smoother(SmoothingStyle::Logarithmic(5.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            amp_attack_ms: FloatParam::new(
                "Attack",
                200.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            amp_release_ms: FloatParam::new(
                "Release",
                100.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            root_sound_id: Arc::new(Mutex::new(None)),
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
            a9,
            a10,
            a11,
            a12,
            a13,
            a14,
            a15,
            a16,
            a17,
            a18,
        }
    }
}

const fn compute_fallback_voice_id(note: u8, channel: u8) -> i32 {
    note as i32 | ((channel as i32) << 16)
}

impl NodeSound {
    /// Get the index of a voice by its voice ID, if the voice exists. This does not immediately
    /// return a reference to the voice to avoid lifetime issues.
    fn get_voice_idx(&mut self, voice_id: i32) -> Option<usize> {
        self.voices
            .iter_mut()
            .position(|voice| matches!(voice, Some(voice) if voice.voice_id == voice_id))
    }

    /// Start a new voice with the given voice ID. If all voices are currently in use, the oldest
    /// voice will be stolen. Returns a reference to the new voice.
    fn start_voice(
        &mut self,
        context: &mut impl ProcessContext<Self>,
        sample_offset: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        velocity: f32,
        sample_rate: f32,
    ) -> &mut Voice {
        let amp_envelope = Smoother::new(SmoothingStyle::Exponential(
            self.params.amp_attack_ms.value(),
        ));
        amp_envelope.reset(0.0);
        amp_envelope.set_target(sample_rate, 1.0);

        let new_voice = Voice {
            voice_id: voice_id.unwrap_or_else(|| compute_fallback_voice_id(note, channel)),
            internal_voice_id: self.next_internal_voice_id,
            channel,
            note,
            _velocity_sqrt: velocity.sqrt(),
            releasing: false,
            amp_envelope,
            voice_gain: None,
        };

        self.next_internal_voice_id = self.next_internal_voice_id.wrapping_add(1);

        match self.voices.iter().position(|voice| voice.is_none()) {
            Some(free_voice_idx) => {
                self.voices[free_voice_idx] = Some(new_voice);
                return self.voices[free_voice_idx].as_mut().unwrap();
            }
            None => {
                let oldest_voice = unsafe {
                    self.voices
                        .iter_mut()
                        .min_by_key(|voice| voice.as_ref().unwrap_unchecked().internal_voice_id)
                        .unwrap_unchecked()
                };

                {
                    let oldest_voice = oldest_voice.as_ref().unwrap();
                    context.send_event(NoteEvent::VoiceTerminated {
                        timing: sample_offset,
                        voice_id: Some(oldest_voice.voice_id),
                        channel: oldest_voice.channel,
                        note: oldest_voice.note,
                    });
                }

                *oldest_voice = Some(new_voice);
                return oldest_voice.as_mut().unwrap();
            }
        }
    }

    /// Start the release process for one or more voice by changing their amplitude envelope. If
    /// `voice_id` is not provided, then this will terminate all matching voices.
    fn start_release_for_voices(
        &mut self,
        sample_rate: f32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
    ) {
        for voice in self.voices.iter_mut() {
            match voice {
                Some(Voice {
                    voice_id: candidate_voice_id,
                    channel: candidate_channel,
                    note: candidate_note,
                    releasing,
                    amp_envelope,
                    ..
                }) if voice_id == Some(*candidate_voice_id)
                    || (channel == *candidate_channel && note == *candidate_note) =>
                {
                    *releasing = true;
                    amp_envelope.style =
                        SmoothingStyle::Exponential(self.params.amp_release_ms.value());
                    amp_envelope.set_target(sample_rate, 0.0);

                    // If this targetted a single voice ID, we're done here. Otherwise there may be
                    // multiple overlapping voices as we enabled support for that in the
                    // `PolyModulationConfig`.
                    if voice_id.is_some() {
                        // return;
                    }
                }
                _ => (),
            }
        }
    }

    /// Immediately terminate one or more voice, removing it from the pool and informing the host
    /// that the voice has ended. If `voice_id` is not provided, then this will terminate all
    /// matching voices.
    fn choke_voices(
        &mut self,
        context: &mut impl ProcessContext<Self>,
        sample_offset: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
    ) {
        for voice in self.voices.iter_mut() {
            match voice {
                Some(Voice {
                    voice_id: candidate_voice_id,
                    channel: candidate_channel,
                    note: candidate_note,
                    ..
                }) if voice_id == Some(*candidate_voice_id)
                    || (channel == *candidate_channel && note == *candidate_note) =>
                {
                    context.send_event(NoteEvent::VoiceTerminated {
                        timing: sample_offset,
                        // Notice how we always send the terminated voice ID here
                        voice_id: Some(*candidate_voice_id),
                        channel,
                        note,
                    });
                    *voice = None;

                    if voice_id.is_some() {
                        return;
                    }
                }
                _ => (),
            }
        }
    }
}

macro_rules! mkparamgetter {
    ($field: ident, $idx: literal, $self: ident, $buff: ident) => {
        let $field = $self.params.$field.value();
        match $buff[$idx].lock() {
            Ok(mut x) => {
                *x = $field;
            }
            Err(_x) => {}
        }
    };
}

fn to_semitones(f1: f32, f2: f32) -> f32 {
    12.0 * f32::log2(f2 / f1)
}
fn from_semitones(f2: f32, n: f32) -> f32 {
    f2 / 2.0_f32.powf(n / 12.0)
}

pub enum BackgroundTasks {
    MidiFileOpen(Arc<Mutex<FileManager>>),
    WavFileOpen(Arc<Mutex<FileManager>>),
}

impl Plugin for NodeSound {
    const NAME: &'static str = "Node Sound";
    const VENDOR: &'static str = "Lubba64";
    const URL: &'static str = "https://lubba-64.github.io/";
    const EMAIL: &'static str = "Lubba64@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = BackgroundTasks;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn task_executor(&mut self) -> TaskExecutor<Self> {
        Box::new(|cx| match cx {
            BackgroundTasks::MidiFileOpen(files) => {
                match files.lock() {
                    Err(_x) => {}
                    Ok(mut x) => {
                        match x.midi_active {
                            None => {}
                            Some(node_id) => {
                                x.midi_file_path = Some((
                                    rfd::FileDialog::new()
                                        .add_filter("audio", &["mid", "midi"])
                                        .pick_file()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default()
                                        .to_string(),
                                    node_id,
                                ));
                                x.midi_active = None;
                            }
                        };
                    }
                };
            }
            BackgroundTasks::WavFileOpen(files) => {
                match files.lock() {
                    Err(_x) => {}
                    Ok(mut x) => {
                        match x.wav_active {
                            None => {}
                            Some(node_id) => {
                                x.wav_file_path = Some((
                                    rfd::FileDialog::new()
                                        .add_filter("audio", &["wav", "mp3", "flac", "ogg"])
                                        .pick_file()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default()
                                        .to_string(),
                                    node_id,
                                ));
                                x.wav_active = None;
                            }
                        };
                    }
                };
            }
        })
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_egui_editor(
            self.params.editor_state.clone(),
            (
                self.params.plugin_state.graph.clone(),
                self.source_sound_buffers.clone(),
                self.sample_rate.clone(),
                self.params.root_sound_id.clone(),
                false,
                self.bpm.clone(),
            ),
            |_, _| {},
            move |egui_ctx, _setter, state| {
                let sound_result_id = &mut match state.3.lock() {
                    Ok(x) => x,
                    Err(_x) => {
                        return;
                    }
                };
                let sample_rate = &mut match state.2.lock() {
                    Ok(x) => x,
                    Err(_x) => {
                        return;
                    }
                };
                let sound_buffers = &mut match state.1.lock() {
                    Ok(x) => x,
                    Err(_x) => {
                        return;
                    }
                };
                let graph = &mut match state.0.lock() {
                    Ok(x) => x,
                    Err(_x) => {
                        return;
                    }
                };
                graph.update_root(egui_ctx);
                // refreshes the graph state to fix bugs with DAW automations that if not refreshed will be null secretly...
                if !state.4 {
                    state.4 = true;
                    let copy_state = copy(&mut graph.state.editor_state, true);
                    delete_nodes(&mut graph.state.editor_state, true);
                    executor::block_on(paste(&mut graph.state.editor_state, None, copy_state));
                }
                if sound_result_id.is_none() || graph.state.user_state.active_node.is_playing() {
                    let mut clear = false;
                    graph.state.user_state.active_node = ActiveNodeState::NoNode;
                    match graph.state.user_state.vst_output_node_id {
                        Some(outputid) => {
                            graph
                                .state
                                ._unserializeable_state
                                .queue
                                .set_bpm(state.5.clone());
                            graph.state.user_state.wavetables.clear();
                            for vidx in 0..MIDI_NOTES_LEN as usize {
                                let speed = from_semitones(
                                    MIDDLE_C_FREQ,
                                    to_semitones(midi_note_to_freq(vidx as u8), MIDDLE_C_FREQ)
                                        + 10.5
                                        + 1.8
                                        - 0.5
                                        + 0.2
                                        + 0.1,
                                ) / MIDDLE_C_FREQ;
                                graph.state._unserializeable_state.queue.clear();
                                graph
                                    .state
                                    ._unserializeable_state
                                    .queue
                                    .set_note_speed(speed);
                                graph
                                    .state
                                    ._unserializeable_state
                                    .queue
                                    .set_sample_rate(**sample_rate);
                                match evaluate_node(
                                    &graph.state.editor_state.graph.clone(),
                                    outputid,
                                    &mut HashMap::new(),
                                    &get_nodes(),
                                    &mut graph.state,
                                ) {
                                    Ok(val) => {
                                        let source_id = match val {
                                            ValueType::AudioSource { value } => value,
                                            _ => {
                                                return;
                                            }
                                        };
                                        **sound_result_id = Some(source_id);
                                        let sound = match graph
                                            .state
                                            ._unserializeable_state
                                            .queue
                                            .clone_sound(source_id.clone())
                                        {
                                            Err(_err) => {
                                                GenericSource::new(Box::new(ConstWave::new(0.0)))
                                            }
                                            Ok(x) => x,
                                        };
                                        sound_buffers[vidx] = Some(GenericSource::new(Box::new(
                                            Speed::new(sound, speed),
                                        )));
                                    }
                                    Err(_err) => {
                                        clear = true;
                                    }
                                };
                            }
                        }
                        None => {
                            clear = true;
                        }
                    }
                    if clear {
                        for buffer in (**sound_buffers).iter_mut() {
                            *buffer = None;
                        }
                        graph.state._unserializeable_state.queue.clear();
                        **sound_result_id = None
                    }
                }
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_samples = buffer.samples();
        let automations;
        let input;
        {
            let state = &match self.params.plugin_state.graph.lock() {
                Ok(x) => x,
                Err(_x) => {
                    return ProcessStatus::KeepAlive;
                }
            }
            .state;
            automations = state._unserializeable_state.automations.0.clone();
            input = state._unserializeable_state.input.0.clone();

            match state.user_state.files.lock() {
                Ok(x) => {
                    if x.midi_active.is_some() {
                        context.execute_background(BackgroundTasks::MidiFileOpen(
                            state.user_state.files.clone(),
                        ));
                    }
                    if x.wav_active.is_some() {
                        context.execute_background(BackgroundTasks::WavFileOpen(
                            state.user_state.files.clone(),
                        ));
                    }
                }
                Err(_x) => {
                    return ProcessStatus::KeepAlive;
                }
            }
        }

        let sample_rate = match self.sample_rate.lock() {
            Ok(mut x) => {
                *x = context.transport().sample_rate;
                *x
            }
            Err(_x) => {
                return ProcessStatus::KeepAlive;
            }
        };
        match self.bpm.lock() {
            Ok(mut x) => {
                *x = context.transport().tempo.unwrap_or(120.0) as f32;
                *x
            }
            Err(_x) => {
                return ProcessStatus::KeepAlive;
            }
        };
        let output = buffer.as_slice();

        let mut next_event = context.next_event();
        let mut block_start: usize = 0;
        let mut block_end: usize = MAX_BLOCK_SIZE.min(num_samples);

        while block_start < num_samples {
            let this_sample_internal_voice_id_start = self.next_internal_voice_id;

            let mut notes_to_reset = vec![];
            'events: loop {
                match next_event {
                    // If the event happens now, then we'll keep processing events
                    Some(event) if (event.timing() as usize) <= block_start => {
                        // This synth doesn't support any of the polyphonic expression events. A
                        // real synth plugin however will want to support those.
                        match event {
                            NoteEvent::NoteOn {
                                timing,
                                voice_id,
                                channel,
                                note,
                                velocity,
                            } => {
                                self.start_voice(
                                    context,
                                    timing,
                                    voice_id,
                                    channel,
                                    note,
                                    velocity,
                                    sample_rate,
                                );
                                notes_to_reset.push(note);
                            }
                            NoteEvent::NoteOff {
                                timing: _,
                                voice_id,
                                channel,
                                note,
                                velocity: _,
                            } => {
                                self.start_release_for_voices(sample_rate, voice_id, channel, note)
                            }
                            NoteEvent::Choke {
                                timing,
                                voice_id,
                                channel,
                                note,
                            } => {
                                self.choke_voices(context, timing, voice_id, channel, note);
                            }
                            NoteEvent::PolyModulation {
                                timing: _,
                                voice_id,
                                poly_modulation_id,
                                normalized_offset,
                            } => {
                                // Polyphonic modulation events are matched to voices using the
                                // voice ID, and to parameters using the poly modulation ID. The
                                // host will probably send a modulation event every N samples. This
                                // will happen before the voice is active, and of course also after
                                // it has been terminated (because the host doesn't know that it
                                // will be). Because of that, we won't print any assertion failures
                                // when we can't find the voice index here.
                                if let Some(voice_idx) = self.get_voice_idx(voice_id) {
                                    let voice = self.voices[voice_idx].as_mut().unwrap();

                                    match poly_modulation_id {
                                        GAIN_POLY_MOD_ID => {
                                            // This should either create a smoother for this
                                            // modulated parameter or update the existing one.
                                            // Notice how this uses the parameter's unmodulated
                                            // normalized value in combination with the normalized
                                            // offset to create the target plain value
                                            let target_plain_value = self
                                                .params
                                                .gain
                                                .preview_modulated(normalized_offset);
                                            let (_, smoother) =
                                                voice.voice_gain.get_or_insert_with(|| {
                                                    (
                                                        normalized_offset,
                                                        self.params.gain.smoothed.clone(),
                                                    )
                                                });

                                            // If this `PolyModulation` events happens on the
                                            // same sample as a voice's `NoteOn` event, then it
                                            // should immediately use the modulated value
                                            // instead of slowly fading in
                                            if voice.internal_voice_id
                                                >= this_sample_internal_voice_id_start
                                            {
                                                smoother.reset(target_plain_value);
                                            } else {
                                                smoother
                                                    .set_target(sample_rate, target_plain_value);
                                            }
                                        }
                                        n => nih_debug_assert_failure!(
                                            "Polyphonic modulation sent for unknown poly \
                                             modulation ID {}",
                                            n
                                        ),
                                    }
                                }
                            }
                            NoteEvent::MonoAutomation {
                                timing: _,
                                poly_modulation_id,
                                normalized_value,
                            } => {
                                // Modulation always acts as an offset to the parameter's current
                                // automated value. So if the host sends a new automation value for
                                // a modulated parameter, the modulated values/smoothing targets
                                // need to be updated for all polyphonically modulated voices.
                                for voice in self.voices.iter_mut().filter_map(|v| v.as_mut()) {
                                    match poly_modulation_id {
                                        GAIN_POLY_MOD_ID => {
                                            let (normalized_offset, smoother) =
                                                match voice.voice_gain.as_mut() {
                                                    Some((o, s)) => (o, s),
                                                    // If the voice does not have existing
                                                    // polyphonic modulation, then there's nothing
                                                    // to do here. The global automation/monophonic
                                                    // modulation has already been taken care of by
                                                    // the framework.
                                                    None => continue,
                                                };
                                            let target_plain_value =
                                                self.params.gain.preview_plain(
                                                    normalized_value + *normalized_offset,
                                                );
                                            smoother.set_target(sample_rate, target_plain_value);
                                        }
                                        n => nih_debug_assert_failure!(
                                            "Automation event sent for unknown poly modulation ID \
                                             {}",
                                            n
                                        ),
                                    }
                                }
                            }
                            _ => (),
                        };

                        next_event = context.next_event();
                    }
                    // If the event happens before the end of the block, then the block should be cut
                    // short so the next block starts at the event
                    Some(event) if (event.timing() as usize) < block_end => {
                        block_end = event.timing() as usize;
                        break 'events;
                    }
                    _ => break 'events,
                }
            }

            let sound_buffers = match self.source_sound_buffers.lock() {
                Ok(x) => x,
                Err(_x) => {
                    return ProcessStatus::KeepAlive;
                }
            };
            let mut voice_sound_buffers = match self.voice_sound_buffers.lock() {
                Ok(x) => x,
                Err(_x) => {
                    return ProcessStatus::KeepAlive;
                }
            };
            for note in notes_to_reset {
                voice_sound_buffers[note as usize] = (sound_buffers[note as usize].clone(), 0);
            }

            let active_voices = self
                .voices
                .iter()
                .filter(|voice| voice.is_some())
                .collect::<Vec<_>>()
                .len() as f32;

            for sample_idx in block_start..block_end {
                match input.lock() {
                    Ok(mut x) => {
                        x.0 = output[0][sample_idx];
                        x.1 = output[1][sample_idx];
                    }
                    _ => {}
                }
                for voice in &mut self.voices.iter_mut().filter_map(|v| v.as_mut()) {
                    let buffer = &mut voice_sound_buffers[voice.note as usize];
                    let amp = voice.amp_envelope.next();
                    mkparamgetter!(a1, 0, self, automations);
                    mkparamgetter!(a2, 1, self, automations);
                    mkparamgetter!(a3, 2, self, automations);
                    mkparamgetter!(a4, 3, self, automations);
                    mkparamgetter!(a5, 4, self, automations);
                    mkparamgetter!(a6, 5, self, automations);
                    mkparamgetter!(a7, 6, self, automations);
                    mkparamgetter!(a8, 7, self, automations);
                    mkparamgetter!(a9, 8, self, automations);
                    mkparamgetter!(a10, 9, self, automations);
                    mkparamgetter!(a11, 10, self, automations);
                    mkparamgetter!(a12, 11, self, automations);
                    mkparamgetter!(a13, 12, self, automations);
                    mkparamgetter!(a14, 13, self, automations);
                    mkparamgetter!(a15, 14, self, automations);
                    mkparamgetter!(a16, 15, self, automations);
                    mkparamgetter!(a17, 16, self, automations);
                    mkparamgetter!(a18, 17, self, automations);
                    match &mut buffer.0 {
                        Some(source) => {
                            let time_index = (buffer.1 + sample_idx) as f32;
                            let left_sample = (source.next(time_index, 0).unwrap_or_default()
                                / active_voices.max(1.0))
                                * amp;
                            let right_sample = (source.next(time_index, 1).unwrap_or_default()
                                / active_voices.max(1.0))
                                * amp;
                            output[0][sample_idx] += left_sample.clamp(-1.0, 1.0);
                            output[1][sample_idx] += right_sample.clamp(-1.0, 1.0);
                        }
                        None => {}
                    }
                }
            }

            const ENVELOPE_THRESHOLD: f32 = 1e-5;
            for voice in self.voices.iter_mut() {
                match voice {
                    Some(v) if v.amp_envelope.previous_value() < ENVELOPE_THRESHOLD => {
                        // This event is very important, as it allows the host to manage its own modulation
                        // voices
                        context.send_event(NoteEvent::VoiceTerminated {
                            timing: block_end as u32,
                            voice_id: Some(v.voice_id),
                            channel: v.channel,
                            note: v.note,
                        });
                        *voice = None;
                    }
                    _ => (),
                }
            }

            block_start = block_end;
            block_end = (block_start + MAX_BLOCK_SIZE).min(num_samples);
        }

        let mut voice_sound_buffers = match self.voice_sound_buffers.lock() {
            Ok(x) => x,
            Err(_x) => {
                return ProcessStatus::KeepAlive;
            }
        };

        for buffer in voice_sound_buffers.iter_mut() {
            buffer.1 += num_samples;
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for NodeSound {
    const CLAP_ID: &'static str = "com.lubba64-plugins-egui.node-sound-egui-2";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A node graph waveform synth");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
        ClapFeature::Instrument,
    ];
}

impl Vst3Plugin for NodeSound {
    const VST3_CLASS_ID: [u8; 16] = *b"NodeSoundLubba64";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Stereo,
    ];
}

nih_export_clap!(NodeSound);
nih_export_vst3!(NodeSound);
