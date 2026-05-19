// src/ui/map_editor/map_canvas/tile_layer.rs
use crate::models::ScreenData;
use eframe::egui;

pub fn render_tiles(
    painter: &egui::Painter,
    scr_rect: egui::Rect,
    screen_data: &ScreenData,
    zoom: f32,
    tileset_texture: &Option<egui::TextureHandle>,
) {
    for y in 0..10 {
        for x in 0..15 {
            let idx = y * 15 + x;
            let tile_id = screen_data.tiles_matrix[idx];

            let t_min_x = scr_rect.min.x + (x as f32 * 32.0 * zoom);
            let t_min_y = scr_rect.min.y + (y as f32 * 32.0 * zoom);
            let tile_rect = egui::Rect::from_min_size(
                egui::pos2(t_min_x, t_min_y),
                egui::vec2(32.0 * zoom, 32.0 * zoom),
            );

            if let Some(tex) = tileset_texture {
                let tile_x = (tile_id % 16) as f32 * 16.0;
                let tile_y = (tile_id / 16) as f32 * 16.0;
                let eps = 0.5;
                let uv_min = egui::pos2((tile_x + eps) / 256.0, (tile_y + eps) / 48.0);
                let uv_max =
                    egui::pos2((tile_x + 16.0 - eps) / 256.0, (tile_y + 16.0 - eps) / 48.0);
                painter.image(
                    tex.id(),
                    tile_rect,
                    egui::Rect::from_min_max(uv_min, uv_max),
                    egui::Color32::WHITE,
                );
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
}

pub fn render_grid(painter: &egui::Painter, scr_rect: egui::Rect, zoom: f32) {
    if zoom > 0.4 {
        let grid_stroke = egui::Stroke::new(
            0.5 * zoom,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20),
        );
        for x in 1..15 {
            let cx = scr_rect.min.x + x as f32 * 32.0 * zoom;
            painter.line_segment(
                [
                    egui::pos2(cx, scr_rect.min.y),
                    egui::pos2(cx, scr_rect.max.y),
                ],
                grid_stroke,
            );
        }
        for y in 1..10 {
            let cy = scr_rect.min.y + y as f32 * 32.0 * zoom;
            painter.line_segment(
                [
                    egui::pos2(scr_rect.min.x, cy),
                    egui::pos2(scr_rect.max.x, cy),
                ],
                grid_stroke,
            );
        }
    }
}

pub fn render_hover_frame(
    painter: &egui::Painter,
    scr_rect: egui::Rect,
    cell_x: u8,
    cell_y: u8,
    zoom: f32,
) {
    // Вычисляем физические экранные координаты конкретной клетки 32x32 под курсором
    let t_min_x = scr_rect.min.x + (cell_x as f32 * 32.0 * zoom);
    let t_min_y = scr_rect.min.y + (cell_y as f32 * 32.0 * zoom);
    let tile_rect = egui::Rect::from_min_size(
        egui::pos2(t_min_x, t_min_y),
        egui::vec2(32.0 * zoom, 32.0 * zoom),
    );

    // Подсвечиваем клетку яркой белой рамкой с небольшой внутренней заливкой
    let hover_stroke = egui::Stroke::new(
        1.5 * zoom,
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 200),
    );
    painter.rect_stroke(tile_rect, 0.0, hover_stroke);
    painter.rect_filled(
        tile_rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 30),
    );
}
