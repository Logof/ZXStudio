// src/core/exporter/exporter_config/platform_macros.rs
use crate::models::ProjectData;
use crate::models::project::CompressionMode;

pub fn process(mut content: String, project: &ProjectData) -> String {
    let profile = if project.config.general.mode_128k {
        "ZX Spectrum 128K (AY)"
    } else {
        "ZX Spectrum 48K"
    };

    // Флаги сжатия для ветки MK1v4 / v4.8
    let toggle_compression = match project.compression_mode {
        CompressionMode::Appack => "#define COMPRESSION_APPACK",
        CompressionMode::Zx7 => "#define COMPRESSION_ZX7",
        CompressionMode::Zx0 => "#define COMPRESSION_ZX0",
    };

    // Ассемблерные низкоуровневые оптимизации
    let toggle_asm_sprites = if project.asm_render_sprites { "#define ASM_RENDER_SPRITES" } else { "// #define ASM_RENDER_SPRITES" };
    let toggle_asm_collision = if project.asm_collision_detect { "#define ASM_COLLISION_DETECT" } else { "// #define ASM_COLLISION_DETECT" };

    content = content
        .replace("{{TARGET_PROFILE}}", profile)
        .replace("{{TOGGLE_COMPRESSION}}", toggle_compression)
        .replace("{{TOGGLE_ASM_RENDER_SPRITES}}", toggle_asm_sprites)
        .replace("{{TOGGLE_ASM_COLLISION_DETECT}}", toggle_asm_collision);

    content
}
