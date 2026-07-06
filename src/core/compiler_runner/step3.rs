// src/core/compiler_runner/step3.rs
use std::path::Path;
use std::sync::mpsc::Sender;

/// ШАГ 3/5: НАТИВНЫЙ токенизатор Sinclair BASIC и сборщик блока loader.tap (Заменяет утилиту bas2tap)
pub fn run_bas2tap_step(base_path: &Path, log_tx: &Sender<String>) -> bool {
    let _ = log_tx.send("👉 Шаг 3/5: Нативная токенизация Sinclair BASIC и сборка загрузчика...".to_string());
    let bin_dir = base_path.join("bin");

    // Формируем токенизированное тело программы Sinclair BASIC прямо в памяти Rust:
    // Строка 10: REM ZXStudio Auto Loader
    // Строка 20: CLEAR 24199
    // Строка 30: LOAD "" CODE
    // Строка 40: RANDOMIZE USR 24200
    let mut basic_program_bytes = Vec::new();

    // --- Строка 10 ---
    basic_program_bytes.extend_from_slice(&[0, 10]); // Номер строки: 10 (High, Low)
    let line_10_body = b"\xEA ZXStudio Auto Loader\r"; // Токен 0xEA = REM
    basic_program_bytes.extend_from_slice(&(line_10_body.len() as u16).to_le_bytes());
    basic_program_bytes.extend_from_slice(line_10_body);

    // --- Строка 20 ---
    basic_program_bytes.extend_from_slice(&[0, 20]); // Номер строки: 20
    // Токен 0xFD = CLEAR. Числа в Sinclair BASIC дублируются в виде текстовой строки 
    // и специального 5-байтного бинарного формата с маркером 0x0E.
    let line_20_body = b"\xFD24199\x0E\x00\x00\x87\x5E\x00\r"; 
    basic_program_bytes.extend_from_slice(&(line_20_body.len() as u16).to_le_bytes());
    basic_program_bytes.extend_from_slice(line_20_body);

    // --- Строка 30 ---
    basic_program_bytes.extend_from_slice(&[0, 30]); // Номер строки: 30
    let line_30_body = b"\xEF\"\" \xAF\r"; // Токен 0xEF = LOAD, Токен 0xAF = CODE
    basic_program_bytes.extend_from_slice(&(line_30_body.len() as u16).to_le_bytes());
    basic_program_bytes.extend_from_slice(line_30_body);

    // --- Строка 40 ---
    basic_program_bytes.extend_from_slice(&[0, 40]); // Номер строки: 40
    let line_40_body = b"\xF9 \xC024200\x0E\x00\x00\x88\x5E\x00\r"; // Токен 0xF9 = RANDOMIZE, 0xC0 = USR
    basic_program_bytes.extend_from_slice(&(line_40_body.len() as u16).to_le_bytes());
    basic_program_bytes.extend_from_slice(line_40_body);

    // --- СБОРКА TAP КОНТЕЙНЕРА В ПАМЯТИ ---
    let mut tap_bytes = Vec::new();

    // 1. Формируем 19-байтный TAP-заголовок бейсик-программы (Программный блок)
    let mut header_block: [u8; 21] = [
        19, 0,  // Длина блока заголовка (19 байт)
        0,      // Флаг типа: 0 = Заголовок ленты
        0,      // Тип данных: 0 = Program (BASIC)
        'L' as u8, 'O' as u8, 'A' as u8, 'D' as u8, 'E' as u8, 'R' as u8, ' ' as u8, ' ' as u8, ' ' as u8, ' ' as u8, // Имя файла (10 байт)
        0, 0,   // Длина данных программы BASIC (запишем ниже)
        10, 0,  // Автостарт со строки 10 (леле-байт формат)
        0, 0,   // Длина области переменных (0 байт)
        0,      // Байт контрольной суммы (checksum)
    ];

    let prog_len = basic_program_bytes.len() as u16;
    header_block[14..16].copy_from_slice(&prog_len.to_le_bytes());
    header_block[18..20].copy_from_slice(&prog_len.to_le_bytes());

    // Считаем XOR контрольную сумму заголовка (начиная со значащего флага типа) [INDEX]
    let mut checksum_hdr = 0u8;
    for b in &header_block[2..20] { checksum_hdr ^= b; }
    header_block[20] = checksum_hdr;
    tap_bytes.extend_from_slice(&header_block);

    // 2. Формируем TAP Data-блок тела BASIC-программы
    let data_len_with_flag_and_chk = prog_len + 2; // +1 байт флага (255) + 1 байт контрольной суммы
    tap_bytes.extend_from_slice(&data_len_with_flag_and_chk.to_le_bytes());
    tap_bytes.push(255); // Маркер блока данных (0xFF)
    tap_bytes.extend_from_slice(&basic_program_bytes);

    // Считаем XOR контрольную сумму блока данных [INDEX]
    let mut checksum_data = 255u8;
    for b in &basic_program_bytes { checksum_data ^= b; }
    tap_bytes.push(checksum_data);

    // Записываем собранный нативным методом TAP-файл на диск [INDEX]
    let output_file_path = bin_dir.join("loader.tap");
    if let Err(e) = std::fs::write(&output_file_path, &tap_bytes) {
        let _ = log_tx.send(format!("❌ Ошибка записи loader.tap: {}", e));
        return false;
    }

    let _ = log_tx.send(format!("   [OK] Нативно токенизирован Sinclair BASIC. Создан загрузчик ({} байт).", tap_bytes.len()));
    true
}
