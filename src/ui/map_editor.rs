use eframe::egui;
use crate::models::{ProjectData, ScreenData, screen::{Enemy, Hotspot}};
use crate::core::validator::ClashError;
use crate::app::states::MapEditMode;

pub fn render_map_editor(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_screen: &mut usize,
    selected_tile: &mut u8,
    clash_errors: &[ClashError],
    map_edit_mode: &mut MapEditMode,
    selected_enemy_type: &mut u8,
    selected_hotspot_type: &mut u8,
    tileset_texture: &Option<egui::TextureHandle>,
) {
    ui.heading("Редактирование игрового мира");

    ui.horizontal(|ui| {
        // --- ЛЕВАЯ ПАНЕЛЬ: Управление слоями и палитры ---
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label("Выбор экрана:");
                ui.add(egui::Slider::new(selected_screen, 0..=(project.map_w * project.map_h - 1)).text("Индекс"));
            });

            ui.group(|ui| {
                ui.label("💡 Активный слой:");
                ui.radio_value(map_edit_mode, MapEditMode::Tiles, "🧱 Статические тайлы");
                ui.radio_value(map_edit_mode, MapEditMode::Enemies, "👾 Враги");
            });

            match map_edit_mode {
                MapEditMode::Tiles => {
                    ui.group(|ui| {
                        ui.label("Палитра тайлов из mappy.png:");
                        egui::Grid::new("palette_grid").spacing([4.0, 4.0]).show(ui, |ui| {
                            let mut grid_index = 0;
                            for t in 0..48 {
                                // УЛУЧШЕНИЕ: Пропускаем индексы 16, 17, 18, так как они вынесены в хотспоты
                                if t == 16 || t == 17 || t == 18 {
                                    continue;
                                }

                                let tile_x = (t % 16) as f32 * 16.0;
                                let tile_y = (t / 16) as f32 * 16.0;

                                let eps = 0.5;
                                let uv_min = egui::pos2((tile_x + eps) / 256.0, (tile_y + eps) / 48.0);
                                let uv_max = egui::pos2((tile_x + 16.0 - eps) / 256.0, (tile_y + 16.0 - eps) / 48.0);
                                let uv_rect = egui::Rect::from_min_max(uv_min, uv_max);

                                let mut button = egui::Button::new("");
                                if *selected_tile == t {
                                    button = button.stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 150, 255)));
                                }

                                let btn_response = ui.add_sized([32.0, 32.0], button);
                                
                                if let Some(tex) = tileset_texture {
                                    let painter = ui.painter();
                                    painter.image(tex.id(), btn_response.rect.shrink(1.0), uv_rect, egui::Color32::WHITE);
                                } else {
                                    let painter = ui.painter();
                                    painter.text(btn_response.rect.center(), egui::Align2::CENTER_CENTER, format!("{:02}", t), egui::FontId::proportional(11.0), egui::Color32::GRAY);
                                }

                                if btn_response.clicked() { *selected_tile = t; }
                                
                                // Сдвигаем индекс сетки вручную для корректного переноса строк без пропусков дыр
                                grid_index += 1;
                                if grid_index % 8 == 0 { ui.end_row(); }
                            }
                        });
                    });
                }
                MapEditMode::Enemies => {
                    ui.group(|ui| {
                        ui.label("Поведение врага:");
                        egui::ComboBox::from_label("Тип")
                            .selected_text(format!("Тип {}", selected_enemy_type))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(selected_enemy_type, 1, "Тип 1-4: Линейный");
                                ui.selectable_value(selected_enemy_type, 5, "Тип 5: Random Призрак");
                                ui.selectable_value(selected_enemy_type, 6, "Тип 6: Обычный Призрак");
                                ui.selectable_value(selected_enemy_type, 7, "Тип 7-10: Квадратор");
                                ui.selectable_value(selected_enemy_type, 11, "Тип 11-14: Маррулер");
                            });
                    });
                }
            }
        });

        ui.separator();

        // --- ПРАВАЯ ПАНЕЛЬ: Интерактивный холст ---
        ui.vertical(|ui| {
            let scr_key = format!("screen_{}", selected_screen);
            let screen_data = project.screens.entry(scr_key).or_insert_with(ScreenData::default);

            ui.label(format!("Холст игрового экрана {} (15x10 тайлов)", selected_screen));

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(15.0 * 32.0, 10.0 * 32.0),
                    egui::Sense::click_and_drag()
                );

                let painter = ui.painter_at(rect);
                
                if response.dragged() || response.clicked() {
                    if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                        if rect.contains(mouse_pos) {
                            let cell_x = ((mouse_pos.x - rect.min.x) / 32.0) as u8;
                            let cell_y = ((mouse_pos.y - rect.min.y) / 32.0) as u8;

                            if *map_edit_mode == MapEditMode::Tiles {
                                let index = (cell_y as usize) * 15 + (cell_x as usize);
                                if index < 150 { screen_data.tiles_matrix[index] = *selected_tile; }
                            }
                        }
                    }
                }

                if response.secondary_clicked() {
                    if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                        if rect.contains(mouse_pos) {
                            let cell_x = ((mouse_pos.x - rect.min.x) / 32.0) as u8;
                            let cell_y = ((mouse_pos.y - rect.min.y) / 32.0) as u8;

                            if *map_edit_mode == MapEditMode::Enemies {
                                screen_data.enemies.retain(|e| e.x != cell_x || e.y != cell_y);
                            }
                        }
                    }
                }

                // СЛОЙ 1: Бесшовная отрисовка тайлов
                for y in 0..10 {
                    for x in 0..15 {
                        let idx = y * 15 + x;
                        let tile_id = screen_data.tiles_matrix[idx];
                        let tile_rect = egui::Rect::from_min_size(
                            egui::pos2(rect.min.x + x as f32 * 32.0, rect.min.y + y as f32 * 32.0),
                            egui::vec2(32.0, 32.0)
                        );

                        if let Some(tex) = tileset_texture {
                            let tile_x = (tile_id % 16) as f32 * 16.0;
                            let tile_y = (tile_id / 16) as f32 * 16.0;
                            let eps = 0.5;
                            let uv_min = egui::pos2((tile_x + eps) / 256.0, (tile_y + eps) / 48.0);
                            let uv_max = egui::pos2((tile_x + 16.0 - eps) / 256.0, (tile_y + 16.0 - eps) / 48.0);
                            let uv_rect = egui::Rect::from_min_max(uv_min, uv_max);

                            painter.image(tex.id(), tile_rect, uv_rect, egui::Color32::WHITE);
                        } else {
                            let fill_color = match tile_id {
                                14 => egui::Color32::from_rgb(50, 100, 200),
                                15 => egui::Color32::from_rgb(200, 50, 50),
                                0 => egui::Color32::BLACK,
                                _ => egui::Color32::from_gray(60),
                            };
                            painter.rect_filled(tile_rect, 0.0, fill_color);
                        }
                    }
                }

                // СЛОЙ 2: Тонкая сетка холста
                let grid_stroke = egui::Stroke::new(0.5, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 25));
                for x in 1..15 {
                    let current_x = rect.min.x + x as f32 * 32.0;
                    painter.line_segment([egui::pos2(current_x, rect.min.y), egui::pos2(current_x, rect.max.y)], grid_stroke);
                }
                for y in 1..10 {
                    let current_y = rect.min.y + y as f32 * 32.0;
                    painter.line_segment([egui::pos2(rect.min.x, current_y), egui::pos2(rect.max.x, current_y)], grid_stroke);
                }

                // СЛОЙ 3: Оверлей врагов
                for enemy in &screen_data.enemies {
                    let enemy_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + enemy.x as f32 * 32.0, rect.min.y + enemy.y as f32 * 32.0), egui::vec2(32.0, 32.0));
                    painter.rect_filled(enemy_rect, 4.0, egui::Color32::from_rgba_unmultiplied(180, 50, 255, 160));
                    painter.rect_stroke(enemy_rect, 4.0, egui::Stroke::new(2.0, egui::Color32::from_rgb(200, 100, 255)));
                    painter.text(enemy_rect.center(), egui::Align2::CENTER_CENTER, format!("E{}", enemy.tp), egui::FontId::proportional(13.0), egui::Color32::WHITE);
                }

                // СЛОЙ 4: Графический оверлей хотспотов на холсте!
                if screen_data.hotspot.tp > 0 {
                    let h_rect = egui::Rect::from_min_size(
                        egui::pos2(rect.min.x + screen_data.hotspot.x as f32 * 32.0, rect.min.y + screen_data.hotspot.y as f32 * 32.0),
                        egui::vec2(32.0, 32.0)
                    );

                    let target_tile_id = match screen_data.hotspot.tp {
                        1 => 17, // Предмет
                        2 => 18, // Ключ
                        3 => 16, // Аптечка
                        _ => 0
                    };

                    if let Some(tex) = tileset_texture {
                        let tile_x = (target_tile_id % 16) as f32 * 16.0;
                        let tile_y = (target_tile_id / 16) as f32 * 16.0;
                        let eps = 0.5;
                        let uv_min = egui::pos2((tile_x + eps) / 256.0, (tile_y + eps) / 48.0);
                        let uv_max = egui::pos2((tile_x + 16.0 - eps) / 256.0, (tile_y + 16.0 - eps) / 48.0);
                        let uv_rect = egui::Rect::from_min_max(uv_min, uv_max);

                        // Отрисовываем реальный пиксель-арт предмета, поверх накладываем легкое золотистое свечение
                        painter.image(tex.id(), h_rect, uv_rect, egui::Color32::WHITE);
                        painter.rect_stroke(h_rect, 0.0, egui::Stroke::new(2.0, egui::Color32::GOLD));
                    } else {
                        // Резервный текстовый маркер, если текстуры нет
                        let mark = match screen_data.hotspot.tp { 1 => "🌟", 2 => "🔑", 3 => "❤️", _ => "?" };
                        painter.text(h_rect.center(), egui::Align2::CENTER_CENTER, mark, egui::FontId::proportional(15.0), egui::Color32::WHITE);
                    }
                }

                // СЛОЙ 5: Оверлей Attribute Clash
                for error in clash_errors {
                    let error_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + (error.x_block as f32 * 16.0), rect.min.y + (error.y_block as f32 * 16.0)), egui::vec2(16.0, 16.0));
                    painter.rect_filled(error_rect, 0.0, egui::Color32::from_rgba_unmultiplied(255, 0, 0, 130));
                    painter.rect_stroke(error_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::RED));
                }
            });
        });
    });
}
