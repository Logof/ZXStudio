use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EngineConfig {
    #[serde(default = "default_true")]
    pub bounding_box_8_bottom: bool,
    #[serde(default)]
    pub bounding_box_8_centered: bool,
    #[serde(default = "default_true")]
    pub small_collision: bool,
    #[serde(default)]
    pub player_check_map_boundaries: bool,
    #[serde(default = "default_true")]
    pub direct_to_play: bool,
    #[serde(default)]
    pub deactivate_keys: bool,
    #[serde(default)]
    pub deactivate_objects: bool,
    #[serde(default)]
    pub deactivate_refills: bool,
    #[serde(default = "default_true")]
    pub only_one_object: bool,
    #[serde(default = "default_1")]
    pub object_count_flag: u32,
    #[serde(default)]
    pub reverse_objects_count: bool,
    #[serde(default)]
    pub deactivate_evil_tile: bool,
    #[serde(default = "default_true")]
    pub player_bounces: bool,
    #[serde(default)]
    pub full_bounce: bool,
    #[serde(default)]
    pub slow_drain: bool,
    #[serde(default = "default_true")]
    pub player_flickers: bool,
    #[serde(default)]
    pub map_bottom_kills: bool,
    #[serde(default)]
    pub walls_stop_enemies: bool,
    #[serde(default)]
    pub everything_is_a_wall: bool,
    #[serde(default = "default_2")]
    pub body_count_on_flag: u32,
    #[serde(default)]
    pub disable_platforms: bool,

    // Преследователи, Фанти и Кокосы
    #[serde(default)]
    pub enable_pursuers: bool,
    #[serde(default = "default_7")]
    pub death_count_and: u32,
    #[serde(default = "default_11")]
    pub death_count_add: u32,
    #[serde(default = "default_2")]
    pub pursuers_max_v: u32,
    #[serde(default = "default_3")]
    pub pursuers_base_cell: u32,
    #[serde(default)]
    pub enable_fanties: bool,
    #[serde(default = "default_2")]
    pub fanties_base_cell: u32,
    #[serde(default = "default_104")]
    pub fanties_sight_distance: u32,
    #[serde(default = "default_256")]
    pub fanties_max_v: u32,
    #[serde(default = "default_16")]
    pub fanties_a: u32,
    #[serde(default = "default_10")]
    pub fanties_life_gauge: u32,
    #[serde(default)]
    pub fanties_type_homing: bool,
    #[serde(default)]
    pub enable_orthoshooters: bool,
    #[serde(default = "default_15")]
    pub orthoshooters_freq: u32,
    #[serde(default)]
    pub orthoshooters_base_cell: u32,
    #[serde(default = "default_8")]
    pub cocos_v: u32,

    // Механика Ящиков и Стрельбы
    #[serde(default)]
    pub player_push_boxes: bool,
    #[serde(default)]
    pub fire_to_push: bool,
    #[serde(default)]
    pub enable_pushed_scripting: bool,
    #[serde(default = "default_1")]
    pub moved_tile_flag: u32,
    #[serde(default = "default_2")]
    pub moved_x_flag: u32,
    #[serde(default = "default_3")]
    pub moved_y_flag: u32,
    #[serde(default)]
    pub pushing_action: bool,
    #[serde(default)]
    pub player_can_fire: bool,
    #[serde(default = "default_1")]
    pub player_can_fire_flag: u32,
    #[serde(default = "default_8")]
    pub player_bullet_speed: u32,
    #[serde(default = "default_3")]
    pub max_bullets: u32,
    #[serde(default = "default_6")]
    pub player_bullet_y_offset: u32,
    #[serde(default)]
    pub player_bullet_x_offset: u32,
    #[serde(default = "default_4")]
    pub enemies_life_gauge: u32,
    #[serde(default)]
    pub limited_bullets: bool,
    #[serde(default = "default_4")]
    pub lb_frames: u32,
    #[serde(default = "default_2")]
    pub lb_frames_flag: u32,
    #[serde(default)]
    pub respawn_on_enter: bool,
    #[serde(default = "default_3")]
    pub fire_min_killable: u32,
    #[serde(default)]
    pub can_fire_up: bool,
    #[serde(default = "default_99")]
    pub max_ammo: u32,
    #[serde(default = "default_50")]
    pub ammo_refill: u32,
    #[serde(default)]
    pub initial_ammo: u32,
    #[serde(default)]
    pub breakable_walls: bool,
    #[serde(default = "default_1")]
    pub breakable_walls_life: u32,

    // Скрипты, Таймер, Управление
    #[serde(default = "default_true")]
    pub activate_scripting: bool,
    #[serde(default = "default_32")]
    pub max_flags: u32,
    #[serde(default = "default_script_key")]
    pub scripting_key: String,
    #[serde(default)]
    pub enable_extern_code: bool,
    #[serde(default)]
    pub enable_fire_zone: bool,
    #[serde(default = "default_3")]
    pub script_page: u32,
    #[serde(default)]
    pub timer_enable: bool,
    #[serde(default = "default_99")]
    pub timer_initial: u32,
    #[serde(default = "default_30")]
    pub timer_refill: u32,
    #[serde(default = "default_32")]
    pub timer_lapse: u32,
    #[serde(default)]
    pub timer_start: bool,
    #[serde(default)]
    pub timer_script_0: bool,
    #[serde(default)]
    pub timer_gameover_0: bool,
    #[serde(default)]
    pub timer_kill_0: bool,
    #[serde(default)]
    pub timer_warp_to: u32,
    #[serde(default = "default_1")]
    pub timer_warp_to_x: u32,
    #[serde(default = "default_1")]
    pub timer_warp_to_y: u32,
    #[serde(default)]
    pub timer_auto_reset: bool,
    #[serde(default)]
    pub show_timer_over: bool,
    #[serde(default)]
    pub enable_checkpoints: bool,
    #[serde(default)]
    pub cp_reset_when_dying: bool,
    #[serde(default)]
    pub cp_reset_also_flags: bool,
    #[serde(default = "default_true")]
    pub player_has_jump: bool,
    #[serde(default)]
    pub player_has_jetpac: bool,
    #[serde(default)]
    pub player_bootee: bool,
    #[serde(default)]
    pub player_vkeys: bool,
    #[serde(default)]
    pub player_disable_gravity: bool,
    #[serde(default = "default_true")]
    pub player_steps_on_enemies: bool,
    #[serde(default = "default_3")]
    pub player_min_killable: u32,
    #[serde(default)]
    pub player_step_sound: bool,
    #[serde(default)]
    pub player_disable_default_heng: bool,
    #[serde(default)]
    pub use_two_buttons: bool,
}

fn default_true() -> bool { true }
fn default_1() -> u32 { 1 }
fn default_2() -> u32 { 2 }
fn default_3() -> u32 { 3 }
fn default_4() -> u32 { 4 }
fn default_6() -> u32 { 6 }
fn default_7() -> u32 { 7 }
fn default_8() -> u32 { 8 }
fn default_10() -> u32 { 10 }
fn default_11() -> u32 { 11 }
fn default_15() -> u32 { 15 }
fn default_30() -> u32 { 30 }
fn default_32() -> u32 { 32 }
fn default_50() -> u32 { 50 }
fn default_99() -> u32 { 99 }
fn default_104() -> u32 { 104 }
fn default_256() -> u32 { 256 }
fn default_16() -> u32 { 16 }
fn default_script_key() -> String { "DOWN".to_string() }

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            bounding_box_8_bottom: true,
            bounding_box_8_centered: false,
            small_collision: true,
            player_check_map_boundaries: false,
            direct_to_play: true,
            deactivate_keys: false,
            deactivate_objects: false,
            deactivate_refills: false,
            only_one_object: true,
            object_count_flag: 1,
            reverse_objects_count: false,
            deactivate_evil_tile: false,
            player_bounces: true,
            full_bounce: false,
            slow_drain: false,
            player_flickers: true,
            map_bottom_kills: false,
            walls_stop_enemies: false,
            everything_is_a_wall: false,
            body_count_on_flag: 2,
            disable_platforms: false,
            enable_pursuers: false,
            death_count_and: 7,
            death_count_add: 11,
            pursuers_max_v: 2,
            pursuers_base_cell: 3,
            enable_fanties: false,
            fanties_base_cell: 2,
            fanties_sight_distance: 104,
            fanties_max_v: 256,
            fanties_a: 16,
            fanties_life_gauge: 10,
            fanties_type_homing: false,
            enable_orthoshooters: false,
            orthoshooters_freq: 15,
            orthoshooters_base_cell: 0,
            cocos_v: 8,
            player_push_boxes: false,
            fire_to_push: false,
            enable_pushed_scripting: false,
            moved_tile_flag: 1,
            moved_x_flag: 2,
            moved_y_flag: 3,
            pushing_action: false,
            player_can_fire: false,
            player_can_fire_flag: 1,
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
            max_ammo: 99,
            ammo_refill: 50,
            initial_ammo: 0,
            breakable_walls: false,
            breakable_walls_life: 1,
            activate_scripting: true,
            max_flags: 32,
            scripting_key: "DOWN".to_string(),
            enable_extern_code: false,
            enable_fire_zone: false,
            script_page: 3,
            timer_enable: false,
            timer_initial: 99,
            timer_refill: 30,
            timer_lapse: 32,
            timer_start: false,
            timer_script_0: false,
            timer_gameover_0: false,
            timer_kill_0: false,
            timer_warp_to: 0,
            timer_warp_to_x: 1,
            timer_warp_to_y: 1,
            timer_auto_reset: false,
            show_timer_over: false,
            enable_checkpoints: false,
            cp_reset_when_dying: false,
            cp_reset_also_flags: false,
            player_has_jump: true,
            player_has_jetpac: false,
            player_bootee: false,
            player_vkeys: false,
            player_disable_gravity: false,
            player_steps_on_enemies: true,
            player_min_killable: 3,
            player_step_sound: false,
            player_disable_default_heng: false,
            use_two_buttons: false,
        }
    }
}
