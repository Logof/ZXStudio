// src/ui/configurator/font_editor.rs
use crate::models::ProjectData;
use eframe::egui;
use image::GenericImageView;

pub fn render_font_editor(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_font_char_idx: &mut usize,
) {
    ui.heading("🔤 Редактор шрифтов ZX Spectrum (8x8)");
    ui.small("Шрифт в движке La Churrera содержит строго 64 символа (ASCII 32-95) в сетке 256x16.");
    ui.add_space(4.0);

    let success_id = ui.make_persistent_id("font_success_flag");
    let size_id = ui.make_persistent_id("font_size_flag");
    let read_id = ui.make_persistent_id("font_read_flag");

    let mut show_success: bool = ui.ctx().data(|d| d.get_temp(success_id)).unwrap_or(false);
    let mut show_size_err: bool = ui.ctx().data(|d| d.get_temp(size_id)).unwrap_or(false);
    let mut show_read_err: bool = ui.ctx().data(|d| d.get_temp(read_id)).unwrap_or(false);

    // Блок импорта шрифта из каноничного файла font.png
    ui.horizontal(|ui| {
        if ui
            .button("📥 Импортировать font.png (256x16)...")
            .on_hover_text("Выберите файл gfx/font.png размером строго 256x16 пикселей")
            .clicked()
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Изображения PNG", &["png"])
                .pick_file()
            {
                match image::open(&path) {
                    Ok(img) => {
                        let (w, h) = img.dimensions();
                        // ============================================================================
                        // ИСПРАВЛЕНО: Валидация строго под промышленный стандарт Mojon Twins (256x16)
                        // ============================================================================
                        if w == 256 && h == 16 {
                            let mut imported_font = vec![0u8; 512]; // 64 символа * 8 байт
                            let rgb_img = img.to_rgb8();

                            for char_idx in 0..64 {
                                // Вычисляем координаты буквы в сетке 2 строки по 32 символа
                                let grid_x = char_idx % 32;
                                let grid_y = char_idx / 32;

                                let char_offset_x = grid_x * 8;
                                let char_offset_y = grid_y * 8;
                                let char_byte_start = char_idx * 8;

                                for y in 0..8 {
                                    let mut row_byte = 0u8;
                                    for x in 0..8 {
                                        let px = rgb_img.get_pixel(
                                            (char_offset_x + x) as u32,
                                            (char_offset_y + y) as u32,
                                        );
                                        // Проверяем, не черный ли пиксель
                                        let is_active = px[0] > 30 || px[1] > 30 || px[2] > 30;
                                        if is_active {
                                            row_byte |= 1 << (7 - x);
                                        }
                                    }
                                    imported_font[char_byte_start + y] = row_byte;
                                }
                            }

                            project.font_data = imported_font;
                            show_success = true;
                            show_size_err = false;
                            show_read_err = false;
                        } else {
                            show_size_err = true;
                            show_success = false;
                            show_read_err = false;
                        }
                    }
                    Err(_) => {
                        show_read_err = true;
                        show_success = false;
                        show_size_err = false;
                    }
                }
            }

            ui.ctx().data_mut(|d| {
                d.insert_temp(success_id, show_success);
                d.insert_temp(size_id, show_size_err);
                d.insert_temp(read_id, show_read_err);
            });
        }

        if show_success {
            ui.colored_label(
                egui::Color32::LIGHT_GREEN,
                "✨ Оригинальный font.png успешно загружен!",
            );
            if ui.small_button("❌ OK").clicked() {
                show_success = false;
                ui.ctx().data_mut(|d| d.insert_temp(success_id, false));
            }
        }
        if show_size_err {
            ui.colored_label(
                egui::Color32::LIGHT_RED,
                "❌ Ошибка: Ожидаются габариты 256x16!",
            );
            if ui.small_button("❌ Закрыть").clicked() {
                show_size_err = false;
                ui.ctx().data_mut(|d| d.insert_temp(size_id, false));
            }
        }
        if show_read_err {
            ui.colored_label(egui::Color32::LIGHT_RED, "❌ Ошибка чтения файла.");
            if ui.small_button("❌ Закрыть").clicked() {
                show_read_err = false;
                ui.ctx().data_mut(|d| d.insert_temp(read_id, false));
            }
        }
    });

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        // --- ЛЕВАЯ ЧАСТЬ: Сетка выбора символов ASCII ---
        ui.vertical(|ui| {
            ui.set_width(360.0);
            ui.label("🗂 Каноничный набор (64 символа):");
            ui.add_space(4.0);

            egui::Frame::group(ui.style())
                .inner_margin(6.0)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_source("font_selector_scroll")
                        .max_height(520.0)
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            // ============================================================================
                            // ИСПРАВЛЕНО: Теперь выводится строго 64 кнопки (ровно 8 строк по 8 штук).
                            // Никаких лишних пустых знаков за пределами таблицы La Churrera.
                            // ============================================================================
                            egui::Grid::new("font_ascii_grid")
                                .spacing([6.0, 6.0])
                                .show(ui, |ui| {
                                    for idx in 0..64 {
                                        let ascii_code = 32 + idx;

                                        // Страховка индекса выделения, если он улетел за рамки 64 символов
                                        if *selected_font_char_idx >= 64 {
                                            *selected_font_char_idx = 0;
                                        }

                                        let (btn_rect, btn_resp) = ui.allocate_exact_size(
                                            egui::vec2(38.0, 48.0),
                                            egui::Sense::click(),
                                        );

                                        let bg_color = if *selected_font_char_idx == idx {
                                            egui::Color32::from_rgb(140, 30, 200)
                                        } else if btn_resp.hovered() {
                                            egui::Color32::from_rgb(45, 45, 55)
                                        } else {
                                            egui::Color32::from_rgb(25, 25, 30)
                                        };

                                        ui.painter().rect_filled(btn_rect, 3.0, bg_color);
                                        ui.painter().rect_stroke(
                                            btn_rect,
                                            3.0,
                                            egui::Stroke::new(1.0, egui::Color32::from_gray(50)),
                                        );

                                        if btn_resp.clicked() {
                                            *selected_font_char_idx = idx;
                                        }

                                        // Попиксельный рендеринг символа прямо ВНУТРИ кнопки
                                        let mini_cell = 2.0;
                                        let preview_origin =
                                            btn_rect.center_top() + egui::vec2(-8.0, 6.0);
                                        let char_offset = idx * 8;

                                        if char_offset + 7 < project.font_data.len() {
                                            for y in 0..8 {
                                                let byte_val = project.font_data[char_offset + y];
                                                for x in 0..8 {
                                                    if (byte_val & (1 << (7 - x))) != 0 {
                                                        let px_min = preview_origin
                                                            + egui::vec2(
                                                                x as f32 * mini_cell,
                                                                y as f32 * mini_cell,
                                                            );
                                                        ui.painter().rect_filled(
                                                            egui::Rect::from_min_size(
                                                                px_min,
                                                                egui::vec2(mini_cell, mini_cell),
                                                            ),
                                                            0.0,
                                                            egui::Color32::WHITE,
                                                        );
                                                    }
                                                }
                                            }
                                        }

                                        let hex_text = format!("0x{:02X}", ascii_code);
                                        ui.painter().text(
                                            btn_rect.center_bottom() + egui::vec2(0.0, -4.0),
                                            egui::Align2::CENTER_BOTTOM,
                                            hex_text,
                                            egui::FontId::monospace(8.0),
                                            egui::Color32::GRAY,
                                        );

                                        if (idx + 1) % 8 == 0 {
                                            ui.end_row();
                                        }
                                    }
                                });
                        });
                });
        });

        ui.separator();

        // --- ПРАВАЯ ЧАСТЬ: Холст пиксельного рисования 8x8 ---
        ui.vertical(|ui| {
            // Страховка индекса
            if *selected_font_char_idx >= 64 {
                *selected_font_char_idx = 0;
            }

            let ascii_code = 32 + *selected_font_char_idx;
            let active_char = char::from_u32(ascii_code as u32).unwrap_or('?');
            ui.label(egui::RichText::new(format!(
                "📝 Редактирование: '{}' (Код: {})",
                active_char, ascii_code
            )));
            ui.add_space(4.0);

            let char_offset = *selected_font_char_idx * 8;

            ui.horizontal(|ui| {
                let grid_size = 240.0;
                let cell_size = grid_size / 8.0;

                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(grid_size, grid_size),
                    egui::Sense::click_and_drag(),
                );
                let painter = ui.painter_at(rect);

                // Отрисовка фона сетки
                painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(15, 15, 20));

                // Обработка кликов и зажатий ЛКМ (рисовать) / ПКЛ (стирать)
                if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                    if rect.contains(mouse_pos) {
                        let cell_x = ((mouse_pos.x - rect.min.x) / cell_size).floor() as usize;
                        let cell_y = ((mouse_pos.y - rect.min.y) / cell_size).floor() as usize;

                        if cell_x < 8 && cell_y < 8 {
                            if ui.ctx().input(|i| i.pointer.primary_down()) {
                                if char_offset + cell_y < project.font_data.len() {
                                    project.font_data[char_offset + cell_y] |= 1 << (7 - cell_x);
                                }
                            } else if ui.ctx().input(|i| i.pointer.secondary_down()) {
                                if char_offset + cell_y < project.font_data.len() {
                                    project.font_data[char_offset + cell_y] &= !(1 << (7 - cell_x));
                                }
                            }
                        }
                    }
                }

                // Визуализация пикселей сетки шрифта
                for y in 0..8 {
                    if char_offset + y >= project.font_data.len() {
                        continue;
                    }
                    let byte_val = project.font_data[char_offset + y];
                    for x in 0..8 {
                        let bit_is_set = (byte_val & (1 << (7 - x))) != 0;

                        let c_min =
                            rect.min + egui::vec2(x as f32 * cell_size, y as f32 * cell_size);
                        let cell_rect =
                            egui::Rect::from_min_size(c_min, egui::vec2(cell_size, cell_size));

                        let fill_color = if bit_is_set {
                            egui::Color32::from_rgb(0, 255, 150)
                        } else {
                            egui::Color32::TRANSPARENT
                        };

                        painter.rect_filled(cell_rect.shrink(1.0), 0.0, fill_color);
                        painter.rect_stroke(
                            cell_rect,
                            0.0,
                            egui::Stroke::new(
                                0.5,
                                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 15),
                            ),
                        );
                    }
                }

                ui.add_space(16.0);

                ui.vertical(|ui| {
                    ui.label("👁 Превью:");
                    ui.add_space(4.0);

                    let p_size = 64.0;
                    let p_cell = p_size / 8.0;
                    let (p_rect, _) =
                        ui.allocate_exact_size(egui::vec2(p_size, p_size), egui::Sense::hover());
                    let p_painter = ui.painter_at(p_rect);

                    p_painter.rect_filled(p_rect, 0.0, egui::Color32::BLACK);

                    for y in 0..8 {
                        if char_offset + y >= project.font_data.len() {
                            continue;
                        }
                        let byte_val = project.font_data[char_offset + y];
                        for x in 0..8 {
                            if (byte_val & (1 << (7 - x))) != 0 {
                                let pc_min =
                                    p_rect.min + egui::vec2(x as f32 * p_cell, y as f32 * p_cell);
                                p_painter.rect_filled(
                                    egui::Rect::from_min_size(pc_min, egui::vec2(p_cell, p_cell)),
                                    0.0,
                                    egui::Color32::WHITE,
                                );
                            }
                        }
                    }

                    ui.add_space(12.0);
                    if ui.button("🗑 Очистить").clicked() {
                        for y in 0..8 {
                            if char_offset + y < project.font_data.len() {
                                project.font_data[char_offset + y] = 0;
                            }
                        }
                    }
                    if ui.button("🔄 Инверсия").clicked() {
                        for y in 0..8 {
                            if char_offset + y < project.font_data.len() {
                                project.font_data[char_offset + y] =
                                    !project.font_data[char_offset + y];
                            }
                        }
                    }
                });
            });
        });
    });
}
