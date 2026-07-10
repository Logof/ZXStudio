// src/core/pipeline/gfx_task.rs
use super::{BuildContext, PipelineError, TaskStatus};
use crate::core::utils::png2scr::convert_png_to_scr;
use crate::core::utils::ts2bin::convert_tileset_to_bin;
use crate::models::ProjectData;
use std::path::PathBuf;

struct GfxAsset {
    name: &'static str,
    src: PathBuf,
    dst: PathBuf,
}

pub fn process_graphics(
    project: &ProjectData, // Используется для извлечения font_data
    ctx: &BuildContext,
) -> Result<TaskStatus, PipelineError> {
    let assets_dir = ctx.project_path.join("gfx");

    // Декларативно описываем все заставки проекта
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

    // Единый конвейер обработки экранов (Исправлено: убран лишний аргумент 0)
    for asset in assets.iter().filter(|a| a.src.exists()) {
        match convert_png_to_scr(&asset.src, &asset.dst) {
            Ok(()) => success_count += 1,
            Err(e) => warnings.push(format!("{} ({})", asset.name, e)),
        }
    }

    // ============================================================================
    // ИСПРАВЛЕНО: Сборка оригинального tileset.bin
    // ============================================================================
    let work_png_path = assets_dir.join("work.png");
    
    // Внимание: В MTE MK1 файл должен называться строго tileset.bin для ассемблерных инклюдов
    let output_ts_bin_path = ctx.output_dev_path.join("tileset.bin");

    let mut ts_bin_generated = false;
    if work_png_path.exists() {
        // Передаем правильные параметры: ОЗУ шрифта, PNG, целевой путь и цвет 7 (White Ink)
        match convert_tileset_to_bin(&work_png_path, &output_ts_bin_path, 7) {
            Ok(()) => ts_bin_generated = true,
            Err(e) => warnings.push(format!("work.png (ts2bin error: {})", e)),
        }
    } else {
        warnings.push(
            "work.png (Файл тайлсета отсутствует в gfx/, сборка tileset.bin пропущена)".to_string(),
        );
    }

    // Формирование отчета в консоль
    if !warnings.is_empty() && success_count == 0 && !ts_bin_generated {
        return Ok(TaskStatus::Warning(format!(
            "Ошибки сборки графики: {}",
            warnings.join(", ")
        )));
    }

    let mut success_msg = format!("Собрано экранов-заставок: {}/2. ", success_count);

    if ts_bin_generated {
        success_msg.push_str("Бинарник тайлсета и шрифта tileset.bin успешно упакован!");
    } else {
        success_msg.push_str("Внимание: tileset.bin не был сгенерирован.");
    }

    if success_count > 0 || ts_bin_generated {
        Ok(TaskStatus::Success(success_msg))
    } else {
        Ok(TaskStatus::Skipped(
            "Графические ресурсы отсутствуют в gfx/, шаг пропущен".to_string(),
        ))
    }
}