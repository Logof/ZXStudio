// src/core/compressors.rs
use crate::models::project::CompressionMode;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::Sender;

/// Системный сервис для упаковки бинарных файлов (карт, тайлсетов, экранов) 
/// встроенными средствами или внешними консольными утилитами ZX7 / ZX0.
pub struct ResourceCompressor;

impl ResourceCompressor {
    /// Принимает путь к исходному бинарнику, тип сжатия и жмет его в целевой файл.
    /// Автоматически подстраивает пути к компрессорам в зависимости от платформы ОС.
    pub fn compress_resource(
        base_path: &Path,
        input_file: &Path,
        output_file: &Path,
        mode: CompressionMode,
        log_tx: &Sender<String>,
    ) -> bool {
        match mode {
            CompressionMode::Appack => {
                let _ = log_tx.send("   [Packer] Использование стандартного Appack/Apultra (Без внешнего сжатия)...".to_string());
                // Движок La Churrera по умолчанию умеет читать сырые/appack данные, 
                // поэтому просто копируем файл, если внешние упаковщики не выбраны.
                if let Err(e) = std::fs::copy(input_file, output_file) {
                    let _ = log_tx.send(format!("❌ Ошибка копирования ресурса: {}", e));
                    return false;
                }
                true
            }
            CompressionMode::Zx7 => {
                let _ = log_tx.send("   [Packer] Запуск оптимального сжатия ZX7 Саукаса...".to_string());
                Self::run_external_compressor("zx7", base_path, input_file, output_file, log_tx)
            }
            CompressionMode::Zx0 => {
                let _ = log_tx.send("   [Packer] Запуск максимального сжатия ZX0 Саукаса...".to_string());
                // Флаг -f форсирует перезапись выходного файла, если он уже существует
                Self::run_external_compressor("zx0", base_path, input_file, output_file, log_tx)
            }
        }
    }

    /// Внутренний шелл-раннер для подпроцессов zx7 и zx0
    fn run_external_compressor(
        tool_name: &str,
        base_path: &Path,
        input_file: &Path,
        output_file: &Path,
        log_tx: &Sender<String>,
    ) -> bool {
        // Проверяем операционную систему для вызова правильного бинарника из внутренней папки bin/
        let exe_name = if cfg!(target_os = "windows") {
            format!("{}.exe", tool_name)
        } else {
            tool_name.to_string()
        };

        // Утилиты лежат во внутреннем тулчейне проекта: bin/compiler/tools/
        let tool_path = base_path.join("bin").join("tools").join(&exe_name);

        if !tool_path.exists() {
            let _ = log_tx.send(format!(
                "❌ Ошибка компрессора: Утилита по пути {:?} не найдена! Проверьте целостность папки bin/tools/.",
                tool_path
            ));
            return false;
        }

        let mut cmd = Command::new(&tool_path);
        cmd.current_dir(base_path);

        // Формируем аргументы: утилиты Саукаса принимают синтаксис [инструмент] [вход] [выход]
        // Для zx0 можно дополнительно передать флаг быстрого сжатия или форсирования
        if tool_name == "zx0" {
            cmd.arg("-f"); // Force overwrite
        }
        
        cmd.arg(input_file);
        cmd.arg(output_file);

        match cmd.output() {
            Ok(output) => {
                if !output.status.success() {
                    let stderr_str = String::from_utf8_lossy(&output.stderr);
                    let _ = log_tx.send(format!("❌ Ошибка упаковщика {}: {}", tool_name, stderr_str));
                    return false;
                }

                // Считаем экономию памяти для вывода красивой статистики в Консоль IDE
                let orig_size = input_file.metadata().map(|m| m.len()).unwrap_or(0);
                let comp_size = output_file.metadata().map(|m| m.len()).unwrap_or(0);
                
                let _ = log_tx.send(format!(
                    "   [OK] Файл успешно упакован через {}. Сжатие: {} байт -> {} байт.",
                    tool_name.to_uppercase(), orig_size, comp_size
                ));
                true
            }
            Err(e) => {
                let _ = log_tx.send(format!("❌ Не удалось запустить процесс компрессии {}: {}", tool_name, e));
                false
            }
        }
    }
}
