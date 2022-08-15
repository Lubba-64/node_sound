use std::time::Duration;

use rodio::{
    source::{ChannelVolume, SineWave},
    OutputStream, Sample, Source,
};
use sound_graph::nodes::{sawtooth_node, AsFiniteSource, FiniteSource, SawToothWave, SquareWave};

use crate::sound_graph::{nodes::get_nodes, types::NodeDefinitions};
mod sound_graph;
fn main() {
    let sin = SineWave::new(100.0);
    let square = SquareWave::new(100.0);
    let saw = SawToothWave::new(100.0);
    let sounds: [FiniteSource<f32>; 3] = [
        sin.as_finite(Duration::new(1, 0)),
        square.as_finite(Duration::new(1, 0)),
        saw.as_finite(Duration::new(1, 0)),
    ];

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let res = stream_handle.play_raw(
        sounds[0]
            .clone()
            .amplify(0.5)
            .as_finite(Duration::from_secs_f32(0.5))
            .mix(sounds[1].clone())
            .as_finite(Duration::from_secs_f32(0.25))
            .convert_samples(),
    );

    match res {
        Ok(_x) => {}
        Err(_x) => {
            panic!()
        }
    }

    ///std::thread::sleep(std::time::Duration::from_secs(5));
    use eframe::egui::Visuals;

    eframe::run_native(
        "Egui node graph example",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Box::new(sound_graph::graph::NodeGraphExample::new())
        }),
    );
}
