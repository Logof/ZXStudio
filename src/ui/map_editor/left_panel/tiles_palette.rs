use eframe::egui;
use crate::models::ProjectData;

pub fn render(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_tile: &mut u8,
    tileset_texture: &Option<egui::TextureHandle>,
) {
    ui.label("Палитра тайлов (work.png):");
    ui.add_space(4.0);

    // Выводим сетку блоков 0..47 строго по 4 штуки в ряд
    egui::Grid::new("palette_grid").spacing([6.0, 6.0]).show(ui, |ui| {
        let mut grid_index = 0;
        for t in 0..48 {
            let tile_x = (t % 16) as f32 * 16.0;
            let tile_y = (t / 16) as f32 * 16.0;

            let eps = 0.5;
            let uv_min = egui::pos2((tile_x + eps) / 256.0, (tile_y + eps) / 48.0);
            let uv_max = egui::pos2((tile_x + 16.0 - eps) / 256.0, (tile_y + 16.0 - eps) / 48.0);
            let uv_rect = egui::Rect::from_min_max(uv_min, uv_max);

            let mut button = egui::Button::new("");
            if *selected_tile == t {
                button = button.stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 150, 255)));
            }

            let btn_response = ui.add_sized([36.0, 36.0], button);
            
            if let Some(tex) = tileset_texture {
                ui.painter().image(tex.id(), btn_response.rect.shrink(2.0), uv_rect, egui::Color32::WHITE);
            } else {
                ui.painter().text(btn_response.rect.center(), egui::Align2::CENTER_CENTER, format!("{:02}", t), egui::FontId::proportional(12.0), egui::Color32::GRAY);
            }

            if btn_response.clicked() { 
                *selected_tile = t; 
            }
            
            grid_index += 1;
            if grid_index % 4 == 0 { 
                ui.end_row(); 
            }
        }
    });

    // --- УНИВЕРСАЛЬНЫЙ ИНСПЕКТОР ФИЗИЧЕСКИХ СВОЙСТВ ТАЙЛА ---
    if (*selected_tile as usize) < project.tile_behaviours.len() {
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);
        ui.label(format!("🧱 Свойства тайла №{:02}:", selected_tile));
        
        let t_idx = *selected_tile as usize;
        let mut current_beh = project.tile_behaviours[t_idx];
        
        egui::ComboBox::from_id_source("tile_physics_combo")
            .selected_text(match current_beh { 
                0 => "🚶 0: Walkable", 
                1 => "💀 1: Kills", 
                4 => "🧗 4: Platform", 
                8 => "🧱 8: Obstacle", 
                _ => "Маска физики" 
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut current_beh, 0, "🚶 0: Walkable (Проходимый)");
                ui.selectable_value(&mut current_beh, 1, "💀 1: Kills (Шипы/Лава)");
                ui.selectable_value(&mut current_beh, 4, "🧗 4: Platform (Полупроходимый)");
                ui.selectable_value(&mut current_beh, 8, "🧱 8: Obstacle (Стена/Блок)");
            });
            
        project.tile_behaviours[t_idx] = current_beh;

        // ИНСПЕКТОР РОЛЕЙ ДВИЖКА (14..19) СТРОГО ПО LA CHURRERA MTE MK1
        if *selected_tile >= 14 && *selected_tile <= 19 {
            ui.add_space(6.0);
            ui.group(|ui| {
                ui.label("⚙️ Настройки La Churrera:");
                
                match *selected_tile {
                    14 => {
                        ui.checkbox(&mut project.role_pushbox_active, "📦 PLAYER_PUSH_BOXES")
                            .on_hover_text("Активировать ящик. Игрок сможет толкать этот тайл.");
                    }
                    15 => {
                        ui.checkbox(&mut project.role_key_active, "🔑 ACTIVATE_KEYS_AND_LOCKS") 
                            .on_hover_text("Активировать замок. Препятствие, исчезающее при касании замка ключом.");
                    }
                    16 => {
                        ui.checkbox(&mut project.role_refill_active, "❤️ REFILLS_WORK")
                            .on_hover_text("Активировать регенерацию здоровья при повторном посещении экрана.");
                    }
                    17 => {
                        ui.checkbox(&mut project.role_collectable_active, "🌟 ITEMS_WORK")
                            .on_hover_text("Активировать сбор предметов. Тайл станет подбираемым лутом.");
                    }
                    18 => {
                        ui.checkbox(&mut project.role_key_active, "🔑 KEYS_WORK")
                            .on_hover_text("Активировать ключи. Тайл станет ключом для дверей (Тайла 15).");
                    }
                    19 => {
                        let mut temp_decor = true;
                        ui.checkbox(&mut temp_decor, "🎨 RANDOM_DECORATION_T19")
                            .on_hover_text("Случайная подмена пустого тайла 0 на тайл 19 для красоты.");
                    }
                    _ => {}
                }
                ui.small(format!("Фиксированная роль MTE MK1"));
            });
        }
    }
}
