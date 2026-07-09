// src/core/exporter/exporter_config/mechanics_physics.rs
use crate::models::ProjectData;

pub fn process(mut content: String, project: &ProjectData) -> String {
    let toggle_pushable = if project.config.shooting_boxes.player_push_boxes {
        "#define PLAYER_PUSH_BOXES"
    } else {
        "// #define PLAYER_PUSH_BOXES"
    };

    let toggle_direct_to_play = if project.config.mechanics_enemies.direct_to_play { "#define DIRECT_TO_PLAY" } else { "// #define DIRECT_TO_PLAY" };

    content = content
        .replace("{{TOGGLE_DIRECT_TO_PLAY}}", toggle_direct_to_play)
        .replace("{{TOGGLE_PLAYER_PUSH_BOXES}}", toggle_pushable)
        .replace("{{TOGGLE_DEACTIVATE_KEYS}}", if project.config.mechanics_enemies.deactivate_keys { "#define DEACTIVATE_KEYS" } else { "// #define DEACTIVATE_KEYS" })
        .replace("{{TOGGLE_DEACTIVATE_OBJECTS}}", if project.config.mechanics_enemies.deactivate_objects { "#define DEACTIVATE_OBJECTS" } else { "// #define DEACTIVATE_OBJECTS" })
        .replace("{{TOGGLE_ONLY_ONE_OBJECT}}", if project.config.mechanics_enemies.only_one_object { "#define ONLY_ONE_OBJECT" } else { "// #define ONLY_ONE_OBJECT" })
        .replace("{{TOGGLE_DEACTIVATE_EVIL_TILE}}", if project.config.mechanics_enemies.deactivate_evil_tile { "#define DEACTIVATE_EVIL_TILE" } else { "// #define DEACTIVATE_EVIL_TILE" })
        .replace("{{TOGGLE_EVIL_TILE_SIMPLE}}", "// #define EVIL_TILE_SIMPLE")
        .replace("{{TOGGLE_DEACTIVATE_EVIL_ZONE}}", "#define DEACTIVATE_EVIL_ZONE")
        .replace("{{EVIL_ZONE_FRAME_COUNT}}", "8")
        .replace("{{EVIL_ZONE_BEEPS_COUNT}}", "32")
        .replace("{{EVIL_ZONE_FREQ}}", "3")
        .replace("{{TOGGLE_EVIL_ZONE_CONDITIONAL}}", "// #define EVIL_ZONE_CONDITIONAL")
        .replace("{{TOGGLE_PLAYER_BOUNCES}}", if project.config.player_physics.player_bounces { "#define PLAYER_BOUNCES" } else { "// #define PLAYER_BOUNCES" })
        .replace("{{PLAYER_FLICKERS}}", if project.config.player_physics.player_flickers { "50" } else { "0" })
        .replace("{{TOGGLE_DEACTIVATE_REFILLS}}", if project.config.mechanics_enemies.deactivate_refills { "#define DEACTIVATE_REFILLS" } else { "// #define DEACTIVATE_REFILLS" })
        .replace("{{TOGGLE_LEGACY_REFILLS}}", "#define LEGACY_REFILLS")
        .replace("{{MAX_FLAGS}}", &project.config.shooting_boxes.max_flags.to_string())
        .replace("{{TOGGLE_PLAYER_DIZZY}}", "// #define PLAYER_DIZZY")
        .replace("{{ENEMIES_LIFE_GAUGE}}", &project.config.shooting_boxes.enemies_life_gauge.to_string())
        .replace("{{TOGGLE_WALLS_STOP_ENEMIES}}", if project.config.mechanics_enemies.walls_stop_enemies { "#define WALLS_STOP_ENEMIES" } else { "// #define WALLS_STOP_ENEMIES" })
        .replace("{{TOGGLE_EVERYTHING_IS_A_WALL}}", if project.config.mechanics_enemies.everything_is_a_wall { "#define EVERYTHING_IS_A_WALL" } else { "// #define EVERYTHING_IS_A_WALL" })
        .replace("{{TOGGLE_ENEMIES_MAY_BE_PARALIZED}}", "// #define ENEMIES_MAY_BE_PARALIZED")
        .replace("{{TOGGLE_PARALYZED_DONT_KILL}}", "// #define PARALYZED_DONT_KILL")
        .replace("{{TOGGLE_ENEMIES_COLLIDE}}", "#define ENEMIES_COLLIDE")
        .replace("{{ENEMIES_COLLIDE_MASK}}", "8")
        .replace("{{TOGGLE_PLATFORMS_ON_FLAG}}", "// #define PLATFORMS_ON_FLAG")
        .replace("{{TOGGLE_PACKED_ENEMS}}", "// #define PACKED_ENEMS")
        .replace("{{TOGGLE_FIXED_ENEMS_LIMITS}}", "// #define FIXED_ENEMS_LIMITS")
        .replace("{{TOGGLE_USE_COINS}}", "// #define USE_COINS")
        .replace("{{COIN_TILE}}", "13")
        .replace("{{COIN_BEH}}", "16")
        .replace("{{COIN_FLAG}}", "1")
        .replace("{{COINS_REFILL}}", "1")
        .replace("{{COIN_TILE_DEACT_SUBS}}", "0")
        .replace("{{TOGGLE_COINS_DEACTIVABLE}}", "// #define COINS_DEACTIVABLE")
        .replace("{{TOGGLE_COINS_SCRIPTING}}", "// #define COINS_SCRIPTING")
        .replace("{{TOGGLE_COINS_PERSISTENT}}", "// #define COINS_PERSISTENT")
        .replace("{{TOGGLE_RESPAWN_REENTER}}", if project.config.shooting_boxes.respawn_on_enter { "#define RESPAWN_ON_ENTER" } else { "// #define RESPAWN_ON_ENTER" })
        .replace("{{TOGGLE_RESPAWN_SHOW_LEVEL}}", "// #define RESPAWN_SHOW_LEVEL")
        .replace("{{TOGGLE_RESPAWN_FLICKER}}", "// #define RESPAWN_FLICKER")
        .replace("{{TOGGLE_RESET_BODY_COUNT_ON_ENTER}}", "// #define RESET_BODY_COUNT_ON_ENTER")
        .replace("{{TOGGLE_USE_SUICIDE_KEY}}", "// #define USE_SUICIDE_KEY")
        .replace("{{TOGGLE_PUSH_OVER_FLOOR}}", "// #define PUSH_OVER_FLOOR")
        .replace("{{TOGGLE_PUSH_AND_PULL}}", "// #define PUSH_AND_PULL")
        .replace("{{TOGGLE_PUSH_AND_PULL_PILES}}", "// #define PUSH_AND_PULL_PILES")
        .replace("{{PLAYER_GRAB_FRAME}}", "2")
        .replace("{{TOGGLE_FALLING_BOXES}}", "// #define FALLING_BOXES")
        .replace("{{FALLING_BOXES_SPEED}}", "4")
        .replace("{{TOGGLE_ENEMIES_BLOCK_BOXES}}", "// #define ENEMIES_BLOCK_BOXES")
        .replace("{{TOGGLE_BOXES_KILL_ENEMIES}}", "// #define BOXES_KILL_ENEMIES")
        .replace("{{BOXES_ONLY_KILL_TYPE}}", "1")
        .replace("{{TOGGLE_BOXES_KILL_PLAYER}}", "// #define BOXES_KILL_PLAYER")
        .replace("{{TOGGLE_PLAYER_CAN_FIRE}}", if project.config.shooting_boxes.player_can_fire { "#define PLAYER_CAN_FIRE" } else { "// #define PLAYER_CAN_FIRE" })
        .replace("{{PLAYER_BULLET_SPEED}}", &project.config.shooting_boxes.player_bullet_speed.to_string())
        .replace("{{MAX_BULLETS}}", &project.config.shooting_boxes.max_bullets.to_string())
        .replace("{{PLAYER_BULLET_Y_OFFSET}}", &project.config.shooting_boxes.player_bullet_y_offset.to_string())
        .replace("{{PLAYER_AX_RECOIL}}", "128")
        .replace("{{TOGGLE_FIRING_DRAINS_LIFE}}", "// #define FIRING_DRAINS_LIFE")
        .replace("{{FIRING_DRAIN_AMOUNT}}", "2")
        .replace("{{TOGGLE_ENABLE_SWORD}}", "// #define ENABLE_SWORD")
        .replace("{{TOGGLE_SWORD_UP}}", "// #define SWORD_UP")
        .replace("{{SWORD_LINEAL_DAMAGE}}", "1")
        .replace("{{SWORD_FLYING_DAMAGE}}", "1")
        .replace("{{SWORD_PARALYZES}}", "32")
        .replace("{{SWORD_HIT_FRAME}}", "2")
        .replace("{{TOGGLE_GENITAL_HIT_FRAMES}}", "// #define GENITAL_HIT_FRAMES")
        .replace("{{SWORD_STAB}}", "5")
        .replace("{{TOGGLE_ENABLE_BREAKABLE}}", if project.config.shooting_boxes.breakable_walls { "#define BREAKABLE_WALLS" } else { "// #define BREAKABLE_WALLS" })
        .replace("{{MAX_BREAKABLE_FRAMES}}", "8")
        .replace("{{BREAKABLE_WALLS_LIFE}}", &project.config.shooting_boxes.breakable_walls_life.to_string())
        .replace("{{BREAKABLE_WALLS_BREAKING}}", &project.config.shooting_boxes.breakable_walls_breaking.to_string())
        .replace("{{BREAKABLE_WALLS_BROKEN}}", &project.config.shooting_boxes.breakable_walls_broken.to_string());

    content
}
