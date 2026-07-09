// src/ui/configurator/general/mod.rs
pub mod level_management;
pub mod platform_settings;
pub mod compression_asm;
pub mod audio_system;
pub mod rendering_settings;

use crate::app::menu_bar::AppTranslations;
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    // Безопасно извлекаем кэш переводов из временных данных контекста egui
    let translations = ui
        .ctx()
        .data(|d| d.get_temp::<AppTranslations>(egui::Id::new("translations_cache")))
        .unwrap_or_else(|| AppTranslations::load(crate::app::menu_bar::Language::Ru));

    let is_english = translations.menu.lang_select.contains("Language");

    // 1. Блок управления уровнями проекта (Multilevel)
    level_management::render(ui, project, is_english);
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(6.0);

    // 2. Блок системных настроек платформы и графической архитектуры
    platform_settings::render(ui, project, is_english);
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(6.0);

    // 3. Блок компрессии ресурсов и ASM-оптимизаций Z80 (Специфика v4.8)
    compression_asm::render(ui, project, is_english);
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(6.0);

    // 4. Блок аудиосистемы (Wyz / Arkos)
    audio_system::render(ui, project, is_english);
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(6.0);

    // 5. Блок рендеринга и ограничения фреймрейта
    rendering_settings::render(ui, project, is_english);
}

/// Общая сервисная функция сброса текстурного контекста при смене TileMode или Уровня
pub fn trigger_graphics_reset(ui: &mut egui::Ui) {
    ui.ctx().data_mut(|d| {
        d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
        d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
    });
}
