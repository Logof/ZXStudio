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
    let view_w = canvas_rect.width();
    let view_h = canvas_rect.height();
    let world_w = total_w * zoom;
    let world_h = total_h * zoom;

    // Горизонтальный скроллбар (только если мир шире экрана)
    if world_w > view_w {
        let bar_rect = egui::Rect::from_min_max(
            egui::pos2(canvas_rect.min.x, canvas_rect.max.y - thickness),
            egui::pos2(canvas_rect.max.x - thickness, canvas_rect.max.y),
        );
        ui.painter().rect_filled(
            bar_rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(30, 30, 30, 100),
        );

        let handle_width = ((view_w / world_w) * bar_rect.width()).clamp(20.0, bar_rect.width());
        let max_pan_x = world_w - view_w;
        let progress_x = (-pan.x / max_pan_x).clamp(0.0, 1.0);
        let handle_left = bar_rect.min.x + progress_x * (bar_rect.width() - handle_width);

        let handle_rect = egui::Rect::from_min_size(
            egui::pos2(handle_left, bar_rect.min.y),
            egui::vec2(handle_width, thickness),
        );
        let handle_id = ui.make_persistent_id("canvas_h_scroll");
        let handle_resp = ui.interact(handle_rect, handle_id, egui::Sense::drag());

        if handle_resp.dragged() {
            let delta_x = handle_resp.drag_delta().x;
            let percent_delta = delta_x / (bar_rect.width() - handle_width);
            pan.x -= percent_delta * max_pan_x;
        }
        ui.painter()
            .rect_filled(handle_rect, 4.0, egui::Color32::from_gray(100));
    }

    // Вертикальный скроллбар
    if world_h > view_h {
        let bar_rect = egui::Rect::from_min_max(
            egui::pos2(canvas_rect.max.x - thickness, canvas_rect.min.y),
            egui::pos2(canvas_rect.max.x, canvas_rect.max.y - thickness),
        );
        ui.painter().rect_filled(
            bar_rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(30, 30, 30, 100),
        );

        let handle_height = ((view_h / world_h) * bar_rect.height()).clamp(20.0, bar_rect.height());
        let max_pan_y = world_h - view_h;
        let progress_y = (-pan.y / max_pan_y).clamp(0.0, 1.0);
        let handle_top = bar_rect.min.y + progress_y * (bar_rect.height() - handle_height);

        let handle_rect = egui::Rect::from_min_size(
            egui::pos2(bar_rect.min.x, handle_top),
            egui::vec2(thickness, handle_height),
        );
        let handle_id = ui.make_persistent_id("canvas_v_scroll");
        let handle_resp = ui.interact(handle_rect, handle_id, egui::Sense::drag());

        if handle_resp.dragged() {
            let delta_y = handle_resp.drag_delta().y;
            let percent_delta = delta_y / (bar_rect.height() - handle_height);
            pan.y -= percent_delta * max_pan_y;
        }
        ui.painter()
            .rect_filled(handle_rect, 4.0, egui::Color32::from_gray(100));
    }

    // Ограничение вылета карты за границы видимости при ручном перемещении
    pan.x = pan.x.clamp(-(total_w * zoom - view_w).max(0.0), 0.0);
    pan.y = pan.y.clamp(-(total_h * zoom - view_h).max(0.0), 0.0);
}
