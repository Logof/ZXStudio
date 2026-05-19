// src/ui/configurator/tile_behaviour.rs
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.strong("🧱 Интерактивная матрица поведения тайлов (behs[])");
    ui.label("Нажмите на тайл, чтобы настроить его физические свойства и коллизии с игроком.");
    ui.add_space(8.0);

    // ВЗАИМОСВЯЗЬ: Проверяем флаг сжатия уровней из модуля map_goals
    let is_compressed = project.config.map_goals.compressed_levels;

    if is_compressed {
        ui.colored_label(
            egui::Color32::GOLD,
            "ℹ️ Сейчас включен макрос COMPRESSED_LEVELS. Массив behs[] деактивирован движком (свойства берутся из поуровневых файлов).",
        );
    }

    ui.add_enabled_ui(!is_compressed, |ui| {
        // Загружаем текстуру тайлсета из временного хранилища egui ctx, если она была загружена
        let tileset_tex: Option<egui::TextureHandle> =
            ui.ctx().data(|d| d.get_temp(egui::Id::new("tileset_tex")));

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Рисуем сетку 4 строки по 12 тайлов (всего 48 элементов)
            egui::Grid::new("behs_matrix_grid")
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    for tile_id in 0..48 {
                        // Синхронизировано под новую модель TileBehaviourConfig
                        let current_val = &mut project.config.tile_behaviour.behs[tile_id];

                        ui.vertical(|ui| {
                            // 1. Отрисовка графического мини-тайла, если доступна текстура
                            if let Some(ref tex) = tileset_tex {
                                let tex_size = tex.size_vec2();

                                // Размер одного тайла в пикселях
                                let tile_size_px = 16.0;
                                // Вычисляем шаг сетки атласа в UV-координатах (0.0 - 1.0)
                                let tw = tile_size_px / tex_size.x;
                                let th = tile_size_px / tex_size.y;

                                // Находим координаты левого верхнего угла тайла в пикселях
                                let px = (tile_id % 16) as f32 * tile_size_px;
                                let py = (tile_id / 16) as f32 * tile_size_px;

                                // ПОЛУПИКСЕЛЬНЫЙ СДВИГ ДЛЯ ИСКЛЮЧЕНИЯ BLEEDING (Сдвиг на 0.5 пикселя внутрь ячейки)
                                let eps_x = 0.5 / tex_size.x;
                                let eps_y = 0.5 / tex_size.y;

                                // Переводим пиксельные координаты в UV с жестким внутренним отступом
                                let uv_min = egui::pos2(
                                    (px / tex_size.x) + eps_x,
                                    (py / tex_size.y) + eps_y,
                                );
                                let uv_max = egui::pos2(
                                    ((px + tile_size_px) / tex_size.x) - eps_x,
                                    ((py + tile_size_px) / tex_size.y) - eps_y,
                                );

                                let uv_rect = egui::Rect::from_min_max(uv_min, uv_max);

                                // Отрисовка тайла с аппаратным сглаживанием текстуры (Nearest Neighbor для Pixel Art)
                                ui.add(
                                    egui::Image::new(egui::load::SizedTexture::new(
                                        tex.id(),
                                        egui::vec2(32.0, 32.0),
                                    ))
                                    .uv(uv_rect),
                                );
                            } else {
                                // Если графики нет, рисуем серую заглушку с номером тайла
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(32.0, 32.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter()
                                    .rect_filled(rect, 2.0, egui::Color32::from_gray(40));
                                ui.painter().text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    tile_id.to_string(),
                                    egui::FontId::proportional(12.0),
                                    egui::Color32::LIGHT_GRAY,
                                );
                            }

                            // 2. Выпадающее меню быстрых пресетов поведения или кнопка настройки
                            ui.menu_button(format!("ID {} ({})", tile_id, current_val), |ui| {
                                ui.strong(format!("Свойства тайла #{}", tile_id));

                                // Разбираем битовую маску на логические флаги движка
                                let mut is_kills = (*current_val & 1) != 0;
                                let mut is_hides = (*current_val & 2) != 0;
                                let mut is_platform = (*current_val & 4) != 0;
                                let mut is_obstacle = (*current_val & 8) != 0;
                                let mut is_breakable = (*current_val & 16) != 0;

                                if ui
                                    .checkbox(&mut is_obstacle, "Сплошная Стена (8)")
                                    .changed()
                                {
                                    toggle_bit(current_val, 8, is_obstacle);
                                }
                                if ui.checkbox(&mut is_kills, "Шипы / Убийца (1)").changed()
                                {
                                    toggle_bit(current_val, 1, is_kills);
                                }
                                if ui.checkbox(&mut is_hides, "Скрывает игрока (2)").changed()
                                {
                                    toggle_bit(current_val, 2, is_hides);
                                }
                                if ui
                                    .checkbox(&mut is_platform, "Платформа (Полупроницаемая) (4)")
                                    .changed()
                                {
                                    toggle_bit(current_val, 4, is_platform);
                                }
                                if ui
                                    .checkbox(&mut is_breakable, "Разрушаемая стена (16)")
                                    .changed()
                                {
                                    toggle_bit(current_val, 16, is_breakable);
                                }

                                ui.separator();
                                if ui
                                    .radio_value(current_val, 10, "Ящик / Замок (Спец. блок 10)")
                                    .clicked()
                                {}
                                if ui
                                    .radio_value(current_val, 0, "Пусто / Проходимо (0)")
                                    .clicked()
                                {}
                            });
                        });

                        // Перенос строки в сетке каждые 12 тайлов
                        if (tile_id + 1) % 12 == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    });
}

fn toggle_bit(val: &mut u8, bit: u8, set: bool) {
    if set {
        *val |= bit;
        if bit == 8 {
            *val &= !4;
        }
        if bit == 4 {
            *val &= !8;
        }
    } else {
        *val &= !bit;
    }
}
