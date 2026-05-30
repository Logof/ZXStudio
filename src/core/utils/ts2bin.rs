// src/core/utils/ts2bin.rs
use image::RgbImage;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

/// Конвертирует цвет ПК (RGB) в каноничный 3-битный индекс цвета ZX Spectrum (+ бит яркости)
fn rgb_to_speccy_colour(r: u8, g: u8, b: u8) -> u8 {
    let mut res = 0u8;
    // Детекция красного
    if r >= 128 {
        res |= 2;
        if r >= 240 {
            res |= 128;
        } // Взводим технический флаг яркости
    }
    // Детекция зеленого
    if g >= 128 {
        res |= 4;
        if g >= 240 {
            res |= 128;
        }
    }
    // Детекция синего
    if b >= 128 {
        res |= 1;
        if b >= 240 {
            res |= 128;
        }
    }
    res
}

/// Анализирует блок 8x8 пикселей изображения, рассчитывает байт атрибута (цвета)
/// и побитовую маску битмапа по правилам оригинального компилятора Mojon Twins.
fn process_8x8_cell(
    img: &RgbImage,
    x0: u32,
    y0: u32,
    tileset_buffer: &mut [u8; 2304],
    idx: usize,
    default_ink: i32,
) {
    // 1. Считываем цвет самого первого пикселя (дефолтный PAPER)
    let p0 = img.get_pixel(x0, y0);
    let mut c1 = rgb_to_speccy_colour(p0[0], p0[1], p0[2]);
    let mut c2 = c1;

    // Сканируем блок на наличие второго контрастного цвета
    for y in 0..8 {
        for x in 0..8 {
            let px = img.get_pixel(x0 + x, y0 + y);
            let c = rgb_to_speccy_colour(px[0], px[1], px[2]);
            if c != c1 {
                c2 = c;
            }
        }
    }

    // Обработка бита яркости (BRIGHT)
    let mut bright_offset = 0u8;
    if (c1 & 128) != 0 || (c2 & 128) != 0 {
        bright_offset = 64; // Бит 6 в байте атрибута Spectrum
    }

    // Снимаем технические флаги яркости для чистой маски цвета (0..7)
    c1 &= 127;
    c2 &= 127;

    // Если блок монохромный — подставляем дефолтные чернила (Ink)
    if c1 == c2 {
        if default_ink != -1 {
            c1 = default_ink as u8;
        } else if c2 < 4 {
            c1 = 7; // Белые чернила на темном фоне
        } else {
            c1 = 0; // Черные чернила на светлом фоне
        }
    }

    // Каноничное правило: более темный цвет уходит в PAPER (c2), более светлый — в INK (c1)
    if c2 > c1 {
        std::mem::swap(&mut c1, &mut c2);
    }

    // Собираем байт атрибута Spectrum: FLASH(0) + BRIGHT(64) + PAPER(3 бита) + INK(3 бита)
    let attr = bright_offset + (c2 * 8) + c1;

    // Записываем байт атрибута в финальную секцию файла (начиная со смещения 2048)
    if 2048 + idx < 2304 {
        tileset_buffer[2048 + idx] = attr;
    }

    // 2. Строим растровую маску битмапа (8 байт на ячейку)
    let bitmap_start = idx * 8;
    for y in 0..8 {
        let mut row_byte = 0u8;
        for x in 0..8 {
            let px = img.get_pixel(x0 + x, y0 + y);
            let c = rgb_to_speccy_colour(px[0], px[1], px[2]) & 127;
            // Если цвет пикселя совпадает с цветом чернил (INK) — взводим бит!
            if c == c1 {
                row_byte |= 1 << (7 - x);
            }
        }
        // ============================================================================
        // ИСПРАВЛЕНО: Безопасное приведение переменной `y` к типу `usize` перед
        // математическим сложением с `bitmap_start`. Ошибки E0308 и E0277 устранены.
        // ============================================================================
        let current_idx = bitmap_start + (y as usize);
        if current_idx < 2048 {
            tileset_buffer[current_idx] = row_byte;
        }
    }
}

/// Главная промышленная функция сборки тайлсета.
/// Автоматически упаковывает переданные байты шрифта и файл work.png в бинарник ts.bin (2304 байта)
pub fn compile_tileset_to_bin<P: AsRef<Path>>(
    font_data: &[u8],
    work_png_path: P,
    output_bin_path: P,
    default_ink: i32,
) -> Result<()> {
    let mut tileset_buffer = [0u8; 2304];

    // --- ЭТАП 1: Перенос отредактированного шрифта (0..511 байт) ---
    // Копируем первые 64 символа из памяти проекта в начало буфера битмапов
    let font_bytes_to_copy = font_data.len().min(512);
    tileset_buffer[..font_bytes_to_copy].copy_from_slice(&font_data[..font_bytes_to_copy]);

    // Для шрифта прошиваем стандартные белые атрибуты (BRIGHT=64 + PAPER=0 + INK=7 -> 71)
    for idx in 0..64 {
        tileset_buffer[2048 + idx] = 64 + 7;
    }

    // --- ЭТАП 2: Парсинг и квантование файла тайлов work.png (512..2303 байт) ---
    if let Ok(img) = image::open(work_png_path) {
        let rgb_img = img.to_rgb8();
        let mut idx = 64; // Тайлы стартуют строго после 64 символов шрифта

        let mut tile_x = 0u32;
        let mut tile_y = 0u32;

        // Экосистема La Churrera содержит строго 48 метатайлов 16x16
        for _i in 0..48 {
            // Нарезаем метатайл 16x16 на 4 чанка UDG 8x8 по каноничному Z-зигзагу:
            // Чанк 1: Top-Left
            process_8x8_cell(
                &rgb_img,
                tile_x,
                tile_y,
                &mut tileset_buffer,
                idx,
                default_ink,
            );
            idx += 1;

            // Чанк 2: Top-Right
            process_8x8_cell(
                &rgb_img,
                tile_x + 8,
                tile_y,
                &mut tileset_buffer,
                idx,
                default_ink,
            );
            idx += 1;

            // Чанк 3: Bottom-Left
            process_8x8_cell(
                &rgb_img,
                tile_x,
                tile_y + 8,
                &mut tileset_buffer,
                idx,
                default_ink,
            );
            idx += 1;

            // Чанк 4: Bottom-Right
            process_8x8_cell(
                &rgb_img,
                tile_x + 8,
                tile_y + 8,
                &mut tileset_buffer,
                idx,
                default_ink,
            );
            idx += 1;

            // Шагаем по сетке оригинального файла work.png (ширина картинки 256 пикселей, шаг 16)
            tile_x += 16;
            if tile_x >= 256 {
                tile_x = 0;
                tile_y += 16;
            }
        }
    } else {
        // Если файла work.png нет — забиваем оставшуюся часть буфера дефолтными пустыми тайлами стен
        for i in 512..2048 {
            tileset_buffer[i] = 0;
        }
        for i in 2112..2304 {
            tileset_buffer[i] = 71;
        }
    }

    // --- ЭТАП 3: Запись готового монолита на диск ---
    let mut file = File::create(output_bin_path)?;
    file.write_all(&tileset_buffer)?;
    Ok(())
}

/// Конвертирует sprites_extra.png (32x16) в Си-массив для взрыва.
/// Левая часть (16x16) — спрайт, правая часть (16x16) — маска прозрачности.
pub fn convert_extra_explosion_to_c_bytes<P: AsRef<std::path::Path>>(
    path: P,
) -> std::io::Result<String> {
    let img = image::open(path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let rgb_img = img.to_rgb8();
    let mut c_array_str = String::new();

    c_array_str.push_str("unsigned char sprite_expl [] = {\n\t");

    // Выгружаем 16 строк по 2 байта на кадр спрайта (левая половина картинки)
    for y in 0..16 {
        let mut byte_l = 0u8;
        let mut byte_r = 0u8;
        for x in 0..8 {
            if rgb_img.get_pixel(x, y)[0] > 30 {
                byte_l |= 1 << (7 - x);
            }
            if rgb_img.get_pixel(x + 8, y)[0] > 30 {
                byte_r |= 1 << (7 - x);
            }
        }
        c_array_str.push_str(&format!("0x{:02X}, 0x{:02X}, ", byte_l, byte_r));
    }
    c_array_str.push_str("\n\t");

    // Выгружаем 16 строк маски прозрачности (правая половина картинки, X смещен на 16)
    for y in 0..16 {
        let mut mask_l = 0u8;
        let mut mask_r = 0u8;
        for x in 0..8 {
            if rgb_img.get_pixel(x + 16, y)[0] > 30 {
                mask_l |= 1 << (7 - x);
            }
            if rgb_img.get_pixel(x + 24, y)[0] > 30 {
                mask_r |= 1 << (7 - x);
            }
        }
        c_array_str.push_str(&format!("0x{:02X}, 0x{:02X}", mask_l, mask_r));
        if y < 15 {
            c_array_str.push_str(", ");
        }
    }

    c_array_str.push_str("\n};\n\n");
    Ok(c_array_str)
}

/// Конвертирует sprites_bullet.png (16x8) в Си-массив для летящей пули.
/// Левая часть (8x8) — спрайт, правая часть (8x8) — маска прозрачности.
pub fn convert_bullet_to_c_bytes<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    let img = image::open(path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let rgb_img = img.to_rgb8();
    let mut c_array_str = String::new();

    c_array_str.push_str("unsigned char sprite_bullet [] = {\n\t");

    // Выгружаем 8 строк спрайта (1 байт на строку)
    for y in 0..8 {
        let mut byte_val = 0u8;
        for x in 0..8 {
            if rgb_img.get_pixel(x, y)[0] > 30 {
                byte_val |= 1 << (7 - x);
            }
        }
        c_array_str.push_str(&format!("0x{:02X}, ", byte_val));
    }
    c_array_str.push_str("\n\t");

    // Выгружаем 8 строк маски пули (X смещен на 8)
    for y in 0..8 {
        let mut mask_val = 0u8;
        for x in 0..8 {
            if rgb_img.get_pixel(x + 8, y)[0] > 30 {
                mask_val |= 1 << (7 - x);
            }
        }
        c_array_str.push_str(&format!("0x{:02X}", mask_val));
        if y < 7 {
            c_array_str.push_str(", ");
        }
    }

    c_array_str.push_str("\n};\n\n");
    Ok(c_array_str)
}
