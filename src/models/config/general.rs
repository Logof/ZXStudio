use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneralConfig {
    #[serde(default = "default_min_faps")]
    pub min_faps_per_frame: u32,
    #[serde(default)]
    pub use_arkos_player: bool,
    #[serde(default)]
    pub arkos_sfx_channel: u32,
    #[serde(default)]
    pub veng_selector: bool,
    #[serde(default)]
    pub use_map_custom_decoder: bool,

    #[serde(default = "default_player_ini")]
    pub player_ini_x: u32,
    #[serde(default = "default_player_ini")]
    pub player_ini_y: u32,
    #[serde(default = "default_99")]
    pub scr_fin: u32,
    #[serde(default = "default_99")]
    pub player_fin_x: u32,
    #[serde(default = "default_99")]
    pub player_fin_y: u32,
    #[serde(default = "default_99")]
    pub player_num_objetos: u32,
    #[serde(default = "default_life")]
    pub player_life_ini: u32,
    #[serde(default = "default_1")]
    pub player_refill: u32,
    #[serde(default)]
    pub compressed_levels: bool,
    #[serde(default)]
    pub per_level_spriteset: bool,
    #[serde(default = "default_1")]
    pub max_levels: u32,
    #[serde(default)]
    pub refill_me: bool,
    #[serde(default)]
    pub no_reset_stats: bool,

    // РАЗДЕЛ IV: КОНСТАНТЫ ФИЗИЧЕСКОГО ДВИЖКА
    #[serde(default = "default_512")]
    pub player_max_vy_cayendo: u32,
    #[serde(default = "default_48")]
    pub player_g: u32,
    #[serde(default = "default_96")]
    pub player_vy_inicial_salto: u32,
    #[serde(default = "default_312")]
    pub player_max_vy_saltando: u32,
    #[serde(default = "default_48")]
    pub player_incr_salto: u32,
    #[serde(default = "default_32")]
    pub player_incr_jetpac: u32,
    #[serde(default = "default_256")]
    pub player_max_vy_jetpac: u32,
    #[serde(default = "default_256")]
    pub player_max_vx: u32,
    #[serde(default = "default_48")]
    pub player_ax: u32,
    #[serde(default = "default_64")]
    pub player_rx: u32,
}

fn default_min_faps() -> u32 { 2 }
fn default_player_ini() -> u32 { 1 }
fn default_99() -> u32 { 99 }
fn default_life() -> u32 { 15 }
fn default_1() -> u32 { 1 }
fn default_512() -> u32 { 512 }
fn default_48() -> u32 { 48 }
fn default_96() -> u32 { 96 }
fn default_312() -> u32 { 312 }
fn default_32() -> u32 { 32 }
fn default_256() -> u32 { 256 }
fn default_64() -> u32 { 64 }

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            min_faps_per_frame: 2,
            use_arkos_player: false,
            arkos_sfx_channel: 0,
            veng_selector: false,
            use_map_custom_decoder: false,
            player_ini_x: 1,
            player_ini_y: 7,
            scr_fin: 99,
            player_fin_x: 99,
            player_fin_y: 99,
            player_num_objetos: 99,
            player_life_ini: 15,
            player_refill: 1,
            compressed_levels: false,
            per_level_spriteset: false,
            max_levels: 1,
            refill_me: false,
            no_reset_stats: false,
            player_max_vy_cayendo: 512,
            player_g: 48,
            player_vy_inicial_salto: 96,
            player_max_vy_saltando: 312,
            player_incr_salto: 48,
            player_incr_jetpac: 32,
            player_max_vy_jetpac: 256,
            player_max_vx: 256,
            player_ax: 48,
            player_rx: 64,
        }
    }
}
