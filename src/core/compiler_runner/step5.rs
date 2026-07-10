// src/core/compiler_runner/step5.rs
use std::path::Path;
use std::sync::mpsc::Sender;
use std::fs;

/// ШАГ 5/5: Нативная сборка TAP-контейнера в точном соответствии с make.bat от Mojon Twins
pub fn run_tape_concat_step(base_path: &Path, project_name: &str, log_tx: &Sender<String>) -> bool {
    let _ = log_tx.send("👉 Шаг 5/5: Финальный контейнерный монтаж релиза в game.tap...".to_string());
    
    let dev_dir = base_path.join("dev");
    let loading_bin_path = dev_dir.join("loading.bin");
    let game_bin_path = dev_dir.join(format!("{}.bin", project_name));
    
    let mut final_tap_bytes = Vec::new();

    // 1. БЛОК 1: Подшиваем нативный BASIC-загрузчик (loader.tap) из шага 3
    let loader_path = dev_dir.join("loader.tap");
    if let Ok(loader_bytes) = fs::read(&loader_path) {
        final_tap_bytes.extend_from_slice(&loader_bytes);
    } else {
        let _ = log_tx.send("❌ Ошибка финализации: Нативный файл dev/loader.tap не найден!".to_string());
        return false;
    }

    // 2. БЛОК 2: Эквивалент бинарной склейки заставки: bin2tap.exe -a 32768 loading.bin
    if loading_bin_path.exists() {
        if let Ok(scr_bytes) = fs::read(&loading_bin_path) {
            let mut scr_header = create_code_header("loading", scr_bytes.len() as u16, 32768);
            final_tap_bytes.extend_from_slice(&scr_header);
            
            // Данные заставки
            let len_bytes = ((scr_bytes.len() + 2) as u16).to_le_bytes();
            final_tap_bytes.extend_from_slice(&len_bytes);
            final_tap_bytes.push(255);
            final_tap_bytes.extend_from_slice(&scr_bytes);
            final_tap_bytes.push(calculate_checksum(&scr_bytes, 255));
            let _ = log_tx.send("   [Append] Успешно подшит блок заставки (loading.bin) по адресу 32768.".to_string());
        }
    } else {
        let _ = log_tx.send("⚠️ [Warning] Скомпилированный файл заставки dev/loading.bin отсутствует, пропуск блока.".to_string());
    }

    // 3. БЛОК 3: Эквивалент склейки кода игры: bin2tap.exe -a 24200 cheril.bin
    if game_bin_path.exists() {
        if let Ok(game_bytes) = fs::read(&game_bin_path) {
            let mut game_header = create_code_header(project_name, game_bytes.len() as u16, 24200);
            final_tap_bytes.extend_from_slice(&game_header);

            // Данные игры
            let len_bytes = ((game_bytes.len() + 2) as u16).to_le_bytes();
            final_tap_bytes.extend_from_slice(&len_bytes);
            final_tap_bytes.push(255);
            final_tap_bytes.extend_from_slice(&game_bytes);
            final_tap_bytes.push(calculate_checksum(&game_bytes, 255));
            let _ = log_tx.send(format!("   [Append] Успешно подшит машинный код ({}.bin) по адресу 24200.", project_name));
        }
    } else {
        let _ = log_tx.send(format!("❌ Критическая ошибка: Скомпилированный бинарник игры dev/{}.bin не найден!", project_name));
        return false;
    }

    // Сохраняем финальный .tap-релиз в корень проекта разработки
    let release_tap_path = base_path.join(format!("{}.tap", project_name));
    match fs::write(&release_tap_path, &final_tap_bytes) {
        Ok(_) => {
            let _ = log_tx.send("\n🎉 СБОРКА УСПЕШНО ЗАВЕРШЕНА ПО СПЕЦИФИКАЦИИ MOJON TWINS!".to_string());
            let _ = log_tx.send(format!("💾 Файл релиза: {:?}", release_tap_path.to_string_lossy()));
            true
        }
        Err(e) => {
            let _ = log_tx.send(format!("❌ Ошибка записи финального TAP образа: {}", e));
            false
        }
    }
}

/// Хелпер для создания стандартного 17-байтного Си-заголовка блока CODE
fn create_code_header(name: &str, length: u16, start_addr: u16) -> [u8; 21] {
    let mut header: [u8; 21] = [0; 21];
    header[0..2].copy_from_slice(&17u16.to_le_bytes()); // Длина блока заголовка
    header[2] = 0;  // Флаг: Заголовок
    header[3] = 3;  // Тип: CODE

    // Имя файла (ровно 10 байт, дополняем пробелами)
    let mut name_bytes = [b' '; 10];
    let src_bytes = name.as_bytes();
    let limit = src_bytes.len().min(10);
    name_bytes[0..limit].copy_from_slice(&src_bytes[0..limit]);
    header[4..14].copy_from_slice(&name_bytes);

    header[14..16].copy_from_slice(&length.to_le_bytes());     // Длина данных
    header[18..20].copy_from_slice(&start_addr.to_le_bytes()); // Стартовый адрес загрузки в ОЗУ
    header[16..18].copy_from_slice(&32768u16.to_le_bytes());   // Параметр 2 (обычно 32768)

    // Считаем XOR контрольную сумму заголовка
    let mut chk = 0u8;
    for b in &header[2..20] { chk ^= b; }
    header[20] = chk;

    header
}

fn calculate_checksum(bytes: &[u8], initial: u8) -> u8 {
    let mut chk = initial;
    for b in bytes { chk ^= b; }
    chk
}