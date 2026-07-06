// src/core/pipeline/script_task.rs
use super::{BuildContext, PipelineError, TaskStatus};
use crate::models::ProjectData;
use std::fs::{self, File};
use std::io::Write;

pub fn export_scripts(
    project: &ProjectData,
    ctx: &BuildContext,
) -> Result<TaskStatus, PipelineError> {
    // 1. Формируем путь к рабочей папке скриптов внутри проекта: /script/
    let script_dir = ctx.project_path.join("script");
    
    // Безопасность: если папки script почему-то нет на диске, создаем её
    if !script_dir.exists() {
        fs::create_dir_all(&script_dir)?;
    }

    let output_spt_path = script_dir.join("script.spt");

    // 2. Логика генерации или обновления файла сценария
    let final_script_text = if output_spt_path.exists() {
        // Если сценарий уже был создан или отредактирован пользователем на диске — 
        // мы бережно считываем его текущее состояние для валидации и конвейера
        fs::read_to_string(&output_spt_path).unwrap_or_default()
    } else {
        // Если это первый запуск или папка пуста — генерируем стартовый эталонный 
        // шаблон Churscript, автоматически размечая секции под ВСЕ созданные уровни в GUI
        let mut template = String::new();
        template.push_str("// MTE MK1 (La Churrera) - Автоматически сгенерированный сценарий кампании\n");
        template.push_str("// Добавляйте команды триггеров внутрь блоков BEGIN/END для каждого уровня.\n");
        
        for (idx, level) in project.levels.iter().enumerate() {
            template.push_str(&format!(
                "\nLEVEL {}\n// Настройки триггеров мира для уровня: {}\nBEGIN\n\tON_ENTER\n\tBEGIN\n\t\t// Выполняется при заходе в комнату\n\tEND\nEND\n", 
                idx, level.name
            ));
        }
        
        // Сразу запекаем созданный шаблон на диск
        let mut file = File::create(&output_spt_path)?;
        file.write_all(template.as_bytes())?;
        
        template
    };

    // Подсчитываем статистику: сколько секций LEVEL объявлено в тексте скрипта для логов консоли
    let level_sections_count = final_script_text.matches("LEVEL").count();

    Ok(TaskStatus::Success(format!(
        "Файл сценария 'script/script.spt' успешно синхронизирован ({} байт). Найдено активных секций уровней: {}.",
        final_script_text.len(),
        level_sections_count
    )))
}
