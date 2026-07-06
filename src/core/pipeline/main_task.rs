// src/core/pipeline/main_task.rs
use super::{BuildContext, PipelineError, TaskStatus};
use crate::models::ProjectData;
use std::fs::File;
use std::io::Write;

pub fn generate_multilevel_main(
    project: &ProjectData,
    ctx: &BuildContext,
) -> Result<TaskStatus, PipelineError> {
    if project.levels.is_empty() {
        return Ok(TaskStatus::Skipped(
            "Список уровней пуст, создание главного Си-диспетчера пропущено.".to_string(),
        ));
    }

    let mut c_code = String::new();

    // 1. Формируем промышленный заголовок файла
    c_code.push_str("// MTE MK1 (La Churrera) - Главный Си-диспетчер мультилевела\n");
    c_code.push_str("// ВНИМАНИЕ: Файл создан автоматически в IDE ZXStudio, ручные правки будут стёрты.\n\n");

    c_code.push_str("#define MULTILEVEL_GAME\n");
    c_code.push_str(&format!("#define MAX_LEVELS {}\n\n", project.levels.len()));

    c_code.push_str("#include \"config.h\"\n\n");

    // 2. Циклически подключаем Си-заголовки сгенерированных ресурсов для каждого уровня
    c_code.push_str("// --- ПОДКЛЮЧЕНИЕ РЕСУРСОВ ВСЕХ УРОВНЕЙ КАМПАНИИ ---\n");
    for (level_idx, level_data) in project.levels.iter().enumerate() {
        c_code.push_str(&format!("// Уровень {}: {}\n", level_idx, level_data.name));
        c_code.push_str(&format!("#include \"map/map{}.h\"\n", level_idx));
        c_code.push_str(&format!("#include \"dev/enems{}.h\"\n\n", level_idx));
    }

    // 3. Строим аппаратные массивы указателей Z80 для динамического переключения миров
    c_code.push_str("// --- ТАБЛИЦЫ СМЕЩЕНИЯ УКАЗАТЕЛЕЙ ПАМЯТИ Z80 ---\n");
    
    // Массив указателей на карты комнат
    c_code.push_str("unsigned char *levels_maps[] = {\n");
    for i in 0..project.levels.len() {
        c_code.push_str(&format!("\tmapa_level_{},\n", i));
    }
    if c_code.ends_with(",\n") { c_code.truncate(c_code.len() - 2); c_code.push_str("\n"); }
    c_code.push_str("};\n\n");

    // Массив указателей на структуры врагов (baddies)
    c_code.push_str("MALOTE *levels_malotes[] = {\n");
    for i in 0..project.levels.len() {
        c_code.push_str(&format!("\tmalotes_level_{},\n", i));
    }
    if c_code.ends_with(",\n") { c_code.truncate(c_code.len() - 2); c_code.push_str("\n"); }
    c_code.push_str("};\n\n");

    // Массив указателей на хотспоты предметов
    c_code.push_str("HOTSPOT *levels_hotspots[] = {\n");
    for i in 0..project.levels.len() {
        c_code.push_str(&format!("\thotspots_level_{},\n", i));
    }
    if c_code.ends_with(",\n") { c_code.truncate(c_code.len() - 2); c_code.push_str("\n"); }
    c_code.push_str("};\n\n");

    // 4. Формируем главный игровой цикл Спектрума для последовательного прохождения кампании
    c_code.push_str("// --- ТОЧКА ВХОДА И ОРКЕСТРАЦИЯ КАМПАНИИ ДВИЖКА ---\n");
    c_code.push_str("void main (void) {\n");
    c_code.push_str("\tunsigned char current_level = 0;\n\n");
    c_code.push_str("\t// Инициализация оборудования, экрана и звукового чипа AY\n");
    c_code.push_str("\tsetup_spectrum_hardware();\n\n");
    c_code.push_str("\twhile (current_level < MAX_LEVELS) {\n");
    c_code.push_str("\t\t// Динамическая подмена указателей перед инициализацией уровня движком\n");
    c_code.push_str("\t\tp_mapa = levels_maps[current_level];\n");
    c_code.push_str("\t\tp_malotes = levels_malotes[current_level];\n");
    c_code.push_str("\t\tp_hotspots = levels_hotspots[current_level];\n\n");
    c_code.push_str("\t\t// Запуск основного ядра игровой сессии La Churrera\n");
    c_code.push_str("\t\trun_game_level();\n\n");
    c_code.push_str("\t\t// Проверка триггера завершения уровня (через макрос NEXT_LEVEL)\n");
    c_code.push_str("\t\tif (level_cleared_flag) {\n");
    c_code.push_str("\t\t\tcurrent_level++; // Переход на следующий этап кампании\n");
    c_code.push_str("\t\t} else {\n");
    c_code.push_str("\t\t\tbreak; // Игрок потратил все жизни — Game Over\n");
    c_code.push_str("\t\t}\n");
    c_code.push_str("\t}\n\n");
    c_code.push_str("\t// Финал игры: запуск финальной заставки или возврат в меню\n");
    c_code.push_str("\tif (current_level == MAX_LEVELS) {\n");
    c_code.push_str("\t\tshow_victory_screen();\n");
    c_code.push_str("\t} else {\n");
    c_code.push_str("\t\tshow_game_over_screen();\n");
    c_code.push_str("\t}\n");
    c_code.push_str("}\n");

    // Выжигаем churromain.c строго в корень проекта, откуда его заберёт Z88DK
    let output_file_path = ctx.project_path.join("churromain.c");
    let mut file = File::create(&output_file_path)?;
    file.write_all(c_code.as_bytes())?;

    Ok(TaskStatus::Success(format!(
        "Сгенерирован мультилевельный Си-диспетчер 'churromain.c'. Залинковано уровней: {}.",
        project.levels.len()
    )))
}
