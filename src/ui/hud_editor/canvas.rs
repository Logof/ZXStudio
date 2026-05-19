// src/ui/hud_editor/canvas.rs
use super::sidebar::get_mut_coords;
use super::types::HudItemMetadata;
use crate::models::config::hud_rendering::HudRenderingConfig;
use eframe::egui;

pub fn render_canvas(
    ui: &mut egui::Ui,
    elements: &[HudItemMetadata],
    hud_config: &mut HudRenderingConfig,
    hud_frame: &Option<egui::TextureHandle>,
    drag_id: egui::Id,
    mut current_drag: Option<String>,
) {
    ui.vertical(|ui| {
        let native_width = 256.0;
        let native_height = 192.0;
        let zoom = 2.5;
        let scale = |v: f32| v * zoom;
        let cell_size = 8.0 * zoom;

        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(scale(native_width), scale(native_height)),
                egui::Sense::click_and_drag(),
            );
            let painter = ui.painter_at(rect);

            let vx = hud_config.viewport_x as f32;
            let vy = hud_config.viewport_y as f32;
            let view_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x + vx * cell_size, rect.min.y + vy * cell_size),
                egui::vec2(30.0 * cell_size, 20.0 * cell_size),
            );

            let viewport_drag_id = "viewport_zone".to_string();

            // 1. ПЕРЕХВАТ МЫШИ (Drag Start)
            if response.drag_started() {
                if let Some(mouse) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                    let mut found_id = None;

                    for item in elements.iter() {
                        // Используем безопасный хелпер (иммутабельно через разыменование ссылки)
                        let (cx, cy) = get_read_coords(item.id, hud_config);
                        if cx != 99 && cy != 99 {
                            let item_rect = egui::Rect::from_min_size(
                                egui::pos2(
                                    rect.min.x + cx as f32 * cell_size,
                                    rect.min.y + cy as f32 * cell_size,
                                ),
                                egui::vec2(item.width_blocks as f32 * cell_size, cell_size),
                            );
                            if item_rect.contains(mouse) {
                                found_id = Some(item.id.to_string());
                                break;
                            }
                        }
                    }

                    if found_id.is_none() && view_rect.contains(mouse) {
                        found_id = Some(viewport_drag_id.clone());
                    }

                    if let Some(id) = found_id {
                        current_drag = Some(id.clone());
                        ui.ctx().data_mut(|d| d.insert_temp(drag_id, id));
                    }
                }
            }

            // 2. ОБРАБОТКА ДВИЖЕНИЯ (Drag Move)
            if response.dragged() {
                if let Some(ref active_id) = current_drag {
                    if let Some(mouse) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                        let cell_x = ((mouse.x - rect.min.x) / cell_size).floor() as i32;
                        let cell_y = ((mouse.y - rect.min.y) / cell_size).floor() as u32;

                        if active_id == &viewport_drag_id {
                            hud_config.viewport_x = cell_x.clamp(0, 32 - 30) as u32;
                            hud_config.viewport_y = cell_y.clamp(0, 24 - 20);
                        } else {
                            // Безопасно запрашиваем мутабельный доступ СТРОГО к одному элементу по требованию
                            let (x, y) = get_mut_coords(active_id, hud_config);
                            let max_x = 32 - 2; // Все наши элементы по 2 блока
                            *x = cell_x.clamp(0, max_x) as u32;
                            *y = cell_y.clamp(0, 23);
                        }
                    }
                }
            }

            if response.drag_released() {
                current_drag = None;
                ui.ctx().data_mut(|d| d.remove_temp::<String>(drag_id));
            }

            // 3. ОТРИСОВКА (Фон + Сетка + Рамка)
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(16, 16, 16));
            if let Some(frame_tex) = hud_frame {
                painter.image(
                    frame_tex.id(),
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }

            let grid_stroke = egui::Stroke::new(
                0.5,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 12),
            );
            for x in 1..32 {
                let cx = rect.min.x + x as f32 * cell_size;
                painter.line_segment(
                    [egui::pos2(cx, rect.min.y), egui::pos2(cx, rect.max.y)],
                    grid_stroke,
                );
            }
            for y in 1..24 {
                let cy = rect.min.y + y as f32 * cell_size;
                painter.line_segment(
                    [egui::pos2(rect.min.x, cy), egui::pos2(rect.max.x, cy)],
                    grid_stroke,
                );
            }

            painter.rect_stroke(
                view_rect,
                0.0,
                egui::Stroke::new(1.5, egui::Color32::LIGHT_BLUE),
            );
            painter.text(
                view_rect.center(),
                egui::Align2::CENTER_CENTER,
                "🎮 VIEWPORT (30x20)",
                egui::FontId::monospace(scale(5.0)),
                egui::Color32::LIGHT_BLUE,
            );

            // 4. РЕНДЕРИНГ ИНДИКАТОРОВ НА ХОЛСТЕ
            let mut has_collision = false;
            let check_real_overlap = |viewport: egui::Rect, indicator: egui::Rect| -> bool {
                indicator.min.x < viewport.max.x
                    && indicator.max.x > viewport.min.x
                    && indicator.min.y < viewport.max.y
                    && indicator.max.y > viewport.min.y
            };

            for item in elements.iter() {
                let (cx, cy) = get_read_coords(item.id, hud_config);
                if cx == 99 || cy == 99 {
                    continue;
                }

                let item_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        rect.min.x + cx as f32 * cell_size,
                        rect.min.y + cy as f32 * cell_size,
                    ),
                    egui::vec2(item.width_blocks as f32 * cell_size, cell_size),
                );

                painter.rect_filled(item_rect, 1.0, item.color.linear_multiply(0.8));
                painter.rect_stroke(item_rect, 1.0, egui::Stroke::new(0.8, egui::Color32::WHITE));

                let left_half = egui::Rect::from_min_max(
                    item_rect.min,
                    egui::pos2(item_rect.min.x + cell_size, item_rect.max.y),
                );
                painter.text(
                    left_half.center(),
                    egui::Align2::CENTER_CENTER,
                    item.icon,
                    egui::FontId::monospace(scale(5.0)),
                    egui::Color32::WHITE,
                );

                let right_half = egui::Rect::from_min_max(
                    egui::pos2(item_rect.min.x + cell_size, item_rect.min.y),
                    item_rect.max,
                );
                painter.text(
                    right_half.center(),
                    egui::Align2::CENTER_CENTER,
                    "00",
                    egui::FontId::monospace(scale(5.0)),
                    egui::Color32::WHITE,
                );

                if check_real_overlap(view_rect, item_rect) {
                    has_collision = true;
                }
            }

            if has_collision {
                let err_rect = egui::Rect::from_min_size(
                    egui::pos2(rect.min.x, rect.max.y - 25.0),
                    egui::vec2(scale(native_width), 25.0),
                );
                painter.rect_filled(err_rect, 0.0, egui::Color32::from_rgb(180, 0, 0));
                painter.text(
                    err_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "⚠️ ОШИБКА: ИНДИКАТОРЫ ЗАЛЕЗАЮТ ВНУТРЬ ИГРОВОЙ ЗОНЫ!",
                    egui::FontId::proportional(12.0),
                    egui::Color32::WHITE,
                );
            }
        });
    });
}

/// Хелпер для безопасного иммутабельного чтения координат без конфликта заимствований
fn get_read_coords(id: &str, hud_config: &HudRenderingConfig) -> (u32, u32) {
    match id {
        "life" => (hud_config.life_x, hud_config.life_y),
        "objects" => (hud_config.objects_x, hud_config.objects_y),
        "objects_icon" => (hud_config.objects_icon_x, hud_config.objects_icon_y),
        "keys" => (hud_config.keys_x, hud_config.keys_y),
        "killed" => (hud_config.killed_x, hud_config.killed_y),
        "ammo" => (hud_config.ammo_x, hud_config.ammo_y),
        "timer" => (hud_config.timer_x, hud_config.timer_y),
        _ => (99, 99),
    }
}
