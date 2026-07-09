// src/core/exporter/exporter_config/mod.rs
pub mod platform_macros;
pub mod mechanics_physics;
pub mod hud_macros;
pub mod movement_physics;

use crate::models::ProjectData;
use chrono::Local;
use std::fs;
use std::path::Path;

/// Главный диспетчер генерации конфигурации dev/config.h
pub fn build_and_write_config_h(
    project_path: &str,
    project: &ProjectData,
) -> Result<(), std::io::Error> {
    let game_root = Path::new(project_path);

    // Безопасность: если папки проекта физически не существует, создаем её
    if !game_root.exists() {
        fs::create_dir_all(&game_root)?;
    }

    let template_path = "templates/config.h.template";
    let target_path = game_root.join("dev/config.h");

    // Читаем сырой шаблон
    let mut output_content = fs::read_to_string(template_path)?;

    // Подставляем метаданные сборки
    let current_date = Local::now().format("%Y-%m-%d").to_string();
    output_content = output_content.replace("{{GENERATION_DATE}}", &current_date);

    // 1. Применяем макросы платформы, памяти и компрессии MK1v4/v4.8
    output_content = platform_macros::process(output_content, project);

    // 2. Применяем настройки игровых механик и логики объектов
    output_content = mechanics_physics::process(output_content, project);

    // 3. Применяем параметры HUD-интерфейса и анимаций знакомест
    output_content = hud_macros::process(output_content, project);

    // 4. Применяем Fixed-Point константы движения и прыжков
    output_content = movement_physics::process(output_content, project);

    // 5. Генерируем и дописываем Си-массив поведения тайлов
    let beh_string = generate_behaviours_array(project);
    output_content.push_str("\n\n// Сгенерировано автоматически с помощью ZXStudio\n");
    output_content.push_str(&beh_string);

    // Финальная бинарная запись на диск
    fs::write(target_path, output_content.into_bytes())?;

    Ok(())
}

/// Сервисный генератор структуры comportamiento_tiles
fn generate_behaviours_array(project: &ProjectData) -> String {
    let mut beh_string = String::from("unsigned char comportamiento_tiles [] = {\n\t");
    for (i, beh) in project.config.tile_behaviour.behs.iter().enumerate() {
        beh_string.push_str(&format!("{}, ", beh));
        if (i + 1) % 16 == 0 && i < 47 {
            beh_string.push_str("\n\t");
        }
    }
    beh_string.push_str("\n};");
    beh_string
}
