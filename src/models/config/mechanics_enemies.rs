use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MechanicsEnemiesConfig {
    pub player_check_map_boundaries: bool, // #define PLAYER_CHECK_MAP_BOUNDARIES
    pub direct_to_play: bool,              // #define DIRECT_TO_PLAY
    pub deactivate_keys: bool,             // #define DEACTIVATE_KEYS
    pub deactivate_objects: bool,          // #define DEACTIVATE_OBJECTS
    pub deactivate_refills: bool,          // #define DEACTIVATE_REFILLS
    pub only_one_object: bool,             // #define ONLY_ONE_OBJECT
    pub object_count: u32,                 // #define OBJECT_COUNT
    pub reverse_objects_count: bool,       // #define REVERSE_OBJECTS_COUNT
    pub deactivate_evil_tile: bool,        // #define DEACTIVATE_EVIL_TILE
    pub custom_evil_tile_check: bool,      // #define CUSTOM_EVIL_TILE_CHECK
    pub map_bottom_kills: bool,            // #define MAP_BOTTOM_KILLS
    pub walls_stop_enemies: bool,          // #define WALLS_STOP_ENEMIES
    pub everything_is_a_wall: bool,        // #define EVERYTHING_IS_A_WALL
    pub body_count_on: u32,                // #define BODY_COUNT_ON (0 = deactivated)
    pub disable_platforms: bool,           // #define DISABLE_PLATFORMS
    pub custom_lock_clear: bool,           // #define CUSTOM_LOCK_CLEAR

    // Pursuers (Тип 7)
    pub enable_pursuers: bool,                 // #define ENABLE_PURSUERS
    pub death_count_and: u32,                  // #define DEATH_COUNT_AND
    pub death_count_add: u32,                  // #define DEATH_COUNT_ADD
    pub pursuers_max_v: u32,                   // #define PURSUERS_MAX_V
    pub pursuers_base_cell: u32,               // #define PURSUERS_BASE_CELL
    pub pursuers_dont_spawn_in_obstacle: bool, // #define PURSUERS_DONT_SPAWN_IN_OBSTACLE

    // Fanties (Тип 6)
    pub enable_fanties: bool,        // #define ENABLE_FANTIES
    pub fanties_base_cell: u32,      // #define FANTIES_BASE_CELL
    pub fanties_sight_distance: u32, // #define FANTIES_SIGHT_DISTANCE
    pub fanties_max_v: u32,          // #define FANTIES_MAX_V
    pub fanties_a: u32,              // #define FANTIES_A
    pub fanties_life_gauge: u32,     // #define FANTIES_LIFE_GAUGE
    pub fanties_type_homing: bool,   // #define FANTIES_TYPE_HOMING

    // Orthoshooters
    pub enable_orthoshooters: bool,   // #define ENABLE_ORTHOSHOOTERS
    pub orthoshooters_base_cell: u32, // #define ORTHOSHOOTERS_BASE_CELL
}

impl Default for MechanicsEnemiesConfig {
    fn default() -> Self {
        Self {
            player_check_map_boundaries: false,
            direct_to_play: true,
            deactivate_keys: false,
            deactivate_objects: false,
            deactivate_refills: false,
            only_one_object: false,
            object_count: 1,
            reverse_objects_count: true,
            deactivate_evil_tile: false,
            custom_evil_tile_check: false,
            map_bottom_kills: false,
            walls_stop_enemies: true,
            everything_is_a_wall: false,
            body_count_on: 0,
            disable_platforms: false,
            custom_lock_clear: false,
            enable_pursuers: false,
            death_count_and: 7,
            death_count_add: 11,
            pursuers_max_v: 2,
            pursuers_base_cell: 3,
            pursuers_dont_spawn_in_obstacle: false,
            enable_fanties: false,
            fanties_base_cell: 2,
            fanties_sight_distance: 64,
            fanties_max_v: 256,
            fanties_a: 16,
            fanties_life_gauge: 10,
            fanties_type_homing: false,
            enable_orthoshooters: false,
            orthoshooters_base_cell: 0,
        }
    }
}
