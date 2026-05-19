use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.strong("📐 Проекция камеры и игровой режим");
    ui.checkbox(
        &mut project.config.movement_controls.player_genital,
        "PLAYER_GENITAL (Включить Top View / Вид сверху)",
    );

    // Дополнительные параметры вида сверху
    ui.add_enabled_ui(project.config.movement_controls.player_genital, |ui| {
        ui.horizontal(|ui| {
            ui.checkbox(
                &mut project.config.movement_controls.top_over_side,
                "TOP_OVER_SIDE (Приоритет вертикального движения)",
            );
            ui.checkbox(
                &mut project.config.movement_controls.player_bounce_with_walls,
                "PLAYER_BOUNCE_WITH_WALLS (Отскок при ударе об стену)",
            );
        });
    });

    ui.separator();
    ui.strong("🏃 Вертикальное поведение и прыжки (Side View)");

    // Блокируем выбор типа прыжка, если активен вид сверху (Top View использует только горизонтальный движок физики)
    ui.add_enabled_ui(!project.config.movement_controls.player_genital, |ui| {
        ui.label("Выберите один тип базового перемещения персонажа:");
        ui.radio_value(
            &mut project.config.movement_controls.engine_type,
            0,
            "PLAYER_BOOTEE (Постоянные авто-прыжки без остановки)",
        );
        ui.radio_value(
            &mut project.config.movement_controls.engine_type,
            1,
            "PLAYER_HAS_JUMP (Классический контролируемый прыжок)",
        );
        ui.radio_value(
            &mut project.config.movement_controls.engine_type,
            2,
            "PLAYER_HAS_JETPAC (Реактивный ранец / Джетпак)",
        );
    });

    ui.separator();
    ui.strong("⚔️ Прыжки на врагов и механики");
    ui.checkbox(
        &mut project.config.movement_controls.steps_on_enemies,
        "PLAYER_STEPS_ON_ENEMIES (Убивать врагов прыжком сверху)",
    );
    ui.add_enabled_ui(project.config.movement_controls.steps_on_enemies, |ui| {
        egui::Grid::new("steps_on_enemies_grid").show(ui, |ui| {
            ui.label("Активировать только при значении флага (0 = без флага):");
            ui.add(
                egui::DragValue::new(&mut project.config.movement_controls.player_can_step_on_flag)
                    .clamp_range(0..=31),
            );
            ui.end_row();

            ui.label("Минимальный ID убиваемого врага:");
            ui.add(
                egui::DragValue::new(&mut project.config.movement_controls.player_min_killable)
                    .clamp_range(1..=7),
            );
            ui.end_row();
        });
    });

    ui.checkbox(
        &mut project.config.movement_controls.player_step_sound,
        "PLAYER_STEP_SOUND (Включить звук шагов)",
    );
    ui.checkbox(
        &mut project.config.movement_controls.player_disable_default_heng,
        "PLAYER_DISABLE_DEFAULT_HENG (Отключить дефолтный горизонтальный движок)",
    );

    ui.separator();
    ui.strong("🕹️ Карта клавиш управления (sp_UDK keys)");
    ui.checkbox(
        &mut project.config.movement_controls.use_two_buttons,
        "USE_TWO_BUTTONS (Режим двухкнопочного джойстика)",
    );

    egui::Grid::new("keys_grid")
        .spacing([10.0, 6.0])
        .show(ui, |ui| {
            ui.label("Вверх:");
            ui.add(
                egui::TextEdit::singleline(&mut project.config.movement_controls.key_up)
                    .desired_width(60.0),
            );
            ui.label("Вниз:");
            ui.add(
                egui::TextEdit::singleline(&mut project.config.movement_controls.key_down)
                    .desired_width(60.0),
            );
            ui.end_row();

            ui.label("Влево:");
            ui.add(
                egui::TextEdit::singleline(&mut project.config.movement_controls.key_left)
                    .desired_width(60.0),
            );
            ui.label("Вправо:");
            ui.add(
                egui::TextEdit::singleline(&mut project.config.movement_controls.key_right)
                    .desired_width(60.0),
            );
            ui.end_row();

            ui.label("Огонь:");
            ui.add(
                egui::TextEdit::singleline(&mut project.config.movement_controls.key_fire)
                    .desired_width(60.0),
            );
        });

    ui.separator();
    ui.strong("📈 Субпиксельная Fixed-Point физика (Деление на 64)");

    ui.label("Горизонтальное ускорение (также применяется вертикально для Top View):");
    egui::Grid::new("horiz_physics_grid").show(ui, |ui| {
        ui.label("Макс. скорость (PLAYER_MAX_VX):");
        ui.add(egui::Slider::new(
            &mut project.config.movement_controls.player_max_vx,
            16..=512,
        ));
        ui.colored_label(
            egui::Color32::LIGHT_BLUE,
            format!(
                "{:.3} пикс/кадр",
                project.config.movement_controls.player_max_vx as f32 / 64.0
            ),
        );
        ui.end_row();

        ui.label("Ускорение (PLAYER_AX):");
        ui.add(egui::Slider::new(
            &mut project.config.movement_controls.player_ax,
            1..=128,
        ));
        ui.colored_label(
            egui::Color32::LIGHT_BLUE,
            format!(
                "{:.3} пикс/кадр²",
                project.config.movement_controls.player_ax as f32 / 64.0
            ),
        );
        ui.end_row();

        ui.label("Трение (PLAYER_RX):");
        ui.add(egui::Slider::new(
            &mut project.config.movement_controls.player_rx,
            1..=128,
        ));
        ui.colored_label(
            egui::Color32::LIGHT_BLUE,
            format!(
                "{:.3} пикс/кадр²",
                project.config.movement_controls.player_rx as f32 / 64.0
            ),
        );
        ui.end_row();
    });

    // Отображаем вертикальные константы только если не включен вид сверху
    if !project.config.movement_controls.player_genital {
        ui.add_space(4.0);
        ui.label("Вертикальное ускорение:");
        egui::Grid::new("vert_physics_grid").show(ui, |ui| {
            ui.label("Макс. скорость падения (PLAYER_MAX_VY_CAYENDO):");
            ui.add(egui::Slider::new(
                &mut project.config.movement_controls.player_max_vy_cayendo,
                64..=1024,
            ));
            ui.end_row();

            ui.label("Гравитация (PLAYER_G):");
            ui.add(egui::Slider::new(
                &mut project.config.movement_controls.player_g,
                1..=128,
            ));
            ui.end_row();

            // Константы обычного прыжка
            if project.config.movement_controls.engine_type == 1 {
                ui.label("Импульс прыжка (PLAYER_VY_INICIAL_SALTO):");
                ui.add(egui::Slider::new(
                    &mut project.config.movement_controls.player_vy_inicial_salto,
                    16..=512,
                ));
                ui.end_row();

                ui.label("Макс. скорость прыжка (PLAYER_MAX_VY_SALTANDO):");
                ui.add(egui::Slider::new(
                    &mut project.config.movement_controls.player_max_vy_saltando,
                    16..=512,
                ));
                ui.end_row();

                ui.label("Ускорение при прыжке (PLAYER_INCR_SALTO):");
                ui.add(egui::Slider::new(
                    &mut project.config.movement_controls.player_incr_salto,
                    1..=128,
                ));
                ui.end_row();
            }

            // Константы реактивного ранца
            if project.config.movement_controls.engine_type == 2 {
                ui.label("Тяга джетпака (PLAYER_INCR_JETPAC):");
                ui.add(egui::Slider::new(
                    &mut project.config.movement_controls.player_incr_jetpac,
                    1..=128,
                ));
                ui.end_row();

                ui.label("Макс. скорость джетпака (PLAYER_MAX_VY_JETPAC):");
                ui.add(egui::Slider::new(
                    &mut project.config.movement_controls.player_max_vy_jetpac,
                    16..=512,
                ));
                ui.end_row();
            }
        });
    }
}
