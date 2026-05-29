// src/core/pipeline/sprite_task.rs
use super::{BuildContext, PipelineError, TaskStatus};
use crate::models::ProjectData;
use std::fs::File;
use std::io::Write;

pub fn export_enemy_data(
    project: &ProjectData,
    ctx: &BuildContext,
) -> Result<TaskStatus, PipelineError> {
    let map_w = project.config.map_goals.map_w as usize;
    let map_h = project.config.map_goals.map_h as usize;
    let total_screens = map_w * map_h;

    if total_screens == 0 {
        return Ok(TaskStatus::Skipped(
            "Размеры карты равны нулю, экспорт enems.h пропущен".to_string(),
        ));
    }

    let file_path = ctx.output_dev_path.join("enems.h");
    let mut file = File::create(&file_path)?;

    let mut n_enems_types = vec![0; 16]; // Расширим до 16 для поддержки типов 9..14
    let mut n_hotspot_types = vec![0; 8];

    writeln!(file, "// MTE MK1 (la Churrera) v5.0")?;
    writeln!(
        file,
        "// Сгенерировано автоматически с помощью ZX Spectrum Core IDE\n"
    )?;

    writeln!(file, "typedef struct {{")?;
    writeln!(file, "\tunsigned char x, y;")?;
    writeln!(file, "\tunsigned char x1, y1, x2, y2;")?;
    writeln!(file, "\tchar mx, my;")?;
    writeln!(file, "\tchar t;")?;
    writeln!(file, "#ifdef PLAYER_CAN_FIRE")?;
    writeln!(file, "\tunsigned char life;")?;
    writeln!(file, "#endif")?;
    writeln!(file, "}} MALOTE;\n")?;

    writeln!(file, "MALOTE malotes [] = {{")?;

    for scr_idx in 0..total_screens {
        let scr_key = format!("screen_{}", scr_idx);
        writeln!(file, "\t// Screen {}", scr_idx)?;

        let empty_enemies = Vec::new();
        let screen_enemies = project
            .screens
            .get(&scr_key)
            .map(|s| &s.enemies)
            .unwrap_or(&empty_enemies);

        for slot in 0..3 {
            if slot < screen_enemies.len() {
                let enemy = &screen_enemies[slot];

                // Перевод координат ячеек (0..14, 0..9) в пиксели Спектрума (16px на тайл)
                let px_x = enemy.x as u16 * 16;
                let px_y = (enemy.y as u16 * 16);

                // Вычисляем min/max для Си-структуры, чтобы не вывернуть рамки
                let final_x1 = (enemy.x1.min(enemy.x2) as u16).clamp(0, 14) * 16;
                let final_x2 = (enemy.x1.max(enemy.x2) as u16).clamp(0, 14) * 16;
                let final_y1 = ((enemy.y1.min(enemy.y2) as u16).clamp(0, 9) * 16) + 16;
                let final_y2 = ((enemy.y1.max(enemy.y2) as u16).clamp(0, 9) * 16) + 16;

                let mut ai_type = enemy.type_id;
                if ai_type == 0 {
                    ai_type = 1;
                }

                // --- 🎯 ЧЕСТНЫЙ МАТЕМАТИЧЕСКИЙ РАСЧЕТ СТАРТОВЫХ ВЕКТОРОВ СКОРОСТЕЙ mx, my ---
                // Скорость (шаг) зависит от типа ИИ (обычно 1, для быстрых врагов или платформ может быть 2 или 4)
                let base_speed: i8 = match ai_type {
                    3 => 2, // Быстрый тип линейного хода (например)
                    _ => 1, // Дефолтная скорость
                };

                let mut mx: i8 = 0;
                let mut my: i8 = 0;

                let is_diagonal = enemy.x1 != enemy.x2 && enemy.y1 != enemy.y2;

                if is_diagonal {
                    // НАПРАВЛЕНИЕ 1: Диагональный рикошет (баг-фича)
                    // Стартуем векторы в сторону финишной точки P2 (x2, y2) относительно старта P1 (x1, y1)
                    mx = if enemy.x2 >= enemy.x1 {
                        base_speed
                    } else {
                        -base_speed
                    };
                    my = if enemy.y2 >= enemy.y1 {
                        base_speed
                    } else {
                        -base_speed
                    };
                } else {
                    // НАПРАВЛЕНИЕ 2: Классические горизонтальные / вертикальные оси
                    let is_horizontal =
                        enemy.x1 != enemy.x2 || (enemy.y1 == enemy.y2 && enemy.x1 == enemy.x);

                    if is_horizontal && enemy.x1 != enemy.x2 {
                        // Смотрим, куда направлен вектор от STRT (x1) к FNSH (x2)
                        mx = if enemy.x2 >= enemy.x1 {
                            base_speed
                        } else {
                            -base_speed
                        };
                    } else if !is_horizontal && enemy.y1 != enemy.y2 {
                        // Смотрим, куда направлен вектор от STRT (y1) к FNSH (y2)
                        my = if enemy.y2 >= enemy.y1 {
                            base_speed
                        } else {
                            -base_speed
                        };
                    }
                }

                let type_idx = (ai_type as usize).min(15);
                n_enems_types[type_idx] += 1;

                write!(
                    file,
                    " \t{{{}, {}, {}, {}, {}, {}, {}, {}, {}}}",
                    px_x, px_y, final_x1, final_y1, final_x2, final_y2, mx, my, ai_type
                )?;
            } else {
                write!(file, " \t{{0, 0, 0, 0, 0, 0, 0, 0, 0}}")?;
            }

            if scr_idx == total_screens - 1 && slot == 2 {
                writeln!(file)?;
            } else {
                writeln!(file, ",")?;
            }
        }
        if scr_idx < total_screens - 1 {
            writeln!(file)?;
        }
    }
    writeln!(file, "}};\n")?;

    // Формируем дефайны подсчета
    for t in 0..15 {
        if t == 4 || t == 5 || t == 6 || t == 7 || t == 8 {
            continue;
        } // Пропускаем неиспользуемые индексы
        writeln!(file, "#define N_ENEMS_TYPE_{} {}", t, n_enems_types[t])?;
    }
    writeln!(
        file,
        "\n#define BADDIES_COUNT (N_ENEMS_TYPE_1+N_ENEMS_TYPE_2+N_ENEMS_TYPE_3)\n"
    )?;

    // ЧАСТЬ 2: Экспорт Хотспотов
    writeln!(file, "typedef struct {{")?;
    writeln!(file, "\tunsigned char xy, tipo, act;")?;
    writeln!(file, "}} HOTSPOT;\n")?;

    writeln!(file, "HOTSPOT hotspots [] = {{")?;

    for scr_idx in 0..total_screens {
        let scr_key = format!("screen_{}", scr_idx);
        let mut xy_byte = 0u8;
        let mut h_type = 0u8;

        if let Some(screen_data) = project.screens.get(&scr_key) {
            let hotspot = &screen_data.hotspot;
            if hotspot.type_id > 0 {
                xy_byte = (hotspot.y << 4) | (hotspot.x & 0x0F);
                h_type = hotspot.type_id;
                let type_idx = (h_type as usize).min(7);
                n_hotspot_types[type_idx] += 1;
            }
        }

        write!(file, "\t{{{}, {}, 0}}", xy_byte, h_type)?;
        if scr_idx == total_screens - 1 {
            writeln!(file)?;
        } else {
            writeln!(file, ",\n")?;
        }
    }
    writeln!(file, "}};\n")?;

    for (t, count) in n_hotspot_types.iter().enumerate() {
        writeln!(file, "#define N_HOTSPOTS_TYPE_{} {}", t, count)?;
    }

    Ok(TaskStatus::Success(format!(
        "Файл dev/enems.h успешно пересобран. Векторы скоростей mx/my рассчитаны математически."
    )))
}
