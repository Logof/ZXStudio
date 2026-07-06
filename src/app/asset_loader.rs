// src/app/asset_loader.rs
use crate::app::ZxIdeApp;
use crate::core::image_processor::TileSlicer;
use eframe::egui;

pub fn process_asset_loading(app: &mut ZxIdeApp, ctx: &egui::Context) {
    if app.wizard_active {
        return;
    }

    // Определяем текущий язык из кэша перевода, чтобы динамически локализовать логи
    let is_english = app.translations.menu.lang_select.contains("Language");

    // Перехват сигналов смены графического режима из конфигуратора
    let reset_flag = egui::Id::new("trigger_reset_tileset_graphics");
    let need_reset = ctx
        .data_mut(|d| d.remove_temp::<bool>(reset_flag))
        .unwrap_or(false);
    if need_reset {
        app.tileset_texture = None;
    }

    let clear_flag = egui::Id::new("trigger_clear_sliced_textures");
    let need_clear = ctx
        .data_mut(|d| d.remove_temp::<bool>(clear_flag))
        .unwrap_or(false);
    if need_clear {
        app.sliced_tile_textures.clear(); // Мгновенно опустошаем палитру, убирая залипание старого режима
    }

    // ============================================================================
    // ИСПРАВЛЕНИЕ: Перехват глобального сигнала сброса кэша масок спрайтов
    // ============================================================================
    let id_mask_flag = egui::Id::new("sprites_mask_calculated_flag");
    // Если текстура спрайтов сброшена в None (из-за переключения проекта), принудительно стираем флаг маски
    if app.sprites_texture.is_none() {
        ctx.data_mut(|d| {
            let _ = d.remove_temp::<bool>(id_mask_flag);
        });
    }
    // ============================================================================

    // Собираем базовый путь
    let gfx_dir = std::path::Path::new(&app.project_path).join("gfx");

    // Вычисляем ожидаемые параметры под текущий режим активного уровня
    let current_mode = app.project.levels[app.project.current_level_index].tile_mode;
    let expected_tiles_count = match current_mode {
        crate::models::project::TileMode::Packed16 => 20,
        crate::models::project::TileMode::Packed16WithShadows => 32, // Ровно 32 тайла (16 основных + 12 теней + 4 спец)
        crate::models::project::TileMode::Extended48 => 48,
    };

    // Если папки gfx нет или палитра пуста (например, после очистки) — генерируем отказоустойчивые плейсхолдеры
    if app.sliced_tile_textures.is_empty() {
        let work_path = gfx_dir.join("work.png");
        let mut loaded_successfully = false;

        if work_path.exists() && work_path.is_file() {
            if let Ok(color_img) =
                crate::core::image_processor::load_work_png_to_image_ram(&work_path)
            {
                // Пытаемся нарезать реальный файл
                if let Ok(sliced_images) = TileSlicer::slice_tiles(&color_img, current_mode) {
                    app.tileset_texture = Some(ctx.load_texture(
                        "autoscan_tileset",
                        color_img,
                        egui::TextureOptions::NEAREST,
                    ));

                    app.sliced_tile_textures.clear();
                    for (index, tile_img) in sliced_images.into_iter().enumerate() {
                        let tex_handle = ctx.load_texture(
                            format!("tile_{}", index),
                            tile_img,
                            egui::TextureOptions::NEAREST,
                        );
                        app.sliced_tile_textures.push(tex_handle);
                    }
                    loaded_successfully = true;

                    // ЛOКАЛИЗАЦИЯ СТАТУСА ТАЙЛСЕТА
                    app.status_message = if is_english {
                        format!(
                            "⚡ Tileset synchronized! Sliced tiles: {} (Mode: {})",
                            app.sliced_tile_textures.len(),
                            current_mode.name()
                        )
                    } else {
                        format!(
                            "⚡ Тайлсет синхронизирован! Нарезано тайлов: {} (Режим: {})",
                            app.sliced_tile_textures.len(),
                            current_mode.name()
                        )
                    };
                }
            }
        }

        // РЕЗЕРВНЫЙ АВТО-ПЛЕЙСХОЛДЕР: Если файла нет или его размер не совпадает с новым TileMode
        if !loaded_successfully {
            app.sliced_tile_textures.clear();
            for index in 0..expected_tiles_count {
                // Создаем пустышку 16x16 заполненную темно-серым цветом
                let fill_color = match index {
                    14 => egui::Color32::from_rgb(40, 70, 150), // Плейсхолдер пуш-блока
                    15 => egui::Color32::from_rgb(150, 40, 40), // Плейсхолдер замка
                    16..=18 => egui::Color32::from_rgb(40, 120, 40), // Окрашиваем спец-зоны хотспотов
                    _ => egui::Color32::from_gray(45),
                };
                let placeholder_img = egui::ColorImage::new([16, 16], fill_color);
                let tex_handle = ctx.load_texture(
                    format!("tile_placeholder_{}", index),
                    placeholder_img,
                    egui::TextureOptions::NEAREST,
                );
                app.sliced_tile_textures.push(tex_handle);
            }

            // ЛОКАЛИЗАЦИЯ СТАТУСА ЗАГЛУШЕК
            app.status_message = if is_english {
                format!(
                    "ℹ️ Created palette placeholders ({} pcs) for mode: {}. Update gfx/work.png!",
                    expected_tiles_count,
                    current_mode.name()
                )
            } else {
                format!(
                    "ℹ️ Созданы заглушки палитры ({} шт.) для режима: {}. Обновите gfx/work.png!",
                    expected_tiles_count,
                    current_mode.name()
                )
            };
        }
    }

    // 2. БЕЗОПАСНАЯ ЗАГРУЗКА СПРАЙТСЕТА (sprites.png)
    if app.sprites_texture.is_none() {
        let sprites_path = gfx_dir.join("sprites.png");
        if sprites_path.exists() && sprites_path.is_file() {
            let already_calculated = ctx
                .data(|d| d.get_temp::<bool>(id_mask_flag))
                .unwrap_or(false);

            if !already_calculated {
                let mask_result = crate::core::image_processor::generate_sprite_masks(
                    &sprites_path.to_string_lossy(),
                );
                if mask_result.is_ok() {
                    ctx.data_mut(|d| d.insert_temp(id_mask_flag, true));
                } else {
                    app.status_message = if is_english {
                        "⚠️ Warning: Failed to generate automatic sprite masks".to_string()
                    } else {
                        "⚠️ Предупреждение: Не удалось сгенерировать автоматические маски спрайтов"
                            .to_string()
                    };
                }
            }

            match crate::core::image_processor::load_png_to_image(&sprites_path) {
                Ok(color_img) => {
                    app.sprites_texture = Some(ctx.load_texture(
                        "autoscan_sprites",
                        color_img,
                        egui::TextureOptions::NEAREST,
                    ));
                }
                Err(err) => {
                    app.status_message = if is_english {
                        format!("❌ Error decoding sprites.png: {}", err)
                    } else {
                        format!("❌ Ошибка декодирования sprites.png: {}", err)
                    };
                }
            }
        }
    }

    // 3. БЕЗОПАСНАЯ ЗАГРУЗКА РАМКИ HUD (marco.png)
    if app.hud_frame_texture.is_none() {
        let marco_path = gfx_dir.join("marco.png");
        if marco_path.exists() && marco_path.is_file() {
            match crate::core::image_processor::load_png_to_image(&marco_path) {
                Ok(color_img) => {
                    app.hud_frame_texture = Some(ctx.load_texture(
                        "autoscan_hud_frame",
                        color_img,
                        egui::TextureOptions::NEAREST,
                    ));
                }
                Err(err) => {
                    app.status_message = if is_english {
                        format!("⚠️ Failed to decode marco.png: {}", err)
                    } else {
                        format!("⚠️ Не удалось декодировать marco.png: {}", err)
                    };
                }
            }
        }
    }
}
