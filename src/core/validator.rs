use image::RgbImage;
use std::collections::HashSet;

pub struct ClashError {
    pub x_block: usize,
    pub y_block: usize,
}

pub fn validate_attribute_clash(image_path: &str) -> Result<Vec<ClashError>, image::ImageError> {
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
                    unique_colors.insert((px, px, px));
                }
            }

            if unique_colors.len() > 2 {
                errors.push(ClashError {
                    x_block: block_x,
                    y_block: block_y,
                });
            }
        }
    }

    Ok(errors)
}
