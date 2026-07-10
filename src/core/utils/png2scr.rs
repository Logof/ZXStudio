// src/core/utils/png2scr.rs
use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::GenericImageView;

/// Нативная замена png2scr. Превращает 256x192 PNG в каноничный Spectrum Screen (6912 байт).
pub fn convert_png_to_scr(png_path: &Path, scr_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(png_path)?;
    
    let mut screen_bytes = vec![0u8; 6912];
    
    // 1. Кодируем пиксели с учетом хитрой интерлиньяжной адресации видеопамяти ZX Spectrum
    for y in 0..192 {
        let block = y / 64;
        let line_inside_block = (y % 64) / 8;
        let row_inside_char = y % 8;
        
        let spectrum_y_offset = (block * 2048) + (row_inside_char * 256) + (line_inside_block * 32);

        for char_x in 0..32 {
            let mut byte_val = 0u8;
            for bit in 0..8 {
                let px = img.get_pixel((char_x * 8 + bit) as u32, y as u32);
                let is_lit = px[3] > 0 && (px[0] > 120 || px[1] > 120 || px[2] > 120);
                if is_lit {
                    byte_val |= 1 << (7 - bit);
                }
            }
            screen_bytes[spectrum_y_offset + char_x] = byte_val;
        }
    }

    // 2. Генерируем карту атрибутов цвета (White Ink, Black Paper, Bright) [INDEX]
    for attr_idx in 0..768 {
        screen_bytes[6144 + attr_idx] = 7 | (1 << 6); // 7 = White, Bit 6 = BRIGHT
    }

    let mut file = File::create(scr_path)?;
    file.write_all(&screen_bytes)?;
    Ok(())
}