use crate::models::ProjectData;
use eframe::egui;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum DraggedElement {
    #[default]
    None,
    Viewport,
    Life,
    Objects,
    Keys,
    Kills,
}

pub fn render_hud_editor(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    hud_frame: &Option<egui::TextureHandle>,
) {
    ui.heading("📺 Интерактивный HUD-конструктор экрана ZX Spectrum");
    ui.add_space(8.0);

    let drag_id = egui::Id::new("hud_editor_drag_state");
    let mut current_drag = ui
        .ctx()
        .data(|d| d.get_temp::<DraggedElement>(drag_id))
        .unwrap_or_default();

    let mut has_life = project.config.hud_rendering.life_x != 99;
    let mut has_objects = project.config.hud_rendering.objects_icon_x != 99;
    let mut has_keys = project.config.hud_rendering.keys_x != 99;
    let mut has_kills = project.config.hud_rendering.killed_x != 99;

    // ============================================================================
    // ОСНОВНАЯ ДВУХКОЛОНОЧНАЯ РАЗМЕТКА: ИНТЕРФЕЙС СЛЕВА, ХОЛСТ СПРАВА
    // ============================================================================
    ui.horizontal_top(|ui| {

        // ------------------------------------------------------------------------
        // ЛЕВАЯ КОЛОНКА (Ширина фиксирована, элементы выстроены друг под другом)
        // ------------------------------------------------------------------------
        ui.allocate_ui(egui::vec2(280.0, ui.available_height()), |ui| {
            ui.vertical(|ui| {

                // БЛОК 1: Активные индикаторы (Управление видимостью)
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.strong("➕ АКТИВНЫЕ ИНДИКАТОРЫ (2х1 знакоместа):");
                    ui.add_space(4.0);

                    if ui.checkbox(&mut has_life, "❤️ Счетчик жизней").changed() {
                        if has_life { project.config.hud_rendering.life_x = 30; project.config.hud_rendering.life_y = 2; }
                        else { project.config.hud_rendering.life_x = 99; project.config.hud_rendering.life_y = 99; }
                    }
                    if ui.checkbox(&mut has_objects, "🌟 Счетчик предметов").changed() {
                        if has_objects { project.config.hud_rendering.objects_icon_x = 30; project.config.hud_rendering.objects_icon_y = 5; }
                        else { project.config.hud_rendering.objects_icon_x = 99; project.config.hud_rendering.objects_icon_y = 99; }
                    }
                    if ui.checkbox(&mut has_keys, "🔑 Счетчик ключей").changed() {
                        if has_keys { project.config.hud_rendering.keys_x = 30; project.config.hud_rendering.keys_y = 8; }
                        else { project.config.hud_rendering.keys_x = 99; project.config.hud_rendering.keys_y = 99; }
                    }
                    if ui.checkbox(&mut has_kills, "💀 Счетчик убийств").changed() {
                        if has_kills { project.config.hud_rendering.killed_x = 30; project.config.hud_rendering.killed_y = 11; }
                        else { project.config.hud_rendering.killed_x = 99; project.config.hud_rendering.killed_y = 99; }
                    }
                });

                ui.add_space(6.0);

                // БЛОК 2: Числовые координаты (Строго под Блоком 1)
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.strong("📊 КООРДИНАТЫ ИКОНКИ (Левая ячейка):");
                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.strong("Экран: ");
                        ui.add(egui::DragValue::new(&mut project.config.hud_rendering.viewport_x).prefix("X: "));
                        ui.add(egui::DragValue::new(&mut project.config.hud_rendering.viewport_y).prefix("Y: "));
                    });

                    if has_life || has_objects || has_keys || has_kills {
                        ui.separator();
                    }

                    if has_life { ui.horizontal(|ui| {
                         ui.set_width(ui.available_width());
                        ui.strong("❤️: ");
                        ui.add(egui::DragValue::new(&mut project.config.hud_rendering.life_x).prefix("X: ")); ui.add(egui::DragValue::new(&mut project.config.hud_rendering.life_y).prefix("Y: ")); }); }
                    if has_objects {
                         ui.set_width(ui.available_width());
                        ui.strong("🌟: ");
                        ui.horizontal(|ui| { ui.add(egui::DragValue::new(&mut project.config.hud_rendering.objects_icon_x).prefix("X: ")); ui.add(egui::DragValue::new(&mut project.config.hud_rendering.objects_icon_y).prefix("Y: ")); }); }
                    if has_keys {
                         ui.set_width(ui.available_width());
                        ui.strong("🔑: ");
                        ui.horizontal(|ui| { ui.add(egui::DragValue::new(&mut project.config.hud_rendering.keys_x).prefix("X: ")); ui.add(egui::DragValue::new(&mut project.config.hud_rendering.keys_y).prefix("Y: ")); }); }
                    if has_kills {
                         ui.set_width(ui.available_width());
                        ui.strong("💀: ");
                        ui.horizontal(|ui| { ui.add(egui::DragValue::new(&mut project.config.hud_rendering.killed_x).prefix("X: ")); ui.add(egui::DragValue::new(&mut project.config.hud_rendering.killed_y).prefix("Y: ")); }); }
                });

                ui.add_space(12.0);
                ui.label("ℹ️ Каждый маркер занимает ровно 2 ячейки в длину и 1 ячейку в высоту (иконка слева, счетчик справа).");
                ui.weak("💡 Перетаскивайте мышкой синюю игровую зону или цветные индикаторы прямо на графическом экране справа.");
            });
        });

        ui.add_space(16.0); // Отступ между левой панелью и холстом

        // ------------------------------------------------------------------------
        // ПРАВАЯ КОЛОНКА (Интерактивный графический холст Spectrum)
        // ------------------------------------------------------------------------
        ui.vertical(|ui| {
            let native_width = 256.0;
            let native_height = 192.0;
            let zoom = 2.5;
            let scale = |v: f32| v * zoom;
            let cell_size = 8.0 * zoom;

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(scale(native_width), scale(native_height)),
                    egui::Sense::click_and_drag()
                );
                let painter = ui.painter_at(rect);

                let vx = project.config.hud_rendering.viewport_x as f32;
                let vy = project.config.hud_rendering.viewport_y as f32;

                let h_life = egui::pos2(project.config.hud_rendering.life_x as f32, project.config.hud_rendering.life_y as f32);
                let h_obj = egui::pos2(project.config.hud_rendering.objects_icon_x as f32, project.config.hud_rendering.objects_icon_y as f32);
                let h_keys = egui::pos2(project.config.hud_rendering.keys_x as f32, project.config.hud_rendering.keys_y as f32);
                let h_kills = egui::pos2(project.config.hud_rendering.killed_x as f32, project.config.hud_rendering.killed_y as f32);

                let get_horizontal_rect = |pos: egui::Pos2| {
                    egui::Rect::from_min_size(
                        egui::pos2(rect.min.x + pos.x * cell_size, rect.min.y + pos.y * cell_size),
                        egui::vec2(2.0 * cell_size, 1.0 * cell_size),
                    )
                };

                let view_rect = egui::Rect::from_min_size(
                    egui::pos2(rect.min.x + vx * cell_size, rect.min.y + vy * cell_size),
                    egui::vec2(30.0 * cell_size, 20.0 * cell_size),
                );

                let life_rect = get_horizontal_rect(h_life);
                let obj_rect = get_horizontal_rect(h_obj);
                let key_rect = get_horizontal_rect(h_keys);
                let kills_rect = get_horizontal_rect(h_kills);

                // Захват мыши (Drag Start)
                if response.drag_started() {
                    if let Some(mouse) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                        if has_life && life_rect.contains(mouse) { current_drag = DraggedElement::Life; }
                        else if has_objects && obj_rect.contains(mouse) { current_drag = DraggedElement::Objects; }
                        else if has_keys && key_rect.contains(mouse) { current_drag = DraggedElement::Keys; }
                        else if has_kills && kills_rect.contains(mouse) { current_drag = DraggedElement::Kills; }
                        else if view_rect.contains(mouse) { current_drag = DraggedElement::Viewport; }
                    }
                    ui.ctx().data_mut(|d| d.insert_temp(drag_id, current_drag));
                }

                // Перетаскивание по сетке знакомест (Drag Move)
                if response.dragged() && current_drag != DraggedElement::None {
                    if let Some(mouse) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                        let cell_x = ((mouse.x - rect.min.x) / cell_size).floor() as u32;
                        let cell_y = ((mouse.y - rect.min.y) / cell_size).floor() as u32;

                        match current_drag {
                            DraggedElement::Viewport => {
                                project.config.hud_rendering.viewport_x = cell_x.clamp(0, 32 - 30);
                                project.config.hud_rendering.viewport_y = cell_y.clamp(0, 24 - 20);
                            }
                            DraggedElement::Life => {
                                project.config.hud_rendering.life_x = cell_x.clamp(0, 32 - 2);
                                project.config.hud_rendering.life_y = cell_y.clamp(0, 23);
                            }
                            DraggedElement::Objects => {
                                project.config.hud_rendering.objects_icon_x = cell_x.clamp(0, 32 - 2);
                                project.config.hud_rendering.objects_icon_y = cell_y.clamp(0, 23);
                                project.config.hud_rendering.objects_icon_x = project.config.hud_rendering.objects_icon_x;
                                project.config.hud_rendering.objects_icon_y = project.config.hud_rendering.objects_icon_y;
                            }
                            DraggedElement::Keys => {
                                project.config.hud_rendering.keys_x = cell_x.clamp(0, 32 - 2);
                                project.config.hud_rendering.keys_y = cell_y.clamp(0, 23);
                            }
                            DraggedElement::Kills => {
                                project.config.hud_rendering.killed_x = cell_x.clamp(0, 32 - 2);
                                project.config.hud_rendering.killed_y = cell_y.clamp(0, 23);
                            }
                            _ => {}
                        }
                    }
                }

                if response.drag_released() {
                    current_drag = DraggedElement::None;
                    ui.ctx().data_mut(|d| d.insert_temp(drag_id, current_drag));
                }

                // Отрисовка фона и загруженной рамки marco.png
                painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(16, 16, 16));
                if let Some(frame_tex) = hud_frame {
                    painter.image(frame_tex.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
                }

                // Шаг сетки — ровно 8 оригинальных пикселей
                let grid_stroke = egui::Stroke::new(0.5, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 12));
                for x in 1..32 { let cx = rect.min.x + x as f32 * cell_size; painter.line_segment([egui::pos2(cx, rect.min.y), egui::pos2(cx, rect.max.y)], grid_stroke); }
                for y in 1..24 { let cy = rect.min.y + y as f32 * cell_size; painter.line_segment([egui::pos2(rect.min.x, cy), egui::pos2(rect.max.x, cy)], grid_stroke); }

                // Игровая рабочая зона (Viewport 30x20)
                painter.rect_stroke(view_rect, 0.0, egui::Stroke::new(1.5, egui::Color32::LIGHT_BLUE));
                painter.text(view_rect.center(), egui::Align2::CENTER_CENTER, "🎮 VIEWPORT (30x20)", egui::FontId::monospace(scale(5.0)), egui::Color32::LIGHT_BLUE);

                // Отрисовка индикаторов 2х1 (Слева направо: иконка -> цифры)
                let draw_horizontal_indicator = |painter: &egui::Painter, r: egui::Rect, icon: &str, num: &str, color: egui::Color32| {
                    painter.rect_filled(r, 1.0, color);
                    painter.rect_stroke(r, 1.0, egui::Stroke::new(0.8, egui::Color32::WHITE));

                    let left_half = egui::Rect::from_min_max(r.min, egui::pos2(r.min.x + cell_size, r.max.y));
                    painter.text(left_half.center(), egui::Align2::CENTER_CENTER, icon, egui::FontId::monospace(scale(5.0)), egui::Color32::WHITE);

                    let right_half = egui::Rect::from_min_max(egui::pos2(r.min.x + cell_size, r.min.y), r.max);
                    painter.text(right_half.center(), egui::Align2::CENTER_CENTER, num, egui::FontId::monospace(scale(5.0)), egui::Color32::WHITE);
                };

                if has_life { draw_horizontal_indicator(&painter, life_rect, "❤️", "05", egui::Color32::from_rgba_unmultiplied(200, 40, 40, 220)); }
                if has_objects { draw_horizontal_indicator(&painter, obj_rect, "🌟", "00", egui::Color32::from_rgba_unmultiplied(40, 120, 200, 220)); }
                if has_keys { draw_horizontal_indicator(&painter, key_rect, "🔑", "00", egui::Color32::from_rgba_unmultiplied(180, 140, 20, 220)); }
                if has_kills { draw_horizontal_indicator(&painter, kills_rect, "💀", "00", egui::Color32::from_rgba_unmultiplied(100, 100, 100, 220)); }

                // Строгая валидация пересечений (Соприкосновение границами — разрешено)
                let mut has_collision = false;
                let check_real_overlap = |viewport: egui::Rect, indicator: egui::Rect| -> bool {
                    indicator.min.x < viewport.max.x &&
                    indicator.max.x > viewport.min.x &&
                    indicator.min.y < viewport.max.y &&
                    indicator.max.y > viewport.min.y
                };

                if has_life && check_real_overlap(view_rect, life_rect) { has_collision = true; }
                if has_objects && check_real_overlap(view_rect, obj_rect) { has_collision = true; }
                if has_keys && check_real_overlap(view_rect, key_rect) { has_collision = true; }
                if has_kills && check_real_overlap(view_rect, kills_rect) { has_collision = true; }

                if has_collision {
                    let err_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x, rect.max.y - 25.0), egui::vec2(scale(native_width), 25.0));
                    painter.rect_filled(err_rect, 0.0, egui::Color32::from_rgb(180, 0, 0));
                    painter.text(err_rect.center(), egui::Align2::CENTER_CENTER, "⚠️ ОШИБКА: ИНДИКАТОРЫ ЗАЛЕЗАЮТ ВНУТРЬ ИГРОВОЙ ЗОНЫ!", egui::FontId::proportional(12.0), egui::Color32::WHITE);
                }
            });
        });
    });
}
