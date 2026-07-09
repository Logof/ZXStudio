// src/core/compiler_runner/step2.rs
use crate::models::ProjectData;
use crate::core::compressors::ResourceCompressor;
use std::process::Command;
use std::path::Path;
use std::sync::mpsc::Sender;

/// ШАГ 2/5: Сжатие бинарных ресурсов уровня и запуск Си-компилятора Z80 (Z88DK / zcc)
pub fn run_z88dk_step(
    base_path: &Path,
    z88dk_path: &str,
    compile_command: &str,
    is_multilevel: bool,
    project: &ProjectData, // 🔥 Передаем данные проекта для извлечения флага компрессии
    log_tx: &Sender<String>,
) -> bool {
    let _ = log_tx.send("👉 Шаг 2/5: Подготовка ресурсов и запуск компилятора Си Z88DK (zcc)...".to_string());
    
    // --- ИНТЕГРАЦИЯ ЭТАПА 3: Сжатие карты перед компиляцией кодов игры ---
    let map_raw_path = base_path.join("map").join("mapa.bin");
    let map_packed_path = base_path.join("dev").join("mapa.bin.packed");

    if map_raw_path.exists() {
        let _ = log_tx.send("   [Info] Обнаружен исходный файл карты. Запуск компрессора...".to_string());
        if !ResourceCompressor::compress_resource(
            base_path,
            &map_raw_path,
            &map_packed_path,
            project.compression_mode, // Используем выбранный в UI (general/compression_asm.rs) упаковщик
            log_tx,
        ) {
            let _ = log_tx.send("❌ Ошибка сборщика: Не удалось упаковать бинарные ресурсы карты!".to_string());
            return false;
        }
    } else {
        let _ = log_tx.send("   [Warning] Исходный файл map/mapa.bin не найден. Пропуск сжатия карты.".to_string());
    }
    // ---------------------------------------------------------------------

    // Разрезаем строку сборки из настроек IDE по пробелам на исполняемый файл и аргументы
    let parts: Vec<&str> = compile_command.split_whitespace().collect();
    if parts.is_empty() {
        let _ = log_tx.send("❌ Ошибка сборщика: Строка компиляции пуста!".to_string());
        return false;
    }

    // Если в настройках явно прописан путь к Z88DK, привязываемся к нему, иначе ищем zcc глобально в PATH ОС
    let zcc_exe = if !z88dk_path.is_empty() {
        Path::new(z88dk_path).join(parts[0]).to_string_lossy().to_string()
    } else {
        parts[0].to_string()
    };

    let mut zcc_cmd = Command::new(&zcc_exe);
    zcc_cmd.current_dir(base_path);

    // Накатываем аргументы командной строки в Command буфер Rust
    for arg in parts.iter().skip(1) {
        // КРИТИЧЕСКИЙ ФИКС: Если включен мультилевел, на лету заменяем main.c на churromain.c
        if is_multilevel && *arg == "main.c" {
            zcc_cmd.arg("churromain.c");
        } else {
            zcc_cmd.arg(*arg);
        }
    }

    match zcc_cmd.output() {
        Ok(output) => {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let stderr_str = String::from_utf8_lossy(&output.stderr);

            // Транслируем сырой текстовый лог Си-компилятора напрямую на вкладку Console в реальном времени
            if !stdout_str.is_empty() { let _ = log_tx.send(format!("[zcc out] {}", stdout_str)); }
            if !stderr_str.is_empty() { let _ = log_tx.send(format!("[zcc err] {}", stderr_str)); }

            if !output.status.success() {
                let _ = log_tx.send("❌ КРИТИЧЕСКАЯ ОШИБКА: Z88DK прервал компиляцию Си-кода!".to_string());
                return false;
            }
            let _ = log_tx.send("   [OK] Машинный код Z80 успешно скомпилирован в bin/game.bin.".to_string());
            true
        }
        Err(e) => {
            let _ = log_tx.send(format!(
                "❌ Не удалось запустить бинарник Z88DK ({}). Проверьте пути к компилятору в Настройках IDE!",
                e
            ));
            false
        }
    }
}
