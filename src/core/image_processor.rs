use image::{GenericImage, GenericImageView, Rgb, RgbImage};
use std::path::Path;
use eframe::egui;

/// Импорт тайлсета: читает work.png, зануляет тайл 0 (16x16) и сохраняет mappy.png
pub fn process_tileset_for_mappy(source_path: &str, target_path: &str) -> Result<(), image::ImageError> {
    if !Path::new(source_path).exists() {
        return Err(image::ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Файл {} не найден", source_path),
        )));
    }
    let img = image::open(source_path)?;
    let mut rgb_img: RgbImage = img.to_rgb8();
    
    // Зануляем область тайла 0 (координаты X: 0..16, Y: 0..16) в черный цвет
    for y in 0..16 {
        for x in 0..16 {
            rgb_img.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }
    rgb_img.save(target_path)?;
    Ok(())
}

/// Спецификация Mojon Twins: Парсинг интеркалированных масок (Шаг спрайта = 32 пикселя)
pub fn generate_sprite_masks(sprites_path: &str) -> Result<(), image::ImageError> {
    if !Path::new(sprites_path).exists() {
        return Err(image::ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Файл {} не найден", sprites_path),
        )));
    }

    let img = image::open(sprites_path)?;
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    if height != 32 || width != 256 {
        return Err(image::ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Неверный размер sprites.png! Ожидалось 256x32, получено {}x{}", width, height),
        )));
    }

    let mut output_img = rgb_img.clone();

    // Обрабатываем обе строки (Y=0: Игрок, Y=16: Враги/Платформы)
    for row_y in [0, 16] {
        // Шаг цикла = 32 пикселя (16 пикселей спрайт + 16 пикселей маска)
        for block_x in (0..256).step_by(32) {
            let sprite_start_x = block_x;
            let mask_start_x = block_x + 16;

            // Попиксельно сканируем окно спрайта 16x16
            for y in 0..16 {
                for x in 0..16 {
                    let px_x = sprite_start_x + x;
                    let pixel_color = rgb_img.get_pixel(px_x, row_y + y);

                    // Спецификация Mojon Twins: RGB(0,0,0) — пустой фон, остальное — тело объекта
                    let mask_pixel = if pixel_color[0] == 0 && pixel_color[1] == 0 && pixel_color[2] == 0 {
                        Rgb([255, 255, 255]) // Белый цвет маски указывает Спектруму на прозрачность
                    } else {
                        Rgb([0, 0, 0])       // Черный цвет маски вырезает силуэт под наложение
                    };

                    // Записываем сгенерированный пиксель маски в соседнее правое окно 16x16
                    output_img.put_pixel(mask_start_x + x, row_y + y, mask_pixel);
                }
            }
        }
    }

    output_img.save(sprites_path)?;
    Ok(())
}

// ============================================================================
// ДОБАВЛЕНО: Утилита фоновой конвертации для egui видеотекстур
// ============================================================================
pub fn load_png_to_image<P: AsRef<Path>>(path: P) -> Result<egui::ColorImage, String> {
    let img = image::open(path).map_err(|e| format!("Ошибка чтения PNG: {}", e))?;
    let size = [img.width() as usize, img.height() as usize];
    let rgba_pixels = img.to_rgba8().into_raw();
    
    Ok(egui::ColorImage::from_rgba_unmultiplied(size, &rgba_pixels))
}


pub fn load_work_png_to_image_ram<P: AsRef<Path>>(path: P) -> Result<egui::ColorImage, String> {
    // 1. Открываем оригинальный work.png
    let img = image::open(path).map_err(|e| format!("Ошибка чтения PNG: {}", e))?;
    let mut rgb_img = img.to_rgb8();

    // 2. Монолитно зануляем область Тайла 0 в оперативной памяти
    for y in 0..16 {
        for x in 0..16 {
            rgb_img.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }

    // 3. Конвертируем модифицированный буфер в формат RGBA, который ожидает egui
    let size = [rgb_img.width() as usize, rgb_img.height() as usize];
    
    // Создаем вектор пикселей с альфа-каналом (ставим opaque непрозрачность 255)
    let mut rgba_pixels = Vec::with_capacity(size[0] * size[1] * 4);
    for pixel in rgb_img.pixels() {
        rgba_pixels.push(pixel[0]);
        rgba_pixels.push(pixel[1]);
        rgba_pixels.push(pixel[2]);
        rgba_pixels.push(255); // Непрозрачный альфа-канал
    }

    Ok(egui::ColorImage::from_rgba_unmultiplied(size, &rgba_pixels))
}