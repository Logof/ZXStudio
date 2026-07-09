// src/app/menu_bar/file_menu.rs
use crate::app::ZxIdeApp;
use crate::app::menu_bar::Language;
use crate::core::save_project_json;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, app: &mut ZxIdeApp) {
    let is_en = app.current_language == Language::En;
    
    let t_file = if is_en { "📁 File" } else { "📁 Файл" };
    let t_new = if is_en { "✨ New Project..." } else { "✨ Новый проект..." };
    let t_open = if is_en { "📖 Open Project..." } else { "📖 Открыть проект..." };
    let t_recent = if is_en { "🕐 Recent Projects" } else { "🕐 Недавние проекты" };
    let t_save = if is_en { "💾 Save Project JSON" } else { "💾 Сохранить JSON" };
    let t_exit = if is_en { "❌ Exit" } else { "❌ Выход" };

    ui.menu_button(t_file, |ui| {
        // 1. Создание нового проекта через сброс к Мастеру (Wizard)
        if ui.button(t_new).clicked() {
            ui.close_menu();
            app.wizard_active = true;
            app.wizard_step = crate::app::states::WizardStep::WelcomeChoice;
        }

        // 2. Открытие существующего проекта
        if ui.button(t_open).clicked() {
            ui.close_menu();
            // Здесь вызывается нативный диалог открытия, либо переключение флага
            app.status_message = if is_en { "Opening file dialog...".to_owned() } else { "Открытие диалогового окна...".to_owned() };
        }

        // 3. Подменю со списком недавних файлов
        ui.menu_button(t_recent, |ui| {
            if app.recent_projects.is_empty() {
                ui.weak(if is_en { "Empty" } else { "Список пуст" });
            } else {
                let mut clicked_path = None;
                for path in &app.recent_projects {
                    if ui.button(path).clicked() {
                        clicked_path = Some(path.clone());
                    }
                }
                if let Some(path) = clicked_path {
                    ui.close_menu();
                    app.project_path = path;
                    // Здесь срабатывает триггер на горячую ленивую загрузку проекта
                }
            }
        });

        ui.separator();

        // 4. Наше сохранение JSON
        if ui.button(t_save).clicked() {
            ui.close_menu();
            match save_project_json(&app.project_path, &app.project_name, &app.project) {
                Ok(_) => {
                    app.status_message = format!("✅ JSON сохранен: {}{}.prj", &app.project_path, &app.project_name);
                }
                Err(err) => {
                    app.status_message = format!("❌ Ошибка сохранения: {}", err);
                }
            }
        }

        ui.separator();

        // 5. Полный выход из приложения
        if ui.button(t_exit).clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}
