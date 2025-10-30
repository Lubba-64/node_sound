use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppTheme {
    Light,
    Dark,
    Blue,
    Green,
    Purple,
    Orange,
    Pink,
    Red,
    Yellow,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self::Dark
    }
}

impl AppTheme {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Light,
            Self::Dark,
            Self::Blue,
            Self::Green,
            Self::Purple,
            Self::Orange,
            Self::Pink,
            Self::Red,
            Self::Yellow,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Blue => "Blue",
            Self::Green => "Green",
            Self::Purple => "Purple",
            Self::Orange => "Orange",
            Self::Pink => "Pink",
            Self::Red => "Red",
            Self::Yellow => "Yellow",
        }
    }

    pub fn apply_theme(&self, ctx: &egui::Context) {
        let mut visuals = match self {
            Self::Light => egui::Visuals::light(),
            _ => egui::Visuals::dark(),
        };

        match self {
            Self::Light | Self::Dark => {
                // Use default light/dark themes as-is
            }
            Self::Blue => {
                visuals.dark_mode = true;
                visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 255));

                // Widgets
                visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(25, 35, 65);
                visuals.widgets.noninteractive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 220, 255));
                visuals.widgets.noninteractive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 70, 100));

                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(40, 60, 110);
                visuals.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(240, 240, 255));

                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 90, 170);
                visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

                visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 75, 140);
                visuals.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 255, 255));

                visuals.widgets.open.bg_fill = egui::Color32::from_rgb(70, 100, 180);

                // Selection
                visuals.selection.bg_fill = egui::Color32::from_rgb(40, 90, 200);
                visuals.selection.stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 150, 255));

                // Hyperlink
                visuals.hyperlink_color = egui::Color32::from_rgb(100, 150, 255);

                // Background colors
                visuals.faint_bg_color = egui::Color32::from_rgb(30, 40, 70);
                visuals.extreme_bg_color = egui::Color32::from_rgb(15, 25, 50);
                visuals.code_bg_color = egui::Color32::from_rgb(20, 30, 55);

                // Status colors
                visuals.warn_fg_color = egui::Color32::from_rgb(255, 200, 100);
                visuals.error_fg_color = egui::Color32::from_rgb(255, 100, 100);

                // Window
                visuals.window_fill = egui::Color32::from_rgb(25, 35, 65);
                visuals.window_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 70, 120));
                visuals.window_corner_radius = egui::CornerRadius::same(4);
                visuals.window_shadow = egui::Shadow::default();

                // Panel
                visuals.panel_fill = egui::Color32::from_rgb(20, 30, 55);

                // Popup
                visuals.popup_shadow = egui::Shadow::default();
            }
            Self::Green => {
                visuals.dark_mode = true;
                visuals.override_text_color = Some(egui::Color32::from_rgb(220, 255, 220));

                // Widgets
                visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(25, 45, 30);
                visuals.widgets.noninteractive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 255, 220));
                visuals.widgets.noninteractive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 90, 60));

                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(35, 70, 45);
                visuals.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(240, 255, 240));

                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(50, 120, 70);
                visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

                visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(42, 95, 58);
                visuals.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 255, 255));

                visuals.widgets.open.bg_fill = egui::Color32::from_rgb(55, 130, 80);

                // Selection
                visuals.selection.bg_fill = egui::Color32::from_rgb(45, 140, 85);
                visuals.selection.stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 120));

                // Hyperlink
                visuals.hyperlink_color = egui::Color32::from_rgb(100, 200, 120);

                // Background colors
                visuals.faint_bg_color = egui::Color32::from_rgb(30, 50, 35);
                visuals.extreme_bg_color = egui::Color32::from_rgb(15, 35, 20);
                visuals.code_bg_color = egui::Color32::from_rgb(20, 40, 25);

                // Status colors
                visuals.warn_fg_color = egui::Color32::from_rgb(255, 200, 100);
                visuals.error_fg_color = egui::Color32::from_rgb(255, 100, 100);

                // Window
                visuals.window_fill = egui::Color32::from_rgb(25, 45, 30);
                visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 90, 60));
                visuals.window_corner_radius = egui::CornerRadius::same(4);
                visuals.window_shadow = egui::Shadow::default();

                // Panel
                visuals.panel_fill = egui::Color32::from_rgb(20, 40, 25);

                // Popup
                visuals.popup_shadow = egui::Shadow::default();
            }
            Self::Purple => {
                visuals.dark_mode = true;
                visuals.override_text_color = Some(egui::Color32::from_rgb(240, 220, 255));

                // Widgets
                visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(40, 25, 60);
                visuals.widgets.noninteractive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(240, 220, 255));
                visuals.widgets.noninteractive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 50, 90));

                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(60, 40, 90);
                visuals.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(250, 230, 255));

                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(90, 60, 140);
                visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

                visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(75, 50, 115);
                visuals.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 255, 255));

                visuals.widgets.open.bg_fill = egui::Color32::from_rgb(100, 70, 150);

                // Selection
                visuals.selection.bg_fill = egui::Color32::from_rgb(110, 75, 170);
                visuals.selection.stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(160, 120, 220));

                // Hyperlink
                visuals.hyperlink_color = egui::Color32::from_rgb(160, 120, 220);

                // Background colors
                visuals.faint_bg_color = egui::Color32::from_rgb(45, 30, 65);
                visuals.extreme_bg_color = egui::Color32::from_rgb(30, 20, 45);
                visuals.code_bg_color = egui::Color32::from_rgb(35, 25, 50);

                // Status colors
                visuals.warn_fg_color = egui::Color32::from_rgb(255, 200, 100);
                visuals.error_fg_color = egui::Color32::from_rgb(255, 100, 100);

                // Window
                visuals.window_fill = egui::Color32::from_rgb(40, 25, 60);
                visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 50, 90));
                visuals.window_corner_radius = egui::CornerRadius::same(4);
                visuals.window_shadow = egui::Shadow::default();

                // Panel
                visuals.panel_fill = egui::Color32::from_rgb(35, 25, 50);

                // Popup
                visuals.popup_shadow = egui::Shadow::default();
            }
            Self::Orange => {
                visuals.dark_mode = true;
                visuals.override_text_color = Some(egui::Color32::from_rgb(255, 230, 200));

                // Widgets
                visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(65, 40, 15);
                visuals.widgets.noninteractive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 230, 200));
                visuals.widgets.noninteractive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 70, 30));

                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(90, 55, 20);
                visuals.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 240, 220));

                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(140, 85, 30);
                visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

                visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(115, 70, 25);
                visuals.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 255, 255));

                visuals.widgets.open.bg_fill = egui::Color32::from_rgb(150, 90, 35);

                // Selection
                visuals.selection.bg_fill = egui::Color32::from_rgb(170, 100, 40);
                visuals.selection.stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 140, 60));

                // Hyperlink
                visuals.hyperlink_color = egui::Color32::from_rgb(220, 140, 60);

                // Background colors
                visuals.faint_bg_color = egui::Color32::from_rgb(70, 45, 20);
                visuals.extreme_bg_color = egui::Color32::from_rgb(50, 30, 10);
                visuals.code_bg_color = egui::Color32::from_rgb(60, 35, 15);

                // Status colors
                visuals.warn_fg_color = egui::Color32::from_rgb(255, 180, 80);
                visuals.error_fg_color = egui::Color32::from_rgb(255, 100, 100);

                // Window
                visuals.window_fill = egui::Color32::from_rgb(65, 40, 15);
                visuals.window_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 70, 30));
                visuals.window_corner_radius = egui::CornerRadius::same(4);
                visuals.window_shadow = egui::Shadow::default();

                // Panel
                visuals.panel_fill = egui::Color32::from_rgb(60, 35, 15);

                // Popup
                visuals.popup_shadow = egui::Shadow::default();
            }
            Self::Pink => {
                visuals.dark_mode = true;
                visuals.override_text_color = Some(egui::Color32::from_rgb(255, 220, 240));

                // Widgets
                visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(60, 25, 50);
                visuals.widgets.noninteractive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 220, 240));
                visuals.widgets.noninteractive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 50, 80));

                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(80, 40, 70);
                visuals.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 230, 245));

                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(130, 60, 110);
                visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

                visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(105, 50, 90);
                visuals.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 255, 255));

                visuals.widgets.open.bg_fill = egui::Color32::from_rgb(140, 65, 120);

                // Selection
                visuals.selection.bg_fill = egui::Color32::from_rgb(160, 70, 140);
                visuals.selection.stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(210, 100, 180));

                // Hyperlink
                visuals.hyperlink_color = egui::Color32::from_rgb(210, 100, 180);

                // Background colors
                visuals.faint_bg_color = egui::Color32::from_rgb(65, 30, 55);
                visuals.extreme_bg_color = egui::Color32::from_rgb(45, 20, 40);
                visuals.code_bg_color = egui::Color32::from_rgb(50, 25, 45);

                // Status colors
                visuals.warn_fg_color = egui::Color32::from_rgb(255, 200, 100);
                visuals.error_fg_color = egui::Color32::from_rgb(255, 100, 100);

                // Window
                visuals.window_fill = egui::Color32::from_rgb(60, 25, 50);
                visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 50, 80));
                visuals.window_corner_radius = egui::CornerRadius::same(4);
                visuals.window_shadow = egui::Shadow::default();

                // Panel
                visuals.panel_fill = egui::Color32::from_rgb(50, 25, 45);

                // Popup
                visuals.popup_shadow = egui::Shadow::default();
            }
            Self::Red => {
                visuals.dark_mode = true;
                visuals.override_text_color = Some(egui::Color32::from_rgb(255, 220, 220));

                // Widgets
                visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(65, 25, 25);
                visuals.widgets.noninteractive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 220, 220));
                visuals.widgets.noninteractive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 50, 50));

                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(90, 35, 35);
                visuals.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 230, 230));

                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(140, 50, 50);
                visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

                visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(115, 40, 40);
                visuals.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 255, 255));

                visuals.widgets.open.bg_fill = egui::Color32::from_rgb(150, 55, 55);

                // Selection
                visuals.selection.bg_fill = egui::Color32::from_rgb(170, 60, 60);
                visuals.selection.stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 100, 100));

                // Hyperlink
                visuals.hyperlink_color = egui::Color32::from_rgb(220, 100, 100);

                // Background colors
                visuals.faint_bg_color = egui::Color32::from_rgb(70, 30, 30);
                visuals.extreme_bg_color = egui::Color32::from_rgb(50, 20, 20);
                visuals.code_bg_color = egui::Color32::from_rgb(55, 25, 25);

                // Status colors
                visuals.warn_fg_color = egui::Color32::from_rgb(255, 200, 100);
                visuals.error_fg_color = egui::Color32::from_rgb(255, 120, 120);

                // Window
                visuals.window_fill = egui::Color32::from_rgb(65, 25, 25);
                visuals.window_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 50, 50));
                visuals.window_corner_radius = egui::CornerRadius::same(4);
                visuals.window_shadow = egui::Shadow::default();

                // Panel
                visuals.panel_fill = egui::Color32::from_rgb(55, 25, 25);

                // Popup
                visuals.popup_shadow = egui::Shadow::default();
            }
            Self::Yellow => {
                visuals.dark_mode = true;
                visuals.override_text_color = Some(egui::Color32::from_rgb(255, 255, 200));

                // Widgets
                visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(65, 60, 15);
                visuals.widgets.noninteractive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 200));
                visuals.widgets.noninteractive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 95, 30));

                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(90, 85, 20);
                visuals.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 220));

                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(140, 130, 30);
                visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

                visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(115, 105, 25);
                visuals.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 255, 255));

                visuals.widgets.open.bg_fill = egui::Color32::from_rgb(150, 140, 35);

                // Selection
                visuals.selection.bg_fill = egui::Color32::from_rgb(170, 160, 40);
                visuals.selection.stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 210, 60));

                // Hyperlink
                visuals.hyperlink_color = egui::Color32::from_rgb(220, 210, 60);

                // Background colors
                visuals.faint_bg_color = egui::Color32::from_rgb(70, 65, 20);
                visuals.extreme_bg_color = egui::Color32::from_rgb(50, 45, 10);
                visuals.code_bg_color = egui::Color32::from_rgb(60, 55, 15);

                // Status colors - adjusted for yellow theme
                visuals.warn_fg_color = egui::Color32::from_rgb(255, 180, 80);
                visuals.error_fg_color = egui::Color32::from_rgb(255, 100, 100);

                // Window
                visuals.window_fill = egui::Color32::from_rgb(65, 60, 15);
                visuals.window_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 95, 30));
                visuals.window_corner_radius = egui::CornerRadius::same(4);
                visuals.window_shadow = egui::Shadow::default();

                // Panel
                visuals.panel_fill = egui::Color32::from_rgb(60, 55, 15);

                // Popup
                visuals.popup_shadow = egui::Shadow::default();
            }
        }
        ctx.set_visuals(visuals);
    }
}
