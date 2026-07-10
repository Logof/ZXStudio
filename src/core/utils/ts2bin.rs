// src/core/utils/ts2bin.rs
use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::GenericImageView;

/// Нативная замена ts2bin. Конвертирует PNG в ретро-бинарник графики ZX Spectrum.
pub fn convert_tileset_to_bin(png_path: &Path, bin_path: &Path, default_attr: u8) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(png_path)?;
    let (width, height) = img.dimensions();
    
    let mut bitmap_bytes = Vec::new();

    // Нарезаем картинку на тайлы 8x8 пикселей строго слева направо, сверху вниз
    for tile_y in 0..(height / 8) {
        for tile_x in 0..(width / 8) {
            // Кодируем 8 строк пикселей текущего знакоместа
            for y in 0..8 {
                let mut byte_row = 0u8;
                for x in 0..8 {
                    let pixel = img.get_pixel(tile_x * 8 + x, tile_y * 8 + y);
                    // Считаем пиксель "активным" (1), если он не полностью прозрачный и не черный
                    let is_lit = pixel[3] > 0 && (pixel[0] > 10 || pixel[1] > 10 || pixel[2] > 10);
                    if is_lit {
                        byte_row |= 1 << (7 - x);
                    }
                }
                bitmap_bytes.push(byte_row);
            }
        }
    }

    // В MTE MK1 формат .bin для тайлсета содержит сначала все маски пикселей,
    // а затем — массив байт атрибутов цвета (по 1 байту на каждое знакоместо)
    let total_tiles = (width / 8) * (height / 8);
    let mut attr_bytes = vec![default_attr; total_tiles as usize];

    let mut final_bin = Vec::new();
    final_bin.extend_from_slice(&bitmap_bytes);
    final_bin.append(&mut attr_bytes);

    let mut file = File::create(bin_path)?;
    file.write_all(&final_bin)?;
    Ok(())
}