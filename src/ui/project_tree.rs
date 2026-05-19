use eframe::egui;
use std::fs;
use std::path::{Path, PathBuf};
use crate::app::states::CustomTab;

/// Рендерит динамическое дерево проекта на основе реальной папки, где лежит .prj файл
pub fn render_project_tree(ui: &mut egui::Ui, absolute_project_path: &str) -> Option<CustomTab> {
    let mut tab_signal = None;

    ui.vertical(|ui| {
        let root_dir = Path::new(absolute_project_path);

        // Извлекаем имя папки для красивого заголовка дерева
        let game_name = root_dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Retro Game");

        ui.colored_label(egui::Color32::from_rgb(0, 255, 255), format!("📁 {}", game_name));
        ui.add_space(4.0);

        if !root_dir.exists() || !root_dir.is_dir() {
            ui.weak("⚠️ Дирекция проекта не найдена на диске.");
            return;
        }

        egui::ScrollArea::vertical()
            .id_source("dynamic_project_tree_scroll")
            .show(ui, |ui| {
                // Запускаем рекурсивный обход, начиная с папки .prj файла
                if let Some(signal) = render_directory_node(ui, root_dir) {
                    tab_signal = Some(signal);
                }
            });
    });

    tab_signal
}

/// Рекурсивная функция для отрисовки папок и файлов
fn render_directory_node(ui: &mut egui::Ui, dir_path: &Path) -> Option<CustomTab> {
    let mut tab_signal = None;

    if let Ok(entries) = fs::read_dir(dir_path) {
        let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|entry| entry.path())).collect();

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
                        if let Some(signal) = render_directory_node(ui, &path) {
                            tab_signal = Some(signal);
                        }
                    });

                header_response.header_response.context_menu(|ui| {
                    ui.set_min_width(180.0);
                    if ui.button(format!("📥 Импортировать в '{}'", name_str)).clicked() {
                        let mut dialog = rfd::FileDialog::new().set_title("Выберите файлы для импорта");

                        dialog = match name_str {
                            "gfx" => dialog.add_filter("Изображения (PNG)", &["png", "PNG"]),
                            "script" => dialog.add_filter("Скрипты (.spt)", &["spt"]),
                            "dev" => dialog.add_filter("Заголовочные файлы Си (.h)", &["h"]),
                            "mus" => dialog.add_filter("Музыка", &["pt3", "mus"]),
                            _ => dialog
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
                let icon = if name_lower.ends_with(".spt") { "📜 " }
                    else if name_lower.ends_with(".h") { "⚙️ " }
                    else if name_lower.ends_with(".png") { "🖼 " }
                    else { "📄 " };

                let display_label = format!("{}{}", icon, name_str);
                let response = ui.selectable_label(false, display_label);

                if response.double_clicked() {
                    if name_lower.ends_with(".spt") {
                        tab_signal = Some(CustomTab::ScriptEditor);
                    } else if name_str == "config.h" {
                        tab_signal = Some(CustomTab::HudEditor);
                    } else if name_lower.ends_with(".h") {
                        tab_signal = Some(CustomTab::Configurator);
                    } else if name_lower.ends_with(".prj") || name_lower.ends_with(".map") {
                        tab_signal = Some(CustomTab::MapCanvas);
                    }
                }

                response.context_menu(|ui| {
                    if ui.button("🗑 Удалить файл").clicked() {
                        let _ = fs::remove_file(&path);
                        ui.close_menu();
                    }
                });
            }
        }
    }

    tab_signal
}
