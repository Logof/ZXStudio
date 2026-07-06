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

    if project.levels.is_empty() {
        return Ok(TaskStatus::Skipped(
            "Массив уровней пуст, экспорт enems.h пропущен".to_string(),
        ));
    }

    // Создаем каталог dev/, если он отсутствует на диске
    if !ctx.output_dev_path.exists() {
        std::fs::create_dir_all(&ctx.output_dev_path)?;
    }

    // ИСПРАВЛЕНИЕ ПОД МУЛЬТИЛЕВЕЛ: Проходим циклом по всем существующим уровням проекта
    for (level_idx, level_data) in project.levels.iter().enumerate() {
        let file_path = ctx.output_dev_path.join(format!("enems{}.h", level_idx));
        let mut file = File::create(&file_path)?;

        let mut n_enems_types = vec![0; 16]; // Поддержка расширенных типов ИИ 9..14
        let mut n_hotspot_types = vec![0; 8];

        writeln!(
            file,
            "// MTE MK1 (la Churrera) - Автоматический экспорт сущностей (Уровень {}: {})",
            level_idx, level_data.name
        )?;
        writeln!(
            file,
            "// ВНИМАНИЕ: Файл создан автоматически в IDE, ручные правки будут стёрты.\n"
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

        // Модифицируем название массива под конкретный уровень
        writeln!(file, "MALOTE malotes_level_{} [] = {{", level_idx)?;

        for scr_idx in 0..total_screens {
            let scr_key = format!("screen_{}", scr_idx);
            writeln!(file, "\t// Screen {}", scr_idx)?;

            let empty_enemies = Vec::new();
            let screen_enemies = level_data
                .screens
                .get(&scr_key)
                .map(|s| &s.enemies)
                .unwrap_or(&empty_enemies);

            for slot in 0..3 {
                if slot < screen_enemies.len() {
                    let enemy = &screen_enemies[slot];

                    // Перевод координат сетки (0..14, 0..9) в реальные ретро-пиксели (шаг 16)
                    let px_x = enemy.x as u16 * 16;
                    let px_y = enemy.y as u16 * 16;

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
                    let base_speed: i8 = match ai_type {
                        3 => 2, // Быстрый тип линейного хода
                        _ => 1, // Дефолтная скорость
                    };

                    let mut mx: i8 = 0;
                    let mut my: i8 = 0;

                    let is_diagonal = enemy.x1 != enemy.x2 && enemy.y1 != enemy.y2;

                    if is_diagonal {
                        mx = if enemy.x2 >= enemy.x1 { base_speed } else { -base_speed };
                        my = if enemy.y2 >= enemy.y1 { base_speed } else { -base_speed };
                    } else {
                        let is_horizontal =
                            enemy.x1 != enemy.x2 || (enemy.y1 == enemy.y2 && enemy.x1 == enemy.x);

                        if is_horizontal && enemy.x1 != enemy.x2 {
                            mx = if enemy.x2 >= enemy.x1 { base_speed } else { -base_speed };
                        } else if !is_horizontal && enemy.y1 != enemy.y2 {
                            my = if enemy.y2 >= enemy.y1 { base_speed } else { -base_speed };
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
                    // Если слот пуст — прошиваем безопасную заглушку для Си
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

        let mut baddies_count_expr = String::new();
        let mut first_baddie_type = true;

        for t in 1..15 {
            if t == 5 || t == 7 || t == 8 {
                continue;
            }
            writeln!(file, "#define N_ENEMS_TYPE_{}_LEVEL_{} {}", t, level_idx, n_enems_types[t])?;

            if n_enems_types[t] > 0 || t <= 4 {
                if !first_baddie_type {
                    baddies_count_expr.push_str("+");
                }
                baddies_count_expr.push_str(&format!("N_ENEMS_TYPE_{}_LEVEL_{}", t, level_idx));
                first_baddie_type = false;
            }
        }

        writeln!(file, "\n#define BADDIES_COUNT_LEVEL_{} ({})\n", level_idx, baddies_count_expr)?;

        // ============================================================================
        // ЧАСТЬ 2: Экспорт Хотспотов (Предметов, Ключей, Жизней уровня)
        // ============================================================================
        writeln!(file, "typedef struct {{")?;
        writeln!(file, "\tunsigned char xy, tipo, act;")?;
        writeln!(file, "}} HOTSPOT;\n")?;

        writeln!(file, "HOTSPOT hotspots_level_{} [] = {{", level_idx)?;

        for scr_idx in 0..total_screens {
            let scr_key = format!("screen_{}", scr_idx);
            let mut xy_byte = 0u8;
            let mut h_type = 0u8;

            if let Some(screen_data) = level_data.screens.get(&scr_key) {
                let hotspot = &screen_data.hotspot;
                if hotspot.type_id > 0 {
                    xy_byte = ((hotspot.y & 0x0F) << 4) | (hotspot.x & 0x0F);
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

        // Выгружаем итоговые макросы количества хотспотов для Си-компилятора
        for (t, count) in n_hotspot_types.iter().enumerate() {
            if t == 0 {
                continue;
            }
            writeln!(file, "#define N_HOTSPOTS_TYPE_{}_LEVEL_{} {}", t, level_idx, count)?;
        }
        writeln!(file)?;
    }

    // ============================================================================
    // НОВОЕ УЛУЧШЕНИЕ (ГЛАВА 5): Сборка глобальных Sprites Extra
    // ============================================================================
    let file_path_global = ctx.output_dev_path.join("enems_extra.h");
    let mut file_global = File::create(&file_path_global)?;
    
    let gfx_dir = ctx.project_path.join("gfx");
    let sprites_extra_path = gfx_dir.join("sprites_extra.png");
    let sprites_bullet_path = gfx_dir.join("sprites_bullet.png");

    let mut extra_info_msg = String::new();

    if sprites_extra_path.exists() {
        match crate::core::utils::ts2bin::convert_extra_explosion_to_c_bytes(&sprites_extra_path) {
            Ok(c_code) => {
                file_global.write_all(c_code.as_bytes())?;
                extra_info_msg.push_str("💥 Взрыв: собран из sprites_extra.png. ");
            }
            Err(e) => {
                return Err(PipelineError::ExportError(format!(
                    "Ошибка парсинга sprites_extra.png: {}",
                    e
                )));
            }
        }
    } else {
        writeln!(file_global, "// Дефолтный спрайт взрыва 16x16 (спрайт + маска)")?;
        writeln!(file_global, "unsigned char sprite_expl [] = {{")?;
        writeln!(file_global, "\t0x00,0x00,0x18,0x18,0x3C,0x3C,0x7E,0x7E,0x7E,0x7E,0x3C,0x3C,0x18,0x18,0x00,0x00,")?;
        writeln!(file_global, "\t0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF")?;
        writeln!(file_global, "}};\n")?;
        extra_info_msg.push_str("💥 Взрыв: прошит дефолтный массив. ");
    }

    if sprites_bullet_path.exists() {
        match crate::core::utils::ts2bin::convert_bullet_to_c_bytes(&sprites_bullet_path) {
            Ok(c_code) => {
                file_global.write_all(c_code.as_bytes())?;
                extra_info_msg.push_str("🏹 Пуля: собрана из sprites_bullet.png.");
            }
            Err(e) => {
                return Err(PipelineError::ExportError(format!(
                    "Ошибка парсинга sprites_bullet.png: {}",
                    e
                )));
            }
        }
    } else {
        // Дефолтный ретро-массив пули 8x8 (спрайт + маска)
        writeln!(file_global, "// Дефолтный спрайт пули 8x8 (спрайт + маска)")?;
        writeln!(file_global, "unsigned char sprite_bullet [] = {{")?;
        writeln!(file_global, "\t0x00,0x18,0x3C,0x3C,0x3C,0x3C,0x18,0x00,")?;
        writeln!(file_global, "\t0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF")?;
        writeln!(file_global, "}};\n")?;
        extra_info_msg.push_str("🏹 Пуля: прошит дефолтный массив (шарик).");
    }

    Ok(TaskStatus::Success(format!(
        "Файлы dev/enems[0..{}].h и dev/enems_extra.h успешно созданы. Синхронизировано эффектов: {}",
        project.levels.len() - 1,
        extra_info_msg
    )))
}
