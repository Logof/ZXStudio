use eframe::egui;
use crate::models::ProjectData;

pub fn render_configurator(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.heading("⚙️ Глобальные настройки игрового баланса (config.h)");
    ui.add_space(8.0);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_width(ui.available_width().min(500.0));

        ui.label("🎮 Параметры игрока и врагов:");
        ui.add_space(4.0);

            // Исправленные пути к новым подструктурам:
        ui.add(egui::Slider::new(&mut project.config.general.player_life_ini, 1..=9).text("Стартовые Жизни"));
        ui.add(egui::Slider::new(&mut project.config.engine.max_bullets, 0..=6).text("Макс. пуль на экране"));
        ui.add(egui::Slider::new(&mut project.scr_inicio, 0..=15).text("Стартовый экран спавна"));
        ui.add(egui::Slider::new(&mut project.config.engine.enemies_life_gauge, 1..=10).text("Прочность врагов (HP)"));

        ui.separator();

        ui.label("📝 Дополнительные флаги компиляции движка:");
        ui.checkbox(&mut project.config.engine.player_can_fire, "PLAYER_CAN_FIRE (Разрешить стрельбу)");
    });
}
