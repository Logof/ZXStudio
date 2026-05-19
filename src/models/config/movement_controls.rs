use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovementControlsConfig {
    pub engine_type: u32,                  // 0=BOOTEE, 1=JUMP, 2=JETPAC
    pub player_max_vy_cayendo: u32,        // #define PLAYER_MAX_VY_CAYENDO
    pub player_g: u32,                     // #define PLAYER_G
    pub player_vy_inicial_salto: u32,      // #define PLAYER_VY_INICIAL_SALTO
    pub player_max_vy_saltando: u32,       // #define PLAYER_MAX_VY_SALTANDO
    pub player_incr_salto: u32,            // #define PLAYER_INCR_SALTO
    pub player_incr_jetpac: u32,           // #define PLAYER_INCR_JETPAC
    pub player_max_vy_jetpac: u32,         // #define PLAYER_MAX_VY_JETPAC
    pub player_max_vx: u32,                // #define PLAYER_MAX_VX
    pub player_ax: u32,                    // #define PLAYER_AX
    pub player_rx: u32,                    // #define PLAYER_RX
    pub steps_on_enemies: bool,            // #define PLAYER_STEPS_ON_ENEMIES
    pub player_can_step_on_flag: u32,      // #define PLAYER_CAN_STEP_ON_FLAG (0 = no flag)
    pub player_min_killable: u32,          // #define PLAYER_MIN_KILLABLE
    pub player_step_sound: bool,           // #define PLAYER_STEP_SOUND
    pub player_disable_default_heng: bool, // #define PLAYER_DISABLE_DEFAULT_HENG

    // Keyboard UDK
    pub use_two_buttons: bool, // #define USE_TWO_BUTTONS
    pub key_fire: String,
    pub key_right: String,
    pub key_left: String,
    pub key_down: String,
    pub key_up: String,

    // Top view
    pub player_genital: bool,           // #define PLAYER_GENITAL
    pub top_over_side: bool,            // #define TOP_OVER_SIDE
    pub player_bounce_with_walls: bool, // #define PLAYER_BOUNCE_WITH_WALLS
}

impl Default for MovementControlsConfig {
    fn default() -> Self {
        Self {
            engine_type: 0,
            player_max_vy_cayendo: 512,
            player_g: 32,
            player_vy_inicial_salto: 64,
            player_max_vy_saltando: 320,
            player_incr_salto: 64,
            player_incr_jetpac: 32,
            player_max_vy_jetpac: 256,
            player_max_vx: 192,
            player_ax: 24,
            player_rx: 32,
            steps_on_enemies: false,
            player_can_step_on_flag: 0,
            player_min_killable: 3,
            player_step_sound: true,
            player_disable_default_heng: false,
            use_two_buttons: false,
            key_fire: "0x017f".to_string(),
            key_right: "0x01df".to_string(),
            key_left: "0x02df".to_string(),
            key_down: "0x01fd".to_string(),
            key_up: "0x01fb".to_string(),
            player_genital: false,
            top_over_side: false,
            player_bounce_with_walls: false,
        }
    }
}
