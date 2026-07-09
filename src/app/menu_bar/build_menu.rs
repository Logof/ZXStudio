// src/app/menu_bar/build_menu.rs
use crate::app::ZxIdeApp;
use crate::app::menu_bar::Language;
use crate::app::states::CustomTab;
use crate::core::pipeline::execute_resource_pipeline;
use crate::core::image_processor::{generate_sprite_masks, process_tileset_for_mappy};
use crate::core::validator::validate_attribute_clash;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, app: &mut ZxIdeApp) {
    let is_en = app.current_language == Language::En;

    let t_build = if is_en { "⚙️ Build" } else { "⚙️ Сборка" };
    let t_compile = if is_en { "🚀 Build game.tap" } else { "🚀 Собрать game.tap" };
    let t_export = if is_en { "📦 Export Resources" } else { "📦 Экспорт ресурсов движка" };
    let t_gfx = if is_en { "🖼️ Import Graphics PNG" } else { "🖼️ Импортировать графику" };
    let t_clash = if is_en { "🔍 Validate Attribute Clash" } else { "🔍 Валидация Attribute Clash заставки" };

    ui.menu_button(t_build, |ui| {
        // 1. Асинхронная сборка ленты .TAP
        if ui.button(format!("{}      F5", t_compile)).clicked() {
            ui.close_menu();
            ui.ctx().data_mut(|d| {
                d.insert_temp(egui::Id::new("trigger_async_compile_and_build"), true);
            });
        }

        ui.separator();

        // 2. Экспорт ресурсов Churscript и карт
        if ui.button(t_export).clicked() {
            ui.close_menu();
            match execute_resource_pipeline(&app.project, &app.project_path) {
                Ok(logs) => {
                    app.status_message = logs.join("\n");
                    ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("tab_switch_signal"), CustomTab::Console));
                }
                Err(err) => {
                    app.status_message = format!("❌ Ошибка сборщика ресурсов:\n{:?}", err);
                    ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("tab_switch_signal"), CustomTab::Console));
                }
            }
        }

        // 3. Конвертация и заливка текстур в VRAM
        if ui.button(t_gfx).clicked() {
            ui.close_menu();
            let mut success = true;
            if let Err(_) = process_tileset_for_mappy("gfx/work.png", "gfx/mappy.png") { success = false; }
            if success { if let Err(_) = generate_sprite_masks("gfx/sprites.png") { success = false; } }

            if success {
                let _ = ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true));
                app.status_message = "✅ Вся графика успешно перечитана и загружена в видеопамять!".to_string();
            }
        }

        // 4. Сканирование картинок заставки
        if ui.button(t_clash).clicked() {
            ui.close_menu();
            if let Ok(errors) = validate_attribute_clash("gfx/title.png") {
                app.clash_errors = errors;
                app.status_message = format!("Найдено коллизий цвета на заставке: {}", app.clash_errors.len());
            }
        }
    });
}
