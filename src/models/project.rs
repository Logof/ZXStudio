use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::screen::ScreenData;
use super::config::{PhysicsConfig, EngineViewMode};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub map_w: usize,
    pub map_h: usize,
    pub scr_inicio: usize,
    pub view_mode: EngineViewMode,
    pub is_128k: bool,
    pub config: PhysicsConfig,
    pub screens: HashMap<String, ScreenData>,
}

impl Default for ProjectData {
    fn default() -> Self {
        let mut screens = HashMap::new();
        screens.insert("screen_0".to_string(), ScreenData::default());

        Self {
            map_w: 4,
            map_h: 4,
            scr_inicio: 0,
            view_mode: EngineViewMode::SideView,
            is_128k: false,
            config: PhysicsConfig::default(),
            screens,
        }
    }
}
