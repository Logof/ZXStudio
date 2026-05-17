use eframe::egui;
use crate::models::ProjectData;
use crate::app::states::MapEditMode;

pub fn render_palette_tiles(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_tile: &mut u8,
    is_active: bool,
    tileset_texture: &Option<egui::TextureHandle>,
) -> Option<MapEditMode> { // Возвращаем Option для безопасного переключения стейта
    let mut mode_signal = None;

    ui.group(|ui| {
        let title_color = if is_active { egui::Color32::LIGHT_BLUE } else { egui::Color32::GRAY };
        ui.colored_label(title_color, "🧱 Кисть: Тайлы (0-47)");
        
        if tileset_texture.is_none() {
            let mut button = egui::Button::new("");
            if *selected_tile == 0 && is_active { 
                button = button.stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 150, 255))); 
            }
            let btn_res = ui.add_sized([28.0, 28.0], button);
            ui.painter().text(btn_res.rect.center(), egui::Align2::CENTER_CENTER, "00", egui::FontId::proportional(10.0), egui::Color32::GRAY);
            if btn_res.clicked() {
                *selected_tile = 0;
                mode_signal = Some(MapEditMode::Tiles);
            }
        } else {
            egui::ScrollArea::vertical().id_source("tiles_p").show(ui, |ui| {
                egui::Grid::new("gt").spacing([2.0, 2.0]).show(ui, |ui| {
                    for t in 0..48 {
                        let tile_x = (t % 16) as f32 * 16.0;
                        let tile_y = (t / 16) as f32 * 16.0;
                        let eps = 0.5;
                        let uv_min = egui::pos2((tile_x + eps) / 256.0, (tile_y + eps) / 48.0);
                        let uv_max = egui::pos2((tile_x + 16.0 - eps) / 256.0, (tile_y + 16.0 - eps) / 48.0);

                        let mut button = egui::Button::new("");
                        if *selected_tile == t && is_active {
                            button = button.stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 150, 255)));
                        }
                        if t >= 14 && t <= 19 { 
                            button = button.fill(egui::Color32::from_rgba_unmultiplied(0, 150, 255, 30)); 
                        }

                        let btn_res = ui.add_sized([28.0, 28.0], button);
                        if let Some(tex) = tileset_texture {
                            ui.painter().image(tex.id(), btn_res.rect.shrink(1.0), egui::Rect::from_min_max(uv_min, uv_max), egui::Color32::WHITE);
                        }

                        if btn_res.clicked() {
                            *selected_tile = t;
                            mode_signal = Some(MapEditMode::Tiles); // Сигнализируем о переключении
                        }
                        if (t + 1) % 4 == 0 { ui.end_row(); }
                    }
                });
            });
        }
    });

    mode_signal
}
