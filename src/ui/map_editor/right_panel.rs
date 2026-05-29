// src/ui/map_editor/right_panel.rs
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
    sliced_tile_textures: &[egui::TextureHandle],
    sprites_texture: &Option<egui::TextureHandle>, // ФИКС: Добавлен проброс текстуры врагов
) {
    ui.vertical(|ui| {
        // Отрисовка холста карты (Передаем массив нарезанных текстур)
        render_map_canvas(
            ui,
            project,
            selected_screen,
            selected_tile,
            clash_errors,
            map_edit_mode,
            *selected_enemy_sprite_slot,
            sliced_tile_textures,
            sprites_texture, // ФИКС: Пробрасываем текстуру на холст
        );

        // Инспектор врагов комнаты (Строго 3 слота для структуры памяти MTE MK1 v5.0)
        let scr_key = format!("screen_{}", selected_screen);
        if let Some(screen_data) = project.screens.get_mut(&scr_key) {
            if !screen_data.enemies.is_empty() && *map_edit_mode == MapEditMode::Enemies {
                ui.add_space(6.0);

                let count = screen_data.enemies.len();
                let is_top_down = project.config.movement_controls.player_genital; // Считываем признак проекта

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(if is_top_down {
                            "👁️ Режим: TOP-DOWN"
                        } else {
                            "🧱 Режим: ПЛАТФОРМЕР"
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.colored_label(
                                if count == 3 {
                                    egui::Color32::LIGHT_RED
                                } else {
                                    egui::Color32::LIGHT_GREEN
                                },
                                format!("{}/3 сущностей", count),
                            );
                        });
                    });
                    ui.add_space(4.0);

                    let mut to_remove = None;

                    for (idx, enemy) in screen_data.enemies.iter_mut().enumerate() {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(180, 50, 255),
                                        format!("[Слот {}]", idx + 1),
                                    );

                                    let raw_gfx_id = match enemy.type_id {
                                        1..=4 => 8,
                                        5 | 6 => 10,
                                        7..=10 => 12,
                                        11..=14 => 14,
                                        _ => 8,
                                    };

                                    // Локализованное имя графики на основе признака проекта
                                    let gfx_name = if raw_gfx_id == 14 && !is_top_down {
                                        "Движ. Платформа (Лифт)"
                                    } else {
                                        "Враг"
                                    };

                                    ui.label(format!("{}: Спрайт 0x{:02X}", gfx_name, raw_gfx_id));
                                    ui.small(format!("Координаты: ({}, {})", enemy.x, enemy.y));

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui.button("🗑").clicked() {
                                                to_remove = Some(idx);
                                            }
                                        },
                                    );
                                });

                                ui.separator();

                                // --- ДИНАМИЧЕСКИЙ ВЫБОР ИИ НА ОСНОВЕ ПРИЗНАКА ПРОЕКТА ---
                                ui.horizontal(|ui| {
                                    ui.small("🧠 Поведение ИИ:");

                                    let mut new_ai = enemy.type_id;
                                    egui::ComboBox::from_id_source(format!(
                                        "ai_combo_calc_{}",
                                        idx
                                    ))
                                    .selected_text(match enemy.type_id {
                                        1 => "Тип 1: Горизонтальный линейный".to_string(),
                                        2 => "Тип 2: Вертикальный линейный".to_string(),
                                        3 => {
                                            if is_top_down {
                                                "Тип 3: Диагональный ход".to_string()
                                            } else {
                                                "Тип 3: Диагональный рикошет".to_string()
                                            }
                                        }
                                        4 => {
                                            if is_top_down {
                                                "Тип 4: Охотник (Преследователь)".to_string()
                                            } else {
                                                "Тип 4: Платформа / Лифт".to_string()
                                            }
                                        }
                                        6 => "Тип 6: Настоящий Призрак Фанти".to_string(),
                                        9 => "Тип 9: Куадратор (Периметр)".to_string(),
                                        11 => "Тип 11: Маррулер (Хаотичный)".to_string(),
                                        _ => format!("Тип ИИ {}", enemy.type_id),
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut new_ai,
                                            1,
                                            "Тип 1: Горизонтальный линейный",
                                        );
                                        ui.selectable_value(
                                            &mut new_ai,
                                            2,
                                            "Тип 2: Вертикальный линейный",
                                        );

                                        if is_top_down {
                                            ui.selectable_value(
                                                &mut new_ai,
                                                3,
                                                "Тип 3: Диагональный ход",
                                            );
                                            ui.selectable_value(
                                                &mut new_ai,
                                                4,
                                                "Тип 4: Охотник (Преследователь)",
                                            );
                                        } else {
                                            ui.selectable_value(
                                                &mut new_ai,
                                                3,
                                                "Тип 3: Диагональный рикошет",
                                            );
                                            ui.selectable_value(
                                                &mut new_ai,
                                                4,
                                                "Тип 4: Платформа / Лифт",
                                            );
                                        }

                                        ui.separator();
                                        ui.selectable_value(
                                            &mut new_ai,
                                            6,
                                            "Тип 6: Настоящий Призрак Фанти",
                                        );
                                        ui.selectable_value(
                                            &mut new_ai,
                                            9,
                                            "Тип 9: Куадратор (Периметр)",
                                        );
                                        ui.selectable_value(
                                            &mut new_ai,
                                            11,
                                            "Тип 11: Маррулер (Хаотичный)",
                                        );
                                    });

                                    if new_ai != enemy.type_id {
                                        enemy.type_id = new_ai;
                                        // Сброс геометрии ручек под правила выбранного ИИ
                                        match new_ai {
                                            1 => {
                                                enemy.x1 = enemy.x;
                                                enemy.x2 = (enemy.x + 2).min(14);
                                                enemy.y1 = enemy.y;
                                                enemy.y2 = enemy.y;
                                            }
                                            2 => {
                                                enemy.x1 = enemy.x;
                                                enemy.x2 = enemy.x;
                                                enemy.y1 = enemy.y;
                                                enemy.y2 = (enemy.y + 2).min(9);
                                            }
                                            3 => {
                                                enemy.x1 = enemy.x;
                                                enemy.x2 = (enemy.x + 2).min(14);
                                                enemy.y1 = enemy.y;
                                                enemy.y2 = (enemy.y + 2).min(9);
                                            }
                                            4 => {
                                                if is_top_down {
                                                    enemy.x1 = enemy.x;
                                                    enemy.x2 = enemy.x;
                                                    enemy.y1 = enemy.y;
                                                    enemy.y2 = enemy.y;
                                                } else {
                                                    enemy.x1 = enemy.x;
                                                    enemy.x2 = enemy.x;
                                                    enemy.y1 = enemy.y;
                                                    enemy.y2 = (enemy.y + 2).min(9);
                                                }
                                            }
                                            5 | 6 => {
                                                enemy.x1 = 0;
                                                enemy.x2 = 0;
                                                enemy.y1 = 0;
                                                enemy.y2 = 0;
                                            }
                                            _ => {
                                                enemy.x1 = enemy.x;
                                                enemy.x2 = (enemy.x + 2).min(14);
                                                enemy.y1 = enemy.y;
                                                enemy.y2 = enemy.y;
                                            }
                                        }
                                    }
                                });
                            });
                        });
                    }

                    if let Some(del_idx) = to_remove {
                        screen_data.enemies.remove(del_idx);
                    }
                });
            }
        }
    });
}
