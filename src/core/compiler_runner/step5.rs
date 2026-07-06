// src/core/compiler_runner/step5.rs
use std::path::Path;
use std::sync::mpsc::Sender;

/// ШАГ 5/5: Нативная побайтовая склейка ретро-ленты и запекание релиза в .tap файл
pub fn run_tape_concat_step(base_path: &Path, log_tx: &Sender<String>) -> bool {
    let _ = log_tx.send("👉 Шаг 5/5: Нативная склейка TAP-блоков и запекание релиза в game.tap...".to_string());
    let bin_dir = base_path.join("bin");
    let loading_scr_path = bin_dir.join("loading.scr");
    let bin_game_path = bin_dir.join("game.bin");

    let mut final_tap_bytes = Vec::new();

    // 1. Читаем Блок 1: Загрузчик BASIC
    let loader_path = bin_dir.join("loader.tap");
    if let Ok(loader_bytes) = std::fs::read(&loader_path) {
        final_tap_bytes.extend_from_slice(&loader_bytes);
    } else {
        let _ = log_tx.send("❌ Ошибка финализации: Не удалось прочитать нативный bin/loader.tap!".to_string());
        return false;
    }

    // 2. Читаем Блок 2: Экран заставки (Упаковываем в TAP-блок заголовка и данных)
    if let Ok(scr_bytes) = std::fs::read(&loading_scr_path) {
        // Нативный TAP-заголовок CODE блока (Имя: "loading   ", Длина: 6912, Адрес: 16384)
        let mut scr_header: [u8; 21] = [
            17, 0,  // Длина блока заголовка (17 байт)
            0,      // Тип блока: 0 = Заголовок ленты
            3,      // Тип данных: 3 = CODE
            b'l', b'o', b'a', b'd', b'i', b'n', b'g', b' ', b' ', b' ', // Имя файла (10 байт)
            0x00, 0x1B, // Длина данных: 6912 леле-байты (0x1B00)
            0x00, 0x40, // Стартовый адрес в ОЗУ экрана: 16384 (0x4000)
            0x00, 0x80, // Параметр 2 (32768)
            0,      // Байт контрольной суммы (checksum)
        ];
        
        let mut checksum_hdr = 0u8;
        for b in &scr_header[2..20] { checksum_hdr ^= b; }
        scr_header[20] = checksum_hdr;
        final_tap_bytes.extend_from_slice(&scr_header);
        
        // TAP Data-блок самого экрана заставки
        let len_bytes = ((scr_bytes.len() + 2) as u16).to_le_bytes();
        final_tap_bytes.extend_from_slice(&len_bytes);
        final_tap_bytes.push(255); // Маркер блока данных (0xFF)
        final_tap_bytes.extend_from_slice(&scr_bytes);
        
        let mut checksum_data = 255u8;
        for b in &scr_bytes { checksum_data ^= b; }
        final_tap_bytes.push(checksum_data);
    }

    // 3. Читаем Блок 3: Машинные коды скомпилированной Си-игры bin/game.bin
    if bin_game_path.exists() {
        if let Ok(game_bytes) = std::fs::read(&bin_game_path) {
            // Нативный TAP-заголовок CODE блока игры (Имя: "game      ", Адрес старта в ОЗУ: 24200)
            let mut game_header: [u8; 21] = [
                17, 0,  // Длина блока заголовка
                0,      // Тип блока: Заголовок
                3,      // Тип данных: CODE
                b'g', b'a', b'm', b'e', b' ', b' ', b' ', b' ', b' ', b' ', // Имя файла
                0, 0,   // Длина данных (пропишем динамически ниже)
                0x88, 0x5E, // Стартовый адрес запуска Си-кода: 24200 (0x5E88)
                0x00, 0x80, // Параметр 2
                0,      // Байт контрольной суммы
            ];
            
            let len_le = (game_bytes.len() as u16).to_le_bytes();
            game_header[12] = len_le[0];
            game_header[13] = len_le[1];

            let mut checksum_hdr = 0u8;
            for b in &game_header[2..20] { checksum_hdr ^= b; }
            game_header[20] = checksum_hdr;
            final_tap_bytes.extend_from_slice(&game_header);

            // TAP Data-блок кодов игры
            let len_bytes = ((game_bytes.len() + 2) as u16).to_le_bytes();
            final_tap_bytes.extend_from_slice(&len_bytes);
            final_tap_bytes.push(255); // Маркер данных
            final_tap_bytes.extend_from_slice(&game_bytes);

            let mut checksum_data = 255u8;
            for b in &game_bytes { checksum_data ^= b; }
            final_tap_bytes.push(checksum_data);
        }
    } else {
        // Если Z88DK настроен на прямой выплюв готового TAP в корень
        let alt_path = base_path.join("game.tap");
        if alt_path.exists() {
            let _ = log_tx.send("   [Info] Обнаружен прямой вывод TAP из компилятора. Финализация...".to_string());
            let _ = log_tx.send("\n🎉 СБОРКА УСПЕШНО ЗАВЕРШЕНА! Итоговый образ запечён на диск.".to_string());
            return true;
        }
        let _ = log_tx.send("❌ Ошибка финализации: Скомпилированный бинарный файл bin/game.bin не найден!".to_string());
        return false;
    }

    // Выпекаем итоговый монолитный TAP-образ в корень папки проекта разработчика
    let release_tap_path = base_path.join("game.tap");
    match std::fs::write(&release_tap_path, &final_tap_bytes) {
        Ok(_) => {
            let _ = log_tx.send("\n🎉 СБОРКА УСПЕШНО ЗАВЕРШЕНА! Итоговый образ запечён на диск.".to_string());
            let _ = log_tx.send(format!("💾 Файл релиза: {:?}", release_tap_path.to_string_lossy()));
            true
        }
        Err(e) => {
            let _ = log_tx.send(format!("❌ Ошибка записи финального TAP образа: {}", e));
            false
        }
    }
}
