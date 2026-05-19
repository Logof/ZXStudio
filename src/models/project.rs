use super::config::PhysicsConfig;
use super::screen::ScreenData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub config: PhysicsConfig,
    pub screens: HashMap<String, ScreenData>,

    // Хранилище ручной активации ролей спец-тайлов в движке (Новое улучшение)
    pub role_pushbox_active: bool,     // Роль для тайла 14
    pub role_lock_active: bool,        // Роль для тайла 15
    pub role_refill_active: bool,      // Роль для тайла 16
    pub role_collectable_active: bool, // Роль для тайла 17
    pub role_key_active: bool,         // Роль для тайла 18
    pub role_alt_bg_active: bool,      // Роль для тайла 19

    pub tile_behaviours: Vec<u8>,
}

impl Default for ProjectData {
    fn default() -> Self {
        let mut screens = HashMap::new();
        screens.insert("screen_0".to_string(), ScreenData::default());

        // Забиваем дефолтный Си-массив поведения, который вы прислали в конце файла:
        // 0 = Walkable, 1 = Kills, 8 = Full Obstacle
        let default_behaviours = vec![
            0, 0, 8, 8, 8, 8, 1, 1, 8, 0, 1, 8, 0, 8, 8,
            8, // Строка 0 (тайлы 0..15)
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, // Строка 1 (тайлы 16..31)
        ];

        Self {
            config: PhysicsConfig::default(),
            screens,
            // По умолчанию роли выключены, геймдизайнер включает их кликом
            role_pushbox_active: false,
            role_lock_active: false,
            role_refill_active: false,
            role_collectable_active: false,
            role_key_active: false,
            role_alt_bg_active: false,

            tile_behaviours: default_behaviours,
        }
    }
}
