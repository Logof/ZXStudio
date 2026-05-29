// src/core/exporter/exporter_enemies.rs
use crate::models::ProjectData;

/// Сборка Си-кода массива malotes, дефайнов количества врагов и массива hotspots (ТЗ: Автовыгрузка в enemies.h)
pub fn build_enemies_source(project: &ProjectData, total_screens: u32) -> String {
    let mut n_enems_type = vec![0; 15]; // Индексы 0..14 под типы ИИ врагов
    let mut body = String::new();

    // Си-структура MALOTE под жесткий стандарт разметки памяти Mojon Twins
    body.push_str("typedef struct {\n");
    body.push_str("\tunsigned char x, y;\n");
    body.push_str("\tunsigned char x1, y1, x2, y2;\n");
    body.push_str("\tsigned char mx, my;\n");
    body.push_str("\tsigned char t;\n");
    body.push_str("} MALOTE;\n\n");

    body.push_str("MALOTE malotes [] = {\n");

    let is_top_down = project.config.movement_controls.player_genital;

    for i in 0..total_screens {
        let scr_key = format!("screen_{}", i);
        body.push_str(&format!("\t// Screen {}\n", i));

        // Получаем врагов текущей комнаты, либо берем пустой вектор
        let mut screen_enemies = match project.screens.get(&scr_key) {
            Some(screen) => screen.enemies.clone(),
            None => Vec::new(),
        };

        // Защитный лимит: в оригинальном движке на экране строго до 3 врагов
        screen_enemies.truncate(3);

        // ФИКС: Дополняем массив пустыми структурами (t = 0),
        // чтобы сохранить фиксированный шаг смещения комнат в памяти Спектрума
        while screen_enemies.len() < 3 {
            screen_enemies.push(crate::models::screen::Enemy {
                id: 0,
                type_id: 0,
                x: 0,
                y: 0,
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                speed: 0,
                sprite_slot: 0,
            });
        }

        for enemy in &screen_enemies {
            // Переводим координаты сетки IDE (тайлы 32х32) в пиксели экрана Спектрума (шаг 32)
            // Движок La Churrera оперирует пиксельными границами для плавного попиксельного сдвига
            let x_px = enemy.x * 32;
            let y_px = enemy.y * 32;

            // Настройка жестких логических привязок по вашему ТЗ:
            // x1, y1 — это всегда место старта сущности на карте
            let x1_px = enemy.x1 * 32;
            let y1_px = enemy.y1 * 32;
            // x2, y2 — это всегда место финиша, куда направлен вектор движения
            let x2_px = enemy.x2 * 32;
            let y2_px = enemy.y2 * 32;

            // Вычисление стартовых векторов смещения mx/my на основе выбранного типа ИИ
            let (mx, my) = if enemy.type_id == 0 {
                (0, 0)
            } else {
                match enemy.type_id {
                    1 => (-1, 0), // Линейный горизонтальный: старт ВЛЕВО
                    2 => (1, 0),  // Линейный горизонтальный: старт ВПРАВО
                    3 => (0, 1),  // Линейный вертикальный: старт ВНИЗ
                    4 => {
                        if is_top_down {
                            (0, 0)
                        }
                        // Преследователь рассчитывает вектор динамически
                        else {
                            (0, -1)
                        } // Платформа-лифт: старт ВВЕРХ
                    }
                    7 => (0, -1),  // Обходчик: старт ВВЕРХ
                    8 => (1, 0),   // Обходчик: старт ВПРАВО
                    9 => (0, 1),   // Обходчик: старт ВНИЗ
                    10 => (-1, 0), // Обходчик: старт ВЛЕВО
                    11 => (1, 0),  // Бродяга: старт ВПРАВО
                    12 => (-1, 0), // Бродяга: старт ВЛЕВО
                    13 => (0, -1), // Бродяга: старт ВВЕРХ
                    14 => (0, 1),  // Бродяга: старт ВНИЗ
                    _ => (0, 0),
                }
            };

            // Скорость движения Attr берется из поля enemy.attr, прописанного в палитре.
            // Если враг пустой (tp=0), скорость принудительно обнуляем
            let current_speed = if enemy.type_id == 0 { 0 } else { enemy.speed };

            // Умножаем стартовые вектора на скорость движения (Attr) для Си-логики шага
            let final_mx = mx * current_speed as i8;
            let final_my = my * current_speed as i8;

            // ФИКС ПОРЯДКА ПОЛЕЙ: Строго подгоняем под typedef struct MALOTE
            // x, y, x1, y1, x2, y2, mx, my, t
            body.push_str(&format!(
                "\t {{ {}, {}, {}, {}, {}, {}, {}, {}, {} }},\n",
                x_px,          // x
                y_px,          // y
                x1_px,         // x1 (Исправлено на позицию старта)
                y1_px,         // y1 (Исправлено на позицию старта)
                x2_px,         // x2 (Куда движется)
                y2_px,         // y2 (Куда движется)
                final_mx,      // mx (Вектор шага по X)
                final_my,      // my (Вектор шага по Y)
                enemy.type_id  // t  (Тип ИИ врага)
            ));

            if (enemy.type_id as usize) < n_enems_type.len() {
                n_enems_type[enemy.type_id as usize] += 1;
            }
        }
    }

    if body.ends_with(",\n") {
        body.truncate(body.len() - 2);
        body.push_str("\n");
    }
    body.push_str("};\n\n");

    // ============================================================================
    // НОВОЕ УЛУЧШЕНИЕ: Автогенерация Си-массива hotspots на базе служебных тайлов
    // ============================================================================
    body.push_str("unsigned char hotspots [] = {\n");

    for i in 0..total_screens {
        let scr_key = format!("screen_{}", i);
        body.push_str(&format!("\t// Screen {}\n", i));

        let (x, y, t) = match project.screens.get(&scr_key) {
            Some(screen) => (screen.hotspot.x, screen.hotspot.y, screen.hotspot.type_id),
            None => (0, 0, 0),
        };

        body.push_str(&format!("\t{}, {}, {},\n", x, y, t));
    }

    if body.ends_with(",\n") {
        body.truncate(body.len() - 2);
        body.push_str("\n");
    }
    body.push_str("};\n\n");

    // Записываем дефайны глобальной статистики ОЗУ
    for (type_id, count) in n_enems_type.iter().enumerate() {
        body.push_str(&format!("#define N_ENEMS_TYPE_{} {}\n", type_id, count));
    }

    // Суммарный лимит BADDIES_COUNT (типы 1 + 2 + 3)
    let baddies_count = n_enems_type[1] + n_enems_type[2] + n_enems_type[3];
    body.push_str(&format!("\n#define BADDIES_COUNT {}\n", baddies_count));

    body
}
