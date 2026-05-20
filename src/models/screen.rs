use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ScreenData {
    pub tiles_matrix: Vec<u8>,
    pub enemies: Vec<Enemy>, // Ограничение движка: строго до 3 врагов (NO_MAX_ENEMS)
    pub hotspot: Hotspot,
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

// ВАЖНО: Добавляем поля границ траектории для Си-компилятора (ТЗ Шаг 6 и Шаг 10)
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Enemy {
    pub tp: u8,          // Тип поведения (1-14)
    pub x: u8,           // Стартовая позиция X на сетке (0..14)
    pub y: u8,           // Стартовая позиция Y на сетке (0..9)
    pub x1: u8,          // Левая/Верхняя граница движения (в тайлах)
    pub x2: u8,          // Правая/Нижняя граница движения (в тайлах)
    pub y1: u8,          // Верхняя граница для Квадраторов
    pub y2: u8,          // Нижняя граница для Квадраторов
    pub sprite_slot: u8, // Индекс графического слота (0..=3), сохраняемый в проект
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Hotspot {
    pub tp: u8,
    pub x: u8,
    pub y: u8,
}
