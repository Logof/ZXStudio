// src/ui/configurator/general/audio_system.rs
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData, is_english: bool) {
    let t_audio = if is_english { "🎵 Audio System (Music & Sound Effects)" } else { "🎵 Аудиосистема (Музыка и Звуковые Эффекты)" };
    let t_arkos = if is_english { "USE_ARKOS_PLAYER (Use Arkos instead of Wyz)" } else { "USE_ARKOS_PLAYER (Использовать Arkos вместо Wyz)" };
    let t_channel = if is_english { "ARKOS_SFX_CHANNEL (SFX Sound Channel)" } else { "ARKOS_SFX_CHANNEL (Звуковой канал SFX)" };
    let t_audio_warn = if is_english { "ℹ️ Arkos Player settings require MODE_128K to be active." } else { "ℹ️ Настройки Arkos Player активны только при включенном MODE_128K." };

    ui.strong(t_audio);
    ui.add_space(4.0);

    ui.add_enabled_ui(project.config.general.mode_128k, |ui| {
        ui.checkbox(&mut project.config.general.use_arkos_player, t_arkos);

        ui.add_enabled_ui(project.config.general.use_arkos_player, |ui| {
            ui.add(egui::Slider::new(&mut project.config.general.arkos_sfx_channel, 0..=2).text(t_channel));
        });
    });

    if !project.config.general.mode_128k {
        ui.colored_label(egui::Color32::GOLD, t_audio_warn);
    }
}
