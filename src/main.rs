use eframe::{egui::CentralPanel, App};
use rodio::{OutputStream, source::Source};
mod components;
use components::*;
mod wave_table;
use wave_table::WavetableOscillator;
mod node_graph;

struct MainApp;

impl App for MainApp{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {ui.label("hello")});
    }
}


fn main() {

    let square_ = change_amplitude(square_wave_table(64), 0.02);
    let sin_ = change_amplitude(sin_wave_table(64), 0.1);
    let test_ = perlin_wave_table(64, 1000000100);
    let combine_ = combine(combine(sin_, square_), test_);
    let mut oscillator = WavetableOscillator::new(44100, combine_);
    oscillator.set_frequency(442.0);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    use eframe::egui::Visuals;

    eframe::run_native(
        "Egui node graph example",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Box::new(node_graph::NodeGraphExample::default())
        }),
    );
}
