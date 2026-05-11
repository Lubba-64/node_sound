use crate::constants::WAVE_TABLE_SIZE;
use eframe::egui::vec2;
use egui_plot::PlotBounds;
use egui_plot::{Line, Plot, PlotPoints};

pub fn wave_table_graph(
    value: &mut Option<Vec<f32>>,
    ui: &mut eframe::egui::Ui,
    id: usize,
    height: f32,
    width: f32,
) {
    let value = match value {
        Some(wave_table) => wave_table,
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
            let wave_table_y_val = i as f64 * 10.0 / WAVE_TABLE_SIZE as f64;
            [wave_table_y_val, value[i].into()]
        })
        .collect();
    let line = Line::new(points);
    let mouse_down = ui.input(|input_state| {
        input_state
            .pointer
            .button_down(eframe::egui::PointerButton::Primary)
    });

    Plot::new(id.to_string())
        .view_aspect(2.0)
        .allow_drag(false)
        .min_size(vec2(width, height))
        .allow_zoom(false)
        .allow_scroll(false)
        .height(height)
        .width(width)
        .allow_double_click_reset(false)
        .center_y_axis(true)
        .y_axis_min_width(1.0)
        .show(ui, |plot_ui| {
            plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, -1.0], [10.0, 1.0]));
            if let Some(click) = plot_ui
                .ctx()
                .pointer_interact_pos()
                .and_then(|pointer_click_pos| Some(plot_ui.plot_from_screen(pointer_click_pos)))
                && click.y > -1.0
                && click.y < 1.0
                && click.x > 0.0
                && click.x < 10.0
                && mouse_down
                && let Some(pointer_pos) = plot_ui.pointer_coordinate()
            {
                let idx_rev_hope = ((pointer_pos.x - pointer_pos.x % 0.1) / 0.1)
                    .clamp(0.0, (WAVE_TABLE_SIZE - 1) as f64)
                    .round() as usize;
                value[idx_rev_hope] = pointer_pos.y as f32;
            }
            plot_ui.line(line)
        });
}
