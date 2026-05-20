// src/models/enemy_ai.rs
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum EnemyAiType {
    /// Слот заблокирован или пуст. Враг отсутствует на экране.
    #[default]
    EmptyOrDisabled = 0,

    /// Тип 1: Линейный горизонтальный / Рикошет (Старт Влево)
    LinearHorizontalLeft = 1,

    /// Тип 2: Линейный горизонтальный / Рикошет (Старт Вправо)
    LinearHorizontalRight = 2,

    /// Тип 3: Линейный вертикальный / Рикошет (Старт Вверх/Вниз)
    LinearVertical = 3,

    /// Тип 4: Платформа-лифт / Активный Охотник
    PlatformOrPursuer = 4,

    /// Тип 5: Летающий призрак (Случайный спавн на экране)
    GhostRandomRespawn = 5,

    /// Тип 6: Призрак Fanty (Агро-преследование игрока)
    GhostFantyHoming = 6,

    /// Тип 7: Обходчик по стенам / Вокруг препятствий (Старт Вверх)
    QuadratorUp = 7,

    /// Тип 8: Обходчик по стенам / Вокруг препятствий (Старт Вправо)
    QuadratorRight = 8,

    /// Тип 9: Обходчик по стенам / Вокруг препятствий (Старт Вниз)
    QuadratorDown = 9,

    /// Тип 10: Обходчик по стенам / Квадратный маршрут (Старт Влево)
    QuadratorLeftOrLift = 10,

    /// Тип 11: Хаотичный патрульный / Бродяга (Старт Вправо)
    MarrulerRight = 11,

    /// Тип 12: Хаотичный патрульный / Бродяга (Старт Влево)
    MarrulerLeft = 12,

    /// Тип 13: Хаотичный патрульный / Бродяга (Старт Вверх)
    MarrulerUp = 13,

    /// Тип 14: Хаотичный патрульный / Бродяга (Старт Вниз)
    MarrulerDown = 14,
}

impl EnemyAiType {
    /// Конвертация сырого u8 байта из JSON-файла в безопасный типизированный Enum
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::LinearHorizontalLeft,
            2 => Self::LinearHorizontalRight,
            3 => Self::LinearVertical,
            4 => Self::PlatformOrPursuer,
            5 => Self::GhostRandomRespawn,
            6 => Self::GhostFantyHoming,
            7 => Self::QuadratorUp,
            8 => Self::QuadratorRight,
            9 => Self::QuadratorDown,
            10 => Self::QuadratorLeftOrLift,
            11 => Self::MarrulerRight,
            12 => Self::MarrulerLeft,
            13 => Self::MarrulerUp,
            14 => Self::MarrulerDown,
            _ => Self::EmptyOrDisabled,
        }
    }

    /// Превращение в сырой байт для экспорта в enems.h
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    /// Генерирует точное русское техническое имя ИИ в зависимости от жанра игры
    pub fn to_ru_name(self, is_top_down: bool) -> &'static str {
        match self {
            Self::EmptyOrDisabled => "Пустой слот (ИИ Выключен)",
            Self::LinearHorizontalLeft => {
                if is_top_down {
                    "0x01: Рикошет / Диагональный ходок (Старт Влево)"
                } else {
                    "0x01: Линейный по горизонтали (Старт Влево)"
                }
            }
            Self::LinearHorizontalRight => {
                if is_top_down {
                    "0x02: Рикошет / Диагональный ходок (Старт Вправо)"
                } else {
                    "0x02: Линейный по горизонтали (Старт Вправо)"
                }
            }
            Self::LinearVertical => {
                if is_top_down {
                    "0x03: Рикошет / Диагональный ходок (Старт по Y)"
                } else {
                    "0x03: Линейный по вертикали (Вверх/Вниз)"
                }
            }
            Self::PlatformOrPursuer => {
                if is_top_down {
                    "0x04: 🎯 АКТИВНЫЙ ОХОТНИК (Погоня по X/Y)"
                } else {
                    "0x04: 🧗 ДВИЖУЩАЯСЯ ПЛАТФОРМА (Лифт)"
                }
            }
            Self::GhostRandomRespawn => "0x05: Призрак-летун (Случайный спавн на экране)",
            Self::GhostFantyHoming => "0x06: Призрак Fanty (Агро-преследование и возврат)",
            Self::QuadratorUp => {
                if is_top_down {
                    "0x07: Обходчик лабиринта / Вокруг камней (Старт Вверх)"
                } else {
                    "0x07: Пристеночник / По уступам (Старт Вверх)"
                }
            }
            Self::QuadratorRight => {
                if is_top_down {
                    "0x08: Обходчик лабиринта / Вокруг камней (Старт Вправо)"
                } else {
                    "0x08: Пристеночник / По уступам (Старт Вправо)"
                }
            }
            Self::QuadratorDown => {
                if is_top_down {
                    "0x09: Обходчик лабиринта / Вокруг камней (Старт Вниз)"
                } else {
                    "0x09: Пристеночник / По уступам (Старт Вниз)"
                }
            }
            Self::QuadratorLeftOrLift => {
                if is_top_down {
                    "0x0A: Обходчик лабиринта / Вокруг камней (Старт Влево)"
                } else {
                    "0x0A: Пристеночник / Квадратный лифт (Старт Влево)"
                }
            }
            Self::MarrulerRight => "0x0B: Хаотичный патрульный / Бродяга (Старт Вправо)",
            Self::MarrulerLeft => "0x0C: Хаотичный патрульный / Бродяга (Старт Влево)",
            Self::MarrulerUp => "0x0D: Хаотичный патрульный / Бродяга (Старт Вверх)",
            Self::MarrulerDown => "0x0E: Хаотичный патрульный / Бродяга (Старт Вниз)",
        }
    }

    /// Подробное развернутое описание физики ИИ для нижней подсказки в палитре IDE
    pub fn to_ru_description(self, is_top_down: bool) -> &'static str {
        match self {
            Self::EmptyOrDisabled => "Враг полностью удален из памяти данного экрана.",
            Self::LinearHorizontalLeft | Self::LinearHorizontalRight | Self::LinearVertical => {
                if is_top_down { "↔️/↕️ Диагональный рикошет:\nХодит по вектору линии. При ударе о стену отскакивает под прямым углом." }
                else { "↔️/↕️ Линейные:\nХодят строго по прямой. Разворачиваются в конечной точке или при ударе о препятствие." }
            }
            Self::PlatformOrPursuer => {
                if is_top_down { "🎯 Умный Охотник:\nКаждый кадр считывает координаты игрока и преследует его кратчайшим путем." }
                else { "🧗 Платформа-Лифт:\nСлужит твердой опорой для ног игрока. Перемещается по заданной траектории." }
            }
            Self::GhostRandomRespawn => "👻 Налётчик:\nСпавнится за пределами экрана наобум. Летит через комнату сквозь стены и исчезает.",
            Self::GhostFantyHoming => "👻 Базовый страж:\nСпит в точке спавна. Заметив игрока в радиусе обзора, летит сквозь стены за ним. Теряя агро, летит назад.",
            Self::QuadratorUp | Self::QuadratorRight | Self::QuadratorDown | Self::QuadratorLeftOrLift => {
                if is_top_down { "🔄 Обходчик препятствий:\nБесконечно патрулирует внешний периметр прямоугольника вокруг внутренних блоков стен." }
                else { "🔄 Пристеночник-паук:\nПолзает по границам очерченной коробки, удерживаясь за стены, уступы или потолок." }
            }
            Self::MarrulerRight | Self::MarrulerLeft | Self::MarrulerUp | Self::MarrulerDown => {
                "🌀 Хаотичный Бродяга:\nИдет напролом сквозь коридоры. Упираясь лбом в стену, случайно выбирает новый вектор хода."
            }
        }
    }

    // Служебные функции разделения категорий для рендерера холста
    pub fn is_linear(self) -> bool {
        matches!(
            self,
            Self::LinearHorizontalLeft
                | Self::LinearHorizontalRight
                | Self::LinearVertical
                | Self::PlatformOrPursuer
        )
    }
    pub fn is_quadrator(self) -> bool {
        matches!(
            self,
            Self::QuadratorUp
                | Self::QuadratorRight
                | Self::QuadratorDown
                | Self::QuadratorLeftOrLift
        )
    }
    pub fn is_marruler(self) -> bool {
        matches!(
            self,
            Self::MarrulerRight | Self::MarrulerLeft | Self::MarrulerUp | Self::MarrulerDown
        )
    }
    pub fn is_ghost(self) -> bool {
        matches!(self, Self::GhostRandomRespawn | Self::GhostFantyHoming)
    }
}

/// Стандартная реализация Display использует Платформер как базовое представление
impl fmt::Display for EnemyAiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ru_name(false))
    }
}
