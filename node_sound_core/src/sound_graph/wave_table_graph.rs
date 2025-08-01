use eframe::egui::vec2;
use egui_plot::PlotBounds;

use crate::constants::WAVE_TABLE_SIZE;

pub fn wave_table_graph(value: &mut Option<Vec<f32>>, ui: &mut eframe::egui::Ui, id: usize) {
    use egui_plot::{Line, Plot, PlotPoints};

    let value = match value {
        Some(x) => x,
        None => {
            return;
        }
    };

    if value.len() == 0 {
        value.extend(Vec::with_capacity(WAVE_TABLE_SIZE).iter());
        return;
    }

    let points: PlotPoints = (0..WAVE_TABLE_SIZE)
        .map(|i| {
            let x = i as f64 * 10.0 / WAVE_TABLE_SIZE as f64;
            [x, value[i].into()]
        })
        .collect();
    let line = Line::new(points);
    let mouse_down = ui.input(|x| x.pointer.button_down(eframe::egui::PointerButton::Primary));

    Plot::new(id.to_string())
        .view_aspect(2.0)
        .allow_drag(false)
        .min_size(vec2(400.0, 100.0))
        .allow_zoom(false)
        .allow_scroll(false)
        .height(100.0)
        .width(500.0)
        .allow_double_click_reset(false)
        .center_y_axis(true)
        .y_axis_min_width(1.0)
        .show(ui, |plot_ui| {
            plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, -1.0], [10.0, 1.0]));
            match plot_ui.ctx().pointer_interact_pos() {
                Some(x) => {
                    let y = plot_ui.plot_from_screen(x).y;
                    if y > -1.0 && y < 1.0 && mouse_down {
                        match plot_ui.pointer_coordinate() {
                            Some(x) => {
                                let idx_rev_hope = ((x.x - x.x % 0.1) / 0.1)
                                    .clamp(0.0, (WAVE_TABLE_SIZE - 1) as f64)
                                    .round()
                                    as usize;
                                value[idx_rev_hope] = x.y as f32;
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }

            plot_ui.line(line)
        });
}
