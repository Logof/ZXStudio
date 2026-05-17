use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub enum EngineViewMode {
    #[default]
    SideView, // Платформер (боковой обзор, активна гравитация)
    TopView,  // Вид сверху (свободное перемещение по 4/8 направлениям)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PhysicsConfig {
    pub player_life_ini: u32,
    pub max_bullets: u32,
    pub hud_life_x: u32,
    pub hud_life_y: u32,

    pub enemies_life_gauge: u8,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            player_life_ini: 5,
            max_bullets: 3,
            hud_life_x: 1,
            hud_life_y: 21,

            enemies_life_gauge: 1,
        }
    }
}
