use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShootingBoxesConfig {
    // Cocos & Push boxes
    pub cocos_v: u32,                  // #define COCOS_V (0 = deactivated)
    pub player_push_boxes: bool,       // #define PLAYER_PUSH_BOXES
    pub fire_to_push: bool,            // #define FIRE_TO_PUSH
    pub enable_pushed_scripting: bool, // #define ENABLE_PUSHED_SCRIPTING
    pub moved_tile_flag: u32,          // #define MOVED_TILE_FLAG
    pub moved_x_flag: u32,             // #define MOVED_X_FLAG
    pub moved_y_flag: u32,             // #define MOVED_Y_FLAG
    pub pushing_action: bool,          // #define PUSHING_ACTION

    // Shooting
    pub player_can_fire: bool,       // #define PLAYER_CAN_FIRE
    pub player_can_fire_flag: u32,   // #define PLAYER_CAN_FIRE_FLAG (0 = no flag)
    pub player_bullet_speed: u32,    // #define PLAYER_BULLET_SPEED
    pub max_bullets: u32,            // #define MAX_BULLETS
    pub player_bullet_y_offset: u32, // #define PLAYER_BULLET_Y_OFFSET
    pub player_bullet_x_offset: u32, // #define PLAYER_BULLET_X_OFFSET
    pub enemies_life_gauge: u32,     // #define ENEMIES_LIFE_GAUGE
    pub limited_bullets: bool,       // #define LIMITED_BULLETS
    pub lb_frames: u32,              // #define LB_FRAMES
    pub lb_frames_flag: u32,         // #define LB_FRAMES_FLAG
    pub respawn_on_enter: bool,      // #define RESPAWN_ON_ENTER
    pub fire_min_killable: u32,      // #define FIRE_MIN_KILLABLE
    pub can_fire_up: bool,           // #define CAN_FIRE_UP
    pub max_ammo: u32,               // #define MAX_AMMO (0 = infinite)
    pub ammo_refill: u32,            // #define AMMO_REFILL
    pub initial_ammo: u32,           // #define INITIAL_AMMO

    // Breakable walls
    pub breakable_walls: bool,         // #define BREAKABLE_WALLS
    pub breakable_walls_life: u32,     // #define BREAKABLE_WALLS_LIFE
    pub breakable_walls_breaking: u32, // #define BREAKABLE_WALLS_BREAKING
    pub breakable_walls_broken: u32,   // #define BREAKABLE_WALLS_BROKEN

    // Scripting & Timer & Checkpoints
    pub activate_scripting: bool,  // #define ACTIVATE_SCRIPTING
    pub max_flags: u32,            // #define MAX_FLAGS
    pub scripting_key: u32,        // 0=DOWN, 1=M, 2=FIRE, 3=NONE
    pub enable_extern_code: bool,  // #define ENABLE_EXTERN_CODE
    pub enable_fire_zone: bool,    // #define ENABLE_FIRE_ZONE
    pub script_page: u32,          // #define SCRIPT_PAGE
    pub timer_enable: bool,        // #define TIMER_ENABLE
    pub timer_initial: u32,        // #define TIMER_INITIAL
    pub timer_refill: u32,         // #define TIMER_REFILL
    pub timer_lapse: u32,          // #define TIMER_LAPSE
    pub timer_start: bool,         // #define TIMER_START
    pub timer_script_0: bool,      // #define TIMER_SCRIPT_0
    pub timer_behavior: u32,       // 0=GAMEOVER_0, 1=KILL_0
    pub timer_warp: bool,          // Enable warp logic
    pub timer_warp_to_screen: u32, // #define TIMER_WARP_TO
    pub timer_warp_to_x: u32,      // #define TIMER_WARP_TO_X
    pub timer_warp_to_y: u32,      // #define TIMER_WARP_TO_Y
    pub timer_auto_reset: bool,    // #define TIMER_AUTO_RESET
    pub show_timer_over: bool,     // #define SHOW_TIMER_OVER
    pub enable_checkpoints: bool,  // #define ENABLE_CHECKPOINTS
    pub cp_reset_when_dying: bool, // #define CP_RESET_WHEN_DYING
    pub cp_reset_also_flags: bool, // #define CP_RESET_ALSO_FLAGS
}

impl Default for ShootingBoxesConfig {
    fn default() -> Self {
        Self {
            cocos_v: 0,
            player_push_boxes: false,
            fire_to_push: false,
            enable_pushed_scripting: false,
            moved_tile_flag: 1,
            moved_x_flag: 2,
            moved_y_flag: 3,
            pushing_action: false,
            player_can_fire: false,
            player_can_fire_flag: 0,
            player_bullet_speed: 8,
            max_bullets: 3,
            player_bullet_y_offset: 6,
            player_bullet_x_offset: 0,
            enemies_life_gauge: 4,
            limited_bullets: false,
            lb_frames: 4,
            lb_frames_flag: 2,
            respawn_on_enter: false,
            fire_min_killable: 3,
            can_fire_up: false,
            max_ammo: 0,
            ammo_refill: 50,
            initial_ammo: 0,
            breakable_walls: true,
            breakable_walls_life: 1,
            breakable_walls_breaking: 23,
            breakable_walls_broken: 0,
            activate_scripting: false,
            max_flags: 32,
            scripting_key: 0,
            enable_extern_code: false,
            enable_fire_zone: false,
            script_page: 3,
            timer_enable: true,
            timer_initial: 50,
            timer_refill: 10,
            timer_lapse: 32,
            timer_start: false,
            timer_script_0: false,
            timer_behavior: 0,
            timer_warp: false,
            timer_warp_to_screen: 0,
            timer_warp_to_x: 1,
            timer_warp_to_y: 1,
            timer_auto_reset: false,
            show_timer_over: false,
            enable_checkpoints: false,
            cp_reset_when_dying: false,
            cp_reset_also_flags: false,
        }
    }
}
