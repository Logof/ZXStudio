// src/ui/configurator/general/compression_asm.rs
use crate::models::project::CompressionMode;
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData, is_english: bool) {
    let t_comp_title = if is_english { "🗜️ Core Resource Compression (MK1v4/v4.8)" } else { "🗜️ Сжатие ресурсов ядра (MK1v4/v4.8)" };
    let t_comp_label = if is_english { "Active Packer:" } else { "Активный упаковщик:" };
    let t_asm_title = if is_english { "⚡ Low-Level Z80 Assembler Optimizations" } else { "⚡ Низкоуровневые Z80 Ассемблерные оптимизации" };
    let t_asm_sprites = if is_english { "ASM_RENDER_SPRITES — Use fast assembly sprite routine" } else { "ASM_RENDER_SPRITES — Быстрая отрисовка спрайтов на ассемблере" };
    let t_asm_collision = if is_english { "ASM_COLLISION_DETECT — Hardware-accelerated tile collision check" } else { "ASM_COLLISION_DETECT — Ускоренная проверка коллизий с тайлами" };

    ui.strong(t_comp_title);
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(t_comp_label);
        egui::ComboBox::from_id_source("config_compression_mode_selector")
            .selected_text(project.compression_mode.name())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut project.compression_mode, CompressionMode::Appack, CompressionMode::Appack.name());
                ui.selectable_value(&mut project.compression_mode, CompressionMode::Zx7, CompressionMode::Zx7.name());
                ui.selectable_value(&mut project.compression_mode, CompressionMode::Zx0, CompressionMode::Zx0.name());
            });
    });

    ui.add_space(6.0);
    ui.strong(t_asm_title);
    ui.add_space(4.0);
    ui.checkbox(&mut project.asm_render_sprites, t_asm_sprites);
    ui.checkbox(&mut project.asm_collision_detect, t_asm_collision);
}
