use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MapGoalsConfig {
    pub map_w: u32,                // #define MAP_W
    pub map_h: u32,                // #define MAP_H
    pub scr_inicio: u32,           // #define SCR_INICIO (Стартовый экран)
    pub player_ini_x: u32,         // #define PLAYER_INI_X
    pub player_ini_y: u32,         // #define PLAYER_INI_Y
    pub scr_fin: u32,              // #define SCR_FIN (99 = deactivated)
    pub player_fin_x: u32,         // #define PLAYER_FIN_X
    pub player_fin_y: u32,         // #define PLAYER_FIN_Y
    pub player_num_objetos: u32,   // #define PLAYER_NUM_OBJETOS
    pub player_life: u32,          // #define PLAYER_LIFE
    pub player_refill: u32,        // #define PLAYER_REFILL
    pub compressed_levels: bool,   // #define COMPRESSED_LEVELS
    pub per_level_spriteset: bool, // #define PER_LEVEL_SPRITESET
    pub max_levels: u32,           // #define MAX_LEVELS
    pub refill_me: bool,           // #define REFILL_ME
    pub no_reset_stats: bool,      // #define NO_RESET_STATS
}

impl Default for MapGoalsConfig {
    fn default() -> Self {
        Self {
            map_w: 25,
            map_h: 1,
            scr_inicio: 0,
            player_ini_x: 1,
            player_ini_y: 5,
            scr_fin: 99,
            player_fin_x: 99,
            player_fin_y: 99,
            player_num_objetos: 10,
            player_life: 10,
            player_refill: 2,
            compressed_levels: false,
            per_level_spriteset: false,
            max_levels: 4,
            refill_me: false,
            no_reset_stats: false,
        }
    }
}
