use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.strong("📏 Физические хитбоксы (Bounding Boxes)");
    ui.label("Тип маски коллизий персонажа внутри ячейки 16x16 (выберите один):");
    ui.add_space(4.0);

    // Логика переключения макросов хитбоксов как Радио-кнопок на bool-флагах модели (PlayerPhysicsConfig)
    let mut current_bbox_type = 0; // 0 = Сплошной 16х16 (если все макросы закомментированы)
    if project.config.player_physics.bounding_box_8_bottom {
        current_bbox_type = 1;
    } else if project.config.player_physics.bounding_box_8_centered {
        current_bbox_type = 2;
    } else if project.config.player_physics.bounding_box_12x2_centered {
        current_bbox_type = 3;
    }

    let mut selected_bbox = current_bbox_type;

    ui.vertical(|ui| {
        if ui
            .radio_value(
                &mut selected_bbox,
                0,
                "Нормальный хитбокс (полные 16x16, макросы отключены)",
            )
            .clicked()
        {
            project.config.player_physics.bounding_box_8_bottom = false;
            project.config.player_physics.bounding_box_8_centered = false;
            project.config.player_physics.bounding_box_12x2_centered = false;
        }
        if ui
            .radio_value(
                &mut selected_bbox,
                1,
                "BOUNDING_BOX_8_BOTTOM (8x8 снизу по центру ячейки)",
            )
            .clicked()
        {
            project.config.player_physics.bounding_box_8_bottom = true;
            project.config.player_physics.bounding_box_8_centered = false;
            project.config.player_physics.bounding_box_12x2_centered = false;
        }
        if ui
            .radio_value(
                &mut selected_bbox,
                2,
                "BOUNDING_BOX_8_CENTERED (8x8 строго по центру ячейки)",
            )
            .clicked()
        {
            project.config.player_physics.bounding_box_8_bottom = false;
            project.config.player_physics.bounding_box_8_centered = true;
            project.config.player_physics.bounding_box_12x2_centered = false;
        }
        if ui
            .radio_value(
                &mut selected_bbox,
                3,
                "BOUNDING_BOX_12X2_CENTERED (Узкий 12x2 по центру ячейки)",
            )
            .clicked()
        {
            project.config.player_physics.bounding_box_8_bottom = false;
            project.config.player_physics.bounding_box_8_centered = false;
            project.config.player_physics.bounding_box_12x2_centered = true;
        }
    });

    ui.add_space(4.0);
    ui.checkbox(
        &mut project.config.player_physics.small_collision,
        "SMALL_COLLISION (Переключить общие коллизии с 12х12 на 8х8)",
    );

    ui.separator();
    ui.strong("💥 Поведение при получении повреждений");

    // Взаимоисключение: отскок (bounces) и мерцание (flickers) не должны работать вместе
    if ui
        .checkbox(
            &mut project.config.player_physics.player_bounces,
            "PLAYER_BOUNCES (Игрок отлетает/отскакивает от врагов)",
        )
        .changed()
        && project.config.player_physics.player_bounces
    {
        project.config.player_physics.player_flickers = false;
    }

    // Взаимосвязь: Дополнительный тонкий тюнинг отскока
    ui.add_enabled_ui(project.config.player_physics.player_bounces, |ui| {
        ui.indent("bounce_properties", |ui| {
            ui.checkbox(
                &mut project.config.player_physics.full_bounce,
                "FULL_BOUNCE (Отскок на максимальной скорости VX)",
            );
            ui.checkbox(
                &mut project.config.player_physics.slow_drain,
                "SLOW_DRAIN (Урон наносится в 4 раза медленнее во время отскока)",
            );
        });
    });

    if ui
        .checkbox(
            &mut project.config.player_physics.player_flickers,
            "PLAYER_FLICKERS (Мерцание неуязвимости вместо физических отскоков)",
        )
        .changed()
        && project.config.player_physics.player_flickers
    {
        project.config.player_physics.player_bounces = false;
        project.config.player_physics.full_bounce = false;
        project.config.player_physics.slow_drain = false;
    }

    ui.separator();
    ui.strong("💀 Жизненный цикл и Спавн");
    ui.checkbox(
        &mut project.config.player_physics.die_and_respawn,
        "DIE_AND_RESPAWN (Мгновенная смерть и респавн на текущем экране)",
    );
    ui.checkbox(
        &mut project.config.player_physics.safe_spot_on_entering,
        "SAFE_SPOT_ON_ENTERING (Запись безопасной точки при входе на новый экран)",
    );
}
