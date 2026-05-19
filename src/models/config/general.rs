use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneralConfig {
    pub mode_128k: bool,              // #define MODE_128K
    pub min_faps_per_frame: u32,      // #define MIN_FAPS_PER_FRAME
    pub use_arkos_player: bool,       // #define USE_ARKOS_PLAYER
    pub arkos_sfx_channel: u32,       // #define ARKOS_SFX_CHANNEL
    pub veng_selector: bool,          // #define VENG_SELECTOR
    pub use_map_custom_decoder: bool, // #define USE_MAP_CUSTOM_DECODER
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            mode_128k: true,
            min_faps_per_frame: 2,
            use_arkos_player: true,
            arkos_sfx_channel: 0,
            veng_selector: false,
            use_map_custom_decoder: false,
        }
    }
}
