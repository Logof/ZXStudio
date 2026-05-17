use eframe::egui;
use crate::models::config::EngineViewMode;
use super::mod_struct::ZxIdeApp;
use super::states::WizardStep;

impl ZxIdeApp {
    pub fn render_project_wizard(&mut self, ctx: &egui::Context) {
        egui::Window::new("🧙 Мастер создания нового проекта")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false).resizable(false).default_width(450.0)
            .show(ctx, |ui| {
                match self.wizard_step {
                    WizardStep::SelectPlatform => {
                        ui.label("Выберите целевую платформу:");
                        ui.radio_value(&mut self.project.is_128k, false, "💾 ZX Spectrum 48K");
                        ui.radio_value(&mut self.project.is_128k, true, "🎹 ZX Spectrum 128K (Музыка AY)");
                        ui.add_space(10.0);
                        ui.label("Режим камеры:");
                        ui.radio_value(&mut self.project.view_mode, EngineViewMode::SideView, "🧗 Side View (Платформер)");
                        ui.radio_value(&mut self.project.view_mode, EngineViewMode::TopView, "🗺️ Top View (Вид сверху)");
                        ui.add_space(20.0);
                        if ui.button("Далее ➡️").clicked() { self.wizard_step = WizardStep::ConfigureWorld; }
                    }
                    WizardStep::ConfigureWorld => {
                        ui.label("Размеры карты в экранах:");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.project.map_w).clamp_range(1..=16));
                            ui.label("Ширина");
                            ui.add(egui::DragValue::new(&mut self.project.map_h).clamp_range(1..=16));
                            ui.label("Высота");
                        });
                        ui.add_space(20.0);
                        if ui.button("🚀 Создать проект!").clicked() {
                            let total_screens = self.project.map_w * self.project.map_h;
                            self.project.screens.clear();
                            for i in 0..total_screens {
                                self.project.screens.insert(format!("screen_{}", i), crate::models::ScreenData::default());
                            }
                            self.wizard_active = false;
                            self.status_message = "Проект успешно инициализирован".to_string();
                        }
                    }
                }
            });
    }
}
