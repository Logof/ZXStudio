// src/core/validator/attribute_clash.rs

use super::{ClashError, ErrorSeverity};
use image::RgbImage;
use std::collections::HashSet;

/// Графический валидатор: сканирует PNG-изображение по сетке знакомест 8x8 пикселей.
/// Если в блоке обнаруживается более 2 уникальных цветов, фиксирует Attribute Clash нарушений палитры Spectrum.
pub fn validate_image_colors(image_path: &str) -> Result<Vec<ClashError>, image::ImageError> {
    let img = image::open(image_path)?;
    let rgb_img: RgbImage = img.to_rgb8();

    let (width, height) = rgb_img.dimensions();
    let mut errors = Vec::new();

    for block_y in 0..(height as usize / 8) {
        for block_x in 0..(width as usize / 8) {
            let mut unique_colors = HashSet::new();

            for pixel_y in 0..8 {
                for pixel_x in 0..8 {
                    let px = rgb_img.get_pixel(
                        (block_x * 8 + pixel_x) as u32,
                        (block_y * 8 + pixel_y) as u32,
                    );
                    // Извлекаем полноценные каналы R, G, B для точной идентификации цвета Спектрума
                    unique_colors.insert((px[0], px[1], px[2]));
                }
            }

            // Железное ограничение ZX Spectrum: строго PAPER + INK (до 2 цветов на блок 8x8)
            if unique_colors.len() > 2 {
                errors.push(ClashError {
                                    // ============================================================================
                                    // ИСПРАВЛЕНО: Заменяем индекс 0 на служебную константу 9999.
                                    // Это изолирует графические ошибки от логической сетки комнат (Экран 0),
                                    // и призрачные квадраты (2,4), (2,8), (10,3) исчезнут с карты раз и навсегда!
                                    // ============================================================================
                                    screen_index: 9999,
                                    cell_x: block_x,
                                    cell_y: block_y,
                                    severity: ErrorSeverity::Warning,
                                    message: format!(
                                        "Конфликт палитры (Attribute Clash) в блоке 8x8 на пиксельной сетке: ({}, {})",
                                        block_x * 8, block_y * 8
                                    ),
                                });
            }
        }
    }

    Ok(errors)
}
