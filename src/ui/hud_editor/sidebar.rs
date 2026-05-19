// src/ui/hud_editor/sidebar.rs
use super::types::HudItemMetadata;
use crate::models::config::hud_rendering::HudRenderingConfig;
use eframe::egui;

pub fn render_sidebar(
    ui: &mut egui::Ui,
    elements: &[HudItemMetadata],
    hud_config: &mut HudRenderingConfig,
) {
    ui.allocate_ui(egui::vec2(280.0, ui.available_height()), |ui| {
        ui.vertical(|ui| {

            // БЛОК 1: Активные индикаторы (Управление видимостью)
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.strong("➕ АКТИВНЫЕ ИНДИКАТОРЫ (2х1 знакоместа):");
                ui.add_space(4.0);

                for item in elements.iter() {
                    let (x, y) = get_mut_coords(item.id, hud_config);
                    let mut is_active = *x != 99;

                    if ui.checkbox(&mut is_active, item.label).changed() {
                        if is_active { *x = 30; *y = 2; }
                        else { *x = 99; *y = 99; }
                    }
                }
            });

            ui.add_space(6.0);

            // БЛОК 2: Числовые координаты
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.strong("📊 КООРДИНАТЫ ИКОНКИ (Левая ячейка):");
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.strong("Экран: ");
                    ui.add(egui::DragValue::new(&mut hud_config.viewport_x).prefix("X: "));
                    ui.add(egui::DragValue::new(&mut hud_config.viewport_y).prefix("Y: "));
                });

                for item in elements.iter() {
                    let (x, y) = get_mut_coords(item.id, hud_config);
                    if *x != 99 && *y != 99 {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.set_width(ui.available_width());
                            ui.colored_label(item.color, format!("{}: ", item.icon));
                            ui.add(egui::DragValue::new(x).clamp_range(0..=31).prefix("X: "));
                            ui.add(egui::DragValue::new(y).clamp_range(0..=23).prefix("Y: "));
                        });
                    }
                }
            });

            ui.add_space(12.0);
            ui.label("ℹ️ Каждый маркер занимает ровно 2 ячейки в длину и 1 ячейку в высоту (иконка слева, счетчик справа).");
        });
    });
}

/// Вспомогательный хелпер сопоставления ID и физических полей структуры для сайдбара
pub fn get_mut_coords<'a>(
    id: &str,
    hud_config: &'a mut HudRenderingConfig,
) -> (&'a mut u32, &'a mut u32) {
    match id {
        "life" => (&mut hud_config.life_x, &mut hud_config.life_y),
        "objects" => (&mut hud_config.objects_x, &mut hud_config.objects_y),
        "objects_icon" => (
            &mut hud_config.objects_icon_x,
            &mut hud_config.objects_icon_y,
        ),
        "keys" => (&mut hud_config.keys_x, &mut hud_config.keys_y),
        "killed" => (&mut hud_config.killed_x, &mut hud_config.killed_y),
        "ammo" => (&mut hud_config.ammo_x, &mut hud_config.ammo_y),
        "timer" => (&mut hud_config.timer_x, &mut hud_config.timer_y),
        _ => panic!("Unknown HUD element ID"),
    }
}
