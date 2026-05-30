use super::config::PhysicsConfig;
use super::screen::ScreenData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileMode {
    /// Экономный режим: 16 тайлов для карты (упаковка 4 бита), 4 спец-тайла.
    Packed16,
    /// Экономный режим с автоматическим расчетом и отображением теней.
    Packed16WithShadows,
    /// Расширенный режим: 48 тайлов (8 бит), спец-тайлы жестко вшиты в индексы.
    Extended48,
}

impl TileMode {
    pub fn name(&self) -> &'static str {
        match self {
            TileMode::Packed16 => "16 Tiles (Экономный)",
            TileMode::Packed16WithShadows => "16 Tiles + Автошейдинг",
            TileMode::Extended48 => "48 Tiles (Максимальный)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TileMode::Packed16 => {
                "Карта сжимается в 2 раза (4 бита на тайл). Доступны тайлы 0-15 для рисования. \
                 Тайлы 16-19 зарезервированы под спец-объекты (жизнь, предметы, ключи, альт-фон)."
            }
            TileMode::Packed16WithShadows => {
                "Включает экономный режим 16 тайлов и генерирует автоматические тени. \
                 Требует двойную высоту тайлсета (нижний ряд содержит затененные копии)."
            }
            TileMode::Extended48 => {
                "Максимальное разнообразие графики (0-47). Карта занимает в 2 раза больше RAM. \
                 Служебные объекты жестко привязаны к индексам 14-18 внутри палитры."
            }
        }
    }

    /// Возвращает ожидаемые размеры файла tileset (work.png) в пикселях
    pub fn expected_dimensions(&self) -> (u32, u32) {
        match self {
            TileMode::Packed16 => (256, 48),
            TileMode::Packed16WithShadows => (256, 96),
            TileMode::Extended48 => (256, 48),
        }
    }

    /// Максимальный индекс тайла, который пользователь может выбрать для рисования на карте
    pub fn max_paintable_tile_index(&self) -> usize {
        match self {
            TileMode::Packed16 | TileMode::Packed16WithShadows => 15,
            TileMode::Extended48 => 47,
        }
    }

    /// Возвращает необходимый размер массива поведений (tile_behaviours) для этого режима
    pub fn behaviours_count(&self) -> usize {
        match self {
            TileMode::Packed16 | TileMode::Packed16WithShadows => 16, // Только для карты
            TileMode::Extended48 => 48,                               // Для всех доступных тайлов
        }
    }

    /// Генерирует дефолтный вектор поведений на основе выбранного режима
    pub fn default_behaviours(&self) -> Vec<u8> {
        let base_16 = vec![0, 0, 8, 8, 8, 8, 1, 1, 8, 0, 1, 8, 0, 8, 8, 8];

        match self {
            TileMode::Packed16 | TileMode::Packed16WithShadows => base_16,
            TileMode::Extended48 => {
                let mut behaviours = base_16;
                behaviours.resize(48, 0);
                behaviours
            }
        }
    }
}

/// Вспомогательная функция для автоматического восстановления старых JSON сохранений
fn default_tile_mode() -> TileMode {
    TileMode::Packed16
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub config: PhysicsConfig,
    pub screens: HashMap<String, ScreenData>,
    #[serde(default = "default_tile_mode")]
    pub tile_mode: TileMode,

    // Хранилище ручной активации ролей спец-тайлов в движке
    pub role_pushbox_active: bool,     // Роль для тайла 14
    pub role_lock_active: bool,        // Роль для тайла 15
    pub role_refill_active: bool,      // Роль для тайла 16
    pub role_collectable_active: bool, // Роль для тайла 17
    pub role_key_active: bool,         // Роль для тайла 18
    pub role_alt_bg_active: bool,      // Роль для тайла 19

    pub tile_behaviours: Vec<u8>,
    #[serde(default = "default_project_font")]
    pub font_data: Vec<u8>,
}

impl Default for ProjectData {
    fn default() -> Self {
        let mut screens = HashMap::new();
        screens.insert("screen_0".to_string(), ScreenData::default());

        let default_mode = TileMode::Packed16;

        let mut default_font = vec![0u8; 768];

        Self {
            config: PhysicsConfig::default(),
            screens,
            tile_mode: default_mode,

            role_pushbox_active: false,
            role_lock_active: false,
            role_refill_active: false,
            role_collectable_active: false,
            role_key_active: false,
            role_alt_bg_active: false,

            tile_behaviours: default_mode.default_behaviours(),
            font_data: default_project_font(), // Переменная гарантированно видна компилятору!
        }
    }
}

fn default_project_font() -> Vec<u8> {
    let mut default_font = vec![0u8; 768];

    // Подгружаем контуры буквы 'A' для инициализации
    let a_idx = 33 * 8;
    default_font[a_idx + 0] = 0b00011000;
    default_font[a_idx + 1] = 0b00100100;
    default_font[a_idx + 2] = 0b01000010;
    default_font[a_idx + 3] = 0b01111110;
    default_font[a_idx + 4] = 0b01000010;
    default_font[a_idx + 5] = 0b01000010;
    default_font[a_idx + 6] = 0b01000010;
    default_font[a_idx + 7] = 0b00000000;

    // Подгружаем контуры буквы 'B'
    let b_idx = 34 * 8;
    default_font[b_idx + 0] = 0b01111100;
    default_font[b_idx + 1] = 0b01000010;
    default_font[b_idx + 2] = 0b01000010;
    default_font[b_idx + 3] = 0b01111100;
    default_font[b_idx + 4] = 0b01000010;
    default_font[b_idx + 5] = 0b01000010;
    default_font[b_idx + 6] = 0b01111100;
    default_font[b_idx + 7] = 0b00000000;

    default_font
}
