// src/core/compiler_runner/step2.rs
use std::process::Command;
use std::path::Path;
use std::sync::mpsc::Sender;

/// ШАГ 2/5: Запуск компилятора Си Z88DK (zcc) с разбором флагов командной строки
pub fn run_z88dk_step(
    base_path: &Path,
    z88dk_path: &str,
    compile_command: &str,
    is_multilevel: bool,
    log_tx: &Sender<String>,
) -> bool {
    let _ = log_tx.send("👉 Шаг 2/5: Запуск компилятора Си Z88DK (zcc)...".to_string());
    
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
            if !stdout_str.is_empty() { let _ = log_tx.send(format!("zcc out: {}", stdout_str)); }
            if !stderr_str.is_empty() { let _ = log_tx.send(format!("zcc err: {}", stderr_str)); }

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
