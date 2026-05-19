use crate::app::states::WizardStep;
use crate::app::ZxIdeApp;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, app: &mut ZxIdeApp) {
    ui.label("Размеры карты в экранах:");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut app.project.config.map_goals.map_w).clamp_range(1..=16));
        ui.label("Ширина");
        ui.add(egui::DragValue::new(&mut app.project.config.map_goals.map_h).clamp_range(1..=16));
        ui.label("Высота");
    });
    ui.add_space(20.0);

    ui.horizontal(|ui| {
        if ui.button("◀ Назад").clicked() {
            app.wizard_step = WizardStep::SelectPlatform;
        }

        if ui.button("🚀 Создать проект!").clicked() {
            // 1. Инициализируем пустую сетку комнат в памяти приложения на основе размеров мира
            let total_screens =
                app.project.config.map_goals.map_w * app.project.config.map_goals.map_h;
            app.project.screens.clear();
            for i in 0..total_screens {
                app.project.screens.insert(
                    format!("screen_{}", i),
                    crate::models::ScreenData::default(),
                );
            }

            // 2. Вызываем сервис дисковой автоматизации ядра
            match crate::core::io::create_project_structure(
                &app.project_path,
                &app.project_name,
                &app.project,
            ) {
                Ok(saved_path) => {
                    // Закрываем мастер приветствия, открывая основную рабочую среду IDE
                    app.wizard_active = false;

                    app.status_message = format!(
                        "🎉 Проект '{}' успешно развернут! Сейв: {:?}",
                        app.project_name, saved_path
                    );
                }
                Err(err_msg) => {
                    // Если диск защищен от записи или указан неверный путь — выводим ошибку в статус-бар и не закрываем визард
                    app.status_message = format!("❌ Ошибка развертывания на диск: {}", err_msg);
                }
            }
        }
    });
}
