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
    let mut total_processed_screens = 0;

    // Создаем выходную директорию dev/, если её ещё нет на диске
    if !ctx.output_dev_path.exists() {
        std::fs::create_dir_all(&ctx.output_dev_path)?;
    }

    // ИСПРАВЛЕНИЕ ПОД МУЛЬТИЛЕВЕЛ: Циклически выжигаем файлы map0.h, map1.h и т.д. для каждого уровня
    for (level_idx, level_data) in project.levels.iter().enumerate() {
        let mut map_cpp_code = String::new();

        // Формируем заголовок Си-файла в стандартах Mojon Twins с указанием индекса и имени уровня
        map_cpp_code.push_str(&format!(
            "// MTE MK1 (La Churrera) - Автоматически сгенерированная карта комнат (Уровень {}: {})\n",
            level_idx, level_data.name
        ));
        map_cpp_code
            .push_str("// ВНИМАНИЕ: Файл создан автоматически в IDE, ручные правки будут стёрты.\n\n");

        map_cpp_code.push_str(&format!("#define MAP_W {}\n", map_w));
        map_cpp_code.push_str(&format!("#define MAP_H {}\n", map_h));
        map_cpp_code.push_str(&format!("#define TOTAL_SCREENS {}\n\n", total_screens));

        // Модифицируем имя массива, чтобы churromain.c мог адресовать нужный указатель по номеру уровня
        map_cpp_code.push_str(&format!("unsigned char mapa_level_{} [] = {{\n", level_idx));

        // Последовательно перебираем все экраны по индексам сетки текущего уровня (0..total_screens)
        for scr_idx in 0..total_screens {
            let scr_key = format!("screen_{}", scr_idx);

            // Если экрана нет в хэшмапе данного уровня, берем пустой по умолчанию
            let default_screen = ScreenData::default();
            let screen_data = level_data.screens.get(&scr_key).unwrap_or(&default_screen);

            map_cpp_code.push_str(&format!("\t// --- ЭКРАН {} ---\n\t", scr_idx));

            // Выгружаем 150 байт матрицы (15x10)
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
                // Форматируем строки по 15 тайлов для удобного чтения программистом
                if y < 9 {
                    map_cpp_code.push_str("\n\t");
                }
            }

            map_cpp_code.push_str("\n\n");
            total_processed_screens += 1;
        }

        // Закрываем Си-массив
        map_cpp_code.push_str("};\n");

        // Формируем уникальный путь назначения: /dev/map{level_idx}.h
        let output_file_path = ctx.output_dev_path.join(format!("map{}.h", level_idx));

        // Записываем файл на диск
        let mut file = File::create(&output_file_path)?;
        file.write_all(map_cpp_code.as_bytes())?;
    }

    Ok(TaskStatus::Success(format!(
        "Сгенерированы Си-заголовки dev/map[0..{}].h. Успешно упаковано {} уровней (всего {} экранов, {} байт).",
        project.levels.len() - 1,
        project.levels.len(),
        total_processed_screens,
        total_processed_screens * 150
    )))
}
