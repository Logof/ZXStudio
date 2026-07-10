// src/core/compiler_runner/step2.rs
use crate::models::ProjectData;
use crate::core::utils::{ts2bin, png2scr};
use crate::core::compressors::ResourceCompressor;
use std::process::Command;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::fs;

/// ШАГ 2/5: Запуск компилятора Си Z88DK с автоматической кроссплатформенной линковкой splib2/cpcrslib
pub fn run_z88dk_step(
    base_path: &Path,
    z88dk_path: &str,
    compile_command: &str,
    is_multilevel: bool,
    project: &ProjectData,
    project_name: &str,
    log_tx: &Sender<String>,
) -> bool {
    let _ = log_tx.send("👉 Шаг 2/5: Генерация бинарных ассетов и запуск Z88DK...".to_string());
    
    let dev_dir = base_path.join("dev");
    let gfx_dir = base_path.join("gfx");
    let game_name = &project_name;

    // ============================================================================
    // 💥 НА ТИВНЫЙ КОНВЕЙЕР РЕСУРСОВ: ВЫПЕКАЕМ ВСЕ .BIN ФАЙЛЫ ИЗ MAKE.BAT СИЛАМИ RUST
    // ============================================================================
    
    // 1. Конвертируем тайлсет: gfx/work.png -> dev/tileset.bin (Замена ts2bin.exe)
    let work_png = gfx_dir.join("work.png");
    let tileset_bin = dev_dir.join("tileset.bin");
    if work_png.exists() {
        let _ = log_tx.send("   [Asset] Нативная сборка тайлсета gfx/work.png -> dev/tileset.bin...".to_string());
        if let Err(e) = ts2bin::convert_tileset_to_bin(&work_png, &tileset_bin, 7) {
            let _ = log_tx.send(format!("❌ Ошибка сборки тайлсета: {}", e));
            return false;
        }
    }

    // 2. Конвертируем экраны заставок (Замена png2scr.exe + appack.exe)
    let screens = vec![("title.png", "title.bin"), ("ending.png", "ending.bin"), ("loading.png", "loading.bin")];
    for (png_name, bin_name) in screens {
        let src_png = gfx_dir.join(png_name);
        let target_bin = dev_dir.join(bin_name);

        if src_png.exists() {
            let _ = log_tx.send(format!("   [Asset] Конвертация и сжатие экрана заставки {}...", png_name));
            let temp_scr = dev_dir.join(format!("{}.scr.tmp", png_name));
            
            // Генерируем 6912-байтный экран Спектрума [INDEX]
            if let Err(e) = png2scr::convert_png_to_scr(&src_png, &temp_scr) {
                let _ = log_tx.send(format!("❌ Ошибка png2scr для {}: {}", png_name, e));
                return false;
            }

            // На лету сжимаем экран выбранным в IDE алгоритмом (ZX7/ZX0/Appack)
            if !ResourceCompressor::compress_resource(&base_path, &temp_scr, &target_bin, project.compression_mode, log_tx) {
                return false;
            }
            let _ = fs::remove_file(temp_scr); // чистим временный .scr
        } else if bin_name == "loading.bin" {
            // Если файла загрузочного экрана нет вообще — делаем пустой плейсхолдер, чтобы компилятор не падал
            let _ = fs::write(&target_bin, vec![0u8; 128]);
        }
    }

    // ============================================================================
    // 🔥 КРИТИЧЕСКИЙ АВТОФИКС СИНТАКСИСА АСCЕМБЛЕРА В ОРИГИНАЛЬНЫХ ФАЙЛАХ MOJON TWINS
    // ============================================================================
    let mainloop_h_path = dev_dir.join("mainloop.h");
    if mainloop_h_path.exists() {
        if let Ok(content) = fs::read_to_string(&mainloop_h_path) {
            // Современный Z88DK не понимает "#(224*64)" и считает знак решетки синтаксической ошибкой
            let fixed = content
                .replace("ld hl, #(224*64)", "ld hl, 224*64")
                .replace("ld hl, #(144*64)", "ld hl, 144*64");
            let _ = fs::write(&mainloop_h_path, fixed);
        }
    }

    // ============================================================================
    // НАСТРОЙКА И ЗАПУСК КОМПИЛЯТОРА СИ (ZCC)
    // ============================================================================
    let bin_output_name = format!("{}.bin", game_name);
    let mut zcc_cmd = Command::new(if !z88dk_path.is_empty() { Path::new(z88dk_path).join("zcc").to_string_lossy().to_string() } else { "zcc".to_string() });
    
    zcc_cmd.current_dir(&dev_dir);

    if !z88dk_path.is_empty() {
        let config_path = Path::new(z88dk_path).parent().unwrap_or(Path::new(z88dk_path))
        .join("usr")
        .join("share")
        .join("z88dk")
        .join("lib")
        .join("config");
        zcc_cmd.env("ZCCCFG", config_path.to_string_lossy().to_string());
    }

    zcc_cmd.args(&["+zx", "-vn", if is_multilevel { "churromain.c" } else { "churromain.c" }, "-o", &bin_output_name, "-lsplib2", "-zorg=24200"]);

    // Подшиваем пути к локальным библиотекам спрайтов splib2/cpcrslib в корне
    let splib2_path = base_path.join("splib2");
    if splib2_path.exists() {
        zcc_cmd.arg(format!("-I{}", splib2_path.to_string_lossy()));
        zcc_cmd.arg(format!("-L{}", splib2_path.to_string_lossy()));
    }

    match zcc_cmd.output() {
        Ok(output) => {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);

            if !stdout_str.is_empty() { let _ = log_tx.send(format!("[zcc out] {}", stdout_str)); }
            if !stderr_str.is_empty() { let _ = log_tx.send(format!("[zcc err] {}", stderr_str)); }

            if !output.status.success() {
                let _ = log_tx.send("❌ ОШИБКА КОМПИЛЯЦИИ: Z88DK прервал сборку!".to_string());
                return false;
            }
            
            let _ = log_tx.send(format!("   [OK] Машинный код успешно скомпилирован в dev/{}.", bin_output_name));
            true
        }
        Err(e) => {
            let _ = log_tx.send(format!("❌ Не удалось запустить Z88DK: {}", e));
            false
        }
    }
}