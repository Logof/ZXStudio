// src/ui/map_editor/map_canvas/entity_layer.rs
use crate::core::validator::ClashError;
use crate::models::{screen::Enemy, ScreenData};
use eframe::egui;

pub fn render_entities(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    scr_rect: egui::Rect,
    screen_data: &mut ScreenData,
    zoom: f32,
    _canvas_response: &egui::Response,
) {
    let mut enemy_to_update: Option<(usize, Enemy)> = None;
    let mouse_pos = ui.ctx().input(|i| i.pointer.hover_pos());
    let is_lkm_down = ui.ctx().input(|i| i.pointer.primary_down());

    let active_drag_enemy_id = egui::Id::new("canvas_active_drag_enemy_idx");
    let active_drag_handle_id = egui::Id::new("canvas_active_drag_handle_type");

    let mut current_dragged_enemy_idx: Option<usize> =
        ui.ctx().data(|d| d.get_temp(active_drag_enemy_id));
    let mut current_dragged_handle_type: Option<u8> =
        ui.ctx().data(|d| d.get_temp(active_drag_handle_id));

    if !is_lkm_down {
        current_dragged_enemy_idx = None;
        current_dragged_handle_type = None;
        ui.ctx().data_mut(|d| {
            d.remove_temp::<usize>(active_drag_enemy_id);
            d.remove_temp::<u8>(active_drag_handle_id);
        });
    }

    for (idx, enemy) in screen_data.enemies.iter().enumerate() {
        let e_rect = egui::Rect::from_min_size(
            egui::pos2(
                scr_rect.min.x + enemy.x as f32 * 32.0 * zoom,
                scr_rect.min.y + enemy.y as f32 * 32.0 * zoom,
            ),
            egui::vec2(32.0 * zoom, 32.0 * zoom),
        );

        painter.rect_filled(
            e_rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(255, 0, 100, 70),
        );
        painter.rect_stroke(
            e_rect,
            0.0,
            egui::Stroke::new(1.0 * zoom, egui::Color32::from_rgb(255, 0, 100)),
        );

        if zoom > 0.4 {
            painter.text(
                e_rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("👾{}", enemy.tp),
                egui::FontId::proportional(11.0 * zoom),
                egui::Color32::WHITE,
            );
        }

        let is_ghost_fanti = enemy.x1 == 0 && enemy.x2 == 0 && enemy.y1 == 0 && enemy.y2 == 0;
        let is_diagonal = !is_ghost_fanti && enemy.x1 != enemy.x2 && enemy.y1 != enemy.y2;

        let id_ai = egui::Id::new("default_enemy_ai_type_ctx");
        let active_palette_ai = ui.ctx().data(|d| d.get_temp::<u8>(id_ai)).unwrap_or(1);

        let is_quadrator = active_palette_ai == 9 || active_palette_ai == 10;
        let is_marruler = active_palette_ai >= 11 && active_palette_ai <= 14;
        let is_linear = !is_ghost_fanti && !is_quadrator && !is_marruler;

        if is_linear {
            let is_horizontal = !is_diagonal
                && (enemy.x1 != enemy.x2 || (enemy.y1 == enemy.y2 && enemy.x1 == enemy.x));

            let color_main = if is_diagonal {
                egui::Color32::from_rgb(255, 165, 0)
            } else if is_horizontal {
                egui::Color32::from_rgb(0, 150, 255)
            } else {
                egui::Color32::from_rgb(0, 255, 100)
            };

            let color_alpha = egui::Color32::from_rgba_unmultiplied(
                color_main.r(),
                color_main.g(),
                color_main.b(),
                30,
            );
            let t_stroke = egui::Stroke::new(2.0 * zoom, color_main);

            let b1_rect = egui::Rect::from_min_size(
                egui::pos2(
                    scr_rect.min.x + enemy.x1 as f32 * 32.0 * zoom,
                    scr_rect.min.y + enemy.y1 as f32 * 32.0 * zoom,
                ),
                egui::vec2(32.0 * zoom, 32.0 * zoom),
            );
            let b2_rect = egui::Rect::from_min_size(
                egui::pos2(
                    scr_rect.min.x + enemy.x2 as f32 * 32.0 * zoom,
                    scr_rect.min.y + enemy.y2 as f32 * 32.0 * zoom,
                ),
                egui::vec2(32.0 * zoom, 32.0 * zoom),
            );

            // Отрисовка чистой линии траектории
            painter.line_segment([b1_rect.center(), b2_rect.center()], t_stroke);

            // 🟩 ОКНО 1: СТАРТ (Остается классическим квадратом)
            painter.rect_filled(b1_rect, 0.0, color_alpha);
            painter.rect_stroke(b1_rect, 0.0, egui::Stroke::new(1.0 * zoom, color_main));

            // 🔵 ОКНО 2: ФИНИШ (Становится красивым кругом радиусом в половину тайла)
            let circle_radius = 16.0 * zoom;
            painter.circle_filled(b2_rect.center(), circle_radius, color_alpha);
            painter.circle_stroke(
                b2_rect.center(),
                circle_radius,
                egui::Stroke::new(1.0 * zoom, color_main),
            );

            // Центральные маленькие маркеры для драга (Ручка 1 - Квадрат, Ручка 2 - Круг)
            let handle_size = 10.0 * zoom;
            let h1_rect = egui::Rect::from_center_size(
                b1_rect.center(),
                egui::vec2(handle_size, handle_size),
            );
            painter.rect_filled(h1_rect, 2.0, color_main);

            painter.circle_filled(b2_rect.center(), handle_size / 2.0, color_main);

            if zoom > 0.5 {
                // Новая scannable-индикация: STRT (Start) и FNSH (Finish)
                painter.text(
                    b1_rect.min + egui::vec2(2.0, 2.0) * zoom,
                    egui::Align2::LEFT_TOP,
                    "S",
                    egui::FontId::monospace(8.0 * zoom),
                    color_main,
                );
                painter.text(
                    b2_rect.max - egui::vec2(2.0, 2.0) * zoom,
                    egui::Align2::RIGHT_BOTTOM,
                    "F",
                    egui::FontId::monospace(8.0 * zoom),
                    color_main,
                );
            }
        } else if is_quadrator || is_marruler {
            let color_zone = if is_quadrator {
                egui::Color32::from_rgb(0, 255, 150)
            } else {
                egui::Color32::from_rgb(255, 100, 255)
            };
            let text_zone = if is_quadrator {
                "🔄 ЗОНА AI (КУАДРАТОР)"
            } else {
                "🎲 ЗОНА AI (МАРРУЛЕР)"
            };

            let t_stroke = egui::Stroke::new(1.5 * zoom, color_zone);
            let box_rect = egui::Rect::from_min_max(
                egui::pos2(
                    scr_rect.min.x + enemy.x1 as f32 * 32.0 * zoom,
                    scr_rect.min.y + enemy.y1 as f32 * 32.0 * zoom,
                ),
                egui::pos2(
                    scr_rect.min.x + (enemy.x2 as f32 * 32.0 + 32.0) * zoom,
                    scr_rect.min.y + (enemy.y2 as f32 * 32.0 + 32.0) * zoom,
                ),
            );

            painter.rect_filled(
                box_rect,
                0.0,
                egui::Color32::from_rgba_unmultiplied(
                    color_zone.r(),
                    color_zone.g(),
                    color_zone.b(),
                    15,
                ),
            );
            if zoom > 0.5 {
                painter.text(
                    box_rect.min + egui::vec2(4.0, 4.0) * zoom,
                    egui::Align2::LEFT_TOP,
                    text_zone,
                    egui::FontId::monospace(9.0 * zoom),
                    color_zone,
                );
            }
            painter.rect_stroke(box_rect, 0.0, t_stroke);

            let b1_rect = egui::Rect::from_min_size(
                egui::pos2(
                    scr_rect.min.x + enemy.x1 as f32 * 32.0 * zoom,
                    scr_rect.min.y + enemy.y1 as f32 * 32.0 * zoom,
                ),
                egui::vec2(16.0 * zoom, 16.0 * zoom),
            );
            let b2_rect = egui::Rect::from_min_size(
                egui::pos2(
                    scr_rect.min.x + enemy.x2 as f32 * 32.0 * zoom + 16.0 * zoom,
                    scr_rect.min.y + enemy.y2 as f32 * 32.0 * zoom + 16.0 * zoom,
                ),
                egui::vec2(16.0 * zoom, 16.0 * zoom),
            );

            painter.rect_filled(b1_rect, 1.0, color_zone);
            painter.rect_filled(b2_rect, 1.0, color_zone);
        } else if is_ghost_fanti {
            if zoom > 0.5 {
                painter.text(
                    e_rect.max + egui::vec2(2.0, 2.0) * zoom,
                    egui::Align2::LEFT_TOP,
                    "👻 ФАНТИ AI",
                    egui::FontId::monospace(8.0 * zoom),
                    egui::Color32::LIGHT_BLUE,
                );
            }
        }
    }

    // ============================================================================
    // ВЫЧИСЛЕНИЕ ДВИЖЕНИЯ РУЧЕК
    // ============================================================================
    if let (Some(drag_idx), Some(handle_type)) =
        (current_dragged_enemy_idx, current_dragged_handle_type)
    {
        if let Some(pos) = mouse_pos {
            if let Some(enemy) = screen_data.enemies.get(drag_idx) {
                let mut updated_enemy = enemy.clone();

                let world_mouse_x = pos.x - scr_rect.min.x;
                let world_mouse_y = pos.y - scr_rect.min.y;

                let cell_x = ((world_mouse_x / (32.0 * zoom)).floor() as i32).clamp(0, 14) as u8;
                let cell_y = ((world_mouse_y / (32.0 * zoom)).floor() as i32).clamp(0, 9) as u8;

                let id_ai = egui::Id::new("default_enemy_ai_type_ctx");
                let active_palette_ai = ui.ctx().data(|d| d.get_temp::<u8>(id_ai)).unwrap_or(1);

                let is_quadrator = active_palette_ai == 9 || active_palette_ai == 10;
                let is_marruler = active_palette_ai >= 11 && active_palette_ai <= 14;

                if is_quadrator || is_marruler {
                    if handle_type == 1 {
                        updated_enemy.x1 = cell_x.min(enemy.x);
                        updated_enemy.y1 = cell_y.min(enemy.y);
                    } else {
                        updated_enemy.x2 = cell_x.max(enemy.x);
                        updated_enemy.y2 = cell_y.max(enemy.y);
                    }
                } else {
                    if handle_type == 1 {
                        updated_enemy.x1 = cell_x;
                        updated_enemy.y1 = cell_y;
                    } else {
                        updated_enemy.x2 = cell_x;
                        updated_enemy.y2 = cell_y;
                    }
                }

                enemy_to_update = Some((drag_idx, updated_enemy));
            }
        }
    }

    if let Some((idx, updated_enemy)) = enemy_to_update {
        screen_data.enemies[idx] = updated_enemy;
    }
}

pub fn render_clash_errors(
    painter: &egui::Painter,
    scr_rect: egui::Rect,
    clash_errors: &[ClashError],
    zoom: f32,
) {
    for error in clash_errors {
        let err_rect = egui::Rect::from_min_size(
            egui::pos2(
                scr_rect.min.x + (error.x_block as f32 * 16.0 * zoom),
                scr_rect.min.y + (error.y_block as f32 * 16.0 * zoom),
            ),
            egui::vec2(16.0 * zoom, 16.0 * zoom),
        );
        painter.rect_filled(
            err_rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(255, 0, 0, 110),
        );
        painter.rect_stroke(
            err_rect,
            0.0,
            egui::Stroke::new(0.8 * zoom, egui::Color32::RED),
        );
    }
}
