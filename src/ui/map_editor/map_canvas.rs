use eframe::egui;
use crate::models::{ProjectData, ScreenData, screen::Enemy};
use crate::core::validator::ClashError;
use crate::app::states::MapEditMode;

pub fn render_map_canvas(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_screen: &mut usize,
    selected_tile: &mut u8,
    clash_errors: &[ClashError],
    map_edit_mode: &MapEditMode,
    selected_enemy_type: u8,
    tileset_texture: &Option<egui::TextureHandle>,
) {
    let scr_key = format!("screen_{}", selected_screen);
    let screen_data = project.screens.entry(scr_key).or_insert_with(ScreenData::default);

    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(15.0 * 32.0, 10.0 * 32.0),
            egui::Sense::click_and_drag()
        );
        let painter = ui.painter_at(rect);
        
        let mut cell_pos = None;
        if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
            if rect.contains(mouse_pos) {
                let cx = ((mouse_pos.x - rect.min.x) / 32.0) as u8;
                let cy = ((mouse_pos.y - rect.min.y) / 32.0) as u8;
                if cx < 15 && cy < 10 { cell_pos = Some((cx, cy)); }
            }
        }

        // ЛКМ: Рисование тайлов и спавн врагов
        if response.dragged() || response.clicked() {
            if let Some((cell_x, cell_y)) = cell_pos {
                match map_edit_mode {
                    MapEditMode::Tiles => {
                        let index = (cell_y as usize) * 15 + (cell_x as usize);
                        screen_data.tiles_matrix[index] = *selected_tile;
                    }
                    MapEditMode::Enemies => {
                        if response.clicked() && screen_data.enemies.len() < 3 {
                            if !screen_data.enemies.iter().any(|e| e.x == cell_x && e.y == cell_y) {
                                
                                // Извлекаем выбранный в палитре слева дефолтный ИИ (по умолчанию 1)
                                let id_ai = egui::Id::new("default_enemy_ai_type_ctx");
                                let default_ai = ui.ctx().data(|d| d.get_temp::<u8>(id_ai)).unwrap_or(1);

                                // Настраиваем геометрические лимиты Моджонов под тип ИИ
                                let (x1, x2, y1, y2) = match default_ai {
                                    1..=4 => (cell_x, (cell_x + 2).min(14), cell_y, cell_y), // Линейный ход
                                    5 | 6 => (0, 0, 0, 0), // Призраки фанти (тотальное зануление осей)
                                    7..=10 => (cell_x.saturating_sub(1), (cell_x + 2).min(14), cell_y.saturating_sub(1), (cell_y + 2).min(9)), // Рамка Куадратора
                                    11..=14 => (cell_x, cell_x, cell_y, cell_y), // Маррулер
                                    _ => (cell_x, cell_x, cell_y, cell_y)
                                };

                                screen_data.enemies.push(Enemy {
                                    tp: selected_enemy_type, // Наш графический индекс 0..3
                                    x: cell_x,
                                    y: cell_y,
                                    x1,
                                    x2,
                                    y1,
                                    y2,
                                });
                            }
                        }
                    }
                }
            }
        }

        // ПКМ: Стирание
        if response.secondary_clicked() {
            if let Some((cell_x, cell_y)) = cell_pos {
                match map_edit_mode {
                    MapEditMode::Tiles => {
                        let index = (cell_y as usize) * 15 + (cell_x as usize);
                        screen_data.tiles_matrix[index] = 0;
                    }
                    MapEditMode::Enemies => {
                        screen_data.enemies.retain(|e| e.x != cell_x || e.y != cell_y);
                    }
                }
            }
        }

        // СЛОЙ 1: Тайлы
        for y in 0..10 {
            for x in 0..15 {
                let idx = y * 15 + x;
                let tile_id = screen_data.tiles_matrix[idx];
                let tile_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + x as f32 * 32.0, rect.min.y + y as f32 * 32.0), egui::vec2(32.0, 32.0));

                if let Some(tex) = tileset_texture {
                    let tile_x = (tile_id % 16) as f32 * 16.0;
                    let tile_y = (tile_id / 16) as f32 * 16.0;
                    let eps = 0.5;
                    let uv_min = egui::pos2((tile_x + eps) / 256.0, (tile_y + eps) / 48.0);
                    let uv_max = egui::pos2((tile_x + 16.0 - eps) / 256.0, (tile_y + 16.0 - eps) / 48.0);
                    painter.image(tex.id(), tile_rect, egui::Rect::from_min_max(uv_min, uv_max), egui::Color32::WHITE);
                } else {
                    let fill_color = match tile_id { 
                        14 => egui::Color32::from_rgb(50, 100, 200), 
                        15 => egui::Color32::from_rgb(200, 50, 50), 
                        0 => egui::Color32::BLACK,
                        _ => egui::Color32::from_gray(60) 
                    };
                    painter.rect_filled(tile_rect, 0.0, fill_color);
                }
            }
        }

        // СЛОЙ 2: Сетка
        let grid_stroke = egui::Stroke::new(0.5, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 25));
        for x in 1..15 { let cx = rect.min.x + x as f32 * 32.0; painter.line_segment([egui::pos2(cx, rect.min.y), egui::pos2(cx, rect.max.y)], grid_stroke); }
        for y in 1..10 { let cy = rect.min.y + y as f32 * 32.0; painter.line_segment([egui::pos2(rect.min.x, cy), egui::pos2(rect.max.x, cy)], grid_stroke); }

        // СЛОЙ 3: Враги и траектории
        for enemy in &screen_data.enemies {
            let enemy_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + enemy.x as f32 * 32.0, rect.min.y + enemy.y as f32 * 32.0), egui::vec2(32.0, 32.0));
            painter.rect_filled(enemy_rect, 0.0, egui::Color32::from_rgba_unmultiplied(255, 0, 100, 70));
            painter.rect_stroke(enemy_rect, 0.0, egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 0, 100)));
            painter.text(enemy_rect.center(), egui::Align2::CENTER_CENTER, format!("👾{}", enemy.tp), egui::FontId::proportional(12.0), egui::Color32::WHITE);

            if enemy.tp >= 0 && enemy.tp <= 3 {
                // Рисование прямых линий для линейных врагов 0..3
                let is_horizontal = enemy.x1 != enemy.x2 || (enemy.y1 == enemy.y2 && enemy.x1 == enemy.x);
                let start_p;
                let end_p;
                let t_stroke = if is_horizontal { egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 150, 255)) } else { egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 255, 100)) };
                
                if is_horizontal {
                    start_p = egui::pos2(rect.min.x + enemy.x1 as f32 * 32.0 + 16.0, rect.min.y + enemy.y as f32 * 32.0 + 16.0);
                    end_p = egui::pos2(rect.min.x + enemy.x2 as f32 * 32.0 + 16.0, rect.min.y + enemy.y as f32 * 32.0 + 16.0);
                } else {
                    start_p = egui::pos2(rect.min.x + enemy.x as f32 * 32.0 + 16.0, rect.min.y + enemy.y1 as f32 * 32.0 + 16.0);
                    end_p = egui::pos2(rect.min.x + enemy.x as f32 * 32.0 + 16.0, rect.min.y + enemy.y2 as f32 * 32.0 + 16.0);
                }
                painter.line_segment([start_p, end_p], t_stroke);
            } else if enemy.tp >= 6 && enemy.tp <= 7 {
                // Рисование рамки для типов 6..7
                let t_stroke = egui::Stroke::new(1.5, egui::Color32::from_rgba_unmultiplied(0, 255, 150, 120));
                let box_rect = egui::Rect::from_min_max(
                    egui::pos2(rect.min.x + enemy.x1 as f32 * 32.0, rect.min.y + enemy.y1 as f32 * 32.0),
                    egui::pos2(rect.min.x + enemy.x2 as f32 * 32.0 + 32.0, rect.min.y + enemy.y2 as f32 * 32.0 + 32.0)
                );
                painter.rect_stroke(box_rect, 0.0, t_stroke);
            }
        }

        // СЛОЙ 4 (Бывшие Хотспоты) — ПОЛНОСТЬЮ УДАЛЕН

        // СЛОЙ 5: Ошибки Attribute Clash
        for error in clash_errors {
            let error_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + (error.x_block as f32 * 16.0), rect.min.y + (error.y_block as f32 * 16.0)), egui::vec2(16.0, 16.0));
            painter.rect_filled(error_rect, 0.0, egui::Color32::from_rgba_unmultiplied(255, 0, 0, 130));
            painter.rect_stroke(error_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::RED));
        }
    });
}
