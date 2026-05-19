use crate::models::{config::EngineViewMode, ProjectData};
use chrono::Local;
use std::fs;
use std::io::Write;

/// Выделенный генератор глобальной конфигурации и физики по эталонному шаблону
pub fn build_and_write_config_h(project: &ProjectData) -> Result<(), std::io::Error> {
    let template_path = "templates/config.h.template";
    let target_path = "dev/config.h";

    let template_content = fs::read_to_string(template_path)?;
    let total_screens = project.config.map_goals.map_w * project.config.map_goals.map_h;

    // 1. Формируем строки макросов условной компиляции на основе флагов из GUI ролей
    let toggle_pushable = if project.role_pushbox_active {
        "#define PLAYER_PUSH_BOXES // Активировано в IDE"
    } else {
        "// #define PLAYER_PUSH_BOXES // Отключено в IDE"
    };

    let toggle_locks_and_keys = if project.role_lock_active || project.role_key_active {
        "#define ACTIVATE_KEYS_AND_LOCKS // Активировано в IDE"
    } else {
        "// #define ACTIVATE_KEYS_AND_LOCKS // Отключено в IDE"
    };

    let toggle_collectables = if project.role_collectable_active {
        "#define ACTIVATE_OBJECTS_COLLECT // Активировано в IDE"
    } else {
        "// #define ACTIVATE_OBJECTS_COLLECT // Отключено в IDE"
    };

    let toggle_alt_bg = if project.role_alt_bg_active {
        "#define ENABLE_RANDOM_ALT_BG // Активировано в IDE"
    } else {
        "// #define ENABLE_RANDOM_ALT_BG // Отключено в IDE"
    };

    let toggle_direct_to_play = if project.config.map_goals.scr_inicio == 0 {
        "#define DIRECT_TO_PLAY"
    } else {
        "// #define DIRECT_TO_PLAY"
    };

    let engine_view_mode_str = if project.config.movement_controls.player_genital {
        "Top View (Вид сверху / Moggy Style)";
    } else {
        "Side View (Платформер)";
    };

    let toggle_view_style = if project.config.movement_controls.player_genital {
        "#define PLAYER_MOGGY_STYLE";
    } else {
        "// #define PLAYER_MOGGY_STYLE"
    };

    // 2. Генерация Си-массива поведения тайлов платформы на полные 48 байт
    let mut beh_string = String::from("unsigned char comportamiento_tiles [] = {\n\t");
    for (i, beh) in project.tile_behaviours.iter().enumerate() {
        beh_string.push_str(&format!("{}, ", beh));
        if (i + 1) % 16 == 0 && i < 47 {
            beh_string.push_str("\n\t");
        }
    }
    beh_string.push_str("\n};");

    let current_date = Local::now().format("%Y-%m-%d").to_string();

    // 3. Выполняем подстановку в наш эталонный config.h.template
    let output_content = template_content
        .replace("{{GENERATION_DATE}}", &current_date)
        .replace(
            "{{TARGET_PROFILE}}",
            if project.is_128k {
                "ZX Spectrum 128K (AY)"
            } else {
                "ZX Spectrum 48K"
            },
        )
        .replace("{{ENGINE_VIEW_MODE}}", engine_view_mode_str)
        .replace("{{TOGGLE_DIRECT_TO_PLAY}}", toggle_direct_to_play)
        .replace("{{TOGGLE_DEACTIVATE_KEYS}}", "// #define DEACTIVATE_KEYS")
        .replace(
            "{{TOGGLE_DEACTIVATE_OBJECTS}}",
            "// #define DEACTIVATE_OBJECTS",
        )
        .replace("{{TOGGLE_ONLY_ONE_OBJECT}}", "// #define ONLY_ONE_OBJECT")
        .replace(
            "{{TOGGLE_DEACTIVATE_EVIL_TILE}}",
            "// #define DEACTIVATE_EVIL_TILE",
        )
        .replace("{{TOGGLE_EVIL_TILE_SIMPLE}}", "// #define EVIL_TILE_SIMPLE")
        .replace(
            "{{TOGGLE_DEACTIVATE_EVIL_ZONE}}",
            "#define DEACTIVATE_EVIL_ZONE",
        )
        .replace("{{EVIL_ZONE_FRAME_COUNT}}", "8")
        .replace("{{EVIL_ZONE_BEEPS_COUNT}}", "32")
        .replace("{{EVIL_ZONE_FREQ}}", "3")
        .replace(
            "{{TOGGLE_EVIL_ZONE_CONDITIONAL}}",
            "// #define EVIL_ZONE_CONDITIONAL",
        )
        .replace("{{TOGGLE_PLAYER_BOUNCES}}", "#define PLAYER_BOUNCES")
        .replace("{{PLAYER_FLICKERS}}", "50")
        .replace(
            "{{TOGGLE_DEACTIVATE_REFILLS}}",
            "// #define DEACTIVATE_REFILLS",
        )
        .replace("{{TOGGLE_LEGACY_REFILLS}}", "#define LEGACY_REFILLS")
        .replace("{{MAX_FLAGS}}", "1")
        .replace("{{TOGGLE_PLAYER_DIZZY}}", "// #define PLAYER_DIZZY")
        .replace(
            "{{ENEMIES_LIFE_GAUGE}}",
            &project.config.enemies_life_gauge.to_string(),
        )
        .replace(
            "{{TOGGLE_ENEMIES_MAY_BE_PARALIZED}}",
            "// #define ENEMIES_MAY_BE_PARALIZED",
        )
        .replace(
            "{{TOGGLE_PARALYZED_DONT_KILL}}",
            "// #define PARALYZED_DONT_KILL",
        )
        .replace("{{TOGGLE_ENEMIES_COLLIDE}}", "#define ENEMIES_COLLIDE")
        .replace("{{ENEMIES_COLLIDE_MASK}}", "8")
        .replace(
            "{{TOGGLE_PLATFORMS_ON_FLAG}}",
            "// #define PLATFORMS_ON_FLAG",
        )
        .replace("{{TOGGLE_PACKED_ENEMS}}", "// #define PACKED_ENEMS")
        .replace(
            "{{TOGGLE_FIXED_ENEMS_LIMITS}}",
            "// #define FIXED_ENEMS_LIMITS",
        )
        .replace("{{TOGGLE_USE_COINS}}", "// #define USE_COINS")
        .replace("{{COIN_TILE}}", "13")
        .replace("{{COIN_BEH}}", "16")
        .replace("{{COIN_FLAG}}", "1")
        .replace("{{COINS_REFILL}}", "1")
        .replace("{{COIN_TILE_DEACT_SUBS}}", "0")
        .replace(
            "{{TOGGLE_COINS_DEACTIVABLE}}",
            "// #define COINS_DEACTIVABLE",
        )
        .replace("{{TOGGLE_COINS_SCRIPTING}}", "// #define COINS_SCRIPTING")
        .replace("{{TOGGLE_COINS_PERSISTENT}}", "// #define COINS_PERSISTENT")
        .replace("{{TOGGLE_FIXED_SCREENS}}", "// #define FIXED_SCREENS")
        .replace("{{TOGGLE_SHOW_LEVEL_INFO}}", "// #define SHOW_LEVEL_INFO")
        .replace(
            "{{TOGGLE_SHOW_LEVEL_SUBLEVEL}}",
            "// #define SHOW_LEVEL_SUBLEVEL",
        )
        .replace("{{TOGGLE_RESPAWN_REENTER}}", "// #define RESPAWN_REENTER")
        .replace(
            "{{TOGGLE_RESPAWN_SHOW_LEVEL}}",
            "// #define RESPAWN_SHOW_LEVEL",
        )
        .replace("{{TOGGLE_RESPAWN_FLICKER}}", "// #define RESPAWN_FLICKER")
        .replace(
            "{{TOGGLE_RESET_BODY_COUNT_ON_ENTER}}",
            "// #define RESET_BODY_COUNT_ON_ENTER",
        )
        .replace("{{TOGGLE_USE_SUICIDE_KEY}}", "// #define USE_SUICIDE_KEY")
        .replace("{{TOGGLE_PLAYER_PUSH_BOXES}}", toggle_pushable)
        .replace("{{TOGGLE_PUSH_OVER_FLOOR}}", "// #define PUSH_OVER_FLOOR")
        .replace("{{TOGGLE_PUSH_AND_PULL}}", "// #define PUSH_AND_PULL")
        .replace(
            "{{TOGGLE_PUSH_AND_PULL_PILES}}",
            "// #define PUSH_AND_PULL_PILES",
        )
        .replace("{{PLAYER_GRAB_FRAME}}", "2")
        .replace("{{TOGGLE_FALLING_BOXES}}", "// #define FALLING_BOXES")
        .replace("{{FALLING_BOXES_SPEED}}", "4")
        .replace(
            "{{TOGGLE_ENEMIES_BLOCK_BOXES}}",
            "// #define ENEMIES_BLOCK_BOXES",
        )
        .replace(
            "{{TOGGLE_BOXES_KILL_ENEMIES}}",
            "// #define BOXES_KILL_ENEMIES",
        )
        .replace("{{BOXES_ONLY_KILL_TYPE}}", "1")
        .replace(
            "{{TOGGLE_BOXES_KILL_PLAYER}}",
            "// #define BOXES_KILL_PLAYER",
        )
        .replace("{{TOGGLE_PLAYER_CAN_FIRE}}", "// #define PLAYER_CAN_FIRE")
        .replace("{{PLAYER_BULLET_SPEED}}", "8")
        .replace("{{MAX_BULLETS}}", &project.config.max_bullets.to_string())
        .replace("{{PLAYER_BULLET_Y_OFFSET}}", "4")
        .replace("{{PLAYER_AX_RECOIL}}", "128")
        .replace(
            "{{TOGGLE_FIRING_DRAINS_LIFE}}",
            "// #define FIRING_DRAINS_LIFE",
        )
        .replace("{{FIRING_DRAIN_AMOUNT}}", "2")
        .replace("{{TOGGLE_ENABLE_SWORD}}", "// #define ENABLE_SWORD")
        .replace("{{TOGGLE_SWORD_UP}}", "// #define SWORD_UP")
        .replace("{{SWORD_LINEAL_DAMAGE}}", "1")
        .replace("{{SWORD_FLYING_DAMAGE}}", "1")
        .replace("{{SWORD_PARALYZES}}", "32")
        .replace("{{SWORD_HIT_FRAME}}", "2")
        .replace(
            "{{TOGGLE_GENITAL_HIT_FRAMES}}",
            "// #define GENITAL_HIT_FRAMES",
        )
        .replace("{{SWORD_STAB}}", "5")
        .replace("{{TOGGLE_ENABLE_BREAKABLE}}", "// #define ENABLE_BREAKABLE")
        .replace("{{MAX_BREAKABLE_FRAMES}}", "8")
        .replace("{{BREAKABLE_BREAKING_TILE}}", "45")
        .replace("{{BREAKABLE_ERASE_TILE}}", "0")
        .replace("{{BREAKABLE_SPAWN_CHANCE}}", "3")
        .replace("{{BREAKABLE_SPAWN_TILE}}", "46")
        .replace(
            "{{TOGGLE_BREAKABLE_PERSISTENT}}",
            "// #define BREAKABLE_PERSISTENT",
        )
        .replace(
            "{{TOGGLE_ENABLE_PERSISTENCE}}",
            "// #define ENABLE_PERSISTENCE",
        )
        .replace("{{PERSIST_CLEAR_TILE}}", "0")
        .replace(
            "{{TOGGLE_ENABLE_FRIGOABABOL}}",
            "// #define ENABLE_FRIGOABABOL",
        )
        .replace("{{FRIGO_MAX_FRAMES}}", "32")
        .replace(
            "{{TOGGLE_FRIGO_UNFREEZE_TIME}}",
            "// #define FRIGO_UNFREEZE_TIME",
        )
        .replace(
            "{{TOGGLE_FRIGO_UNFREEZE_FIRE}}",
            "// #define FRIGO_UNFREEZE_FIRE",
        )
        .replace("{{TOGGLE_FRIGO_NO_FIRE}}", "// #define FRIGO_NO_FIRE")
        .replace("{{TOGGLE_FRIGO_FIGHT}}", "// #define FRIGO_FIGHT")
        .replace(
            "{{TOGGLE_FRIGO_ENEMIES_FREEZE}}",
            "// #define FRIGO_ENEMIES_FREEZE",
        )
        .replace(
            "{{TOGGLE_FRIGO_FROZEN_NO_RX}}",
            "// #define FRIGO_FROZEN_NO_RX",
        )
        .replace("{{TOGGLE_SLIPPERY_TILES}}", "// #define SLIPPERY_TILES")
        .replace("{{TOGGLE_CONVEYOR_TILES}}", "// #define CONVEYOR_TILES")
        .replace("{{TOGGLE_QUICKSAND_TILES}}", "// #define QUICKSAND_TILES")
        .replace("{{TOGGLE_PLAYER_CAN_HIDE}}", "// #define PLAYER_CAN_HIDE")
        .replace("{{TOGGLE_HIDDEN_CAN_MOVE}}", "// #define HIDDEN_CAN_MOVE")
        .replace("{{TOGGLE_RANDOM_RESPAWN}}", "// #define RANDOM_RESPAWN")
        .replace("{{TOGGLE_USE_TYPE_6}}", "// #define USE_TYPE_6")
        .replace(
            "{{TOGGLE_USE_SIGHT_DISTANCE}}",
            "// #define USE_SIGHT_DISTANCE",
        )
        .replace("{{SIGHT_DISTANCE}}", "120")
        .replace("{{FANTY_MAX_V}}", "256")
        .replace("{{FANTY_A}}", "16")
        .replace("{{FANTIES_LIFE_GAUGE}}", "10")
        .replace("{{TOGGLE_MAKE_TYPE_6}}", "// #define MAKE_TYPE_6")
        .replace("{{FANTIES_EXIT_STATE_V}}", "32")
        .replace("{{TOGGLE_ENABLE_CUADRATORS}}", "#define ENABLE_CUADRATORS")
        .replace(
            "{{TOGGLE_ENABLE_MARRULLERS}}",
            "// #define ENABLE_MARRULLERS",
        )
        .replace(
            "{{TOGGLE_ACTIVATE_SCRIPTING}}",
            "// #define ACTIVATE_SCRIPTING",
        )
        .replace("{{TOGGLE_WIN_ON_SCRIPTING}}", "// #define WIN_ON_SCRIPTING")
        .replace("{{TOGGLE_SCRIPTING_DOWN}}", "// #define SCRIPTING_DOWN")
        .replace("{{COUNT_KILLABLE_ON}}", "2")
        .replace("{{TOGGLE_SCRIPTING_KEY_M}}", "// #define SCRIPTING_KEY_M")
        .replace("{{OBJECTS_ON_VAR}}", "2")
        .replace("{{OBJECT_COUNT}}", "3")
        .replace(
            "{{TOGGLE_REENTER_ON_ALL_OBJECTS}}",
            "// #define REENTER_ON_ALL_OBJECTS",
        )
        .replace("{{TOGGLE_MASTER_OF_KEYS}}", toggle_locks_and_keys)
        .replace(
            "{{TOGGLE_PARALYZED_DONT_KILL_ON_VAR}}",
            "// #define PARALYZED_DONT_KILL_ON_VAR",
        )
        .replace(
            "{{TOGGLE_RAMIRO_HOVER_ON_VAR}}",
            "// #define RAMIRO_HOVER_ON_VAR",
        )
        .replace(
            "{{TOGGLE_DISABLE_SLIPPERY_ON_VAR}}",
            "// #define DISABLE_SLIPPERY_ON_VAR",
        )
        .replace("{{MAP_W}}", &project.map_w.to_string())
        .replace("{{MAP_H}}", &project.map_h.to_string())
        .replace("{{TOTAL_SCREENS}}", &total_screens.to_string())
        .replace("{{SCR_INICIO}}", &project.scr_inicio.to_string())
        .replace("{{VIEWPORT_X}}", "0")
        .replace("{{VIEWPORT_Y}}", "2")
        .replace("{{LIFE_X}}", &project.config.hud_life_x.to_string())
        .replace("{{LIFE_Y}}", &project.config.hud_life_y.to_string())
        .replace("{{TOGGLE_DRAW_HI_DIGIT}}", "// #define DRAW_HI_DIGIT")
        .replace("{{LIFE_H_X}}", "1")
        .replace("{{LIFE_H_Y}}", "8")
        .replace("{{OBJECTS_X}}", "30")
        .replace("{{OBJECTS_Y}}", "12")
        .replace("{{OBJECTS_ICON_X}}", "2")
        .replace("{{OBJECTS_ICON_Y}}", "21")
        .replace(
            "{{TOGGLE_REVERSE_OBJECT_COUNT}}",
            "// #define REVERSE_OBJECT_COUNT",
        )
        .replace("{{KEYS_X}}", "30")
        .replace("{{KEYS_Y}}", "16")
        .replace("{{TOGGLE_SHOW_KILLED}}", "// #define SHOW_KILLED")
        .replace("{{TOGGLE_SHOW_TOTAL}}", "// #define SHOW_TOTAL")
        .replace("{{KILLED_X}}", "20")
        .replace("{{KILLED_Y}}", "21")
        .replace("{{TOGGLE_PLAYER_SHOW_ITEM}}", "// #define PLAYER_SHOW_ITEM")
        .replace("{{ITEM_IN_FLAG}}", "4")
        .replace("{{ITEM_FIRST_TILE}}", "17")
        .replace("{{ITEM_SHOW_X}}", "2")
        .replace("{{ITEM_SHOW_Y}}", "21")
        .replace("{{COINS_X}}", "12")
        .replace("{{COINS_Y}}", "23")
        .replace("{{EVIL_GAUGE_X}}", "21")
        .replace("{{EVIL_GAUGE_Y}}", "23")
        .replace("{{LINE_OF_TEXT}}", "23")
        .replace("{{LINE_OF_TEXT_X}}", "1")
        .replace("{{LINE_OF_TEXT_SUBSTR}}", "2")
        .replace("{{LINE_OF_TEXT_ATTR}}", "7")
        .replace("{{GAME_OVER_ATTR}}", "15")
        .replace("{{TOGGLE_USE_AUTO_SHADOWS}}", "// #define USE_AUTO_SHADOWS")
        .replace(
            "{{TOGGLE_USE_AUTO_TILE_SHADOWS}}",
            "// #define USE_AUTO_TILE_SHADOWS",
        )
        .replace("{{TOGGLE_UNPACKED_MAP}}", "// #define UNPACKED_MAP")
        .replace("{{TOGGLE_COLUMN_MAP}}", "// #define COLUMN_MAP")
        .replace("{{TOGGLE_ROW_MAP}}", "// #define ROW_MAP")
        .replace("{{TOGGLE_NO_ALT_BG}}", toggle_alt_bg)
        .replace("{{TOGGLE_NO_MAX_ENEMS}}", "#define NO_MAX_ENEMS")
        .replace(
            "{{TOGGLE_PLAYER_ALTERNATE_ANIMATION}}",
            "// #define PLAYER_ALTERNATE_ANIMATION",
        )
        .replace("{{TOGGLE_TWO_SETS}}", "// #define TWO_SETS")
        .replace("{{TOGGLE_TWO_SETS_REAL}}", "// #define TWO_SETS_REAL")
        .replace("{{TWO_SETS_CONDITION}}", "(n_pant>14?32:0)")
        .replace("{{TOGGLE_MAPPED_TILESETS}}", "// #define MAPPED_TILESETS")
        .replace("{{RLE_MAP}}", "62")
        .replace(
            "{{TOGGLE_ENABLE_ANIMATED_TILES}}",
            "// #define ENABLE_ANIMATED_TILES",
        )
        .replace("{{ANIMATED_TILE}}", "11")
        .replace("{{MAX_ANIMATED_TILES}}", "16")
        .replace("{{NO_MASKS}}", "sp_OR_SPRITE")
        .replace("{{PLAYER_MAX_VY_CAYENDO}}", "512")
        .replace("{{PLAYER_G}}", "32")
        .replace("{{PLAYER_MAX_VY_CAYENDO_H}}", "256")
        .replace("{{PLAYER_G_HOVER}}", "4")
        .replace("{{PLAYER_VY_INICIAL_SALTO}}", "64")
        .replace("{{PLAYER_MAX_VY_SALTANDO}}", "320")
        .replace("{{PLAYER_INCR_SALTO}}", "48")
        .replace("{{PLAYER_INCR_JETPAC}}", "48")
        .replace("{{PLAYER_MAX_VY_JETPAC}}", "384")
        .replace("{{PLAYER_MAX_VX}}", "256")
        .replace("{{PLAYER_AX}}", "32")
        .replace("{{PLAYER_RX}}", "24")
        .replace("{{PLAYER_AX_SLIPPERY}}", "8")
        .replace("{{PLAYER_RX_SLIPPERY}}", "8")
        .replace("{{PLAYER_VX_CONVEYORS}}", "128")
        .replace("{{PLAYER_AX_QUICKSANDS}}", "8")
        .replace("{{PLAYER_RX_QUICKSANDS}}", "64")
        .replace("{{PLAYER_MAX_VX_QUICKSANDS}}", "64")
        .replace("{{PLAYER_VY_SINKING}}", "4")
        .replace("{{TOGGLE_PLAYER_MOGGY_STYLE}}", toggle_view_style)
        .replace("{{TILE_BEHAVIOURS_ARRAY}}", &beh_string);

    let mut file = fs::File::create(target_path)?;
    file.write_all(output_content.as_bytes())?;
    Ok(())
}
