use crate::models::ScreenData;
use eframe::egui;

pub fn render_tiles(
    painter: &egui::Painter,
    scr_rect: egui::Rect,
    screen_data: &ScreenData,
    zoom: f32,
    sliced_tile_textures: &[egui::TextureHandle],
) {
    // 1. Отрисовка стандартной геометрии уровня из матрицы тайлов
    for y in 0..10 {
        for x in 0..15 {
            let idx = y * 15 + x;
            let tile_id = screen_data.tiles_matrix[idx] as usize;

            let t_min_x = scr_rect.min.x + (x as f32 * 32.0 * zoom);
            let t_min_y = scr_rect.min.y + (y as f32 * 32.0 * zoom);
            let tile_rect = egui::Rect::from_min_size(
                egui::pos2(t_min_x, t_min_y),
                egui::vec2(32.0 * zoom, 32.0 * zoom),
            );

            if let Some(tex) = sliced_tile_textures.get(tile_id) {
                let uv_rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
                painter.image(tex.id(), tile_rect, uv_rect, egui::Color32::WHITE);
            } else {
                let fill_color = match tile_id {
                    14 => egui::Color32::from_rgb(50, 100, 200), // Синий для пуш-блока
                    15 => egui::Color32::from_rgb(200, 50, 50),  // Красный для замка
                    0 => egui::Color32::BLACK,
                    _ => egui::Color32::from_gray(60),
                };
                painter.rect_filled(tile_rect, 0.0, fill_color);
            }
        }
    }

    // ============================================================================
    // НОВОЕ УЛУЧШЕНИЕ: Отрисовка установленного на экране Hotspot (Предмета/Ключа)
    // ============================================================================
    let hs = &screen_data.hotspot;
    if hs.type_id != 0 {
        let hs_min_x = scr_rect.min.x + (hs.x as f32 * 32.0 * zoom);
        let hs_min_y = scr_rect.min.y + (hs.y as f32 * 32.0 * zoom);
        let hs_rect = egui::Rect::from_min_size(
            egui::pos2(hs_min_x, hs_min_y),
            egui::vec2(32.0 * zoom, 32.0 * zoom),
        );

        // Определяем графический индекс тайла для превью на основе типа хотспота
        // Переводим внутренний Си-тип обратно в индекс палитры для наглядности художника
        let source_tile_id = match hs.type_id {
            1 => 18, // Hotspot 1 -> Ключ (Тайл 18)
            2 => 16, // Hotspot 2 -> Жизнь (Тайл 16)
            3 => 17, // Hotspot 3 -> Предмет (Тайл 17)
            _ => 0,
        };

        if let Some(tex) = sliced_tile_textures.get(source_tile_id) {
            // Рисуем сам спрайт предмета из палитры
            let uv_rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
            painter.image(tex.id(), hs_rect, uv_rect, egui::Color32::WHITE);

            // Накладываем полупрозрачную цветную рамку-маркер, чтобы геймдизайнер видел, что это спец-зона
            let marker_color = match hs.type_id {
                1 => egui::Color32::from_rgba_unmultiplied(255, 200, 0, 80), // Золотая рамка для Ключа
                2 => egui::Color32::from_rgba_unmultiplied(255, 50, 50, 80), // Красная рамка для Жизни
                3 => egui::Color32::from_rgba_unmultiplied(50, 255, 50, 80), // Зеленая рамка для Квеста
                _ => egui::Color32::TRANSPARENT,
            };
            painter.rect_stroke(hs_rect, 0.0, egui::Stroke::new(1.5 * zoom, marker_color));
        } else {
            // Текстовый плейсхолдер, если графика work.png ещё не подгружена
            let (label, color) = match hs.type_id {
                1 => ("🔑 KEY", egui::Color32::GOLD),
                2 => ("❤️ LIFE", egui::Color32::LIGHT_RED),
                3 => ("🌟 ITEM", egui::Color32::LIGHT_GREEN),
                _ => ("?", egui::Color32::GRAY),
            };
            painter.rect_filled(
                hs_rect,
                2.0,
                egui::Color32::from_rgba_unmultiplied(30, 30, 40, 200),
            );
            painter.text(
                hs_rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(9.0 * zoom),
                color,
            );
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
    let t_min_x = scr_rect.min.x + (cell_x as f32 * 32.0 * zoom);
    let t_min_y = scr_rect.min.y + (cell_y as f32 * 32.0 * zoom);
    let tile_rect = egui::Rect::from_min_size(
        egui::pos2(t_min_x, t_min_y),
        egui::vec2(32.0 * zoom, 32.0 * zoom),
    );

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
