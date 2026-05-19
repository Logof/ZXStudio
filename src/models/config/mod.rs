// src/models/config/mod.rs
pub mod general;
pub mod hud_rendering;
pub mod map_goals;
pub mod mechanics_enemies;
pub mod movement_controls;
pub mod player_physics;
pub mod shooting_boxes;
pub mod tile_behaviour;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhysicsConfig {
    pub general: general::GeneralConfig,
    pub map_goals: map_goals::MapGoalsConfig,
    pub player_physics: player_physics::PlayerPhysicsConfig,
    pub mechanics_enemies: mechanics_enemies::MechanicsEnemiesConfig,
    pub shooting_boxes: shooting_boxes::ShootingBoxesConfig,
    pub movement_controls: movement_controls::MovementControlsConfig,
    pub hud_rendering: hud_rendering::HudRenderingConfig,
    pub tile_behaviour: tile_behaviour::TileBehaviourConfig,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            general: general::GeneralConfig::default(),
            map_goals: map_goals::MapGoalsConfig::default(),
            player_physics: player_physics::PlayerPhysicsConfig::default(),
            mechanics_enemies: mechanics_enemies::MechanicsEnemiesConfig::default(),
            shooting_boxes: shooting_boxes::ShootingBoxesConfig::default(),
            movement_controls: movement_controls::MovementControlsConfig::default(),
            hud_rendering: hud_rendering::HudRenderingConfig::default(),
            tile_behaviour: tile_behaviour::TileBehaviourConfig::default(),
        }
    }
}
