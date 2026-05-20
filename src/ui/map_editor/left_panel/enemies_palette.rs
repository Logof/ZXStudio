// src/ui/map_editor/left_panel/enemies_palette.rs
use crate::models::{enemy_ai::EnemyAiType, screen::ScreenData, ProjectData};
use eframe::egui;

pub fn render(
    ui: &mut egui::Ui,
    project: &mut ProjectData, // Меняем на изменяемую ссылку, чтобы левая панель могла напрямую править врага!
    selected_enemy_sprite_slot: &mut u8,
    sprites_texture: &Option<egui::TextureHandle>,
    selected_screen: usize, // 🔥 Передаем индекс текущей комнаты
) {
    let is_top_down = project.config.movement_controls.player_genital;

    ui.label("Палитра графики (sprites.png):");
    ui.add_space(4.0);

    let mut sprite_slots = vec![
        (0, "👾 Враг Слота 1 (Спрайт 8)", 8),
        (1, "👾 Враг Слота 2 (Спрайт 10)", 10),
        (2, "👾 Враг Слота 3 (Спрайт 12)", 12),
    ];

    if is_top_down {
        sprite_slots.push((3, "👾 Враг Слота 4 (Спрайт 14)", 14));
    } else {
        sprite_slots.push((3, "🧗 Движ. Платформа (Спрайт 14)", 14));
    }

    ui.horizontal(|ui| {
        for &(slot_id, label, raw_sprite_id) in &sprite_slots {
            let mut button = egui::Button::new("");
            if *selected_enemy_sprite_slot == slot_id {
                button = button.stroke(egui::Stroke::new(
                    2.0,
                    egui::Color32::from_rgb(180, 50, 255),
                ));
            }

            let btn_res = ui.add_sized([40.0, 40.0], button).on_hover_text(label);
            if let Some(tex) = sprites_texture {
                let local_index = raw_sprite_id - 8;
                let sprite_x = (local_index as f32) * 32.0;
                let sprite_y = 16.0;

                let eps = 0.5;
                let uv_min = egui::pos2((sprite_x + eps) / 256.0, (sprite_y + eps) / 32.0);
                let uv_max = egui::pos2(
                    (sprite_x + 16.0 - eps) / 256.0,
                    (sprite_y + 16.0 - eps) / 32.0,
                );
                ui.painter().image(
                    tex.id(),
                    btn_res.rect.shrink(2.0),
                    egui::Rect::from_min_max(uv_min, uv_max),
                    egui::Color32::WHITE,
                );
            } else {
                ui.painter().text(
                    btn_res.rect.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("S{}", slot_id),
                    egui::FontId::proportional(12.0),
                    egui::Color32::GRAY,
                );
            }

            if btn_res.clicked() {
                *selected_enemy_sprite_slot = slot_id;
            }
        }
    });

    // ============================================================================
    // 🔥 ПРОМЫШЛЕННЫЙ ИНСПЕКТОР СВОЙСТВ ВЫДЕЛЕННОГО ВРАГА (ВМЕСТО ЗАВИСАЮЩИХ ОКON)
    // ============================================================================
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(4.0);

    let id_context_enemy = egui::Id::new("inspector_selected_enemy_id");
    let selected_enemy_id: Option<u64> = ui.ctx().data(|d| d.get_temp(id_context_enemy));

    if let Some(target_id) = selected_enemy_id {
        let active_scr_key = format!("screen_{}", selected_screen);

        if let Some(screen_data) = project.screens.get_mut(&active_scr_key) {
            if let Some(enemy_idx) = screen_data.enemies.iter().position(|e| e.id == target_id) {
                ui.group(|ui| {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 180, 0),
                        "📝 ИНСПЕКТОР СУЩНОСТИ",
                    );
                    ui.add_space(2.0);

                    let enemy = &mut screen_data.enemies[enemy_idx];
                    let mut current_ai = EnemyAiType::from_u8(enemy.tp);

                    // 1. Изменение интеллекта (ИИ) прямо из инспектора левой панели
                    ui.label("🧠 Поведение этого врага:");
                    egui::ComboBox::from_id_source("inspector_enemy_ai_combo")
                        .selected_text(current_ai.to_ru_name(is_top_down))
                        .show_ui(ui, |ui| {
                            for code in 1..=14 {
                                let ai_variant = EnemyAiType::from_u8(code);
                                ui.selectable_value(
                                    &mut current_ai,
                                    ai_variant,
                                    ai_variant.to_ru_name(is_top_down),
                                );
                            }
                        });
                    enemy.tp = current_ai.to_u8();

                    ui.add_space(2.0);
                    ui.small(current_ai.to_ru_description(is_top_down));
                    ui.separator();

                    // 2. Смена графического слота для выделенного врага
                    ui.label("🎨 Слот графики в sprites.png:");
                    ui.horizontal(|ui| {
                        for slot in 0..=3 {
                            ui.selectable_value(
                                &mut enemy.sprite_slot,
                                slot,
                                format!("Слот {}", slot + 1),
                            );
                        }
                    });

                    ui.add_space(4.0);
                    ui.label(format!("📍 Позиция: X: {}, Y: {}", enemy.x, enemy.y));
                    ui.label(format!("🏁 Траектория: X2: {}, Y2: {}", enemy.x2, enemy.y2));
                    ui.add_space(4.0);

                    // 3. Безопасное удаление врага с карты одной кнопкой
                    if ui.button("❌ Удалить врага с карты").clicked() {
                        screen_data.enemies.remove(enemy_idx);
                        // Снимаем выделение
                        ui.ctx()
                            .data_mut(|d| d.remove_temp::<u64>(id_context_enemy));
                    }
                });
            }
        }
    } else {
        // Подсказка для геймдизайнера, если ничего не выбрано
        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.small("💡 Совет:\nЩёлкните Правой Кнопкой Мыши (ПКЛ) по любому врагу на холсте карты, чтобы открыть его свойства в этом окне.");
            });
        });
    }
}
