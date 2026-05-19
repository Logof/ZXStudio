use eframe::egui;
use crate::app::ZxIdeApp;
use crate::app::states::WizardStep;

pub fn render(ui: &mut egui::Ui, app: &mut ZxIdeApp) {
    ui.vertical_centered(|ui| {
        ui.label("Добро пожаловать в среду разработки Pixel-Art игр под MTE MK1!");
        ui.add_space(16.0);

        let btn_size = egui::vec2(280.0, 32.0);

        // ============================================================================
        // КНОПКА 1: ОТКРЫТИЕ СУЩЕСТВУЮЩЕГО ПРОЕКТА
        // ============================================================================
        if ui.add_sized(btn_size, egui::Button::new("📁 Открыть существующий проект")).clicked() {
            if let Some(file_path) = rfd::FileDialog::new()
                .add_filter("Проект ZX Spectrum (*.prj)", &["prj"])
                .pick_file()
            {
                // Записываем папку, в которой физически лежит этот .prj файл, как ИСТИННЫЙ корень проекта!
                if let Some(root_folder) = file_path.parent() {
                    app.project_path = root_folder.to_string_lossy().into_owned();

                    if let Some(folder_name) = root_folder.file_name() {
                        app.project_name = folder_name.to_string_lossy().into_owned();
                    }
                }
                match crate::core::io::load_project_file(&file_path) {
                    Ok(loaded_project) => {
                        app.project = loaded_project;
                        app.wizard_active = false;
                        app.status_message = format!("🚀 Проект [{}] успешно загружен!", app.project_name);
                    }
                    Err(err_msg) => {
                        app.status_message = format!("❌ Ошибка импорта JSON: {}", err_msg);
                    }
                }
            }
        }

        ui.add_space(8.0);

        // 👇 Добавьте этот блок для отладки загрузки:
        if !app.status_message.is_empty() {
            if app.status_message.starts_with("❌") {
                ui.colored_label(egui::Color32::LIGHT_RED, &app.status_message);
            } else {
                ui.colored_label(egui::Color32::LIGHT_BLUE, &app.status_message);
            }
        }

        // ============================================================================
        // ИСПРАВЛЕННАЯ КНОПКА 2: ПЕРЕХОД К ВЫБОРУ ПЛАТФОРМЫ В МАСТЕРЕ
        // ============================================================================
        if ui.add_sized(btn_size, egui::Button::new("🧙 Мастер создания нового проекта")).clicked() {
            // Сбрасываем данные проекта до дефолтных значений перед настройкой
            app.project = crate::models::ProjectData::default();

            // Включаем визард (на случай, если был выключен)
            app.wizard_active = true;

            // Переключаем шаг Мастера на выбор платформы (SelectPlatform)
            // Подставьте точное название из вашего enum WizardStep, если оно отличается
            app.wizard_step = WizardStep::NameAndPath;

            app.status_message = "Запущен мастер создания нового проекта. Выберите путь для хранения и имя проекта.".to_string();
        }

        ui.add_space(16.0);
        ui.weak("Mojon Twins Ecosystem Compatible Core v1.0.0");
    });
}
