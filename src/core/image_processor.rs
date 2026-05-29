use crate::models::project::TileMode;
use eframe::egui;
use image::{Rgb, RgbImage};
use std::path::Path;

/// Импорт тайлсета: читает work.png, зануляет тайл 0 (16x16) и сохраняет mappy.png
pub fn process_tileset_for_mappy(
    source_path: &str,
    target_path: &str,
) -> Result<(), image::ImageError> {
    if !Path::new(source_path).exists() {
        return Err(image::ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Файл {} не найден", source_path),
        )));
    }
    let img = image::open(source_path)?;
    let mut rgb_img: RgbImage = img.to_rgb8();

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
            format!(
                "Неверный размер sprites.png! Ожидалось 256x32, получено {}x{}",
                width, height
            ),
        )));
    }

    let mut output_img = rgb_img.clone();

    for row_y in [0, 16] {
        for block_x in (0..256).step_by(32) {
            let sprite_start_x = block_x;
            let mask_start_x = block_x + 16;

            for y in 0..16 {
                for x in 0..16 {
                    let px_x = sprite_start_x + x;
                    let pixel_color = rgb_img.get_pixel(px_x, row_y + y);

                    let mask_pixel =
                        if pixel_color[0] == 0 && pixel_color[1] == 0 && pixel_color[2] == 0 {
                            Rgb([255, 255, 255])
                        } else {
                            Rgb([0, 0, 0])
                        };

                    output_img.put_pixel(mask_start_x + x, row_y + y, mask_pixel);
                }
            }
        }
    }

    output_img.save(sprites_path)?;
    Ok(())
}

pub fn load_png_to_image<P: AsRef<Path>>(path: P) -> Result<egui::ColorImage, String> {
    let img = image::open(path).map_err(|e| format!("Ошибка чтения PNG: {}", e))?;
    let size = [img.width() as usize, img.height() as usize];
    let rgba_pixels = img.to_rgba8().into_raw();
    Ok(egui::ColorImage::from_rgba_unmultiplied(size, &rgba_pixels))
}

pub fn load_work_png_to_image_ram<P: AsRef<Path>>(path: P) -> Result<egui::ColorImage, String> {
    let img = image::open(path).map_err(|e| format!("Ошибка чтения PNG: {}", e))?;
    let mut rgb_img = img.to_rgb8();

    for y in 0..16 {
        for x in 0..16 {
            rgb_img.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }

    let size = [rgb_img.width() as usize, rgb_img.height() as usize];
    let mut rgba_pixels = Vec::with_capacity(size[0] * size[1] * 4);
    for pixel in rgb_img.pixels() {
        rgba_pixels.push(pixel[0]);
        rgba_pixels.push(pixel[1]);
        rgba_pixels.push(pixel[2]);
        rgba_pixels.push(255);
    }

    Ok(egui::ColorImage::from_rgba_unmultiplied(size, &rgba_pixels))
}

// ============================================================================
// УЛУЧШЕННЫЙ ОТКАЗОУСТОЙЧИВЫЙ СЛАЙСЕР С АВТОДОЗАПОЛНЕНИЕМ ТЕНИ
// ============================================================================
pub struct TileSlicer;

impl TileSlicer {
    /// Нарезает `egui::ColorImage` на массив тайлов 16x16.
    /// Если файл на диске меньше ожидаемого разрешения, метод автоматически догенерирует
    /// недостающие графические ячейки в памяти, исключая падение UI и пропажу карты.
    pub fn slice_tiles(
        source: &egui::ColorImage,
        mode: TileMode,
    ) -> Result<Vec<egui::ColorImage>, String> {
        let (expected_w, _expected_h) = mode.expected_dimensions();
        let current_w = source.size[0];
        let current_h = source.size[1];

        // Ширину (256 px) проверяем жестко, так как это базис сетки 16 тайлов в ряд
        if current_w != expected_w as usize {
            return Err(format!(
                "Конфликт ширины тайлсета! Ожидалось {} px, получено {} px.",
                expected_w, current_w
            ));
        }

        let tile_size = 16;
        let cols = current_w / tile_size;
        let rows = current_h / tile_size;
        let mut tiles = Vec::new();

        // Лимиты полезных тайлов для каждого режима
        let target_max_count = match mode {
            TileMode::Packed16 => 20,
            TileMode::Packed16WithShadows => 32, // 16 основных + 16 теневых/служебных
            TileMode::Extended48 => 48,
        };

        // Шаг 1: Нарезаем то, что физически присутствует в изображении
        for r in 0..rows {
            for c in 0..cols {
                if tiles.len() >= target_max_count {
                    break;
                }

                let mut tile_pixels = Vec::with_capacity(tile_size * tile_size);
                for y in 0..tile_size {
                    let pixel_y = r * tile_size + y;
                    for x in 0..tile_size {
                        let pixel_x = c * tile_size + x;
                        let index = pixel_y * current_w + pixel_x;
                        tile_pixels.push(source.pixels[index]);
                    }
                }

                tiles.push(egui::ColorImage {
                    size: [tile_size, tile_size],
                    pixels: tile_pixels,
                });
            }
        }

        // ============================================================================
        // ИСПРАВЛЕНО: Автодозаполнение памяти, если высота файла меньше требуемой.
        // Если перешли с 48/16 на Автошейдинг, но работаем со старым файлом (высота 48 вместо 96),
        // дозабиваем недостающие ячейки (до 32) темными плейсхолдерами, сохраняя основную графику на карте!
        // ============================================================================
        while tiles.len() < target_max_count {
            let index = tiles.len();
            let fill_color = match index {
                14 => egui::Color32::from_rgb(40, 70, 150), // Синий пуш-блок
                15 => egui::Color32::from_rgb(150, 40, 40), // Красный замок
                16..=18 => egui::Color32::from_rgb(40, 120, 40), // Зеленые хотспоты
                20..=31 => egui::Color32::from_rgba_unmultiplied(20, 20, 25, 180), // Темные копии для теней
                _ => egui::Color32::from_gray(45),
            };
            tiles.push(egui::ColorImage::new([tile_size, tile_size], fill_color));
        }

        Ok(tiles)
    }
}
