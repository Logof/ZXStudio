// src/core/png2scr.rs
use image::{GenericImageView, Pixel, RgbaImage};
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

#[derive(Clone, Copy, Default)]
struct ColourPair {
    c1: i32,
    c2: i32,
}

/// Перевод RGBA цвета в стандартный 7-битовый индекс цвета ZX Spectrum (FB-код)
fn speccy_colour(r: u8, g: u8, b: u8) -> u8 {
    let mut res = 0;

    // Спектрумовский бит Bright (повышенная яркость), если хотя бы один канал >= 220
    let is_bright = r >= 220 || g >= 220 || b >= 220;

    if r >= 128 {
        res |= 2;
    } // Биты GRB в Spectrum: Green(4), Red(2), Blue(1)
    if g >= 128 {
        res |= 4;
    }
    if b >= 128 {
        res |= 1;
    }

    if is_bright {
        res |= 64; // 64 = 6-й бит яркости (Bright 1)
    }

    res
}

/// Главная функция конвертации PNG в SCR (стандарт Spectrum 256x192)
/// thirds: 0 - полная конвертация (bitmap + attrs), 1..3 - только указанное количество третей экрана
pub fn convert_png_to_scr<P: AsRef<Path>>(png_path: P, scr_path: P, thirds: u8) -> Result<()> {
    // 1. Загружаем PNG изображение
    let img = image::open(png_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
        .to_rgba8();

    let (width, height) = img.dimensions();
    if width != 256 || height != 192 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Изображение должно быть строго размера 256x192 пикселей!",
        ));
    }

    let tc = if thirds == 0 { 3 } else { thirds.min(3) };
    let mut scr_out = vec![0u8; 6912];
    let mut cp_buff = vec![vec![ColourPair::default(); 32]; 24];

    // Вспомогательные переменные для алгоритма подбора цвета (как в оригинале)
    let mut pc1 = 99;
    let mut pc2 = 99;

    // 2. Анализируем знакоместа 8x8 и разрешаем Attribute Clash
    for y in 0..24 {
        for x in 0..31 {
            let p1 = img.get_pixel(x * 8, y * 8).to_rgba();
            let mut c1 = speccy_colour(p1[0], p1[1], p1[2]) as i32;
            let mut c2 = c1;

            // Ищем второй доминирующий цвет в блоке 8x8
            'outer: for i in 0..8 {
                for j in 0..8 {
                    let p2 = img.get_pixel(x * 8 + i, y * 8 + j).to_rgba();
                    c2 = speccy_colour(p2[0], p2[1], p2[2]) as i32;
                    if c2 != c1 {
                        break 'outer; // Нашли несовпадение цветов
                    }
                }
            }

            // Если блок одноцветный, подбираем оптимальную пару PAPER/INK
            if c1 == c2 {
                if pc2 == c2 {
                    c1 = pc1;
                } else if (c1 & 63) < 4 {
                    c1 = 7;
                    if (c2 & 64) != 0 {
                        c1 += 64;
                    }
                } else {
                    c1 = 0;
                    if (c2 & 64) != 0 {
                        c1 += 64;
                    }
                }
            }

            // Сортируем цвета по яркости (в Spectrum цвет с большим индексом идет в INK)
            if (c1 & 63) < (c2 & 63) {
                std::mem::swap(&mut c1, &mut c2);
            }

            cp_buff[y as usize][x as usize] = ColourPair { c1, c2 };
            pc1 = c1;
            pc2 = c2;
        }
    }

    // 3. Формируем Bitmap-слой (Рендеринг в хитрой структуре Спектрума: 3 трети, линии, строки)
    let mut idx = 0;
    for third in 0..3 {
        for line_in_char in 0..8 {
            for char_line in 0..8 {
                for column in 0..32 {
                    let mut byte_val = 0u8;
                    let pixel_x = 8 * column;
                    let pixel_y = third * 64 + line_in_char + char_line * 8;

                    let c1 = cp_buff[(pixel_y / 8) as usize][(pixel_x / 8) as usize].c1;

                    // Собираем 8 пикселей строки в 1 байт маски
                    for i in 0..8 {
                        let p = img.get_pixel(pixel_x + i, pixel_y).to_rgba();
                        let current_col = speccy_colour(p[0], p[1], p[2]) as i32;

                        if current_col == c1 {
                            byte_val |= 1 << (7 - i);
                        }
                    }
                    scr_out[idx] = byte_val;
                    idx += 1;
                }
            }
        }
    }

    // 4. Формируем Атрибутный слой (PAPER, INK, FLASH, BRIGHT) — строго 768 байт в конце файла
    if thirds == 0 {
        for y in 0..24 {
            for x in 0..32 {
                let c1 = cp_buff[y][x].c1;
                let c2 = cp_buff[y][x].c2;

                let b = if (c1 & 64) != 0 || (c2 & 64) != 0 {
                    1
                } else {
                    0
                };
                let pure_c1 = c1 & 63;
                let pure_c2 = c2 & 63;

                // Байт атрибута: Bit 7: Flash, Bit 6: Bright, Bits 3..5: Paper, Bits 0..2: Ink
                let attr_byte = (pure_c1 + 8 * pure_c2 + 64 * b) as u8;
                scr_out[idx] = attr_byte;
                idx += 1;
            }
        }
    }

    // 5. Запись бинарного SCR файла на диск
    let num_bytes = if thirds == 0 {
        6912 // Полный экран (6144 байт графики + 768 байт атрибутов)
    } else {
        2048 * (tc as usize) // Только графические трети (по 2048 байт каждая)
    };

    let mut file = File::create(scr_path)?;
    file.write_all(&scr_out[0..num_bytes])?;

    Ok(())
}
