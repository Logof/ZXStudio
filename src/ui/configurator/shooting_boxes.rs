use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    // Взаимосвязь движка: Проверяем режим вида сверху из модуля movement_controls
    let is_top_view = project.config.movement_controls.player_genital;

    if is_top_view {
        ui.colored_label(
            egui::Color32::LIGHT_RED,
            "⚠️ Внимание: Подсистема стрельбы несовместима с видом сверху (PLAYER_GENITAL)!",
        );
    }

    ui.add_enabled_ui(!is_top_view, |ui| {
        ui.strong("🔫 Боевой движок и Стрельба");
        ui.checkbox(
            &mut project.config.shooting_boxes.player_can_fire,
            "PLAYER_CAN_FIRE (Разрешить стрельбу игроку)",
        );

        ui.add_enabled_ui(project.config.shooting_boxes.player_can_fire, |ui| {
            egui::Grid::new("shooting_flags_grid").show(ui, |ui| {
                ui.label("Доступ к стрельбе по флагу скрипта (0 = всегда доступна):");
                ui.add(
                    egui::DragValue::new(&mut project.config.shooting_boxes.player_can_fire_flag)
                        .clamp_range(0..=31),
                );
                ui.end_row();
            });

            egui::Grid::new("shooting_grid").show(ui, |ui| {
                ui.label("Макс. пуль на экране:");
                ui.add(egui::Slider::new(
                    &mut project.config.shooting_boxes.max_bullets,
                    1..=6,
                ));
                ui.end_row();

                ui.label("Скорость пули (пикселей на кадр):");
                ui.add(egui::Slider::new(
                    &mut project.config.shooting_boxes.player_bullet_speed,
                    1..=16,
                ));
                ui.end_row();

                ui.label("Прочность стандартного врага (HP):");
                ui.add(egui::Slider::new(
                    &mut project.config.shooting_boxes.enemies_life_gauge,
                    1..=20,
                ));
                ui.end_row();

                ui.label("Смещение пули по Y от верха игрока:");
                ui.add(egui::DragValue::new(
                    &mut project.config.shooting_boxes.player_bullet_y_offset,
                ));
                ui.end_row();
            });

            ui.checkbox(
                &mut project.config.shooting_boxes.can_fire_up,
                "CAN_FIRE_UP (Разрешить огонь вертикально вверх и по диагонали)",
            );

            ui.checkbox(
                &mut project.config.shooting_boxes.limited_bullets,
                "LIMITED_BULLETS (Ограничить время жизни/дальность пули фреймами)",
            );
            ui.add_enabled_ui(project.config.shooting_boxes.limited_bullets, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Время жизни (кадры):");
                    ui.add(egui::DragValue::new(
                        &mut project.config.shooting_boxes.lb_frames,
                    ));
                    ui.label("Или задать флагом:");
                    ui.add(egui::DragValue::new(
                        &mut project.config.shooting_boxes.lb_frames_flag,
                    ));
                });
            });

            ui.separator();
            ui.label("Ограничение боезапаса (AMMO):");
            egui::Grid::new("ammo_grid").show(ui, |ui| {
                ui.label("Макс. патронов (0 = бесконечно):");
                ui.add(egui::DragValue::new(
                    &mut project.config.shooting_boxes.max_ammo,
                ));
                ui.end_row();

                ui.label("Стартовый боезапас при входе в игру:");
                ui.add(egui::DragValue::new(
                    &mut project.config.shooting_boxes.initial_ammo,
                ));
                ui.end_row();

                ui.label("Объем пачек патронов на карте (tile 20):");
                ui.add(egui::DragValue::new(
                    &mut project.config.shooting_boxes.ammo_refill,
                ));
                ui.end_row();
            });
            ui.checkbox(
                &mut project.config.shooting_boxes.respawn_on_enter,
                "RESPAWN_ON_ENTER (Респавнить врагов при перезаходе на экран)",
            );
        });
    });

    ui.separator();
    ui.strong("📦 Физика ящиков (Тайл #14)");
    ui.checkbox(
        &mut project.config.shooting_boxes.player_push_boxes,
        "PLAYER_PUSH_BOXES (Разрешить перемещение ящиков)",
    );

    ui.add_enabled_ui(project.config.shooting_boxes.player_push_boxes, |ui| {
        ui.checkbox(
            &mut project.config.shooting_boxes.fire_to_push,
            "FIRE_TO_PUSH (Толкать только при удерживании кнопки ОГОНЬ)",
        );
        ui.checkbox(
            &mut project.config.shooting_boxes.enable_pushed_scripting,
            "ENABLE_PUSHED_SCRIPTING (Вызывать скриптовые триггеры при сдвиге ящика)",
        );
        ui.add_enabled_ui(
            project.config.shooting_boxes.enable_pushed_scripting,
            |ui| {
                egui::Grid::new("pushed_flags_grid").show(ui, |ui| {
                    ui.label("Флаг записи ID затертого тайла под ящиком:");
                    ui.add(egui::DragValue::new(
                        &mut project.config.shooting_boxes.moved_tile_flag,
                    ));
                    ui.end_row();
                    ui.label("Флаг сохранения новой X координаты ящика:");
                    ui.add(egui::DragValue::new(
                        &mut project.config.shooting_boxes.moved_x_flag,
                    ));
                    ui.end_row();
                    ui.label("Флаг сохранения новой Y координаты ящика:");
                    ui.add(egui::DragValue::new(
                        &mut project.config.shooting_boxes.moved_y_flag,
                    ));
                    ui.end_row();
                });
                ui.checkbox(
                    &mut project.config.shooting_boxes.pushing_action,
                    "PUSHING_ACTION (Запуск скрипта PRESS_FIRE при толчке)",
                );
            },
        );
    });

    ui.separator();
    ui.strong("🧱 Разрушаемые элементы");
    ui.checkbox(
        &mut project.config.shooting_boxes.breakable_walls,
        "BREAKABLE_WALLS (Разрушаемые выстрелами стены)",
    );
    ui.add_enabled_ui(project.config.shooting_boxes.breakable_walls, |ui| {
        egui::Grid::new("breakable_grid").show(ui, |ui| {
            ui.label("Хитов для пробития стены (минус 1):");
            ui.add(egui::Slider::new(
                &mut project.config.shooting_boxes.breakable_walls_life,
                0..=5,
            ));
            ui.end_row();

            ui.label("ID тайла стадии анимации разрушения:");
            ui.add(egui::Slider::new(
                &mut project.config.shooting_boxes.breakable_walls_breaking,
                0..=31,
            ));
            ui.end_row();

            ui.label("ID тайла, заменяющего сломанную стену:");
            ui.add(egui::Slider::new(
                &mut project.config.shooting_boxes.breakable_walls_broken,
                0..=31,
            ));
            ui.end_row();
        });
    });

    ui.separator();
    ui.strong("📜 Скриптовый движок MSC и Таймер");
    ui.checkbox(
        &mut project.config.shooting_boxes.activate_scripting,
        "ACTIVATE_SCRIPTING (Включить поддержку скриптов)",
    );
    ui.add_enabled_ui(project.config.shooting_boxes.activate_scripting, |ui| {
        egui::Grid::new("scripting_msc_grid").show(ui, |ui| {
            ui.label("Максимальное количество флагов (MAX_FLAGS):");
            ui.add(egui::Slider::new(
                &mut project.config.shooting_boxes.max_flags,
                16..=128,
            ));
            ui.end_row();

            ui.label("Клавиша выполнения действия сценария:");
            ui.horizontal(|ui| {
                ui.radio_value(&mut project.config.shooting_boxes.scripting_key, 0, "DOWN");
                ui.radio_value(&mut project.config.shooting_boxes.scripting_key, 1, "M");
                ui.radio_value(&mut project.config.shooting_boxes.scripting_key, 2, "FIRE");
                ui.radio_value(&mut project.config.shooting_boxes.scripting_key, 3, "NONE");
            });
            ui.end_row();

            ui.label("Банк/Страница скрипта (128K + сжатие уровней):");
            ui.add(egui::Slider::new(
                &mut project.config.shooting_boxes.script_page,
                1..=7,
            ));
            ui.end_row();
        });
        ui.checkbox(
            &mut project.config.shooting_boxes.enable_extern_code,
            "ENABLE_EXTERN_CODE (Разрешить вызов ассемблерных вставок EXTERN n)",
        );
        ui.checkbox(
            &mut project.config.shooting_boxes.enable_fire_zone,
            "ENABLE_FIRE_ZONE (Включить автотриггеры FIRE в зонах)",
        );
    });

    ui.add_space(4.0);
    ui.checkbox(
        &mut project.config.shooting_boxes.timer_enable,
        "TIMER_ENABLE (Включить игровой таймер)",
    );
    ui.add_enabled_ui(project.config.shooting_boxes.timer_enable, |ui| {
        egui::Grid::new("timer_params_grid").show(ui, |ui| {
            ui.label("Стартовое значение времени:");
            ui.add(egui::DragValue::new(
                &mut project.config.shooting_boxes.timer_initial,
            ));
            ui.end_row();

            ui.label("Восполнение таймера предметом (tile 21):");
            ui.add(egui::DragValue::new(
                &mut project.config.shooting_boxes.timer_refill,
            ));
            ui.end_row();

            ui.label("Интервал уменьшения (кадров на 1 тик таймера):");
            ui.add(egui::Slider::new(
                &mut project.config.shooting_boxes.timer_lapse,
                1..=64,
            ));
            ui.end_row();
        });
        ui.checkbox(
            &mut project.config.shooting_boxes.timer_start,
            "TIMER_START (Запускать отсчет с самого начала игры)",
        );
        ui.checkbox(
            &mut project.config.shooting_boxes.timer_script_0,
            "TIMER_SCRIPT_0 (При обнулении вызывать ON_TIMER_OFF скрипта)",
        );
        ui.checkbox(
            &mut project.config.shooting_boxes.show_timer_over,
            "SHOW_TIMER_OVER (Показывать надпись TIME OVER при конце времени)",
        );

        ui.label("Реакция при обнулении времени:");
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut project.config.shooting_boxes.timer_behavior,
                0,
                "TIMER_GAMEOVER_0 (Экран Game Over)",
            );
            ui.radio_value(
                &mut project.config.shooting_boxes.timer_behavior,
                1,
                "TIMER_KILL_0 (Отнять одну жизнь)",
            );
        });

        ui.add_enabled_ui(project.config.shooting_boxes.timer_behavior == 1, |ui| {
            ui.checkbox(
                &mut project.config.shooting_boxes.timer_warp,
                "Переместить (Warp) игрока на экран смерти от таймера",
            );
            ui.add_enabled_ui(project.config.shooting_boxes.timer_warp, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Экран:");
                    ui.add(egui::DragValue::new(
                        &mut project.config.shooting_boxes.timer_warp_to_screen,
                    ));
                    ui.label("X:");
                    ui.add(egui::DragValue::new(
                        &mut project.config.shooting_boxes.timer_warp_to_x,
                    ));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(
                        &mut project.config.shooting_boxes.timer_warp_to_y,
                    ));
                });
            });
            ui.checkbox(
                &mut project.config.shooting_boxes.timer_auto_reset,
                "TIMER_AUTO_RESET (Сбросить таймер в начальный после потери жизни)",
            );
        });
    });

    ui.separator();
    ui.strong("💾 Точки сохранения (Checkpoints)");
    ui.checkbox(
        &mut project.config.shooting_boxes.enable_checkpoints,
        "ENABLE_CHECKPOINTS (Активировать систему чекпоинтов)",
    );
    ui.add_enabled_ui(project.config.shooting_boxes.enable_checkpoints, |ui| {
        ui.checkbox(
            &mut project.config.shooting_boxes.cp_reset_when_dying,
            "CP_RESET_WHEN_DYING (Откатывать на чекпоинт при смерти)",
        );
        ui.checkbox(
            &mut project.config.shooting_boxes.cp_reset_also_flags,
            "CP_RESET_ALSO_FLAGS (Сбрасывать флаги MSC до состояния сохранения)",
        );
    });
}
