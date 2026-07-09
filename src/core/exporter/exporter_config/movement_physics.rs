// src/core/exporter/exporter_config/movement_physics.rs
use crate::models::ProjectData;

pub fn process(mut content: String, project: &ProjectData) -> String {
    let engine_view_mode_str = if project.config.movement_controls.player_genital { "Top View (Вид сверху)" } else { "Side View (Платформер)" };
    let toggle_view_style = if project.config.movement_controls.player_genital { "#define PLAYER_MOGGY_STYLE" } else { "// #define PLAYER_MOGGY_STYLE" };

    let toggle_bootee = if project.config.movement_controls.engine_type == 0 && !project.config.movement_controls.player_genital {
        "#define PLAYER_BOOTEE"
    } else {
        "// #define PLAYER_BOOTEE"
    };

    content = content
        .replace("{{ENGINE_VIEW_MODE}}", engine_view_mode_str)
        .replace("{{PLAYER_MAX_VY_CAYENDO}}", &project.config.movement_controls.player_max_vy_cayendo.to_string())
        .replace("{{PLAYER_G}}", &project.config.movement_controls.player_g.to_string())
        .replace("{{PLAYER_MAX_VY_CAYENDO_H}}", "256")
        .replace("{{PLAYER_G_HOVER}}", "4")
        .replace("{{PLAYER_VY_INICIAL_SALTO}}", &project.config.movement_controls.player_vy_inicial_salto.to_string())
        .replace("{{PLAYER_MAX_VY_SALTANDO}}", &project.config.movement_controls.player_max_vy_saltando.to_string())
        .replace("{{PLAYER_INCR_SALTO}}", &project.config.movement_controls.player_incr_salto.to_string())
        .replace("{{PLAYER_INCR_JETPAC}}", &project.config.movement_controls.player_incr_jetpac.to_string())
        .replace("{{PLAYER_MAX_VY_JETPAC}}", &project.config.movement_controls.player_max_vy_jetpac.to_string())
        .replace("{{PLAYER_MAX_VX}}", &project.config.movement_controls.player_max_vx.to_string())
        .replace("{{PLAYER_AX}}", &project.config.movement_controls.player_ax.to_string())
        .replace("{{PLAYER_RX}}", &project.config.movement_controls.player_rx.to_string())
        .replace("{{PLAYER_AX_SLIPPERY}}", "8")
        .replace("{{PLAYER_RX_SLIPPERY}}", "8")
        .replace("{{PLAYER_VX_CONVEYORS}}", "128")
        .replace("{{PLAYER_AX_QUICKSANDS}}", "8")
        .replace("{{PLAYER_RX_QUICKSANDS}}", "64")
        .replace("{{PLAYER_MAX_VX_QUICKSANDS}}", "64")
        .replace("{{PLAYER_VY_SINKING}}", "4")
        .replace("{{TOGGLE_PLAYER_BOOTEE}}", toggle_bootee)
        .replace("{{TOGGLE_PLAYER_MOGGY_STYLE}}", toggle_view_style)
        .replace("{{TILE_BEHAVIOURS_ARRAY}}", "comportamiento_tiles");

    content
}
