use eframe::egui;
use crate::models::{ProjectData, config::EngineViewMode};
use crate::app::states::MapEditMode;

pub fn render_palette_enemies(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_enemy_type: &mut u8,
    is_active: bool,
    sprites_texture: &Option<egui::TextureHandle>,
) -> Option<MapEditMode> { // Возвращаем Option для безопасного переключения стейта
    let mut mode_signal = None;

    ui.group(|ui| {
        let title_color = if is_active { egui::Color32::from_rgb(180, 50, 255) } else { egui::Color32::GRAY };
        ui.colored_label(title_color, "👾 Кисть: Враги и Лифты");

        let mut enemy_classes = vec![
            (1, "Враг 1", 8), (2, "Враг 2", 10), (3, "Враг 3", 12),
        ];
        match project.view_mode {
            EngineViewMode::SideView => { enemy_classes.push((4, "Лифт 4", 14)); }
            EngineViewMode::TopView => { enemy_classes.push((4, "Враг 4", 14)); }
        }

        egui::Grid::new("ge").spacing([4.0, 4.0]).show(ui, |ui| {
            for (tp_id, name, raw_sprite_id) in enemy_classes {
                let mut button = egui::Button::new("");
                if *selected_enemy_type == tp_id && is_active {
                    button = button.stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(180, 50, 255)));
                }

                let btn_res = ui.add_sized([32.0, 32.0], button).on_hover_text(name);
                if let Some(tex) = sprites_texture {
                    let local_index = raw_sprite_id - 8;
                    let sprite_x = (local_index as f32) * 32.0;
                    let eps = 0.5;
                    let uv_min = egui::pos2((sprite_x + eps) / 256.0, (16.0 + eps) / 32.0);
                    let uv_max = egui::pos2((sprite_x + 16.0 - eps) / 256.0, (32.0 - eps) / 32.0);
                    ui.painter().image(tex.id(), btn_res.rect.shrink(1.0), egui::Rect::from_min_max(uv_min, uv_max), egui::Color32::WHITE);
                } else {
                    ui.painter().text(btn_res.rect.center(), egui::Align2::CENTER_CENTER, format!("E{}", tp_id), egui::FontId::proportional(10.0), egui::Color32::GRAY);
                }

                if btn_res.clicked() {
                    *selected_enemy_type = tp_id;
                    mode_signal = Some(MapEditMode::Enemies); // Сигнализируем о переключении
                }
            }
        });
    });

    mode_signal
}
