// src/ui/configurator/general/platform_settings.rs
use crate::models::project::TileMode;
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData, is_english: bool) {
    let t_platform = if is_english { "💾 Platform, Memory & Global Properties" } else { "💾 Платформа, Память и Глобальные свойства" };
    let t_mode_128k = if is_english { "MODE_128K — Enable expanded 128K memory mode" } else { "MODE_128K — Включить расширенный режим памяти 128K" };
    let t_veng = if is_english { "VENG_SELECTOR (Advanced Engine Selector)" } else { "VENG_SELECTOR (Расширенный селектор движка)" };
    let t_decoder = if is_english { "USE_MAP_CUSTOM_DECODER (Custom Map Decoder)" } else { "USE_MAP_CUSTOM_DECODER (Кастомный декодер карты)" };
    let t_arch = if is_english { "📊 Graphics Architecture:" } else { "📊 Архитектура графики:" };

    ui.strong(t_platform);
    ui.add_space(6.0);

    ui.checkbox(&mut project.config.general.mode_128k, t_mode_128k);
    ui.checkbox(&mut project.config.general.veng_selector, t_veng);
    ui.checkbox(&mut project.config.general.use_map_custom_decoder, t_decoder);

    ui.add_space(6.0);

    ui.horizontal(|ui| {
        ui.label(t_arch);

        let active_idx = project.current_level_index;
        let old_mode = project.levels[active_idx].tile_mode;

        egui::ComboBox::from_id_source("config_tile_mode_selector")
            .selected_text(project.levels[active_idx].tile_mode.name())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut project.levels[active_idx].tile_mode, TileMode::Packed16, TileMode::Packed16.name());
                ui.selectable_value(&mut project.levels[active_idx].tile_mode, TileMode::Packed16WithShadows, TileMode::Packed16WithShadows.name());
                ui.selectable_value(&mut project.levels[active_idx].tile_mode, TileMode::Extended48, TileMode::Extended48.name());
            });

        if project.levels[active_idx].tile_mode != old_mode {
            let new_mode = project.levels[active_idx].tile_mode;
            project.levels[active_idx].tile_behaviours = new_mode.default_behaviours();
            super::trigger_graphics_reset(ui);
        }
    });

    ui.add_space(6.0);
    render_memory_ram_summary(ui, project, is_english);
}

fn render_memory_ram_summary(ui: &mut egui::Ui, project: &mut ProjectData, is_english: bool) {
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(4.0)
        .inner_margin(8.0)
        .show(ui, |ui| {
            let map_w = project.config.map_goals.map_w as usize;
            let map_h = project.config.map_goals.map_h as usize;
            let total_screens = map_w * map_h;
            let screen_size_bytes = 15 * 10;
            let active_idx = project.current_level_index;

            let total_map_bytes = match project.levels[active_idx].tile_mode {
                TileMode::Packed16 | TileMode::Packed16WithShadows => (total_screens * screen_size_bytes + 1) / 2,
                TileMode::Extended48 => total_screens * screen_size_bytes,
            };

            if is_english {
                ui.small(format!("World size: {} screens. Map weight in RAM: {} bytes.", total_screens, total_map_bytes));
            } else {
                ui.small(format!("Размер игрового мира: {} экранов. Вес карты в RAM: {} байт.", total_screens, total_map_bytes));
            }
        });
}
