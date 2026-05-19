use eframe::egui;
use crate::models::ProjectData;

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

pub fn render_hud_editor(ui: &mut egui::Ui, project: &mut ProjectData, hud_frame: &Option<egui::TextureHandle>) {
    ui.heading("📺 Интерактивный HUD-конструктор экрана ZX Spectrum");
    ui.add_space(8.0);

    let drag_id = egui::Id::new("hud_editor_drag_state");
    let mut current_drag = ui.ctx().data(|d| d.get_temp::<DraggedElement>(drag_id)).unwrap_or_default();

    let mut has_life = project.config.hud.hud_life_x != 99;
    let mut has_objects = project.config.hud.hud_items_x != 99;
    let mut has_keys = project.config.hud.hud_keys_x != 99;
    let mut has_kills = project.config.hud.hud_killed_x != 99;

    // Панель управления с чекбоксами (Исправлено обращение по индексам [0] и [1])
    ui.columns(2, |cols| {
        cols[0].group(|ui| {
            ui.strong("➕ АКТИВНЫЕ ИНДИКАТОРЫ (2х1 знакоместа):");
            if ui.checkbox(&mut has_life, "❤️ Счетчик жизней").changed() {
                if has_life { project.config.hud.hud_life_x = 30; project.config.hud.hud_life_y = 2; }
                else { project.config.hud.hud_life_x = 99; project.config.hud.hud_life_y = 99; }
            }
            if ui.checkbox(&mut has_objects, "🌟 Счетчик предметов").changed() {
                if has_objects { project.config.hud.hud_items_x = 30; project.config.hud.hud_items_y = 5; }
                else { project.config.hud.hud_items_x = 99; project.config.hud.hud_items_y = 99; }
            }
            if ui.checkbox(&mut has_keys, "🔑 Счетчик ключей").changed() {
                if has_keys { project.config.hud.hud_keys_x = 30; project.config.hud.hud_keys_y = 8; }
                else { project.config.hud.hud_keys_x = 99; project.config.hud.hud_keys_y = 99; }
            }
            if ui.checkbox(&mut has_kills, "💀 Счетчик убийств").changed() {
                if has_kills { project.config.hud.hud_killed_x = 30; project.config.hud.hud_killed_y = 11; }
                else { project.config.hud.hud_killed_x = 99; project.config.hud.hud_killed_y = 99; }
            }
        });

        cols[1].group(|ui| {
            ui.strong("📊 КООРДИНАТЫ ИКОНКИ (Левая ячейка 2х1 блочка):");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut project.config.hud.viewport_x).prefix("Экран X: "));
                ui.add(egui::DragValue::new(&mut project.config.hud.viewport_y).prefix("Y: "));
            });
            if has_life { ui.horizontal(|ui| { ui.add(egui::DragValue::new(&mut project.config.hud.hud_life_x).prefix("❤️ X: ")); ui.add(egui::DragValue::new(&mut project.config.hud.hud_life_y).prefix("Y: ")); }); }
            if has_objects { ui.horizontal(|ui| { ui.add(egui::DragValue::new(&mut project.config.hud.hud_items_x).prefix("🌟 X: ")); ui.add(egui::DragValue::new(&mut project.config.hud.hud_items_y).prefix("Y: ")); }); }
            if has_keys { ui.horizontal(|ui| { ui.add(egui::DragValue::new(&mut project.config.hud.hud_keys_x).prefix("🔑 X: ")); ui.add(egui::DragValue::new(&mut project.config.hud.hud_keys_y).prefix("Y: ")); }); }
            if has_kills { ui.horizontal(|ui| { ui.add(egui::DragValue::new(&mut project.config.hud.hud_killed_x).prefix("💀 X: ")); ui.add(egui::DragValue::new(&mut project.config.hud.hud_killed_y).prefix("Y: ")); }); }
        });
    });

    ui.add_space(12.0);

    // ============================================================================
    // МАТЕМАТИКА ЭКРАНА ZX SPECTRUM (СЕТКА С ШАГОМ ЗНАКОМЕСТА 8х8 ИГРОВЫХ ПИКСЕЛЕЙ)
    // ============================================================================
    let native_width = 256.0;
    let native_height = 192.0;
    let zoom = 2.5;
    let scale = |v: f32| v * zoom;
    let cell_size = 8.0 * zoom; // Одно знакоместо 8х8 пикселей на экране ПК

    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(scale(native_width), scale(native_height)),
            egui::Sense::click_and_drag()
        );
        let painter = ui.painter_at(rect);

        let vx = project.config.hud.viewport_x as f32;
        let vy = project.config.hud.viewport_y as f32;

        let h_life = egui::pos2(project.config.hud.hud_life_x as f32, project.config.hud.hud_life_y as f32);
        let h_obj = egui::pos2(project.config.hud.hud_items_x as f32, project.config.hud.hud_items_y as f32);
        let h_keys = egui::pos2(project.config.hud.hud_keys_x as f32, project.config.hud.hud_keys_y as f32);
        let h_kills = egui::pos2(project.config.hud.hud_killed_x as f32, project.config.hud.hud_killed_y as f32);

        // 👇 НАСТРОЙКА ГЕОМЕТРИИ ИНДИКАТОРА: Ровно 2 знакоместа в длину и 1 знакоместо в высоту!
        let get_horizontal_rect = |pos: egui::Pos2| {
            egui::Rect::from_min_size(
                egui::pos2(rect.min.x + pos.x * cell_size, rect.min.y + pos.y * cell_size),
                egui::vec2(2.0 * cell_size, 1.0 * cell_size), // Ширина = 2, Высота = 1 ячейка
            )
        };

        // Рабочая зона игры (30х20 ячеек)
        let view_rect = egui::Rect::from_min_size(
            egui::pos2(rect.min.x + vx * cell_size, rect.min.y + vy * cell_size),
            egui::vec2(30.0 * cell_size, 20.0 * cell_size),
        );

        let life_rect = get_horizontal_rect(h_life);
        let obj_rect = get_horizontal_rect(h_obj);
        let key_rect = get_horizontal_rect(h_keys);
        let kills_rect = get_horizontal_rect(h_kills);

        // 1. ЛОГИКА ЗАХВАТА МЫШЬЮ (По горизонтальным хитбоксам 2х1)
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

        // 2. ЛОГИКА ТАСКАНИЯ И СНАППИНГА ПО СЕТКЕ
        if response.dragged() && current_drag != DraggedElement::None {
            if let Some(mouse) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                let cell_x = ((mouse.x - rect.min.x) / cell_size).floor() as u32;
                let cell_y = ((mouse.y - rect.min.y) / cell_size).floor() as u32;

                match current_drag {
                    DraggedElement::Viewport => {
                        project.config.hud.viewport_x = cell_x.clamp(0, 32 - 30);
                        project.config.hud.viewport_y = cell_y.clamp(0, 24 - 20);
                    }
                    DraggedElement::Life => {
                        project.config.hud.hud_life_x = cell_x.clamp(0, 32 - 2); // Ограничение справа под ширину 2 знакоместа
                        project.config.hud.hud_life_y = cell_y.clamp(0, 23);
                    }
                    DraggedElement::Objects => {
                        project.config.hud.hud_items_x = cell_x.clamp(0, 32 - 2);
                        project.config.hud.hud_items_y = cell_y.clamp(0, 23);
                        // Для Churrera иконка совпадает с началом (левой координатой X)
                        project.config.hud.hud_items_icon_x = project.config.hud.hud_items_x;
                        project.config.hud.hud_items_icon_y = project.config.hud.hud_items_y;
                    }
                    DraggedElement::Keys => {
                        project.config.hud.hud_keys_x = cell_x.clamp(0, 32 - 2);
                        project.config.hud.hud_keys_y = cell_y.clamp(0, 23);
                    }
                    DraggedElement::Kills => {
                        project.config.hud.hud_killed_x = cell_x.clamp(0, 32 - 2);
                        project.config.hud.hud_killed_y = cell_y.clamp(0, 23);
                    }
                    _ => {}
                }
            }
        }

        if response.drag_released() {
            current_drag = DraggedElement::None;
            ui.ctx().data_mut(|d| d.insert_temp(drag_id, current_drag));
        }

        // 3. ОТРИСОВКА ОВЕРЛЕЕВ И СЕТКИ (Шаг сетки — ровно 8 оригинальных пикселей)
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(16, 16, 16));
        if let Some(frame_tex) = hud_frame {
            painter.image(frame_tex.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
        }

        let grid_stroke = egui::Stroke::new(0.5, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 12));
        for x in 1..32 { let cx = rect.min.x + x as f32 * cell_size; painter.line_segment([egui::pos2(cx, rect.min.y), egui::pos2(cx, rect.max.y)], grid_stroke); }
        for y in 1..24 { let cy = rect.min.y + y as f32 * cell_size; painter.line_segment([egui::pos2(rect.min.x, cy), egui::pos2(rect.max.x, cy)], grid_stroke); }

        // Игровая рабочая зона (Viewport 30x20)
        painter.rect_stroke(view_rect, 0.0, egui::Stroke::new(1.5, egui::Color32::LIGHT_BLUE));
        painter.text(view_rect.center(), egui::Align2::CENTER_CENTER, "🎮 VIEWPORT (30x20)", egui::FontId::monospace(scale(5.0)), egui::Color32::LIGHT_BLUE);

        // 👇 ХЕЛПЕР ГОРИЗОНТАЛЬНОГО РЕНДЕРИНГА 2х1 (Слева направо: иконка -> цифры)
        let draw_horizontal_indicator = |painter: &egui::Painter, r: egui::Rect, icon: &str, num: &str, color: egui::Color32| {
            // Рисуем общий фон индикатора
            painter.rect_filled(r, 1.0, color);
            painter.rect_stroke(r, 1.0, egui::Stroke::new(0.8, egui::Color32::WHITE));

            // Вычисляем левую половинку под иконку (1 знакоместо)
            let left_half = egui::Rect::from_min_max(r.min, egui::pos2(r.min.x + cell_size, r.max.y));
            painter.text(left_half.center(), egui::Align2::CENTER_CENTER, icon, egui::FontId::monospace(scale(5.0)), egui::Color32::WHITE);

            // Вычисляем правую половинку под 2 цифры счетчика (1 знакоместо)
            let right_half = egui::Rect::from_min_max(egui::pos2(r.min.x + cell_size, r.min.y), r.max);
            painter.text(right_half.center(), egui::Align2::CENTER_CENTER, num, egui::FontId::monospace(scale(5.0)), egui::Color32::WHITE);
        };

        // Выводим только активные индикаторы 2х1
        if has_life { draw_horizontal_indicator(&painter, life_rect, "❤️", "05", egui::Color32::from_rgba_unmultiplied(200, 40, 40, 220)); }
        if has_objects { draw_horizontal_indicator(&painter, obj_rect, "🌟", "00", egui::Color32::from_rgba_unmultiplied(40, 120, 200, 220)); }
        if has_keys { draw_horizontal_indicator(&painter, key_rect, "🔑", "00", egui::Color32::from_rgba_unmultiplied(180, 140, 20, 220)); }
        if has_kills { draw_horizontal_indicator(&painter, kills_rect, "💀", "00", egui::Color32::from_rgba_unmultiplied(100, 100, 100, 220)); }

        // 4. КРАШ-ВАЛИДАЦИЯ СТОЛКНОВЕНИЙ
        let mut has_collision = false;

        // Вспомогательная функция, которая возвращает true ТОЛЬКО при реальном наложении (пересечении внутренних пикселей)
        let check_real_overlap = |viewport: egui::Rect, indicator: egui::Rect| -> bool {
            // Если индикатор касается снаружи, его границы равны границам игрового поля.
            // Пересечение происходит, только если левая граница зашла внутрь правой,
            // правая — внутрь левой, верхняя — ниже нижней, а нижняя — выше верхней.
            indicator.min.x < viewport.max.x &&
            indicator.max.x > viewport.min.x &&
            indicator.min.y < viewport.max.y &&
            indicator.max.y > viewport.min.y
        };

        if has_life && check_real_overlap(view_rect, life_rect) { has_collision = true; }
        if has_objects && check_real_overlap(view_rect, obj_rect) { has_collision = true; }
        if has_keys && check_real_overlap(view_rect, key_rect) { has_collision = true; }
        if has_kills && check_real_overlap(view_rect, kills_rect) { has_collision = true; }

        // Отрисовка красной плашки ошибки, если наложение всё-таки произошло
        if has_collision {
            let err_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x, rect.max.y - 25.0), egui::vec2(scale(native_width), 25.0));
            painter.rect_filled(err_rect, 0.0, egui::Color32::from_rgb(180, 0, 0));
            painter.text(err_rect.center(), egui::Align2::CENTER_CENTER, "⚠️ ОШИБКА: ИНДИКАТОРЫ ПЕРЕСЕКАЮТ (ЗАЛЕЗАЮТ ВНУТРЬ) ИГРОВУЮ ЗОНУ!", egui::FontId::proportional(12.0), egui::Color32::WHITE);
        }
    });

    ui.add_space(8.0);
    ui.label("ℹ️ Пропорции зафиксированы: каждый маркер занимает ровно 2 ячейки в длину и 1 ячейку в высоту (иконка слева, счетчик справа).");
}
