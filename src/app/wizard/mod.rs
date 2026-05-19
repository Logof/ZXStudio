use eframe::egui;
use crate::app::ZxIdeApp;
use crate::app::states::WizardStep;

mod welcome_choice;
mod name_and_path;
mod select_platform;
mod configure_world;

impl ZxIdeApp {
    pub fn render_project_wizard(&mut self, ctx: &egui::Context) {
        if !self.wizard_active {
            return;
        }

        let title = match self.wizard_step {
            WizardStep::WelcomeChoice => "🧙 Приветствие ZX Spectrum Core IDE",
            WizardStep::NameAndPath => "📁 Создание проекта: Имя и Путь",
            _ => "🧙 Мастер создания нового проекта"
        };

        egui::Window::new(title)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .default_width(480.0)
            .show(ctx, |ui| {
                match self.wizard_step {
                    WizardStep::WelcomeChoice => {
                        welcome_choice::render(ui, self);
                    }
                    WizardStep::NameAndPath => {
                        name_and_path::render(ui, self);
                    }
                    WizardStep::SelectPlatform => {
                        select_platform::render(ui, self);
                    }
                    WizardStep::ConfigureWorld => {
                        configure_world::render(ui, self);
                    }
                }
            });
    }
}
