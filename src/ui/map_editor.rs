use eframe::egui;
use crate::models::{ProjectData, ScreenData};
use crate::core::validator::ClashError;

pub fn render_map_editor(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_screen: &mut usize,
    selected_tile: &mut u8,
    clash_errors: &[ClashError],
) {
    ui.heading("Редактирование игрового мира");

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label("Выбор экрана:");
                ui.add(egui::Slider::new(selected_screen, 0..=(project.map_w * project.map_h - 1)).text("Индекс"));
            });

            ui.group(|ui| {
                ui.label("Палитра тайлов:");
                egui::Grid::new("palette_grid").spacing([4.0, 4.0]).show(ui, |ui| {
                    for t in 0..48 {
                        let label = format!("{:02}", t);
                        let color = match t {
                            14 => Some(egui::Color32::LIGHT_BLUE),
                            15 => Some(egui::Color32::LIGHT_RED),
                            16 => Some(egui::Color32::LIGHT_GREEN),
                            17 => Some(egui::Color32::GOLD),
                            18 => Some(egui::Color32::YELLOW),
                            _ => None
                        };

                        let mut button = egui::Button::new(label);
                        if let Some(c) = color { button = button.fill(c); }

                        if ui.add_sized([24.0, 24.0], button).clicked() {
                            *selected_tile = t;
                        }
                        if (t + 1) % 8 == 0 { ui.end_row(); }
                    }
                });
                ui.label(format!("Выбран тайл: {}", selected_tile));
            });
        });

        ui.separator();

        ui.vertical(|ui| {
            let scr_key = format!("screen_{}", selected_screen);
            let screen_data = project.screens.entry(scr_key).or_insert_with(ScreenData::default);

            ui.horizontal(|ui| {
                ui.label(format!("Холст экрана {} (15x10 тайлов)", selected_screen));
                if !clash_errors.is_empty() {
                    ui.colored_label(egui::Color32::LIGHT_RED, format!("⚠️ Коллизий цвета: {}", clash_errors.len()));
                }
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(15.0 * 24.0, 10.0 * 24.0),
                    egui::Sense::click_and_drag()
                );

                let painter = ui.painter_at(rect);
                
                if response.dragged() || response.clicked() {
                    if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                        if rect.contains(mouse_pos) {
                            let local_x = ((mouse_pos.x - rect.min.x) / 24.0) as usize;
                            let local_y = ((mouse_pos.y - rect.min.y) / 24.0) as usize;
                            let index = local_y * 15 + local_x;
                            if index < 150 {
                                screen_data.tiles_matrix[index] = *selected_tile;
                            }
                        }
                    }
                }

                // Слой 1: Статическая геометрия
                for y in 0..10 {
                    for x in 0..15 {
                        let idx = y * 15 + x;
                        let tile_id = screen_data.tiles_matrix[idx];
                        let tile_rect = egui::Rect::from_min_size(
                            egui::pos2(rect.min.x + x as f32 * 24.0, rect.min.y + y as f32 * 24.0),
                            egui::vec2(24.0, 24.0)
                        );

                        let fill_color = match tile_id {
                            14 => egui::Color32::from_rgb(50, 100, 200),
                            15 => egui::Color32::from_rgb(200, 50, 50),
                            0 => egui::Color32::BLACK,
                            _ => egui::Color32::from_gray(60),
                        };

                        painter.rect_filled(tile_rect, 0.0, fill_color);
                        painter.rect_stroke(tile_rect, 0.0, egui::Stroke::new(0.5, egui::Color32::from_gray(90)));
                        
                        if tile_id > 0 {
                            painter.text(tile_rect.center(), egui::Align2::CENTER_CENTER, tile_id.to_string(), egui::FontId::proportional(10.0), egui::Color32::WHITE);
                        }
                    }
                }

                // Слой 2: Оверлей полупрозрачных зон Attribute Clash
                for error in clash_errors {
                    let error_rect = egui::Rect::from_min_size(
                        egui::pos2(rect.min.x + (error.x_block as f32 * 12.0), rect.min.y + (error.y_block as f32 * 12.0)),
                        egui::vec2(12.0, 12.0)
                    );
                    painter.rect_filled(error_rect, 0.0, egui::Color32::from_rgba_unmultiplied(255, 0, 0, 120));
                    painter.rect_stroke(error_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::RED));
                }
            });
        });
    });
}
