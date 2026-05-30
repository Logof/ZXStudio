// src/core/pipeline/gfx_task.rs
use super::{BuildContext, PipelineError, TaskStatus};
use crate::core::utils::png2scr::convert_png_to_scr;
// ============================================================================
// ИСПРАВЛЕНО: Добавлен импорт нативного ядра ts2bin из утилит проекта
// ============================================================================
use crate::core::utils::ts2bin::compile_tileset_to_bin;
use crate::models::ProjectData;
use std::path::PathBuf;

struct GfxAsset {
    name: &'static str,
    src: PathBuf,
    dst: PathBuf,
}

pub fn process_graphics(
    project: &ProjectData, // Ссылка теперь активно используется для извлечения font_data
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

    // ============================================================================
    // ИСПРАВЛЕНО: Интеграция упаковочного таска ts.bin.
    // Конвейер берёт gfx/work.png и кастомный шрифт из памяти проекта и собирает
    // финальный монолит графики по правилам оригинального CLI-инструментария.
    // ============================================================================
    let work_png_path = assets_dir.join("work.png");
    let output_ts_bin_path = ctx.output_gfx_path.join("ts.bin");

    let mut ts_bin_generated = false;
    if work_png_path.exists() {
        // Запускаем наше нативное Rust-ядро конвертера (default_ink = -1)
        match compile_tileset_to_bin(&project.font_data, &work_png_path, &output_ts_bin_path, -1) {
            Ok(()) => ts_bin_generated = true,
            Err(e) => warnings.push(format!("work.png (ts2bin error: {})", e)),
        }
    } else {
        warnings.push(
            "work.png (Файл тайлсета отсутствует в gfx/, сборка ts.bin пропущена)".to_string(),
        );
    }

    // Интеллектуальное формирование отчета в консоль с учетом ts.bin
    if !warnings.is_empty() && success_count == 0 && !ts_bin_generated {
        return Ok(TaskStatus::Warning(format!(
            "Ошибки сборки графики: {}",
            warnings.join(", ")
        )));
    }

    let mut success_msg = format!("Собрано экранов-заставок: {}/2. ", success_count);

    if ts_bin_generated {
        success_msg.push_str("Бинарник тайлсета и шрифта ts.bin успешно упакован!");
    } else {
        success_msg.push_str("Внимание: ts.bin не был сгенерирован.");
    }

    if success_count > 0 || ts_bin_generated {
        Ok(TaskStatus::Success(success_msg))
    } else {
        Ok(TaskStatus::Skipped(
            "Графические ресурсы отсутствуют в gfx/, шаг пропущен".to_string(),
        ))
    }
}
