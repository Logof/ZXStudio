use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HudRenderingConfig {
    pub viewport_x: u32,                 // #define VIEWPORT_X
    pub viewport_y: u32,                 // #define VIEWPORT_Y
    pub life_x: u32,                     // #define LIFE_X
    pub life_y: u32,                     // #define LIFE_Y
    pub objects_x: u32,                  // #define OBJECTS_X
    pub objects_y: u32,                  // #define OBJECTS_Y
    pub objects_icon_x: u32,             // #define OBJECTS_ICON_X
    pub objects_icon_y: u32,             // #define OBJECTS_ICON_Y
    pub keys_x: u32,                     // #define KEYS_X
    pub keys_y: u32,                     // #define KEYS_Y
    pub killed_x: u32,                   // #define KILLED_X
    pub killed_y: u32,                   // #define KILLED_Y
    pub ammo_x: u32,                     // #define AMMO_X
    pub ammo_y: u32,                     // #define AMMO_Y
    pub timer_x: u32,                    // #define TIMER_X
    pub timer_y: u32,                    // #define TIMER_Y
    pub line_of_text: u32,               // #define LINE_OF_TEXT
    pub line_of_text_x: u32,             // #define LINE_OF_TEXT_X
    pub line_of_text_attr: u32,          // #define LINE_OF_TEXT_ATTR
    pub line_of_text_no_autoerase: bool, // #define LINE_OF_TEXT_NO_AUTOERASE
    pub use_auto_shadows: bool,          // #define USE_AUTO_SHADOWS
    pub use_auto_tile_shadows: bool,     // #define USE_AUTO_TILE_SHADOWS
    pub unpacked_map: bool,              // #define UNPACKED_MAP
    pub packed_map_alt_tile: u32,        // #define PACKED_MAP_ALT_TILE
    pub no_masks: bool,                  // #define NO_MASKS
    pub masked_bullets: bool,            // #define MASKED_BULLETS
    pub player_custom_animation: bool,   // #define PLAYER_CUSTOM_ANIMATION
    pub enable_tilanims: u32,            // #define ENABLE_TILANIMS
    pub pause_abort: bool,               // #define PAUSE_ABORT
    pub get_x_more: bool,                // #define GET_X_MORE
    pub hud_ink: u32,                    // #define HUD_INK
}

impl Default for HudRenderingConfig {
    fn default() -> Self {
        Self {
            viewport_x: 1,
            viewport_y: 3,
            life_x: 5,
            life_y: 1,
            objects_x: 16,
            objects_y: 1,
            objects_icon_x: 99,
            objects_icon_y: 99,
            keys_x: 22,
            keys_y: 1,
            killed_x: 99,
            killed_y: 99,
            ammo_x: 99,
            ammo_y: 99,
            timer_x: 29,
            timer_y: 1,
            line_of_text: 99,
            line_of_text_x: 1,
            line_of_text_attr: 71,
            line_of_text_no_autoerase: false,
            use_auto_shadows: false,
            use_auto_tile_shadows: false,
            unpacked_map: true,
            packed_map_alt_tile: 19,
            no_masks: false,
            masked_bullets: false,
            player_custom_animation: true,
            enable_tilanims: 0,
            pause_abort: false,
            get_x_more: false,
            hud_ink: 4,
        }
    }
}
