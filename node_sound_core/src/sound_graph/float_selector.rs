use eframe::egui::{ComboBox, DragValue};
use egui_extras_xt::knobs::AudioKnob;

use super::note::{Note, NoteValue, Octave};

pub fn float_selector(
    value: &mut f32,
    min: &mut f32,
    max: &mut f32,
    note: &mut NoteValue,
    ui: &mut eframe::egui::Ui,
    param_name: &str,
) {
    ui.horizontal(|ui| {
        ui.label(param_name);
        ui.add(
            AudioKnob::new(value)
                .range(*min..=*max)
                .drag_length(50.0)
                .diameter(20.0),
        );
        ui.add(DragValue::new(value).speed(0.01).range(*min..=*max));

        let octave_res = ComboBox::new(format!("octave_{}", param_name), "")
            .selected_text(note.0.to_string())
            .width(20.0)
            .show_ui(ui, |ui| -> Result<Octave, ()> {
                for octave in Octave::ALL {
                    if ui
                        .add(eframe::egui::Button::new(octave.to_string()))
                        .clicked()
                    {
                        return Ok(octave);
                    }
                }
                return Err(());
            })
            .inner
            .unwrap_or(Err(()));
        let note_res = ComboBox::new(format!("note_{}", param_name), "")
            .selected_text(note.1.to_string())
            .width(20.0)
            .show_ui(ui, |ui| -> Result<Note, ()> {
                for note in Note::ALL {
                    if ui
                        .add(eframe::egui::Button::new(note.to_string()))
                        .clicked()
                    {
                        return Ok(note);
                    }
                }
                return Err(());
            })
            .inner
            .unwrap_or(Err(()));
        *note = NoteValue(octave_res.unwrap_or_default(), note_res.unwrap_or_default());
    });
}
