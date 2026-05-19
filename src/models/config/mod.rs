pub mod general;
pub mod engine;
pub mod hud;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PhysicsConfig {
    pub general: general::GeneralConfig,
    pub engine: engine::EngineConfig,
    pub hud: hud::HudConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum EngineViewMode {
    SideView,
    TopView,
}
