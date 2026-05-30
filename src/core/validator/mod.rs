// src/core/validator/mod.rs
pub mod attribute_clash;
pub mod world_collisions;

use serde::{Deserialize, Serialize};

// ============================================================================
// ИСПРАВЛЕНО: Реэкспортируем функцию под старым именем для обратной совместимости
// ============================================================================
pub use attribute_clash::validate_image_colors as validate_attribute_clash;
pub use world_collisions::WorldValidator;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ErrorSeverity {
    Critical,
    Warning,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ClashError {
    pub screen_index: usize,
    pub cell_x: usize,
    pub cell_y: usize,
    pub severity: ErrorSeverity,
    pub message: String,
}
