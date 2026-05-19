// src/ui/map_editor/map_canvas/entity_layer.rs
use crate::core::validator::ClashError;
use crate::models::ScreenData;
use eframe::egui;

pub fn render_entities(
    painter: &egui::Painter,
    scr_rect: egui::Rect,
    screen_data: &ScreenData,
    zoom: f32,
) {
    for enemy in &screen_data.enemies {
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

        if enemy.tp <= 3 {
            let is_horizontal =
                enemy.x1 != enemy.x2 || (enemy.y1 == enemy.y2 && enemy.x1 == enemy.x);
            let start_p;
            let end_p;
            let t_stroke = if is_horizontal {
                egui::Stroke::new(1.5 * zoom, egui::Color32::from_rgb(0, 150, 255))
            } else {
                egui::Stroke::new(1.5 * zoom, egui::Color32::from_rgb(0, 255, 100))
            };

            if is_horizontal {
                start_p = egui::pos2(
                    scr_rect.min.x + (enemy.x1 as f32 * 32.0 + 16.0) * zoom,
                    scr_rect.min.y + (enemy.y as f32 * 32.0 + 16.0) * zoom,
                );
                end_p = egui::pos2(
                    scr_rect.min.x + (enemy.x2 as f32 * 32.0 + 16.0) * zoom,
                    scr_rect.min.y + (enemy.y as f32 * 32.0 + 16.0) * zoom,
                );
            } else {
                start_p = egui::pos2(
                    scr_rect.min.x + (enemy.x as f32 * 32.0 + 16.0) * zoom,
                    scr_rect.min.y + (enemy.y1 as f32 * 32.0 + 16.0) * zoom,
                );
                end_p = egui::pos2(
                    scr_rect.min.x + (enemy.x as f32 * 32.0 + 16.0) * zoom,
                    scr_rect.min.y + (enemy.y2 as f32 * 32.0 + 16.0) * zoom,
                );
            }
            painter.line_segment([start_p, end_p], t_stroke);
        } else if enemy.tp >= 6 && enemy.tp <= 7 {
            let t_stroke = egui::Stroke::new(
                1.0 * zoom,
                egui::Color32::from_rgba_unmultiplied(0, 255, 150, 120),
            );
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
            painter.rect_stroke(box_rect, 0.0, t_stroke);
        }
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
