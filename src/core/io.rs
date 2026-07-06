use crate::models::ProjectData;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

// Импортируем наши декомпозированные атомарные Си-экспортеры
use super::exporter::exporter_config::build_and_write_config_h;
use super::exporter::exporter_enemies::build_enemies_source_for_level;
use super::exporter::exporter_map::build_map_source_for_level;

pub fn save_project_json(
    project_path: &str,
    project_name: &str,
    project: &ProjectData,
) -> Result<std::path::PathBuf, std::io::Error> {
    // 1. Формируем абсолютный путь к корню папки игры
    let game_root = Path::new(project_path);

    // Безопасность: если папки проекта физически не существует, создаем её
    if !game_root.exists() {
        fs::create_dir_all(&game_root)?;
    }

    // 2. Имя файла должно строго совпадать с именем проекта + расширение .prj
    let file_name = format!("{}.prj", project_name);
    let final_save_path = game_root.join(file_name);

    // 3. Сериализуем и записываем данные со всеми новыми HUD-полями
    let mut file = fs::File::create(&final_save_path)?;
    let json = serde_json::to_string_pretty(project)?;
    file.write_all(json.as_bytes())?;

    // Возвращаем PathBuf наружу, чтобы IDE могла вывести точный путь в статус-бар
    Ok(final_save_path)
}

/// Чистый диспетчер сборки config.h на основе вызова изолированного сервиса
pub fn export_config_h(project_path: &str, project: &ProjectData) -> Result<(), std::io::Error> {
    build_and_write_config_h(project_path, project)
}

/// Чистый диспетчер сборки enems.h на основе вызова изолированных сущностей с поддержкой мультилевела
pub fn export_enems_h(project_path: &str, project: &ProjectData) -> Result<(), std::io::Error> {
    let game_root = Path::new(project_path);

    // Безопасность: если папки проекта физически не существует, создаем её
    if !game_root.exists() {
        fs::create_dir_all(&game_root)?;
    }

    let total_screens = project.config.map_goals.map_w * project.config.map_goals.map_h;

    // Циклически проходим по всем уровням проекта и генерируем enems0.h, enems1.h и т.д.
    for (level_idx, _) in project.levels.iter().enumerate() {
        let target_path = game_root.join(format!("dev/enems{}.h", level_idx));
        let mut final_source = String::new();
        final_source.push_str(&format!(
            "// MTE MK1 (la Churrera) v4 - Level {}\n// Generated автоматически из декомпозированных модулей Rust IDE\n\n",
            level_idx
        ));

        // Вызываем обновленный экспортер для конкретного уровня
        let enemies_code = build_enemies_source_for_level(project, level_idx, total_screens);
        final_source.push_str(&enemies_code);

        let mut file = fs::File::create(target_path)?;
        file.write_all(final_source.as_bytes())?;
    }

    Ok(())
}

// ============================================================================
// ИСПРАВЛЕНИЕ: Диспетчер автоматической сборки и 4-битной упаковки map0.h, map1.h...
// ============================================================================
pub fn export_map_h(project_path: &str, project: &ProjectData) -> Result<(), std::io::Error> {
    let game_root = Path::new(project_path);

    if !game_root.exists() {
        fs::create_dir_all(&game_root)?;
    }

    let total_screens = project.config.map_goals.map_w * project.config.map_goals.map_h;

    // Циклически проходим по всем уровням проекта и генерируем map0.h, map1.h и т.д.
    for (level_idx, _) in project.levels.iter().enumerate() {
        let target_path = game_root.join(format!("map/map{}.h", level_idx));

        // Вызываем компилятор карты с адаптивной логикой сжатия для конкретного уровня
        let map_code = build_map_source_for_level(project, level_idx, total_screens);

        let mut file = fs::File::create(target_path)?;
        file.write_all(map_code.as_bytes())?;
    }

    Ok(())
}

pub fn load_project_file<P: AsRef<Path>>(path: P) -> Result<ProjectData, String> {
    let mut file = fs::File::open(path).map_err(|e| format!("Не удалось открыть файл: {}", e))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Ошибка чтения потока: {}", e))?;

    let project: ProjectData = serde_json::from_str(&content)
        .map_err(|e| format!("Критическая коллизия структуры JSON: {}", e))?;

    Ok(project)
}

pub fn create_project_structure(
    base_path: &str,
    project_name: &str,
    project_data: &ProjectData,
) -> Result<PathBuf, String> {
    // 1. Формируем путь к корневой папке проекта: [Выбранный_Путь]/[Имя_Игры]
    let root_dir = Path::new(base_path).join(project_name);

    // 2. Создаем корневую папку (если она еще не существует)
    fs::create_dir_all(&root_dir)
        .map_err(|e| format!("Не удалось создать корневую папку проекта: {}", e))?;

    // 3. Разворачиваем эталонное дерево подпапок согласно Промышленной Спецификации
    let sub_folders = ["bin", "dev", "gfx", "map", "mus", "script"];
    for folder in &sub_folders {
        let sub_dir = root_dir.join(folder);
        fs::create_dir_all(&sub_dir)
            .map_err(|e| format!("Не удалось создать подпапку '{}': {}", folder, e))?;
    }

    // 4. Сериализуем текущие настройки ProjectData в структурированную JSON-строку с красивыми отступами
    let json_content = serde_json::to_string_pretty(project_data)
        .map_err(|e| format!("Ошибка сериализации данных проекта: {}", e))?;

    // 5. Запекаем единый проектный файл сохранения в папку project_name.prj
    let prj_file_path = root_dir.join(format!("{}.prj", project_name));
    let mut file = fs::File::create(&prj_file_path).map_err(|e| {
        format!(
            "Не удалось создать файл проекта {}.prj: {}",
            project_name, e
        )
    })?;

    file.write_all(json_content.as_bytes())
        .map_err(|e| format!("Ошибка записи данных в файл {}.prj: {}", project_name, e))?;

    Ok(prj_file_path)
}
