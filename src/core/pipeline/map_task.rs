// src/core/pipeline/map_task.rs
use super::{BuildContext, PipelineError, TaskStatus};
use crate::models::{screen::ScreenData, ProjectData};
use std::fs::File;
use std::io::Write;

pub fn export_map_data(
    project: &ProjectData,
    ctx: &BuildContext,
) -> Result<TaskStatus, PipelineError> {
    let map_w = project.config.map_goals.map_w as usize;
    let map_h = project.config.map_goals.map_h as usize;

    if map_w == 0 || map_h == 0 {
        return Ok(TaskStatus::Skipped(
            "Размеры карты равны нулю, шаг экспорта пропущен.".to_string(),
        ));
    }

    if project.levels.is_empty() {
        return Ok(TaskStatus::Skipped(
            "Массив уровней пуст, шаг экспорта пропущен.".to_string(),
        ));
    }

    let total_screens = map_w * map_h;
    let is_multilevel = project.levels.len() > 1;

    // Создаем выходную директорию map/, если её ещё нет на диске
    let map_dir = ctx.project_path.join("map");
    if !map_dir.exists() {
        std::fs::create_dir_all(&map_dir)?;
    }

    let mut total_processed_screens = 0;

    // Циклически проходим по уровням
    for (level_idx, level_data) in project.levels.iter().enumerate() {
        // Если это не мультилевел-проект, и мы зашли на итерацию выше 0 — выходим
        if !is_multilevel && level_idx > 0 {
            break;
        }

        let mut map_cpp_code = String::new();

        // Формируем заголовок Си-файла в зависимости от режима
        if is_multilevel {
            map_cpp_code.push_str(&format!(
                "// MTE MK1 (La Churrera) - Карта комнат (Уровень {}: {})\n",
                level_idx, level_data.name
            ));
        } else {
            map_cpp_code.push_str("// MTE MK1 (La Churrera) - Классическая монолитная карта мира\n");
        }
        map_cpp_code.push_str("// ВНИМАНИЕ: Файл создан автоматически в IDE, ручные правки будут стёрты.\n\n");

        map_cpp_code.push_str(&format!("#define MAP_W {}\n", map_w));
        map_cpp_code.push_str(&format!("#define MAP_H {}\n", map_h));
        map_cpp_code.push_str(&format!("#define TOTAL_SCREENS {}\n\n", total_screens));

        // Выбираем имя массива в зависимости от флага мультилевела
        if is_multilevel {
            map_cpp_code.push_str(&format!("unsigned char mapa_level_{} [] = {{\n", level_idx));
        } else {
            map_cpp_code.push_str("unsigned char mapa [] = {\n");
        }

        // Перебираем все экраны по индексам сетки текущего уровня (0..total_screens)
        for scr_idx in 0..total_screens {
            let scr_key = format!("screen_{}", scr_idx);
            let default_screen = ScreenData::default();
            let screen_data = level_data.screens.get(&scr_key).unwrap_or(&default_screen);

            map_cpp_code.push_str(&format!("\t// --- ЭКРАН {} ---\n\t", scr_idx));

            for y in 0..10 {
                for x in 0..15 {
                    let idx = y * 15 + x;
                    let tile_id = if idx < screen_data.tiles_matrix.len() {
                        screen_data.tiles_matrix[idx]
                    } else {
                        0
                    };

                    map_cpp_code.push_str(&format!("{}, ", tile_id));
                }
                if y < 9 {
                    map_cpp_code.push_str("\n\t");
                }
            }
            map_cpp_code.push_str("\n\n");
            total_processed_screens += 1;
        }

        map_cpp_code.push_str("};\n");

        // Определяем имя выходного файла на диске
        let output_file_path = if is_multilevel {
            map_dir.join(format!("map{}.h", level_idx))
        } else {
            map_dir.join("map.h")
        };

        let mut file = File::create(&output_file_path)?;
        file.write_all(map_cpp_code.as_bytes())?;
    }

    if is_multilevel {
        Ok(TaskStatus::Success(format!(
            "Сгенерированы Си-заголовки map[0..{}].h (Мультилевел-режим).",
            project.levels.len() - 1
        )))
    } else {
        Ok(TaskStatus::Success("Сгенерирован классический монолитный Си-заголовок 'map/map.h'.".to_string()))
    }
}
