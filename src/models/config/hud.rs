use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HudConfig {
    #[serde(default = "default_1")]
    pub viewport_x: u32,
    #[serde(default)]
    pub viewport_y: u32,

    #[serde(default = "default_22")]
    pub hud_life_x: u32,
    #[serde(default = "default_21")]
    pub hud_life_y: u32,

    #[serde(default = "default_17")]
    pub hud_items_x: u32,
    #[serde(default = "default_21")]
    pub hud_items_y: u32,

    #[serde(default = "default_15")]
    pub hud_items_icon_x: u32,
    #[serde(default = "default_21")]
    pub hud_items_icon_y: u32,

    #[serde(default = "default_27")]
    pub hud_keys_x: u32,
    #[serde(default = "default_21")]
    pub hud_keys_y: u32,

    #[serde(default = "default_12")]
    pub hud_killed_x: u32,
    #[serde(default = "default_21")]
    pub hud_killed_y: u32,

    #[serde(default = "default_99")]
    pub hud_ammo_x: u32,
    #[serde(default = "default_99")]
    pub hud_ammo_y: u32,

    #[serde(default = "default_99")]
    pub hud_timer_x: u32,
    #[serde(default = "default_99")]
    pub hud_timer_y: u32,

    #[serde(default = "default_1")]
    pub line_of_text: u32,
    #[serde(default = "default_1")]
    pub line_of_text_x: u32,
    #[serde(default = "default_71")]
    pub line_of_text_attr: u32,
    #[serde(default)]
    pub line_of_text_no_autoerase: bool,

    #[serde(default)]
    pub use_auto_shadows: bool,
    #[serde(default)]
    pub use_auto_tile_shadows: bool,
    #[serde(default)]
    pub unpacked_map: bool,
    #[serde(default = "default_19")]
    pub packed_map_alt_tile: u32,
    #[serde(default)]
    pub no_masks: bool,
    #[serde(default)]
    pub masked_bullets: bool,
    #[serde(default)]
    pub player_custom_animation: bool,
    #[serde(default = "default_32")]
    pub enable_tilanims: u32,
    #[serde(default)]
    pub pause_abort: bool,
    #[serde(default)]
    pub get_x_more: bool,
    #[serde(default = "default_7")]
    pub hud_ink: u32,
}

fn default_0() -> u32 { 0 }
fn default_1() -> u32 { 1 }
fn default_7() -> u32 { 7 }
fn default_12() -> u32 { 12 }
fn default_15() -> u32 { 15 }
fn default_17() -> u32 { 17 }
fn default_19() -> u32 { 19 }
fn default_21() -> u32 { 21 }
fn default_22() -> u32 { 22 }
fn default_27() -> u32 { 27 }
fn default_32() -> u32 { 32 }
fn default_71() -> u32 { 71 }
fn default_99() -> u32 { 99 }

impl Default for HudConfig {
    fn default() -> Self {
        Self {
            viewport_x: 1,
            viewport_y: 0,
            hud_life_x: 22,
            hud_life_y: 21,
            hud_items_x: 17,
            hud_items_y: 21,
            hud_items_icon_x: 15,
            hud_items_icon_y: 21,
            hud_keys_x: 27,
            hud_keys_y: 21,
            hud_killed_x: 12,
            hud_killed_y: 21,
            hud_ammo_x: 99,
            hud_ammo_y: 99,
            hud_timer_x: 99,
            hud_timer_y: 99,
            line_of_text: 1,
            line_of_text_x: 1,
            line_of_text_attr: 71,
            line_of_text_no_autoerase: false,
            use_auto_shadows: false,
            use_auto_tile_shadows: false,
            unpacked_map: false,
            packed_map_alt_tile: 19,
            no_masks: false,
            masked_bullets: false,
            player_custom_animation: false,
            enable_tilanims: 32,
            pause_abort: false,
            get_x_more: false,
            hud_ink: 7,
        }
    }
}
