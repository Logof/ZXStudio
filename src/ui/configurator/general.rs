use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.strong("💾 Платформа, Память и Глобальные свойства");
    ui.add_space(6.0);

    // Архитектурные флаги платформы и памяти
    ui.checkbox(
        &mut project.config.general.mode_128k,
        "MODE_128K — Включить расширенный режим памяти 128K",
    );
    ui.checkbox(
        &mut project.config.general.veng_selector,
        "VENG_SELECTOR (Расширенный селектор движка)",
    );
    ui.checkbox(
        &mut project.config.general.use_map_custom_decoder,
        "USE_MAP_CUSTOM_DECODER (Кастомный декодер карты)",
    );

    ui.separator();
    ui.strong("🎵 Аудиосистема (Музыка и Звуковые Эффекты)");
    ui.add_space(4.0);

    // Взаимосвязь: Arkos Player доступен только в режиме MODE_128K
    ui.add_enabled_ui(project.config.general.mode_128k, |ui| {
        ui.checkbox(
            &mut project.config.general.use_arkos_player,
            "USE_ARKOS_PLAYER (Использовать Arkos вместо Wyz)",
        );

        ui.add_enabled_ui(project.config.general.use_arkos_player, |ui| {
            ui.add(
                egui::Slider::new(&mut project.config.general.arkos_sfx_channel, 0..=2)
                    .text("ARKOS_SFX_CHANNEL (Звуковой канал SFX)"),
            );
        });
    });

    if !project.config.general.mode_128k {
        ui.colored_label(
            egui::Color32::GOLD,
            "ℹ️ Настройки Arkos Player активны только при включенном MODE_128K.",
        );
    }

    ui.separator();
    ui.strong("⏱️ Рендеринг и частота кадров");
    ui.add(
        egui::Slider::new(&mut project.config.general.min_faps_per_frame, 1..=4)
            .text("MIN_FAPS_PER_FRAME (Ограничитель FPS: 1=50fps, 2=25fps)"),
    );
}
