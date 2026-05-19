// src/ui/map_editor/map_canvas/scroll.rs
use eframe::egui;

pub fn render_scrollbars(
    ui: &mut egui::Ui,
    canvas_rect: egui::Rect,
    pan: &mut egui::Vec2,
    total_w: f32,
    total_h: f32,
    zoom: f32,
) {
    let thickness = 10.0;
    let padding_screen = 40.0;
    let view_w = canvas_rect.width();
    let view_h = canvas_rect.height();

    // Добавляем к виртуальному размеру мира отступы, чтобы карта не упиралась в края
    let world_w = total_w * zoom + padding_screen * 2.0;
    let world_h = total_h * zoom + padding_screen * 2.0;

    // Смещаем координатную сетку панорамирования с учетом отступа
    let min_pan_x = -(world_w - view_w).max(0.0) + padding_screen;
    let min_pan_y = -(world_h - view_h).max(0.0) + padding_screen;
    let max_pan_x = padding_screen;
    let max_pan_y = padding_screen;

    // Горизонтальный скроллбар (только если мир шире экрана)
    if world_w > view_w {
        let bar_rect = egui::Rect::from_min_max(
            egui::pos2(canvas_rect.min.x + 4.0, canvas_rect.max.y - thickness - 4.0),
            egui::pos2(canvas_rect.max.x - thickness - 8.0, canvas_rect.max.y - 4.0),
        );
        ui.painter().rect_filled(
            bar_rect,
            3.0,
            egui::Color32::from_rgba_unmultiplied(30, 30, 30, 60),
        );

        let handle_width = ((view_w / world_w) * bar_rect.width()).clamp(20.0, bar_rect.width());
        let total_travel = max_pan_x - min_pan_x;

        let progress_x = if total_travel > 0.0 {
            ((max_pan_x - pan.x) / total_travel).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let handle_left = bar_rect.min.x + progress_x * (bar_rect.width() - handle_width);
        let handle_rect = egui::Rect::from_min_size(
            egui::pos2(handle_left, bar_rect.min.y),
            egui::vec2(handle_width, thickness),
        );

        let handle_id = ui.make_persistent_id("canvas_h_scroll");
        let handle_resp = ui.interact(handle_rect, handle_id, egui::Sense::drag());

        if handle_resp.dragged() && total_travel > 0.0 {
            let delta_x = handle_resp.drag_delta().x;
            let percent_delta = delta_x / (bar_rect.width() - handle_width);
            pan.x -= percent_delta * total_travel;
        }

        let alpha = if handle_resp.hovered() || handle_resp.dragged() {
            160
        } else {
            90
        };
        ui.painter().rect_filled(
            handle_rect,
            3.0,
            egui::Color32::from_rgba_unmultiplied(120, 120, 120, alpha),
        );
    }

    // Вертикальный скроллбар
    if world_h > view_h {
        let bar_rect = egui::Rect::from_min_max(
            egui::pos2(canvas_rect.max.x - thickness - 4.0, canvas_rect.min.y + 4.0),
            egui::pos2(canvas_rect.max.x - 4.0, canvas_rect.max.y - thickness - 8.0),
        );
        ui.painter().rect_filled(
            bar_rect,
            3.0,
            egui::Color32::from_rgba_unmultiplied(30, 30, 30, 60),
        );

        let handle_height = ((view_h / world_h) * bar_rect.height()).clamp(20.0, bar_rect.height());
        let total_travel = max_pan_y - min_pan_y;

        let progress_y = if total_travel > 0.0 {
            ((max_pan_y - pan.y) / total_travel).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let handle_top = bar_rect.min.y + progress_y * (bar_rect.height() - handle_height);
        let handle_rect = egui::Rect::from_min_size(
            egui::pos2(bar_rect.min.x, handle_top),
            egui::vec2(thickness, handle_height),
        );

        let handle_id = ui.make_persistent_id("canvas_v_scroll");
        let handle_resp = ui.interact(handle_rect, handle_id, egui::Sense::drag());

        if handle_resp.dragged() && total_travel > 0.0 {
            let delta_y = handle_resp.drag_delta().y;
            let percent_delta = delta_y / (bar_rect.height() - handle_height);
            pan.y -= percent_delta * total_travel;
        }

        let alpha = if handle_resp.hovered() || handle_resp.dragged() {
            160
        } else {
            90
        };
        ui.painter().rect_filled(
            handle_rect,
            3.0,
            egui::Color32::from_rgba_unmultiplied(120, 120, 120, alpha),
        );
    }

    // Жестко зажимаем панорамирование в рамках рассчитанных лимитов с учетом отступов
    if world_w > view_w {
        pan.x = pan.x.clamp(min_pan_x, max_pan_x);
    } else {
        // Если карта целиком влезает по ширине — центрируем её по оси X
        pan.x = (view_w - total_w * zoom) / 2.0;
    }

    if world_h > view_h {
        pan.y = pan.y.clamp(min_pan_y, max_pan_y);
    } else {
        // Если карта целиком влезает по высоте — центрируем её по оси Y
        pan.y = (view_h - total_h * zoom) / 2.0;
    }
}
