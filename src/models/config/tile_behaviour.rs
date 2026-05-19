use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TileBehaviourConfig {
    pub behs: Vec<u8>, // Массив из 48 элементов поведения тайлов
}

impl Default for TileBehaviourConfig {
    fn default() -> Self {
        Self {
            behs: vec![
                0, 8, 8, 0, 1, 8, 0, 1, 1, 1, 1, 8, 8, 8, 8, 10, 0, 0, 0, 8, 8, 8, 24, 24, 8, 8, 8,
                8, 8, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        }
    }
}
