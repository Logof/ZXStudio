// src/core/exporter/exporter_map.rs
use crate::models::project::TileMode;
use crate::models::ProjectData;

/// Генерирует Си-код (массив mapa[]) для интеграции в движок La Churrera через Z88DK.
/// Автоматически применяет 4-битное попарное сжатие ячеек, если в проекте активен экономный режим.
pub fn build_map_source(project: &ProjectData, total_screens: u32) -> String {
    let mut body = String::new();
    let mode = project.tile_mode;

    body.push_str("// MTE MK1 (La Churrera) - Автоматически сгенерированная карта мира\n");
    body.push_str(&format!("// Режим тайлсета: {}\n\n", mode.name()));

    // Добавляем Си-дефайн формата карты для условной компиляции самого движка
    match mode {
        TileMode::Packed16 | TileMode::Packed16WithShadows => {
            body.push_str("#define MAP_FORMAT_16_TILES\n\n");
        }
        TileMode::Extended48 => {
            body.push_str("// #define MAP_FORMAT_16_TILES (Отключено для 48 тайлов)\n\n");
        }
    }

    body.push_str("const unsigned char mapa [] = {\n");

    for i in 0..total_screens {
        let scr_key = format!("screen_{}", i);
        body.push_str(&format!("\t// --- ЭКРАН №{} ---\n", i));

        // ИСПРАВЛЕНО: переводим тип ссылки на срез &[u8].
        // Статический массив байт теперь идеально инициализируется на этапе компиляции без вызова кучи
        let tiles_matrix: &[u8] = match project.screens.get(&scr_key) {
            Some(screen) => &screen.tiles_matrix,
            None => {
                static DUMMY_MATRIX: [u8; 150] = [0; 150];
                &DUMMY_MATRIX
            }
        };

        match mode {
            // ============================================================================
            // АЛГОРИТМ СЖАТИЯ: Упаковка 4 бита на тайл (2 тайла в 1 байт)
            // ============================================================================
            TileMode::Packed16 | TileMode::Packed16WithShadows => {
                body.push_str("\t");

                // Проходим по 150 тайлам парами (всего 75 итераций по 2 ячейки)
                for chunk_idx in 0..75 {
                    let tile_a = tiles_matrix[chunk_idx * 2] & 0x0F; // Левый тайл (старшие 4 бита)
                    let tile_b = tiles_matrix[chunk_idx * 2 + 1] & 0x0F; // Правый тайл (младшие 4 бита)

                    // Побитовое сложение по спецификации Mojon Twins
                    let packed_byte = (tile_a << 4) | tile_b;

                    body.push_str(&format!("0x{:02X}, ", packed_byte));

                    // Для красоты кода в Си-файле делаем перенос строки каждые 15 байт (одна строка экрана Спектрума)
                    if (chunk_idx + 1) % 15 == 0 && chunk_idx < 74 {
                        body.push_str("\n\t");
                    }
                }
                body.push_str("\n");
            }

            // ============================================================================
            // АЛГОРИТМ БЕЗ СЖАТИЯ: Прямая побайтовая выгрузка (1 тайл в 1 байт)
            // ============================================================================
            TileMode::Extended48 => {
                body.push_str("\t");

                // Выгружаем все 150 тайлов как независимые байты
                for (tile_idx, &tile_id) in tiles_matrix.iter().enumerate() {
                    body.push_str(&format!("0x{:02X}, ", tile_id));

                    // Делаем красивый перенос Си-кода каждые 15 тайлов (один горизонтальный ряд на экране)
                    if (tile_idx + 1) % 15 == 0 && tile_idx < 149 {
                        body.push_str("\n\t");
                    }
                }
                body.push_str("\n");
            }
        }
    }

    // Удаляем завершающую запятую и пробел перед закрытием Си-массива
    if body.ends_with(", \n") {
        body.truncate(body.len() - 3);
        body.push_str("\n");
    } else if body.ends_with(", ") {
        body.truncate(body.len() - 2);
    }

    body.push_str("};\n");
    body
}
