// src/core/pipeline/mod.rs
pub mod gfx_task;
pub mod map_task;
pub mod script_task;
pub mod sprite_task;
pub mod main_task;

use crate::models::ProjectData;
use std::path::{Path, PathBuf};

/// Возможные ошибки в процессе сборки ресурсов
#[derive(Debug)]
pub enum PipelineError {
    IoError(std::io::Error),
    ValidationError(String),
    ExportError(String),
}

impl From<std::io::Error> for PipelineError {
    fn from(err: std::io::Error) -> Self {
        PipelineError::IoError(err)
    }
}

/// Статус выполнения отдельной задачи
pub enum TaskStatus {
    Success(String),
    Warning(String),
    Skipped(String),
}

/// Контекст сборки, содержащий пути и настройки окружения Mojon Twins
pub struct BuildContext {
    pub project_path: PathBuf,
    pub output_dev_path: PathBuf, // Обычно папка /dev/ внутри проекта
    pub output_gfx_path: PathBuf, // Папка /gfx/ для графики
}

impl BuildContext {
    pub fn new(project_path: &str) -> Result<Self, PipelineError> {
        if project_path.is_empty() {
            return Err(PipelineError::ValidationError(
                "Путь к проекту не указан!".to_string(),
            ));
        }

        let base = Path::new(project_path).to_path_buf();
        let dev = base.join("dev");
        let gfx = base.join("gfx");

        Ok(Self {
            project_path: base,
            output_dev_path: dev,
            output_gfx_path: gfx,
        })
    }
}

/// Главная точка входа мастера ресурсов
pub fn execute_resource_pipeline(
    project: &ProjectData,
    project_path: &str,
) -> Result<Vec<String>, PipelineError> {
    let ctx = BuildContext::new(project_path)?;
    let mut logs = Vec::new();

    logs.push("🚀 Запуск промышленного конвейера сборки ресурсов...".to_string());

    // Шаг 1: Экспорт карты мира (15x10 тайловые матрицы)
    match map_task::export_map_data(project, &ctx) {
        Ok(TaskStatus::Success(msg)) => logs.push(format!("✅ Карта: {}", msg)),
        Ok(TaskStatus::Warning(msg)) => logs.push(format!("⚠️ Карта (Предупреждение): {}", msg)),
        Ok(TaskStatus::Skipped(msg)) => logs.push(format!("⏭️ Карта пропущена: {}", msg)),
        Err(PipelineError::IoError(e)) => return Err(PipelineError::IoError(e)),
        Err(PipelineError::ValidationError(e)) => return Err(PipelineError::ValidationError(e)),
        Err(PipelineError::ExportError(e)) => return Err(PipelineError::ExportError(format!("Сбой сборки карты: {}", e))),
    }

    // Шаг 2: Экспорт врагов и траекторий ИИ
    match sprite_task::export_enemy_data(project, &ctx) {
        Ok(TaskStatus::Success(msg)) => logs.push(format!("✅ Сущности: {}", msg)),
        Ok(TaskStatus::Warning(msg)) => logs.push(format!("⚠️ Сущности: {}", msg)),
        Ok(TaskStatus::Skipped(msg)) => logs.push(format!("⏭️ Сущности пропущены: {}", msg)),
        Err(PipelineError::IoError(e)) => return Err(PipelineError::IoError(e)),
        Err(PipelineError::ValidationError(e)) => return Err(PipelineError::ValidationError(e)),
        Err(PipelineError::ExportError(e)) => return Err(PipelineError::ExportError(format!("Сбой сборки врагов: {}", e))),
    }

    // Шаг 3: Конвертация графики, заставок и экранов через png2scr
    match gfx_task::process_graphics(project, &ctx) {
        Ok(TaskStatus::Success(msg)) => logs.push(format!("✅ Графика: {}", msg)),
        Ok(TaskStatus::Warning(msg)) => logs.push(format!("⚠️ Графика: {}", msg)),
        Ok(TaskStatus::Skipped(msg)) => logs.push(format!("⏭️ Графика пропущена: {}", msg)),
        Err(PipelineError::IoError(e)) => return Err(PipelineError::IoError(e)),
        Err(PipelineError::ValidationError(e)) => return Err(PipelineError::ValidationError(e)),
        Err(PipelineError::ExportError(e)) => return Err(PipelineError::ExportError(format!("Сбой конвертации графики: {}", e))),
    }

    // Шаг 4: Generation скриптовых триггеров
    match script_task::export_scripts(project, &ctx) {
        Ok(TaskStatus::Success(msg)) => logs.push(format!("✅ Скрипты: {}", msg)),
        Ok(TaskStatus::Warning(msg)) => logs.push(format!("⚠️ Скрипты: {}", msg)),
        Ok(TaskStatus::Skipped(msg)) => logs.push(format!("⏭️ Скрипты пропущены: {}", msg)),
        Err(PipelineError::IoError(e)) => return Err(PipelineError::IoError(e)),
        Err(PipelineError::ValidationError(e)) => return Err(PipelineError::ValidationError(e)),
        Err(PipelineError::ExportError(e)) => return Err(PipelineError::ExportError(format!("Сбой сборки скриптов: {}", e))),
    }

    // 🎯 ШАГ 5: Интеллектуальный контроль генерации churromain.c
    // Вызывает сборку мультилевельного диспетчера только если уровней больше одного (Legacy-совместимость)
    if project.levels.len() > 1 {
        match main_task::generate_multilevel_main(project, &ctx) {
            Ok(TaskStatus::Success(msg)) => logs.push(format!("✅ Диспетчер Си: {}", msg)),
            Ok(TaskStatus::Warning(msg)) => logs.push(format!("⚠️ Диспетчер Си: {}", msg)),
            Ok(TaskStatus::Skipped(msg)) => logs.push(format!("⏭️ Диспетчер Си пропущен: {}", msg)),
            Err(PipelineError::IoError(e)) => return Err(PipelineError::IoError(e)),
            Err(PipelineError::ValidationError(e)) => return Err(PipelineError::ValidationError(e)),
            Err(PipelineError::ExportError(e)) => return Err(PipelineError::ExportError(format!("Сбой генерации Си-файла: {}", e))),
        }
    } else {
        logs.push("⏭️ Диспетчер Си пропущен: проект содержит 1 уровень (Legacy-режим).".to_string());
    }


    logs.push("✨ Конвейер успешно завершил работу. Все ресурсы синхронизированы!".to_string());
    Ok(logs)
}
