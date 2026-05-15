use eframe::egui;
use crate::models::ProjectData;

pub fn render_configurator(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.heading("Глобальные настройки баланса и UI");

    ui.columns(2, |columns| {
        columns.group(|ui| {
            ui.label("Параметры персонажа:");
            ui.add(egui::Slider::new(&mut project.config.player_life_ini, 1..=9).text("Стартовые Жизни"));
            ui.add(egui::Slider::new(&mut project.config.max_bullets, 0..=6).text("Макс. пуль на экране"));
            ui.add(egui::Slider::new(&mut project.scr_inicio, 0..=15).text("Стартовый экран"));
        });

        columns.group(|ui| {
            ui.label("Конструктор HUD (Знакоместа 32x24):");
            ui.add(egui::Slider::new(&mut project.config.hud_life_x, 0..=31).text("Позиция Жизней X"));
            ui.add(egui::Slider::new(&mut project.config.hud_life_y, 0..=23).text("Позиция Жизней Y"));
            
            if project.config.hud_life_y < 20 && project.config.hud_life_x < 30 {
                ui.colored_label(egui::Color32::LIGHT_RED, "⚠️ Внимание: HUD пересекает игровой Viewport!");
            } else {
                ui.colored_label(egui::Color32::LIGHT_GREEN, "✅ Положение HUD валидно.");
            }
        });
    });
}
