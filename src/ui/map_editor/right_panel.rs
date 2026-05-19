use super::map_canvas::render_map_canvas;
use crate::app::states::MapEditMode;
use crate::core::validator::ClashError;
use crate::models::ProjectData;
use eframe::egui;

pub fn render(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_screen: &mut usize,
    selected_tile: &mut u8,
    clash_errors: &[ClashError],
    map_edit_mode: &MapEditMode,
    selected_enemy_sprite_slot: &mut u8,
    tileset_texture: &Option<egui::TextureHandle>,
) {
    ui.vertical(|ui| {
        let max_screens = (project.config.map_goals.map_w * project.config.map_goals.map_h) as usize - 1;
        // Отрисовка холста карты
        render_map_canvas(
            ui,
            project,
            selected_screen,
            selected_tile,
            clash_errors,
            map_edit_mode,
            *selected_enemy_sprite_slot,
            tileset_texture,
        );

        // Инспектор врагов комнаты (Строго 3 слота MTE MK1)
        let scr_key = format!("screen_{}", selected_screen);
        if let Some(screen_data) = project.screens.get_mut(&scr_key) {
            if !screen_data.enemies.is_empty() && *map_edit_mode == MapEditMode::Enemies {
                ui.add_space(6.0);

                let count = screen_data.enemies.len();
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("👾 Размещенные сущности:");
                        ui.colored_label(
                            if count == 3 { egui::Color32::LIGHT_RED } else { egui::Color32::LIGHT_GREEN },
                            format!("{}/3", count)
                        );
                    });
                    ui.add_space(2.0);

                    let mut to_remove = None;

                    for (idx, enemy) in screen_data.enemies.iter_mut().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.colored_label(egui::Color32::from_rgb(180, 50, 255), format!("[Слот {}]", idx + 1));

                                // Вычисление HEX-кода спрайта для совместимости с ponedor.exe (8, 10, 12, 14)
                                let raw_gfx_id = 8 + (enemy.tp % 4) * 2;
                                ui.label(format!("Графика: Спрайт 0x{:X}", raw_gfx_id));
                                ui.small(format!("Старт: ({}, {})", enemy.x, enemy.y));

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("🗑").clicked() { to_remove = Some(idx); }
                                });
                            });

                            ui.separator();

                            // --- МАТЕМАТИЧЕСКОЕ ВЫЧИСЛЕНИЕ ТИПА ИИ НА ОСНОВЕ ГЕОМЕТРИИ ЛИМИТОВ ---
                            let mut current_ai_mode = if enemy.x1 == 0 && enemy.x2 == 0 && enemy.y1 == 0 && enemy.y2 == 0 {
                                4 // Режим Фанти (зануление осей по спецификации)
                            } else if enemy.x1 != enemy.x2 && enemy.y1 != enemy.y2 {
                                6 // Режим Куадратора (рамочный обход)
                            } else if enemy.x1 == enemy.x2 && enemy.y1 != enemy.y2 {
                                1 // Линейный Вертикальный
                            } else {
                                0 // Линейный Горизонтальный
                            };

                            ui.horizontal(|ui| {
                                ui.small("🧠 Поведение ИИ:");

                                let mut new_ai = current_ai_mode;
                                egui::ComboBox::from_id_source(format!("ai_combo_calc_{}", idx))
                                    .selected_text(match current_ai_mode {
                                        0 => "↔️ Линейный Горизонтальный",
                                        1 => "↕️ Линейный Вертикальный",
                                        6 => "🔄 Обход по Периметру (Куадратор)",
                                        _ => "👻 Летающий преследователь (Фанти)",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut new_ai, 0, "↔️ Линейный Горизонтальный");
                                        ui.selectable_value(&mut new_ai, 1, "↕️ Линейный Вертикальный");
                                        ui.selectable_value(&mut new_ai, 4, "👻 Летающий Преследователь (Фанти)");
                                        ui.selectable_value(&mut new_ai, 6, "🔄 Обход по Периметру (Куадратор)");
                                    });

                                // Если пользователь изменил ИИ, пересчитываем лимиты под паттерн Моджонов
                                if new_ai != current_ai_mode {
                                    current_ai_mode = new_ai;
                                    match new_ai {
                                        0 => { // Горизонтальный
                                            enemy.x1 = enemy.x; enemy.x2 = (enemy.x + 2).min(14);
                                            enemy.y1 = enemy.y; enemy.y2 = enemy.y;
                                        }
                                        1 => { // Вертикальный
                                            enemy.x1 = enemy.x; enemy.x2 = enemy.x;
                                            enemy.y1 = enemy.y; enemy.y2 = (enemy.y + 2).min(9);
                                        }
                                        4 => { // Фанти (Тотальное зануление)
                                            enemy.x1 = 0; enemy.x2 = 0;
                                            enemy.y1 = 0; enemy.y2 = 0;
                                        }
                                        6 => { // Куадратор (Рамка)
                                            enemy.x1 = enemy.x.saturating_sub(1);
                                            enemy.x2 = (enemy.x + 2).min(14);
                                            enemy.y1 = enemy.y.saturating_sub(1);
                                            enemy.y2 = (enemy.y + 2).min(9);
                                        }
                                        _ => {}
                                    }
                                }
                            });

                            ui.add_space(2.0);

                            // --- ОТРЕСОВКА ИНСПЕКТОРА НА ОСНОВЕ ВЫЧИСЛЕННОГО РЕЖИМА ---
                            match current_ai_mode {
                                0 => {
                                    ui.horizontal(|ui| {
                                        ui.small("Границы по X:");
                                        ui.add(egui::DragValue::new(&mut enemy.x1).clamp_range(0..=14).prefix("от "));
                                        ui.add(egui::DragValue::new(&mut enemy.x2).clamp_range(0..=14).prefix("до "));
                                    });
                                }
                                1 => {
                                    ui.horizontal(|ui| {
                                        ui.small("Границы по Y:");
                                        ui.add(egui::DragValue::new(&mut enemy.y1).clamp_range(0..=9).prefix("от "));
                                        ui.add(egui::DragValue::new(&mut enemy.y2).clamp_range(0..=9).prefix("до "));
                                    });
                                }
                                6 => {
                                    ui.small("📐 Габариты прямоугольной рамки:");
                                    ui.horizontal(|ui| {
                                        ui.add(egui::DragValue::new(&mut enemy.x1).clamp_range(0..=14).prefix("Лево X1:"));
                                        ui.add(egui::DragValue::new(&mut enemy.x2).clamp_range(0..=14).prefix("Право X2:"));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.add(egui::DragValue::new(&mut enemy.y1).clamp_range(0..=9).prefix("Верх Y1:"));
                                        ui.add(egui::DragValue::new(&mut enemy.y2).clamp_range(0..=9).prefix("Низ Y2:"));
                                    });
                                }
                                4 => {
                                    ui.colored_label(egui::Color32::LIGHT_BLUE, "👻 Режим Фанти: преследует игрока по всему экрану.\nКоординаты лимитов сброшены в 0 для экспорта.");
                                }
                                _ => {}
                            }
                        });
                        ui.add_space(4.0);
                    }

                    if let Some(idx) = to_remove { screen_data.enemies.remove(idx); }
                });
            }
        }
    });
}
