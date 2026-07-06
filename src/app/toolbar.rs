// src/app/toolbar.rs
use super::ZxIdeApp;
use crate::app::states::CustomTab;
use crate::core::pipeline::execute_resource_pipeline; // 🆕 Импортируем наш новый модульный конвейер
use crate::core::{
    image_processor::{generate_sprite_masks, process_tileset_for_mappy},
    save_project_json,
    validator::validate_attribute_clash,
};
use crate::models::project::LevelData; // Импортируем структуру уровня для создания новых на лету
use eframe::egui; // 🆕 Импортируем перечисление вкладок дока

pub fn render_toolbar(app: &mut ZxIdeApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("toolbar")
        .frame(egui::Frame::none().inner_margin(4.0).fill(egui::Color32::from_rgb(22, 22, 26)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // КНОПКА 1: Сохранение метаданных проекта в JSON
                if ui.button("💾 Сохранить JSON").clicked() {
                    match save_project_json(&app.project_path, &app.project_name, &app.project) {
                        Ok(_) => {
                            app.status_message = format!("✅ Проект успешно сохранен в {}{}.prj!", &app.project_path, &app.project_name);
                        }
                        Err(err) => {
                            app.status_message = format!("❌ Ошибка保存ения проекта: {}", err);
                        }
                    }
                }

                // КНОПКА 2: Промышленный экспорт ресурсов через Task-Based Pipeline
                if ui.button("⚙️ Экспорт Ресурсов")
                    .on_hover_text("Скомпилировать карту, врагов, скрипты и загрузочные SCR-экраны в папки dev/ и gfx/")
                    .clicked()
                {
                    match execute_resource_pipeline(&app.project, &app.project_path) {
                        Ok(logs) => {
                            // Передаем весь лог сборки в статус-сообщение (оно выведется в консоли)
                            app.status_message = logs.join("\n");

                            // Посылаем сигнал док-системе мгновенно переключить фокус на логи
                            ui.ctx().data_mut(|d| {
                                d.insert_temp(egui::Id::new("tab_switch_signal"), CustomTab::Console);
                            });
                        }
                        Err(err) => {
                            // Безопасно выводим ошибку компиляции без падения IDE
                            app.status_message = format!("❌ Критическая ошибка сборки ресурсов:\n{:?}", err);

                            ui.ctx().data_mut(|d| {
                                d.insert_temp(egui::Id::new("tab_switch_signal"), CustomTab::Console);
                            });
                        }
                    }
                }

                // КНОПКА 3: Импорт и подготовка графических ассетов
                if ui.button("🖼️ Импортировать графику").clicked() {
                    let mut success = true;
                    if let Err(_) = process_tileset_for_mappy("gfx/work.png", "gfx/mappy.png") { success = false; }
                    if success { if let Err(_) = generate_sprite_masks("gfx/sprites.png") { success = false; } }

                    if success {
                        if let Ok(img) = image::open("gfx/mappy.png") {
                            let rgb_img = img.to_rgba8();
                            let (w, h) = rgb_img.dimensions();
                            let color_image = egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgb_img.as_flat_samples().samples);
                            app.tileset_texture = Some(ctx.load_texture("tileset_palette", color_image, egui::TextureOptions::NEAREST));
                        }
                        if let Ok(img) = image::open("gfx/sprites.png") {
                            let rgba_img = img.to_rgba8();
                            let (w, h) = rgba_img.dimensions();
                            let color_image = egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba_img.as_flat_samples().samples);
                            app.sprites_texture = Some(ctx.load_texture("sprites_palette", color_image, egui::TextureOptions::NEAREST));
                        }
                        app.status_message = "✅ Вся графика успешно загружена в видеопамять!".to_string();
                    }
                }

                ui.separator();

                // КНОПКА 4: Сканирование внешних PNG картинок на Attribute Clash
                if ui.button("🔍 Валидация Attribute Clash").clicked() {
                    if let Ok(errors) = validate_attribute_clash("gfx/title.png") {
                        app.clash_errors = errors;
                        app.status_message = format!("Найдено коллизий цвета: {}", app.clash_errors.len());
                    }
                }

                ui.separator();

                // ============================================================================
                // ГЛOБАЛЬНЫЙ СЕЛЕКТОР УРОВНЕЙ ДЛЯ НАВЕДЕНИЯ ПОРЯДКА В GUI (Мультилевел)
                // ============================================================================
                ui.label("🎮 Уровень:");
                
                let mut selected_idx = app.project.current_level_index;
                let combobox_res = egui::ComboBox::from_id_source("toolbar_level_selector")
                    .selected_text(format!("[{}] {}", selected_idx + 1, app.project.levels[selected_idx].name))
                    .show_ui(ui, |ui| {
                        for i in 0..app.project.levels.len() {
                            ui.selectable_value(
                                &mut selected_idx,
                                i,
                                format!("[{}] {}", i + 1, app.project.levels[i].name),
                            );
                        }
                    });

                // Если геймдизайнер выбрал другой уровень — мгновенно переключаем контекст всего приложения
                if selected_idx != app.project.current_level_index {
                    app.project.current_level_index = selected_idx;
                    // Очищаем и сбрасываем кэш нарезанной графики под TileMode выбранного уровня
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                        d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
                    });
                }

                // Компактная кнопка мгновенного добавления нового уровня прямо на тулбаре
                if ui.button("➕")
                    .on_hover_text("Быстро добавить новый уровень в текущую кампанию")
                    .clicked()
                {
                    let mut new_level = LevelData::default();
                    new_level.name = format!("Level {}", app.project.levels.len() + 1);

                    // Синхронизируем размер сетки комнат для нового уровня из глобальных параметров config
                    let total_screens = app.project.config.map_goals.map_w * app.project.config.map_goals.map_h;
                    new_level.screens.clear();
                    for i in 0..total_screens {
                        new_level.screens.insert(
                            format!("screen_{}", i),
                            crate::models::ScreenData::default(),
                        );
                    }

                    app.project.levels.push(new_level);
                    app.project.current_level_index = app.project.levels.len() - 1;

                    // Вызываем жесткий сброс палитры для инициализации графического режима нового уровня
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                        d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
                    });
                }
            });
        });
}
