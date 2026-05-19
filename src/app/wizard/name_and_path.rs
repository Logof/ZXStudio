use eframe::egui;
use crate::app::ZxIdeApp;
use crate::app::states::WizardStep;

pub fn render(ui: &mut egui::Ui, app: &mut ZxIdeApp) {
    ui.label("Укажите метаданные для развертывания структуры папок:");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("📝 Название игры:");
        ui.add(egui::TextEdit::singleline(&mut app.project_name)
            .hint_text("Например: my_super_platformer"));
    });
    ui.add_space(6.0);

    ui.label("📁 Директория проекта:");
    ui.horizontal(|ui| {
        ui.add(egui::TextEdit::singleline(&mut app.project_path)
            .hint_text("Выберите папку для исходников..."));
        
        if ui.button("Обзор...").clicked() {
            if let Some(folder) = rfd::FileDialog::new()
                .set_title("Выберите папку для создания проекта")
                .pick_folder() 
            {
                app.project_path = folder.to_string_lossy().into_owned();
            }
        }
    });

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        if ui.button("◀ Назад").clicked() {
            app.wizard_step = WizardStep::WelcomeChoice;
        }
        
        let can_continue = !app.project_name.trim().is_empty() && !app.project_path.trim().is_empty();
        let next_btn = ui.add_enabled(can_continue, egui::Button::new("Далее ➡️"));
        
        if next_btn.clicked() {
            app.wizard_step = WizardStep::SelectPlatform;
        }
        
        if !can_continue {
            ui.weak("⚠️ Заполните название и выберите путь.");
        }
    });
}
