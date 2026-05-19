use eframe::egui;
use crate::app::ZxIdeApp;

pub fn process_asset_loading(app: &mut ZxIdeApp, ctx: &egui::Context) {
    if app.wizard_active {
        return;
    }

    // Собираем базовый путь
    let gfx_dir = std::path::Path::new(&app.project_path).join("gfx");

    // Если папки gfx физически нет, прерываемся без паники приложения
    if !gfx_dir.exists() || !gfx_dir.is_dir() {
        if app.tileset_texture.is_none() && app.sprites_texture.is_none() {
            app.status_message = format!("⚠️ Папка ресурсов не найдена по пути: {}", gfx_dir.display());
        }
        return;
    }

    // 1. БЕЗОПАСНАЯ ЗАГРУЗКА ТАЙЛСЕТА (work.png)
    if app.tileset_texture.is_none() {
        let work_path = gfx_dir.join("work.png");
        if work_path.exists() && work_path.is_file() {
            match crate::core::image_processor::load_work_png_to_image_ram(&work_path) {
                Ok(color_img) => {
                    app.tileset_texture = Some(ctx.load_texture(
                        "autoscan_tileset",
                        color_img,
                        egui::TextureOptions::NEAREST,
                    ));
                    app.status_message = "⚡ Тайлсет [work.png] успешно подгружен!".to_string();
                }
                Err(err) => {
                    app.status_message = format!("❌ Ошибка ОЗУ-модификатора work.png: {}", err);
                }
            }
        } else {
            app.status_message = "⚠️ Файл gfx/work.png отсутствует в папке проекта".to_string();
        }
    }

    // 2. БЕЗОПАСНАЯ ЗАГРУЗКА СПРАЙТСЕТА (sprites.png) С ЗАЩИТОЙ ОТ ПАНИКИ МАСОК
    if app.sprites_texture.is_none() {
        let sprites_path = gfx_dir.join("sprites.png");
        if sprites_path.exists() && sprites_path.is_file() {
            let id_mask_flag = egui::Id::new("sprites_mask_calculated_flag");
            let already_calculated = ctx.data(|d| d.get_temp::<bool>(id_mask_flag)).unwrap_or(false);

            if !already_calculated {
                // Обертываем генератор масок, чтобы сбой в Си/Rust-координатах ядра не вешал UI
                let mask_result = crate::core::image_processor::generate_sprite_masks(&sprites_path.to_string_lossy());
                if mask_result.is_ok() {
                    ctx.data_mut(|d| d.insert_temp(id_mask_flag, true));
                } else {
                    app.status_message = "⚠️ Предупреждение: Не удалось сгенерировать автоматические маски спрайтов".to_string();
                }
            }

            // Загружаем саму текстуру
            match crate::core::image_processor::load_png_to_image(&sprites_path) {
                Ok(color_img) => {
                    app.sprites_texture = Some(ctx.load_texture(
                        "autoscan_sprites",
                        color_img,
                        egui::TextureOptions::NEAREST,
                    ));
                    app.status_message = "⚡ Спрайтсет успешно подгружен в IDE!".to_string();
                }
                Err(err) => {
                    app.status_message = format!("❌ Ошибка декодирования sprites.png: {}", err);
                }
            }
        }
    }

    if app.hud_frame_texture.is_none() {
        let marco_path = gfx_dir.join("marco.png");
        if marco_path.exists() && marco_path.is_file() {
        // Загружаем картинку через ваше ядро/декодер PNG
            match crate::core::image_processor::load_png_to_image(&marco_path) {
                Ok(color_img) => {
                    app.hud_frame_texture = Some(ctx.load_texture(
                        "autoscan_hud_frame",
                        color_img,
                        egui::TextureOptions::NEAREST, // Оставляем четкие пиксели Spectrum
                    ));
                    app.status_message = "⚡ Графическая рамка [marco.png] успешно подгружена в HUD!".to_string();
                }
                Err(err) => {
                    app.status_message = format!("⚠️ Не удалось декодировать marco.png: {}", err);
                }
            }
        }
    }
}
