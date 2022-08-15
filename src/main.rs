use std::time::Duration;

use rodio::{source::SineWave, OutputStream, Source};
use sound_graph::nodes::{AsFiniteSource, FiniteSource, SawToothWave, SquareWave};
mod sound_graph;
use eframe::egui::Visuals;

fn sound_example() {
    let sin = SineWave::new(100.0);
    let square = SquareWave::new(100.0);
    let saw = SawToothWave::new(100.0);
    let sounds: [FiniteSource<f32>; 3] = [
        sin.as_finite(Duration::new(1, 0)),
        square.as_finite(Duration::new(1, 0)),
        saw.as_finite(Duration::new(1, 0)),
    ];

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sound = sounds[0]
        .clone()
        .amplify(0.5)
        .as_finite(Duration::from_secs_f32(0.5))
        .mix(sounds[1].clone())
        .as_finite(Duration::from_secs_f32(0.25));

    let res = stream_handle.play_raw(sound.convert_samples());

    match res {
        Ok(_x) => {}
        Err(_x) => {
            panic!()
        }
    }

    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn main() {
    eframe::run_native(
        "Egui node graph example",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Box::new(sound_graph::graph::NodeGraphExample::new())
        }),
    );
}
