use crate::models::ProjectData;
use eframe::egui;

pub fn render(
    ui: &mut egui::Ui,
    project: &ProjectData,
    selected_enemy_sprite_slot: &mut u8,
    sprites_texture: &Option<egui::TextureHandle>,
) {
    ui.label("Палитра графики (sprites.png):");
    ui.add_space(4.0);

    let mut sprite_slots = vec![
        (0, "👾 Враг Слота 1 (Спрайт 8)", 8),
        (1, "👾 Враг Слота 2 (Спрайт 10)", 10),
        (2, "👾 Враг Слота 3 (Спрайт 12)", 12),
    ];

    if project.config.movement_controls.player_genital {
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

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    // Извлекаем или инициализируем дефолтный тип ИИ (по умолчанию = 1)
    let id_ai = egui::Id::new("default_enemy_ai_type_ctx");
    let mut current_default_ai = ui.ctx().data(|d| d.get_temp::<u8>(id_ai)).unwrap_or(1);

    // ВЫПАДАЮЩИЙ СПИСОК ТИПОВ ПОВЕДЕНИЯ ДВИЖКА (Capítulo 5)
    ui.group(|ui| {
        ui.label("🧠 Поведение по умолчанию при спавне:");

        egui::ComboBox::from_id_source("default_ai_combobox")
            .selected_text(match current_default_ai {
                1..=4 => format!("Тип {} (0x{:02X}) - Линейный", current_default_ai, current_default_ai),
                5 => "Тип 5 (0x05) - Random Respawn".to_string(),
                6 => "Тип 6 (0x06) - Fanties Призрак".to_string(),
                7..=10 => format!("Тип {} (0x{:02X}) - Куадратор", current_default_ai, current_default_ai),
                11..=14 => format!("Тип {} (0x{:02X}) - Маррулер", current_default_ai, current_default_ai),
                _ => format!("Тип {}", current_default_ai)
            })
            .show_ui(ui, |ui| {
                ui.label("Линейные траектории:");
                ui.selectable_value(&mut current_default_ai, 1, "Тип 1 (0x01): Линейный ИИ");
                ui.selectable_value(&mut current_default_ai, 2, "Тип 2 (0x02): Линейный ИИ");
                ui.selectable_value(&mut current_default_ai, 3, "Тип 3 (0x03): Линейный ИИ");
                let t4_name = if project.config.movement_controls.player_genital {
                    "Тип 4 (0x04): Линейный ИИ"
                } else {
                    "Тип 4 (0x04): Платформа / Лифт"
                };

                ui.selectable_value(&mut current_default_ai, 4, t4_name);

                ui.separator();
                ui.label("Воладоры / Летающие:");
                ui.selectable_value(&mut current_default_ai, 5, "Тип 5 (0x05): Random Respawn Призрак");
                ui.selectable_value(&mut current_default_ai, 6, "Тип 6 (0x06): Настоящий Fanty Призрак");

                ui.separator();
                ui.label("Куадраторы (По внешнему борту):");
                ui.selectable_value(&mut current_default_ai, 7, "Тип 7 (0x07): Куадратор");
                ui.selectable_value(&mut current_default_ai, 8, "Тип 8 (0x08): Куадратор");
                ui.selectable_value(&mut current_default_ai, 9, "Тип 9 (0x09): Куадратор");
                let t10_name = if project.config.movement_controls.player_genital {
                    "Тип 10 (0x0A): Куадратор ИИ"
                } else {
                    "Тип 10 (0x0A): Пл. Куадратор (Лифт)"
                };
                ui.selectable_value(&mut current_default_ai, 10, t10_name);

                ui.separator();
                ui.label("Патруллеры марруллеры (Хаотичный ход):");
                ui.selectable_value(&mut current_default_ai, 11, "Тип 11 (0x0B): Маррулер");
                ui.selectable_value(&mut current_default_ai, 12, "Тип 12 (0x0C): Маррулер");
                ui.selectable_value(&mut current_default_ai, 13, "Тип 13 (0x0D): Маррулер");
                ui.selectable_value(&mut current_default_ai, 14, "Тип 14 (0x0E): Маррулер");
            });

        // Сохраняем измененный выбор обратно в контекст
        ui.ctx().data_mut(|d| d.insert_temp(id_ai, current_default_ai));

        ui.add_space(2.0);
        let ai_desc = match current_default_ai {
            1..=4 => "↔️/↕️ Линейные:\nХодят туда-обратно. Если X1!=X2 и Y1!=Y2, рикошетят по диагонали.",
            5 => "👻 Random Respawn:\nАвто-матушка при входе. Спавнится за пределами экрана, если убить линейного врага.",
            6 => "👻 Настоящий Fanty:\nСпавнится в точке установки и преследует. При потере агро летит назад в X1/Y1.",
            7..=10 => "🔄 Куадратор:\nХодит строго по внешнему периметру рамки. Вращение зависит от вектора диагонали.",
            11..=14 => "🌀 Маррулер хаотичный:\nХодит наобум до упора в препятствие, после чего меняет вектор хода.",
            _ => ""
        };
        ui.small(ai_desc);
    });
}
