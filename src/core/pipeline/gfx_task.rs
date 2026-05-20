// src/core/pipeline/gfx_task.rs
use super::{BuildContext, PipelineError, TaskStatus};
use crate::core::utils::png2scr::convert_png_to_scr;
use crate::models::ProjectData;
use std::path::PathBuf;

struct GfxAsset {
    name: &'static str,
    src: PathBuf,
    dst: PathBuf,
}

pub fn process_graphics(
    _project: &ProjectData,
    ctx: &BuildContext,
) -> Result<TaskStatus, PipelineError> {
    let assets_dir = ctx.project_path.join("gfx");

    // Декларативно описываем все заставки проекта. Сюда легко добавлять новые файлы.
    let assets = [
        GfxAsset {
            name: "loading.png",
            src: assets_dir.join("loading.png"),
            dst: ctx.output_gfx_path.join("loading.scr"),
        },
        GfxAsset {
            name: "title.png",
            src: assets_dir.join("title.png"),
            dst: ctx.output_gfx_path.join("title.scr"),
        },
    ];

    let mut success_count = 0;
    let mut warnings = Vec::new();

    // Единый конвейер обработки файлов без дублирования условий
    for asset in assets.iter().filter(|a| a.src.exists()) {
        match convert_png_to_scr(&asset.src, &asset.dst, 0) {
            Ok(()) => success_count += 1,
            Err(e) => warnings.push(format!("{} ({})", asset.name, e)),
        }
    }

    // Интеллектуальное формирование отчета в консоль
    if !warnings.is_empty() {
        return Ok(TaskStatus::Warning(format!(
            "Успешно: {}/2. Ошибки: {}",
            success_count,
            warnings.join(", ")
        )));
    }

    match success_count {
        2 => Ok(TaskStatus::Success(
            "Все заставки (loading.scr и title.scr) успешно пересобраны!".to_string(),
        )),
        1 => Ok(TaskStatus::Success(
            "Собрана одна доступная заставка проекта".to_string(),
        )),
        _ => Ok(TaskStatus::Skipped(
            "Файлы заставок в assets/ отсутствуют, шаг пропущен".to_string(),
        )),
    }
}
