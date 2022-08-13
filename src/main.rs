use eframe::{run_native, epi::App, egui::CentralPanel, NativeOptions};
use rodio::{OutputStream, source::Source};
mod components;
use components::*;
mod wave_table;
use wave_table::WavetableOscillator;

struct MainApp;

impl App for MainApp{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {ui.label("hello")});
    }

    fn name(&self) -> &str {
        "HELLO WORLD!!!"
    }
}


fn main() {

    let square_ = change_amplitude(square_wave_table(64), 0.02);
    let sin_ = change_amplitude(sin_wave_table(64), 0.1);
    let combine_ = combine(sin_, square_);

    let test_ = perlin_wave_table(64, 1000000100);

    // println!("{:#?}", test_);

    let mut oscillator = WavetableOscillator::new(44100, combine_);

    oscillator.set_frequency(442.0);
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let _result = stream_handle.play_raw(oscillator.convert_samples());


    let app = MainApp;
    let native_options = NativeOptions::default();
    run_native(Box::new(app), native_options)
}
