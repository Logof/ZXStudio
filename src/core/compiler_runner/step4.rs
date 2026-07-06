// src/core/compiler_runner/step4.rs
use std::path::Path;
use std::sync::mpsc::Sender;

/// ШАГ 4/5: Сканирование и адаптивная подготовка экрана заставки (loading.scr)
pub fn run_loading_screen_step(base_path: &Path, log_tx: &Sender<String>) -> bool {
    let _ = log_tx.send("👉 Шаг 4/5: Подготовка загрузочного экрана заставки (loading.scr)...".to_string());
    let bin_dir = base_path.join("bin");
    let loading_scr_path = bin_dir.join("loading.scr");

    if !loading_scr_path.exists() {
        // Нативно выпекаем Spectrum-эквивалент экрана (6912 байт)
        let mut dummy_scr = vec![0u8; 6144]; // Черные пиксели фона
        dummy_scr.resize(6144 + 768, 7);     // Заполняем атрибуты цветом 7 (White Ink / Black Paper)

        match std::fs::write(&loading_scr_path, &dummy_scr) {
            Ok(_) => {
                let _ = log_tx.send("   [Placeholder] Создан пустой Spectrum-экран заставки bin/loading.scr.".to_string());
            }
            Err(e) => {
                let _ = log_tx.send(format!("❌ Ошибка инициализации плейсхолдера экрана: {}", e));
                return false;
            }
        }
    } else {
        let _ = log_tx.send("   [OK] Обнаружен нативный экран заставки bin/loading.scr.".to_string());
    }
    true
}
