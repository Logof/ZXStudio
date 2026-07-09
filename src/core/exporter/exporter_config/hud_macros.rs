// src/core/exporter/exporter_config/hud_macros.rs
use crate::models::ProjectData;

pub fn process(mut content: String, project: &ProjectData) -> String {
    content = content
        .replace("{{OBJECTS_X}}", &project.config.hud_rendering.objects_x.to_string())
        .replace("{{OBJECTS_Y}}", &project.config.hud_rendering.objects_y.to_string())
        .replace("{{OBJECTS_ICON_X}}", &project.config.hud_rendering.objects_icon_x.to_string())
        .replace("{{OBJECTS_ICON_Y}}", &project.config.hud_rendering.objects_icon_y.to_string())
        .replace("{{TOGGLE_REVERSE_OBJECT_COUNT}}", if project.config.mechanics_enemies.reverse_objects_count { "#define REVERSE_OBJECT_COUNT" } else { "// #define REVERSE_OBJECT_COUNT" })
        .replace("{{KEYS_X}}", &project.config.hud_rendering.keys_x.to_string())
        .replace("{{KEYS_Y}}", &project.config.hud_rendering.keys_y.to_string())
        .replace("{{TOGGLE_SHOW_KILLED}}", "// #define SHOW_KILLED")
        .replace("{{TOGGLE_SHOW_TOTAL}}", "// #define SHOW_TOTAL")
        .replace("{{KILLED_X}}", &project.config.hud_rendering.killed_x.to_string())
        .replace("{{KILLED_Y}}", &project.config.hud_rendering.killed_y.to_string())
        .replace("{{TOGGLE_PLAYER_SHOW_ITEM}}", "// #define PLAYER_SHOW_ITEM")
        .replace("{{ITEM_IN_FLAG}}", "4")
        .replace("{{ITEM_FIRST_TILE}}", "17")
        .replace("{{ITEM_SHOW_X}}", "2")
        .replace("{{ITEM_SHOW_Y}}", "21")
        .replace("{{COINS_X}}", &project.config.hud_rendering.ammo_x.to_string())
        .replace("{{COINS_Y}}", &project.config.hud_rendering.ammo_y.to_string())
        .replace("{{EVIL_GAUGE_X}}", &project.config.hud_rendering.timer_x.to_string())
        .replace("{{EVIL_GAUGE_Y}}", &project.config.hud_rendering.timer_y.to_string())
        .replace("{{LINE_OF_TEXT}}", &project.config.hud_rendering.line_of_text.to_string())
        .replace("{{LINE_OF_TEXT_X}}", &project.config.hud_rendering.line_of_text_x.to_string())
        .replace("{{LINE_OF_TEXT_SUBSTR}}", "2")
        .replace("{{LINE_OF_TEXT_ATTR}}", &project.config.hud_rendering.line_of_text_attr.to_string())
        .replace("{{GAME_OVER_ATTR}}", "15")
        .replace("{{TOGGLE_USE_AUTO_SHADOWS}}", if project.config.hud_rendering.use_auto_shadows { "#define USE_AUTO_SHADOWS" } else { "// #define USE_AUTO_SHADOWS" })
        .replace("{{TOGGLE_USE_AUTO_TILE_SHADOWS}}", if project.config.hud_rendering.use_auto_tile_shadows { "#define USE_AUTO_TILE_SHADOWS" } else { "// #define USE_AUTO_TILE_SHADOWS" })
        .replace("{{TOGGLE_UNPACKED_MAP}}", if project.config.hud_rendering.unpacked_map { "#define UNPACKED_MAP" } else { "// #define UNPACKED_MAP" })
        .replace("{{TOGGLE_COLUMN_MAP}}", "// #define COLUMN_MAP")
        .replace("{{TOGGLE_ROW_MAP}}", "// #define ROW_MAP")
        .replace("{{TOGGLE_NO_MAX_ENEMS}}", "#define NO_MAX_ENEMS")
        .replace("{{TOGGLE_PLAYER_ALTERNATE_ANIMATION}}", "// #define PLAYER_ALTERNATE_ANIMATION")
        .replace("{{TOGGLE_TWO_SETS}}", "// #define TWO_SETS")
        .replace("{{TOGGLE_TWO_SETS_REAL}}", "// #define TWO_SETS_REAL")
        .replace("{{TWO_SETS_CONDITION}}", "(n_pant>14?32:0)")
        .replace("{{TOGGLE_MAPPED_TILESETS}}", "// #define MAPPED_TILESETS")
        .replace("{{RLE_MAP}}", "62")
        .replace("{{TOGGLE_ENABLE_ANIMATED_TILES}}", "// #define ENABLE_ANIMATED_TILES")
        .replace("{{ANIMATED_TILE}}", "11")
        .replace("{{MAX_ANIMATED_TILES}}", &project.config.hud_rendering.enable_tilanims.to_string())
        .replace("{{NO_MASKS}}", if project.config.hud_rendering.no_masks { "sp_OR_SPRITE" } else { "sp_MASK_SPRITE" });

    content
}
