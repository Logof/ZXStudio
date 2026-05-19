use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerPhysicsConfig {
    pub bounding_box_8_bottom: bool,      // #define BOUNDING_BOX_8_BOTTOM
    pub bounding_box_8_centered: bool,    // #define BOUNDING_BOX_8_CENTERED
    pub bounding_box_12x2_centered: bool, // #define BOUNDING_BOX_12X2_CENTERED
    pub small_collision: bool,            // #define SMALL_COLLISION
    pub player_bounces: bool,             // #define PLAYER_BOUNCES
    pub full_bounce: bool,                // #define FULL_BOUNCE
    pub slow_drain: bool,                 // #define SLOW_DRAIN
    pub player_flickers: bool,            // #define PLAYER_FLICKERS
    pub die_and_respawn: bool,            // #define DIE_AND_RESPAWN
    pub safe_spot_on_entering: bool,      // #define SAFE_SPOT_ON_ENTERING
}

impl Default for PlayerPhysicsConfig {
    fn default() -> Self {
        Self {
            bounding_box_8_bottom: true,
            bounding_box_8_centered: false,
            bounding_box_12x2_centered: false,
            small_collision: true,
            player_bounces: false,
            full_bounce: false,
            slow_drain: false,
            player_flickers: false,
            die_and_respawn: true,
            safe_spot_on_entering: true,
        }
    }
}
