use eframe::egui;
use super::mod_struct::ZxIdeApp;
use crate::core::{save_project_json, export_config_h};

pub fn render_menu_bar(app: &mut ZxIdeApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("📁 Проект", |ui| {
                if ui.button("💾 Сохранить JSON").clicked() {
                    let _ = save_project_json(&app.project);
                    app.status_message = "Проект сохранен в map/mapa.prj".to_string();
                    ui.close_menu();
                }
                if ui.button("⚙️ Сгенерировать config.h").clicked() {
                    let _ = export_config_h(&app.project);
                    app.status_message = "Заголовки Си сгенерированы по шаблону".to_string();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Выход").clicked() {
                    std::process::exit(0);
                }
            });

            ui.menu_button("🛠️ Утилиты", |ui| {
                if ui.button("Новый проект (Мастер)").clicked() {
                    app.wizard_active = true;
                    app.wizard_step = super::states::WizardStep::SelectPlatform;
                    ui.close_menu();
                }
            });

            // --- УЛУЧШЕНИЕ №3: ИНТЕРАКТИВНОЕ МЕНЮ ВКЛЮЧЕНИЯ/ОТКЛЮЧЕНИЯ ХОТСПOТОВ ---
            ui.menu_button("⭐ Хотспоты", |ui| {
                ui.checkbox(&mut app.enable_hotspot_items, "🌟 Включить Предметы (Тайл 17)");
                ui.checkbox(&mut app.enable_hotspot_keys, "🔑 Включить Ключи (Тайл 18)");
                ui.checkbox(&mut app.enable_hotspot_refills, "❤️ Включить Аптечки (Тайл 16)");
            });
        });
    });
}
