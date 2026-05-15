use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ScreenData {
    pub tiles_matrix: Vec<u8>, // 150 байт (15x10 тайлов)
    pub enemies: Vec<Enemy>,   // Ограничение движка: строго до 3 врагов
    pub hotspot: Hotspot,      // Ограничение движка: строго 1 хотспот
}

impl Default for ScreenData {
    fn default() -> Self {
        Self {
            tiles_matrix: vec![0; 150],
            enemies: Vec::new(),
            hotspot: Hotspot { tp: 0, x: 0, y: 0 },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Enemy {
    pub tp: u8, // Тип поведения (1-14)
    pub x: u8,
    pub y: u8,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Hotspot {
    pub tp: u8, // Тип хотспота (1 - предмет, 2 - ключ, 3 - рефил)
    pub x: u8,
    pub y: u8,
}
