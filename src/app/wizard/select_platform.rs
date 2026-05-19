use eframe::egui;
use crate::app::ZxIdeApp;
use crate::app::states::WizardStep;
use crate::models::config::EngineViewMode;

pub fn render(ui: &mut egui::Ui, app: &mut ZxIdeApp) {
    ui.label("Выберите целевую платформу:");
    ui.radio_value(&mut app.project.is_128k, false, "💾 ZX Spectrum 48K (Beeper)");
    ui.radio_value(&mut app.project.is_128k, true, "🎹 ZX Spectrum 128K (Музыка AY)");
    ui.add_space(10.0);
    
    ui.label("Режим камеры:");
    ui.radio_value(&mut app.project.view_mode, EngineViewMode::SideView, "🧗 Side View (Платформер)");
    ui.radio_value(&mut app.project.view_mode, EngineViewMode::TopView, "🗺️ Top View (Вид сверху)");
    ui.add_space(20.0);
    
    ui.horizontal(|ui| {
        if ui.button("◀ Назад").clicked() {
            app.wizard_step = WizardStep::NameAndPath;
        }
        if ui.button("Далее ➡️").clicked() { 
            app.wizard_step = WizardStep::ConfigureWorld; 
        }
    });
}
