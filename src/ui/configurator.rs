use eframe::egui;
use crate::models::ProjectData;

pub fn render_configurator(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.heading("Глобальные настройки баланса и HUD-интерфейса");
    ui.add_space(8.0);

    // В egui 0.25 метод .columns() передает в замыкание срез &mut [Ui]
    ui.columns(2, |columns| {
        
        // ============================================================================
        // ЛЕВАЯ КОЛОНКА: ОБРАЩАЕМСЯ К ЭЛЕМЕНТУ МАССИВА ПО ИНДЕКСУ [0]
        // ============================================================================
        columns[0].group(|ui| {
            ui.label("⚙️ Настройки игрового баланса:");
            ui.add(egui::Slider::new(&mut project.config.player_life_ini, 1..=9).text("Стартовые Жизни"));
            ui.add(egui::Slider::new(&mut project.config.max_bullets, 0..=6).text("Макс. пуль на экране"));
            ui.add(egui::Slider::new(&mut project.scr_inicio, 0..=15).text("Стартовый экран спавна"));
            ui.add(egui::Slider::new(&mut project.config.enemies_life_gauge, 1..=10).text("Прочность врагов (HP)"));
            
            ui.separator();
            ui.label("🎮 Текущие Си-координаты макросов:");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut project.config.hud_life_x).prefix("Жизни X:"));
                ui.add(egui::DragValue::new(&mut project.config.hud_life_y).prefix("Y:"));
            });
            ui.horizontal(|ui| {
                ui.label("ℹ️ Изменяйте положение счетчиков на графическом экране справа с помощью ЛКМ.");
            });
        });

        // ============================================================================
        // ПРАВАЯ КОЛОНКА: ОБРАЩАЕМСЯ К ЭЛЕМЕНТУ МАССИВА ПО ИНДЕКСУ [1]
        // ============================================================================
        columns[1].group(|ui| {
            ui.label("📺 Визуальный HUD-редактор экрана ZX Spectrum:");
            ui.add_space(4.0);

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(32.0 * 12.0, 24.0 * 12.0),
                    egui::Sense::click_and_drag()
                );
                let painter = ui.painter_at(rect);

                // 1. ОТРИСОВКА ОВЕРЛЕЯ ИГРОВОГО ОКНА (VIEWPORT) ИЗ CONFIG.H
                let viewport_rect = egui::Rect::from_min_size(
                    egui::pos2(rect.min.x + 0.0 * 12.0, rect.min.y + 2.0 * 12.0),
                    egui::vec2(30.0 * 12.0, 20.0 * 12.0)
                );
                painter.rect_filled(viewport_rect, 0.0, egui::Color32::from_gray(35));
                painter.rect_stroke(viewport_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::from_gray(80)));
                painter.text(viewport_rect.center(), egui::Align2::CENTER_CENTER, "🎮 Игровая Зона (Viewport 30x20)", egui::FontId::proportional(13.0), egui::Color32::from_gray(100));

                // 2. ОБРАБОТКА ПЕРЕТАСКИВАНИЯ МЫШИ (u32 СЕТКА)
                if response.clicked() || response.dragged() {
                    if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                        if rect.contains(mouse_pos) {
                            let cell_x = ((mouse_pos.x - rect.min.x) / 12.0) as u32;
                            let cell_y = ((mouse_pos.y - rect.min.y) / 12.0) as u32;

                            if cell_x >= 30 {
                                if cell_y >= 4 && cell_y <= 9 {
                                    project.config.hud_life_x = 30;
                                    project.config.hud_life_y = cell_y;
                                }
                            } else {
                                project.config.hud_life_x = 30;
                                project.config.hud_life_y = cell_y.clamp(0, 23);
                            }
                        }
                    }
                }

                // 3. ОТРИСОВКА ТОНКОЙ СЕТКИ ЗНАКОМЕСТ 32x24
                let grid_stroke = egui::Stroke::new(0.5, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 12));
                for x in 1..32 {
                    let cx = rect.min.x + x as f32 * 12.0;
                    painter.line_segment([egui::pos2(cx, rect.min.y), egui::pos2(cx, rect.max.y)], grid_stroke);
                }
                for y in 1..24 {
                    let cy = rect.min.y + y as f32 * 12.0;
                    painter.line_segment([egui::pos2(rect.min.x, cy), egui::pos2(rect.max.x, cy)], grid_stroke);
                }

                // 4. ГРАФИЧЕСКИЙ ВЫВОД СЧЕТЧИКОВ ПОВЕРХ СЕТКИ
                let life_rect = egui::Rect::from_min_size(
                    egui::pos2(rect.min.x + (project.config.hud_life_x as f32) * 12.0, rect.min.y + (project.config.hud_life_y as f32) * 12.0),
                    egui::vec2(2.0 * 12.0, 1.0 * 12.0)
                );
                painter.rect_filled(life_rect, 2.0, egui::Color32::from_rgba_unmultiplied(255, 50, 50, 180));
                painter.rect_stroke(life_rect, 2.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
                painter.text(life_rect.center(), egui::Align2::CENTER_CENTER, "❤️", egui::FontId::proportional(10.0), egui::Color32::WHITE);

                let obj_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + 30.0 * 12.0, rect.min.y + 12.0 * 12.0), egui::vec2(2.0 * 12.0, 1.0 * 12.0));
                painter.rect_filled(obj_rect, 2.0, egui::Color32::from_rgba_unmultiplied(50, 150, 255, 120));
                painter.text(obj_rect.center(), egui::Align2::CENTER_CENTER, "🌟", egui::FontId::proportional(10.0), egui::Color32::WHITE);

                let key_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x + 30.0 * 12.0, rect.min.y + 16.0 * 12.0), egui::vec2(2.0 * 12.0, 1.0 * 12.0));
                painter.rect_filled(key_rect, 2.0, egui::Color32::from_rgba_unmultiplied(255, 215, 0, 120));
                painter.text(key_rect.center(), egui::Align2::CENTER_CENTER, "🔑", egui::FontId::proportional(10.0), egui::Color32::WHITE);

                // 5. ВАЛИДАЦИЯ ТЕРРИТОРИИ HUD
                if project.config.hud_life_y >= 2 && project.config.hud_life_y < 22 && project.config.hud_life_x < 30 {
                    let err_rect = egui::Rect::from_min_size(egui::pos2(rect.min.x, rect.min.y), egui::vec2(32.0 * 12.0, 20.0));
                    painter.rect_filled(err_rect, 0.0, egui::Color32::from_rgba_unmultiplied(255, 0, 0, 180));
                    painter.text(err_rect.center(), egui::Align2::CENTER_CENTER, "⚠️ КОЛЛИЗИЯ: HUD ПЕРЕСЕКАЕТ ИГРОВОЙ ЭКРАН!", egui::FontId::proportional(11.0), egui::Color32::WHITE);
                }
            });
            ui.label("💡 Инструкция: Кликните ЛКМ по правой свободной полосе (X=30) на высоте от 4 до 9 ячейки, чтобы переместить счетчик Жизней.");
        });
    });
}
