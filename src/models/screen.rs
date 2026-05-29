use crate::models::project::TileMode;
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
            hotspot: Hotspot {
                type_id: 0,
                x: 0,
                y: 0,
            },
        }
    }
}

impl ScreenData {
    /// Безопасно устанавливает тайл на сетке экрана 15x10 и автоматически
    /// синхронизирует его со служебными сущностями Hotspot (ТЗ: Автовыгрузка в enemies.h)
    pub fn set_tile_and_sync(&mut self, cell_x: u8, cell_y: u8, tile_id: u8, mode: TileMode) {
        let index = (cell_y as usize) * 15 + (cell_x as usize);
        if index >= self.tiles_matrix.len() {
            return;
        }

        // Проверяем, является ли устанавливаемый тайл служебной горячей точкой (Hotspot)
        // Идентификация одинакова как для Packed16 (в секции спецтайлов), так и для Extended48
        match tile_id {
            16 => {
                // Тайл пополнения (Жизнь/Патроны) -> Hotspot Type 2
                self.hotspot = Hotspot {
                    type_id: 2,
                    x: cell_x,
                    y: cell_y,
                };
                // Движок La Churrera рендерит хотспоты аппаратно из enemies.h поверх пустого фона,
                // поэтому на самой карте мы оставляем проходимый пустой тайл 0
                self.tiles_matrix[index] = 0;
            }
            17 => {
                // Квестовый предмет (Сбор лута) -> Hotspot Type 3
                self.hotspot = Hotspot {
                    type_id: 3,
                    x: cell_x,
                    y: cell_y,
                };
                self.tiles_matrix[index] = 0;
            }
            18 => {
                // Ключ от дверей уровня -> Hotspot Type 1
                self.hotspot = Hotspot {
                    type_id: 1,
                    x: cell_x,
                    y: cell_y,
                };
                self.tiles_matrix[index] = 0;
            }
            _ => {
                // Если мы перезаписываем ячейку обычным тайлом (стена, декор, замок 15, ящик 14),
                // и там ранее стояли координаты текущего хотспота — зануляем хотспот данного экрана
                if self.hotspot.x == cell_x && self.hotspot.y == cell_y && self.hotspot.type_id != 0
                {
                    self.hotspot = Hotspot {
                        type_id: 0,
                        x: 0,
                        y: 0,
                    };
                }

                // Записываем стандартный тайл в матрицу карты
                self.tiles_matrix[index] = tile_id;
            }
        }
    }
}

// ВАЖНО: Добавляем поля границ траектории для Си-компилятора (ТЗ Шаг 6 и Шаг 10)
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Enemy {
    // Уникальный ID для точечного выделения и редактирования в UI.
    // serde(default) гарантирует совместимость со старыми файлами без ID.
    #[serde(default = "generate_ui_id")]
    pub id: u64,
    pub type_id: u8, // Тип поведения (1-14)
    pub x: u8,       // Стартовая позиция X на сетке (0..14)
    pub y: u8,       // Стартовая позиция Y на сетке (0..9)
    pub x1: u8,      // Левая/Верхняя граница движения (в тайлах)
    pub x2: u8,      // Правая/Нижняя граница движения (в тайлах)
    pub y1: u8,      // Верхняя граница для Квадраторов
    pub y2: u8,      // Нижняя граница для Квадраторов

    // Скорость движения (шаг в пикселях: 1, 2 или 4)
    // Атрибут default(default_speed) обеспечивает совместимость со старыми файлами
    #[serde(default = "default_speed")]
    pub speed: u8,
    pub sprite_slot: u8, // Индекс графического слота (0..=3), сохраняемый в проект
}

fn default_speed() -> u8 {
    2 // Самая безопасная и стандартная скорость для MTE MK1
}

/// Генератор дефолтных ID для старых сохранений
fn generate_ui_id() -> u64 {
    // Используем простой таймштамп или случайное число для генерации ID в UI
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Hotspot {
    pub type_id: u8,
    pub x: u8,
    pub y: u8,
}
