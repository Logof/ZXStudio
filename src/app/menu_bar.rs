use eframe::egui;
use super::ZxIdeApp;
// ФИКС: Импортируем CustomTab строго из модуля состояний нашего приложения app
use super::states::CustomTab;
use crate::core::{save_project_json, export_config_h, export_enems_h};

pub fn render_menu_bar(app: &mut ZxIdeApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("📁 Проект", |ui| {
                if ui.button("💾 Сохранить JSON").clicked() {
                    match save_project_json(&app.project_path, &app.project_name, &app.project) {
                        Ok(_) => {
                            app.status_message = format!("✅ Проект успешно сохранен в {}{}.prj!", &app.project_path, &app.project_name);
                        }
                        Err(err) => {
                            app.status_message = format!("❌ Ошибка сохранения проекта: {}", err);
                        }
                    }
                    ui.close_menu();
                }
                if ui.button("⚙️ Сгенерировать Си-код").clicked() {
                    let _ = export_config_h(&app.project_path, &app.project);
                    let _ = export_enems_h(&app.project_path, &app.project);
                    app.status_message = "Заголовки Си сгенерированы по шаблонам".to_string();
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

            // 📺 МЕНЮ УПРАВЛЕНИЯ ГЕОМЕТРИЕЙ ОКOН IDE (ИСПРАВЛЕНЫ ИМПОРТЫ ТИПОВ)
            ui.menu_button("📺 Окно", |ui| {
                if ui.button("🔄 Сбросить макет панелей").clicked() {
                    let mut default_state = egui_dock::DockState::new(vec![
                        CustomTab::MapCanvas,
                        CustomTab::ScriptEditor,
                        CustomTab::Configurator,
                        CustomTab::HudEditor,
                    ]);
                    let surface = default_state.main_surface_mut();
                    let root_node = egui_dock::NodeIndex::root();

                    // Консоль строго вниз
                    let [top_node, _bottom_node] = surface.split_below(root_node, 0.80, vec![CustomTab::Console]);
                    // Дерево проекта строго влево
                    let [_left_node, _main_work_node] = surface.split_left(top_node, 0.18, vec![CustomTab::ProjectTree]);

                    app.dock_state = default_state;
                    app.status_message = "Расположение панелей успешно восстановлено!".to_string();
                    ui.close_menu();
                }
            });
        });
    });
}
