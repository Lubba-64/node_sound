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
                if ui
                    .add(eframe::egui::Button::new(Octave::O0.to_string()))
                    .clicked()
                {
                    return Ok(Octave::O0);
                }
                if ui
                    .add(eframe::egui::Button::new(Octave::O1.to_string()))
                    .clicked()
                {
                    return Ok(Octave::O1);
                }
                if ui
                    .add(eframe::egui::Button::new(Octave::O2.to_string()))
                    .clicked()
                {
                    return Ok(Octave::O2);
                }
                if ui
                    .add(eframe::egui::Button::new(Octave::O3.to_string()))
                    .clicked()
                {
                    return Ok(Octave::O3);
                }
                if ui
                    .add(eframe::egui::Button::new(Octave::O4.to_string()))
                    .clicked()
                {
                    return Ok(Octave::O4);
                }
                if ui
                    .add(eframe::egui::Button::new(Octave::O5.to_string()))
                    .clicked()
                {
                    return Ok(Octave::O5);
                }
                if ui
                    .add(eframe::egui::Button::new(Octave::O6.to_string()))
                    .clicked()
                {
                    return Ok(Octave::O6);
                }
                if ui
                    .add(eframe::egui::Button::new(Octave::O7.to_string()))
                    .clicked()
                {
                    return Ok(Octave::O7);
                }
                if ui
                    .add(eframe::egui::Button::new(Octave::O8.to_string()))
                    .clicked()
                {
                    return Ok(Octave::O8);
                }
                return Err(());
            })
            .inner
            .unwrap_or(Err(()));
        let note_res = ComboBox::new(format!("note_{}", param_name), "")
            .selected_text(note.1.to_string())
            .width(20.0)
            .show_ui(ui, |ui| -> Result<Note, ()> {
                if ui
                    .add(eframe::egui::Button::new(Note::C.to_string()))
                    .clicked()
                {
                    return Ok(Note::C);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::CS.to_string()))
                    .clicked()
                {
                    return Ok(Note::CS);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::D.to_string()))
                    .clicked()
                {
                    return Ok(Note::D);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::DS.to_string()))
                    .clicked()
                {
                    return Ok(Note::DS);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::E.to_string()))
                    .clicked()
                {
                    return Ok(Note::E);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::F.to_string()))
                    .clicked()
                {
                    return Ok(Note::F);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::FS.to_string()))
                    .clicked()
                {
                    return Ok(Note::FS);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::G.to_string()))
                    .clicked()
                {
                    return Ok(Note::G);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::GS.to_string()))
                    .clicked()
                {
                    return Ok(Note::GS);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::A.to_string()))
                    .clicked()
                {
                    return Ok(Note::A);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::AS.to_string()))
                    .clicked()
                {
                    return Ok(Note::AS);
                }

                if ui
                    .add(eframe::egui::Button::new(Note::B.to_string()))
                    .clicked()
                {
                    return Ok(Note::B);
                }
                return Err(());
            })
            .inner
            .unwrap_or(Err(()));
        match (note_res, octave_res) {
            (Ok(note_res), Err(_)) => {
                *note = NoteValue(note.0.clone(), note_res);
                *value = note.match_freq();
            }
            (Err(_), Ok(octave_res)) => {
                *note = NoteValue(octave_res, note.1.clone());
                *value = note.match_freq();
            }
            (Ok(note_res), Ok(octave_res)) => {
                *note = NoteValue(octave_res, note_res);
                *value = note.match_freq();
            }
            (Err(_), Err(_)) => {}
        }
    });
}
