use eframe::egui;
use super::mod_struct::ZxIdeApp;
use crate::core::{save_project_json, export_config_h, export_enems_h, validator::validate_attribute_clash, image_processor::{process_tileset_for_mappy, generate_sprite_masks}};

pub fn render_toolbar(app: &mut ZxIdeApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("toolbar")
        .frame(egui::Frame::none().inner_margin(4.0).fill(egui::Color32::from_rgb(22, 22, 26)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("💾 Сохранить JSON").clicked() {
                    let _ = save_project_json(&app.project);
                    app.status_message = "Проект сохранен".to_string();
                }
                
                if ui.button("⚙️ Экспорт Ресурсов").on_hover_text("Сгенерировать config.h и enems.h для Си-компилятора").clicked() {
                    let mut success = true;
                    if let Err(e) = export_config_h(&app.project) {
                        app.status_message = format!("Ошибка config.h: {}", e);
                        success = false;
                    }
                    if success {
                        if let Err(e) = export_enems_h(&app.project) {
                            app.status_message = format!("Ошибка enems.h: {}", e);
                            success = false;
                        }
                    }
                    if success {
                        app.status_message = "✅ Ресурсы dev/ успешно экспортированы!".to_string();
                    }
                }

                if ui.button("🖼️ Импортировать графику").clicked() {
                    let mut success = true;
                    if let Err(e) = process_tileset_for_mappy("gfx/work.png", "gfx/mappy.png") { success = false; }
                    if success { if let Err(_) = generate_sprite_masks("gfx/sprites.png") { success = false; } }
                    
                    if success {
                        if let Ok(img) = image::open("gfx/mappy.png") {
                            let rgb_img = img.to_rgba8();
                            let (w, h) = rgb_img.dimensions();
                            let color_image = egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgb_img.as_flat_samples().samples);
                            app.tileset_texture = Some(ctx.load_texture("tileset_palette", color_image, egui::TextureOptions::NEAREST));
                        }
                        if let Ok(img) = image::open("gfx/sprites.png") {
                            let rgba_img = img.to_rgba8();
                            let (w, h) = rgba_img.dimensions();
                            let color_image = egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba_img.as_flat_samples().samples);
                            app.sprites_texture = Some(ctx.load_texture("sprites_palette", color_image, egui::TextureOptions::NEAREST));
                        }
                        app.status_message = "✅ Вся графика успешно загружена в видеопамять!".to_string();
                    }
                }

                ui.separator();
                if ui.button("🔍 Валидация Attribute Clash").clicked() {
                    if let Ok(errors) = validate_attribute_clash("gfx/title.png") {
                        app.clash_errors = errors;
                        app.status_message = format!("Найдено коллизий цвета: {}", app.clash_errors.len());
                    }
                }
            });
        });
}
