// src/ui/project_tree.rs
use crate::app::menu_bar::AppTranslations; // Импортируем нашу глобальную локализацию
use crate::app::states::CustomTab;
use crate::models::project::{LevelData, ProjectData}; // Импортируем структуры уровней и проекта
use eframe::egui;
use std::fs;
use std::path::{Path, PathBuf};

/// Рендерит динамическое дерево проекта на основе реальной папки, где лежит .prj файл
/// Добавлен аргумент `project` для вывода виртуальной ветки уровней из памяти
pub fn render_project_tree(ui: &mut egui::Ui, absolute_project_path: &str, project: &mut ProjectData) -> Option<CustomTab> {
    let mut tab_signal = None;

    // Безопасно извлекаем кэш переводов из временных данных контекста egui.
    let translations = ui
        .ctx()
        .data(|d| d.get_temp::<AppTranslations>(egui::Id::new("translations_cache")))
        .unwrap_or_else(|| AppTranslations::load(crate::app::menu_bar::Language::Ru));

    ui.vertical(|ui| {
        let root_dir = Path::new(absolute_project_path);

        // Извлекаем имя папки для красивого заголовка дерева
        let game_name = root_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Retro Game");

        ui.colored_label(
            egui::Color32::from_rgb(0, 255, 255),
            format!("📁 {}", game_name),
        );
        ui.add_space(4.0);

        if !root_dir.exists() || !root_dir.is_dir() {
            let error_msg = if translations.menu.lang_select.contains("Language") {
                "⚠️ Project directory not found on disk."
            } else {
                "⚠️ Директория проекта не найдена на диске."
            };
            ui.weak(error_msg);
            return;
        }

        // ============================================================================
        // ФИКС СКРОЛЛА: Включаем жесткую стабилизацию контейнера ScrollArea
        // ============================================================================
        egui::ScrollArea::vertical()
            .id_source("dynamic_project_tree_scroll")
            .auto_shrink([false, false]) // ЗАПРЕЩАЕТ прыжки и хаотичное сжатие дерева посередине экрана
            .max_width(ui.available_width())
            .show(ui, |ui| {
                // --------------------------------------------------------------------
                // ВЕЛИКОЕ УЛУЧШЕНИЕ GUI: Виртуальная ветка ОЗУ для контроля кампании уровней
                // --------------------------------------------------------------------
                let level_tree_title = if translations.menu.lang_select.contains("Language") {
                    "🗺️ Campaign Levels"
                } else {
                    "🗺️ Кампания уровней"
                };

                let t_add_level = if translations.menu.lang_select.contains("Language") { "➕ Add New Level" } else { "➕ Добавить новый уровень" };
                let t_del_level = if translations.menu.lang_select.contains("Language") { "🗑 Delete Level" } else { "🗑 Удалить уровень" };

                let header_res = egui::CollapsingHeader::new(level_tree_title)
                    .id_source("virtual_levels_campaign_node")
                    .default_open(true)
                    .show(ui, |ui| {
                        let active_idx = project.current_level_index;
                        let mut level_to_remove = None;

                        for i in 0..project.levels.len() {
                            let is_current = i == active_idx;
                            
                            // Подсвечиваем текущий редактируемый уровень бирюзовым цветом
                            let text = if is_current {
                                egui::RichText::new(format!("▶ [Level {}] {}", i + 1, project.levels[i].name))
                                    .color(egui::Color32::from_rgb(0, 255, 255))
                                    .strong()
                            } else {
                                egui::RichText::new(format!("  [Level {}] {}", i + 1, project.levels[i].name))
                                    .color(egui::Color32::GRAY)
                            };

                            let item_res = ui.selectable_label(is_current, text);
                            
                            // При клике на ветку уровня — сквозным образом переключаем контекст всей IDE
                            if item_res.clicked() {
                                project.current_level_index = i;
                                ui.ctx().data_mut(|d| {
                                    d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                                    d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
                                });
                            }

                            // Индивидуальное контекстное меню для удаления конкретного уровня
                            item_res.context_menu(|ui| {
                                ui.set_min_width(140.0);
                                // Защита: движок не может существовать без уровней, кнопка блокируется
                                ui.add_enabled_ui(project.levels.len() > 1, |ui| {
                                    if ui.button(t_del_level).clicked() {
                                        level_to_remove = Some(i);
                                        ui.close_menu();
                                    }
                                });
                            });
                        }

                        // Безопасное удаление выбранного уровня вне цикла итерации (Защита Borrow Checker)
                        if let Some(del_idx) = level_to_remove {
                            project.levels.remove(del_idx);
                            // Если удалили текущий уровень или вышли за рамки индекса — сбрасываем на нулевой
                            if project.current_level_index >= project.levels.len() {
                                project.current_level_index = 0;
                            }
                            // Сигнализируем графическому модулю перестроить палитру
                            ui.ctx().data_mut(|d| {
                                d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                                d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
                            });
                        }
                    });

                // Глобальное контекстное меню для ВСЕЙ папки «Кампания уровней» — добавление нового уровня
                header_res.header_response.context_menu(|ui| {
                    ui.set_min_width(180.0);
                    if ui.button(t_add_level).clicked() {
                        let mut new_level = LevelData::default();
                        new_level.name = format!("Level {}", project.levels.len() + 1);

                        // Автоматически генерируем пустую сетку комнат под размеры игрового мира из конфига
                        let total_screens = project.config.map_goals.map_w * project.config.map_goals.map_h;
                        new_level.screens.clear();
                        for i in 0..total_screens {
                            new_level.screens.insert(
                                format!("screen_{}", i),
                                crate::models::ScreenData::default(),
                            );
                        }

                        project.levels.push(new_level);
                        // Автоматически фокусируемся на свежесозданном уровне кампании
                        project.current_level_index = project.levels.len() - 1;

                        // Горячий сброс палитр
                        ui.ctx().data_mut(|d| {
                            d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                            d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
                        });
                        ui.close_menu();
                    }
                });

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                // Запускаем рекурсивный обход физических папок, начиная с корня .prj файла
                if let Some(signal) = render_directory_node(ui, root_dir, &translations) {
                    tab_signal = Some(signal);
                }
            });
    });

    tab_signal
}

/// Рекурсивная функция для отрисовки папок и файлов
fn render_directory_node(
    ui: &mut egui::Ui,
    dir_path: &Path,
    translations: &AppTranslations,
) -> Option<CustomTab> {
    let mut tab_signal = None;
    let is_english = translations.menu.lang_select.contains("Language");

    if let Ok(entries) = fs::read_dir(dir_path) {
        let mut paths: Vec<PathBuf> = entries
            .filter_map(|e| e.ok().map(|entry| entry.path()))
            .collect();

        // Сортировка: сначала папки, потом файлы
        paths.sort_by(|a, b| {
            if a.is_dir() && b.is_file() {
                std::cmp::Ordering::Less
            } else if a.is_file() && b.is_dir() {
                std::cmp::Ordering::Greater
            } else {
                a.file_name().cmp(&b.file_name())
            }
        });

        for path in paths {
            let name_str = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");

            if name_str.starts_with('.') {
                continue;
            }

            if path.is_dir() {
                let header_title = format!("📂 {}", name_str);

                let header_response = egui::CollapsingHeader::new(header_title)
                    .id_source(format!("dyn_node_{}", path.to_string_lossy()))
                    .default_open(name_str == "script" || name_str == "map" || name_str == "gfx")
                    .show(ui, |ui| {
                        if let Some(signal) = render_directory_node(ui, &path, translations) {
                            tab_signal = Some(signal);
                        }
                    });

                header_response.header_response.context_menu(|ui| {
                    ui.set_min_width(180.0);

                    // Контекстное меню импорта ресурсов
                    let import_label = if is_english {
                        format!("📥 Import into '{}'", name_str)
                    } else {
                        format!("📥 Импортировать в '{}'", name_str)
                    };

                    if ui.button(import_label).clicked() {
                        let title = if is_english {
                            "Select files to import"
                        } else {
                            "Выберите файлы для импорта"
                        };
                        let mut dialog = rfd::FileDialog::new().set_title(title);

                        dialog = match name_str {
                            "gfx" => dialog.add_filter(
                                if is_english { "Images (PNG)" } else { "Изображения (PNG)" },
                                &["png", "PNG"],
                            ),
                            "script" => dialog.add_filter(
                                if is_english { "Scripts (.spt)" } else { "Скрипты (.spt)" },
                                &["spt"],
                            ),
                            "dev" => dialog.add_filter(
                                if is_english { "C Header Files (.h)" } else { "Заголовочные файлы Си (.h)" },
                                &["h"],
                            ),
                            "mus" => dialog.add_filter(
                                if is_english { "Music" } else { "Музыка" },
                                &["pt3", "mus"],
                            ),
                            _ => dialog,
                        };

                        if let Some(external_files) = dialog.pick_files() {
                            for ext_file in external_files {
                                if let Some(filename) = ext_file.file_name() {
                                    let target = path.join(filename);
                                    let _ = fs::copy(&ext_file, &target);
                                }
                            }
                        }
                        ui.close_menu();
                    }
                });
            } else if path.is_file() {
                let name_lower = name_str.to_lowercase();
                let icon = if name_lower.ends_with(".spt") {
                    "📜 "
                } else if name_lower.ends_with(".h") {
                    "⚙️ "
                } else if name_lower.ends_with(".png") {
                    "🖼 "
                } else {
                    "📄 "
                };

                let display_label = format!("{}{}", icon, name_str);
                let response = ui.selectable_label(false, display_label);

                if response.double_clicked() {
                    let is_in_script_dir = path.components().any(|c| c.as_os_str() == "script");
                    let is_in_dev_dir = path.components().any(|c| c.as_os_str() == "dev");

                    if (is_in_script_dir && name_lower.ends_with(".spt")) || (is_in_dev_dir && name_lower.ends_with(".h")) {
                        if name_lower == "config.h" {
                            tab_signal = Some(CustomTab::Configurator);
                        } else {
                            tab_signal = Some(CustomTab::ScriptEditor);

                            if let Some(path_str) = path.to_str() {
                                ui.ctx().data_mut(|d| {
                                    d.insert_temp(
                                        egui::Id::new("trigger_load_script_file"),
                                        path_str.to_string(),
                                    );
                                });
                            }
                        }
                    } else if name_lower.ends_with(".prj") || name_lower.ends_with(".map") {
                        tab_signal = Some(CustomTab::MapCanvas);
                    }
                }

                response.context_menu(|ui| {
                    let delete_label = if is_english {
                        "🗑 Delete file"
                    } else {
                        "🗑 Удалить файл"
                    };
                    if ui.button(delete_label).clicked() {
                        let _ = fs::remove_file(&path);
                        ui.close_menu();
                    }
                });
            }
        }
    }

    tab_signal
}
