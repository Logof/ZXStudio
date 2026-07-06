use crate::models::ProjectData;
use eframe::egui;

pub fn render(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_tile: &mut u8,
    sliced_tile_textures: &[egui::TextureHandle],
) {
    ui.label("Палитра тайлов (work.png):");
    ui.add_space(4.0);

    // ИСПРАВЛЕНО ПОД МУЛЬТИЛЕВЕЛ: Выделяем контекст комнат активного уровня
    let active_idx = project.current_level_index;
    let current_level = &mut project.levels[active_idx];
    let mode = current_level.tile_mode;

    // Получаем общее количество тайлов, физически присутствующих в текущем файле/режиме
    let total_tiles = match mode {
        crate::models::project::TileMode::Packed16 => 20, // 16 для карты + 4 спец-тайла
        crate::models::project::TileMode::Packed16WithShadows => sliced_tile_textures.len(),
        crate::models::project::TileMode::Extended48 => 48,
    };

    // Лимит для обычной отрисовки стен/декора (15 или 47)
    let max_paintable = mode.max_paintable_tile_index();

    // БЕЗОПАСНОСТЬ КИСТИ: Если индекс вышел за рамки полного набора графики, сбрасываем в 0
    if *selected_tile as usize >= total_tiles {
        *selected_tile = 0;
    }

    // Выводим сетку блоков строго по 4 штуки в ряд
    egui::Grid::new("palette_grid").spacing([6.0, 6.0]).show(ui, |ui| {
        let mut grid_index = 0;

        for t in 0..total_tiles {
            let is_standard_tile = t <= max_paintable;

            // Стилизуем кнопку тайла
            let mut button = egui::Button::new("");
            if *selected_tile as usize == t {
                // Выделяем активную кисть синим цветом
                button = button.stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 150, 255)));
            } else if !is_standard_tile {
                // Серый пунктир для служебных тайлов, чтобы визуально отличать их от обычных стен
                button = button.stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(120)));
            }

            // ИСПРАВЛЕНО: сохраняем мутабельный ответ для цепочки вызовов hover
            let mut btn_response = ui.add_sized([36.0, 36.0], button);

            // Рендерим текстуру тайла
            if let Some(tex) = sliced_tile_textures.get(t) {
                let tint = if is_standard_tile { egui::Color32::WHITE } else { egui::Color32::from_gray(180) };
                ui.painter().image(tex.id(), btn_response.rect.shrink(2.0), egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), tint);
            } else {
                ui.painter().text(btn_response.rect.center(), egui::Align2::CENTER_CENTER, format!("{:02}", t), egui::FontId::proportional(12.0), egui::Color32::GRAY);
            }

            // Добавляем понятный Hover-текст, перезаписывая btn_response (устранение E0382)
            if !is_standard_tile {
                btn_response = match t {
                    16 => { btn_response.on_hover_text("❤️ Тайл авто-генерации ЖИЗНИ / ПЕРЕЗАРЯДКИ.\nКликните им на карту, чтобы разместить хотспот.") }
                    17 => { btn_response.on_hover_text("🌟 Тайл авто-генерации ПРЕДМЕТА.\nКликните им на карту, чтобы разместить хотспот.") }
                    18 => { btn_response.on_hover_text("🔑 Тайл авто-генерации КЛЮЧА.\nКликните им на карту, чтобы разместить хотспот.") }
                    19 => { btn_response.on_hover_text("🎨 Тайл случайного фона №19.\n(Используется движком аппаратно).") }
                    _ => { btn_response.on_hover_text("⚙️ Служебный тайл движка.") }
                };
            } else {
                btn_response = btn_response.on_hover_text(format!("Тайл карты №{}", t));
            }

            // РАЗРЕШАЕМ КЛИК: Теперь выбрать можно ЛЮБОЙ тайл палитры!
            if t != 19 || is_standard_tile {
                if btn_response.clicked() {
                    *selected_tile = t as u8;
                }
            }

            grid_index += 1;
            if grid_index % 4 == 0 {
                ui.end_row();
            }
        }
    });

    // --- УНИВЕРСАЛЬНЫЙ ИНСПЕКТОР ФИЗИЧЕСКИХ СВОЙСТВ ТАЙЛА ---
    if (*selected_tile as usize) < current_level.tile_behaviours.len() {
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);
        ui.label(format!("🧱 Свойства тайла №{:02}:", selected_tile));

        let t_idx = *selected_tile as usize;
        let mut current_beh = current_level.tile_behaviours[t_idx];

        egui::ComboBox::from_id_source("tile_physics_combo")
            .selected_text(match current_beh {
                0 => "🚶 0: Walkable",
                1 => "💀 1: Kills",
                2 => "🌿 2: Hide (Кусты/Маскировка)",
                4 => "🧗 4: Platform",
                8 => "🧱 8: Obstacle",
                10 => "⚙️ 10: Interactable",
                16 => "💥 16: Destructible",
                _ => "Маска физики (Комбинированная)",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut current_beh, 0, "🚶 0: Walkable (Проходимый)");
                ui.selectable_value(&mut current_beh, 1, "💀 1: Kills (Шипы/Лава)");
                ui.selectable_value(
                    &mut current_beh,
                    2,
                    "🌿 2: Hide (Скрывает игрока от врагов)",
                );
                ui.selectable_value(&mut current_beh, 4, "🧗 4: Platform (Полупроходимый)");
                ui.selectable_value(&mut current_beh, 8, "🧱 8: Obstacle (Стена/Блок)");
                ui.selectable_value(&mut current_beh, 10, "⚙️ 10: Interactable (Спец-объект)");
                ui.selectable_value(
                    &mut current_beh,
                    16,
                    "💥 16: Destructible (Разрушаемый выстрелом)",
                );
            });

        current_level.tile_behaviours[t_idx] = current_beh;

        // ИНСПЕКТОР РОЛЕЙ ДВИЖКА СТРОГО ПО LA CHURRERA MTE MK1 С УЧЕТОМ РЕЖИМА
        let current_tile = *selected_tile;
        let is_extended = mode == crate::models::project::TileMode::Extended48;

        let target_role_tile = if is_extended {
            if current_tile >= 14 && current_tile <= 18 {
                Some(current_tile)
            } else {
                None
            }
        } else {
            if current_tile >= 14 && current_tile <= 19 {
                Some(current_tile)
            } else {
                None
            }
        };

        if let Some(role_tile) = target_role_tile {
            ui.add_space(6.0);
            ui.group(|ui| {
                ui.label("⚙️ Настройки La Churrera:");

                match role_tile {
                    14 => {
                        ui.checkbox(&mut current_level.role_pushbox_active, "📦 PLAYER_PUSH_BOXES")
                            .on_hover_text("Активировать ящик. Игрок сможет толкать этот тайл.");
                    }
                    15 => {
                        ui.checkbox(&mut current_level.role_lock_active, "🔑 ACTIVATE_KEYS_AND_LOCKS")
                            .on_hover_text("Активировать замок. Препятствие, исчезающее при касании замка ключом.");
                    }
                    16 => {
                        if is_extended {
                            ui.checkbox(&mut current_level.role_refill_active, "❤️ REFILLS_WORK")
                                .on_hover_text("Активировать регенерацию здоровья при взятии тайла.");
                        } else {
                            ui.label("ℹ️ Настройка роли доступна в секции спец-тайлов палитры.");
                        }
                    }
                    17 => {
                        if is_extended {
                            ui.checkbox(&mut current_level.role_collectable_active, "🌟 ITEMS_WORK")
                                .on_hover_text("Активировать сбор предметов. Тайл станет подбираемым лутом.");
                        } else {
                            ui.label("ℹ️ Настройка роли доступна в секции спец-тайлов палитры.");
                        }
                    }
                    18 => {
                        if is_extended {
                            ui.checkbox(&mut current_level.role_key_active, "🔑 KEYS_WORK")
                                .on_hover_text("Активировать ключи. Тайл станет ключом для дверей.");
                        } else {
                            ui.label("ℹ️ Настройка роли доступна в секции спец-тайлов палитры.");
                        }
                    }
                    19 => {
                        if !is_extended {
                            let mut temp_decor = true;
                            ui.checkbox(&mut temp_decor, "🎨 RANDOM_DECORATION_T19")
                                .on_hover_text("Случайная подмена пустого тайла 0 на тайл 19 для красоты.");
                        }
                    }
                    _ => {}
                }
                ui.small(format!("Фиксированная роль в режиме {}", mode.name()));
            });
        }
    }
}
