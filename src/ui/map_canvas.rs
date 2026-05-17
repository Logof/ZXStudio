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
) {
    let scr_key = format!("screen_{}", selected_screen);
    let screen_data = project.screens.entry(scr_key).or_insert_with(ScreenData::default);

    ui.label(format!("Холст игрового экрана {} (15x10 тайлов)", selected_screen));

    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(15.0 * 32.0, 10.0 * 32.0),
            egui::Sense::click_and_drag()
        );
        let painter = ui.painter_at(rect);
        
        // Клик ЛКМ — Рисование / Спавн
        if response.dragged() || response.clicked() {
            if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                if rect.contains(mouse_pos) {
                    let cell_x = ((mouse_pos.x - rect.min.x) / 32.0) as u8;
                    let cell_y = ((mouse_pos.y - rect.min.y) / 32.0) as u8;

                    match map_edit_mode {
                        MapEditMode::Tiles => {
                            let index = (cell_y as usize) * 15 + (cell_x as usize);
                            if index < 150 { screen_data.tiles_matrix[index] = *selected_tile; }
                        }
                        MapEditMode::Enemies => {
                            if screen_data.enemies.len() < 3 && ui.ctx().input(|i| i.pointer.any_pressed()) {
                                if !screen_data.enemies.iter().any(|e| e.x == cell_x && e.y == cell_y) {
                                    screen_data.enemies.push(Enemy {
                                        tp: selected_enemy_type,
                                        x: cell_x,
                                        y: cell_y,
                                        x1: cell_x.saturating_sub(2),
                                        x2: (cell_x + 2).min(14),
                                        y1: cell_y.saturating_sub(2),
                                        y2: (cell_y + 2).min(9),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Клик ПКМ — Стирание
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

        // Отрисовка тайлов
        for y in 0..10 {
            for x in 0..15 {
                let idx = y * 15 + x;
                let tile_id = screen_data.tiles_matrix[idx];
                let tile_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + x as f32 * 32.0, rect.min.y + y as f32 * 32.0), egui::vec2(32.0, 32.0));

                if let Some(tex) = &project.role_pushbox_active.then(|| ()).and_then(|_| None).or(ui.ctx().data(|d| d.get_temp(egui::Id::new("tileset_tex")))) {
                    let tex_handle: &egui::TextureHandle = tex;
                    let tile_x = (tile_id % 16) as f32 * 16.0;
                    let tile_y = (tile_id / 16) as f32 * 16.0;
                    let eps = 0.5;
                    let uv_min = egui::pos2((tile_x + eps) / 256.0, (tile_y + eps) / 48.0);
                    let uv_max = egui::pos2((tile_x + 16.0 - eps) / 256.0, (tile_y + 16.0 - eps) / 48.0);
                    painter.image(tex_handle.id(), tile_rect, egui::Rect::from_min_max(uv_min, uv_max), egui::Color32::WHITE);
                } else {
                    let fill_color = match tile_id { 14 => egui::Color32::BLUE, 15 => egui::Color32::RED, _ => egui::Color32::BLACK };
                    painter.rect_filled(tile_rect, 0.0, fill_color);
                }
            }
        }

        // Полупрозрачная сетка
        let grid_stroke = egui::Stroke::new(0.5, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 25));
        for x in 1..15 { let cx = rect.min.x + x as f32 * 32.0; painter.line_segment([egui::pos2(cx, rect.min.y), egui::pos2(cx, rect.max.y)], grid_stroke); }
        for y in 1..10 { let cy = rect.min.y + y as f32 * 32.0; painter.line_segment([egui::pos2(rect.min.x, cy), egui::pos2(rect.max.x, cy)], grid_stroke); }

        // Оверлей врагов и траекторий
        for enemy in &screen_data.enemies {
            let enemy_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + enemy.x as f32 * 32.0, rect.min.y + enemy.y as f32 * 32.0), egui::vec2(32.0, 32.0));
            painter.rect_filled(enemy_rect, 4.0, egui::Color32::from_rgba_unmultiplied(180, 50, 255, 160));
            painter.rect_stroke(enemy_rect, 4.0, egui::Stroke::new(2.0, egui::Color32::from_rgb(200, 100, 255)));
            painter.text(enemy_rect.center(), egui::Align2::CENTER_CENTER, format!("E{}", enemy.tp), egui::FontId::proportional(13.0), egui::Color32::WHITE);

            let t_stroke = egui::Stroke::new(1.5, egui::Color32::from_rgba_unmultiplied(0, 255, 150, 120));
            if enemy.tp >= 1 && enemy.tp <= 4 {
                let is_horizontal = enemy.x1 != enemy.x2 || (enemy.y1 == enemy.y2 && enemy.x1 == enemy.x);
                            
                let start_p;
                let end_p;
                let t_stroke;

                if is_horizontal {
                    // Горизонтальная траектория — рисуем бирюзовую прямую линию по X
                    t_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 150, 255));
                    start_p = egui::pos2(rect.min.x + enemy.x1 as f32 * 32.0 + 16.0, rect.min.y + enemy.y as f32 * 32.0 + 16.0);
                    end_p = egui::pos2(rect.min.x + enemy.x2 as f32 * 32.0 + 16.0, rect.min.y + enemy.y as f32 * 32.0 + 16.0);
                } else {
                    // Вертикальная траектория — рисуем неоновую зеленую прямую линию по Y
                    t_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 255, 100));
                    start_p = egui::pos2(rect.min.x + enemy.x as f32 * 32.0 + 16.0, rect.min.y + enemy.y1 as f32 * 32.0 + 16.0);
                    end_p = egui::pos2(rect.min.x + enemy.x as f32 * 32.0 + 16.0, rect.min.y + enemy.y2 as f32 * 32.0 + 16.0);
                }

                // Отрисовываем вектор хода и крайние точки разворота ИИ
                painter.line_segment([start_p, end_p], t_stroke);
                painter.circle_filled(start_p, 4.0, egui::Color32::WHITE);
                painter.circle_filled(end_p, 4.0, egui::Color32::WHITE);
            } else if enemy.tp >= 7 && enemy.tp <= 10 {
                let box_rect = egui::Rect::from_min_max(
                    egui::pos2(rect.min.x + enemy.x1 as f32 * 32.0, rect.min.y + enemy.y1 as f32 * 32.0),
                    egui::pos2(rect.min.x + enemy.x2 as f32 * 32.0 + 32.0, rect.min.y + enemy.y2 as f32 * 32.0 + 32.0)
                );
                painter.rect_stroke(box_rect, 0.0, t_stroke);
            }
        }

        // Оверлей Attribute Clash
        for error in clash_errors {
            let error_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + (error.x_block as f32 * 16.0), rect.min.y + (error.y_block as f32 * 16.0)), egui::vec2(16.0, 16.0));
            painter.rect_filled(error_rect, 0.0, egui::Color32::from_rgba_unmultiplied(255, 0, 0, 130));
            painter.rect_stroke(error_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::RED));
        }
    });
}
