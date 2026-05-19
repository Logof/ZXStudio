use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.strong("📺 Настройки графического окна и координат HUD");
    ui.label("Диапазон для знакомест Spectrum: X (0-31), Y (0-23). Значение 99 отключает элемент.");
    ui.add_space(4.0);

    // Раздел 1: Глобальный игровой вьюпорт
    ui.horizontal(|ui| {
        ui.label("Игровое окно (VIEWPORT):");
        ui.label("X:");
        ui.add(
            egui::DragValue::new(&mut project.config.hud_rendering.viewport_x).clamp_range(0..=31),
        );
        ui.label("Y:");
        ui.add(
            egui::DragValue::new(&mut project.config.hud_rendering.viewport_y).clamp_range(0..=23),
        );
    });

    ui.separator();
    ui.strong("🎯 Позиции элементов интерфейса (HUD)");
    ui.add_space(4.0);

    // Массив элементов для генерации интерфейса через цикл (синхронизирован с HudRenderingConfig)
    let hud_elements = [
        (
            "❤️ Жизнь (LIFE)",
            &mut project.config.hud_rendering.life_x,
            &mut project.config.hud_rendering.life_y,
        ),
        (
            "📦 Предметы (OBJECTS)",
            &mut project.config.hud_rendering.objects_x,
            &mut project.config.hud_rendering.objects_y,
        ),
        (
            "🎨 Иконка предметов (OBJECTS_ICON)",
            &mut project.config.hud_rendering.objects_icon_x,
            &mut project.config.hud_rendering.objects_icon_y,
        ),
        (
            "🔑 Ключи (KEYS)",
            &mut project.config.hud_rendering.keys_x,
            &mut project.config.hud_rendering.keys_y,
        ),
        (
            "💀 Убито врагов (KILLED)",
            &mut project.config.hud_rendering.killed_x,
            &mut project.config.hud_rendering.killed_y,
        ),
        (
            "🔫 Патроны (AMMO)",
            &mut project.config.hud_rendering.ammo_x,
            &mut project.config.hud_rendering.ammo_y,
        ),
        (
            "⏱️ Таймер (TIMER)",
            &mut project.config.hud_rendering.timer_x,
            &mut project.config.hud_rendering.timer_y,
        ),
    ];

    egui::Grid::new("hud_coords_grid")
        .spacing([12.0, 6.0])
        .show(ui, |ui| {
            for (name, x, y) in hud_elements {
                let is_disabled = *x == 99 || *y == 99;

                if is_disabled {
                    ui.colored_label(egui::Color32::DARK_GRAY, format!("❌ {}", name));
                } else {
                    ui.label(format!("✨ {}", name));
                }

                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::DragValue::new(x).clamp_range(0..=99));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(y).clamp_range(0..=99));
                });
                ui.end_row();
            }
        });

    ui.separator();
    ui.strong("📜 Вывод текста из скриптов (LINE_OF_TEXT)");
    egui::Grid::new("text_line_grid").show(ui, |ui| {
        ui.label("Строка вывода (Y):");
        ui.add(
            egui::DragValue::new(&mut project.config.hud_rendering.line_of_text)
                .clamp_range(0..=23),
        );
        ui.end_row();

        ui.label("Смещение (X):");
        ui.add(
            egui::DragValue::new(&mut project.config.hud_rendering.line_of_text_x)
                .clamp_range(0..=31),
        );
        ui.end_row();

        ui.label("Атрибут цвета текста (ATTR):");
        ui.add(
            egui::DragValue::new(&mut project.config.hud_rendering.line_of_text_attr)
                .clamp_range(0..=255),
        );
        ui.end_row();
    });
    ui.checkbox(
        &mut project.config.hud_rendering.line_of_text_no_autoerase,
        "LINE_OF_TEXT_NO_AUTOERASE (Отключить автоочистку текста)",
    );

    ui.separator();
    ui.strong("🎨 Эффекты рендеринга и тени");
    ui.columns(2, |columns| {
        columns[0].checkbox(
            &mut project.config.hud_rendering.use_auto_shadows,
            "USE_AUTO_SHADOWS (Авто-тени атрибутов)",
        );
        columns[0].checkbox(
            &mut project.config.hud_rendering.use_auto_tile_shadows,
            "USE_AUTO_TILE_SHADOWS (Тени тайлами 32-47)",
        );
        columns[0].checkbox(
            &mut project.config.hud_rendering.unpacked_map,
            "UNPACKED_MAP (Несжатая карта)",
        );
        columns[0].checkbox(
            &mut project.config.hud_rendering.no_masks,
            "NO_MASKS (Бинарный OR без масок)",
        );

        columns[1].checkbox(
            &mut project.config.hud_rendering.masked_bullets,
            "MASKED_BULLETS (Маски для пуль)",
        );
        columns[1].checkbox(
            &mut project.config.hud_rendering.player_custom_animation,
            "PLAYER_CUSTOM_ANIMATION (Кастомная анимация)",
        );
        columns[1].checkbox(
            &mut project.config.hud_rendering.pause_abort,
            "PAUSE_ABORT (Добавить кнопки Pause/Abort)",
        );
        columns[1].checkbox(
            &mut project.config.hud_rendering.get_x_more,
            "GET_X_MORE (Показывать 'Get X More')",
        );
    });

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label("Альтернативный тайл карты:");
        ui.add(
            egui::DragValue::new(&mut project.config.hud_rendering.packed_map_alt_tile)
                .clamp_range(0..=47),
        );

        ui.add_space(20.0);
        ui.label("Анимация тайлов (первая пара):");
        ui.add(
            egui::DragValue::new(&mut project.config.hud_rendering.enable_tilanims)
                .clamp_range(0..=47),
        );
    });

    ui.separator();
    ui.horizontal(|ui| {
        ui.label("HUD_INK (Атрибут цвета шрифта интерфейса):");
        ui.add(egui::Slider::new(&mut project.config.hud_rendering.hud_ink, 0..=7).text("ID"));
    });

    let colors = [
        "Black", "Blue", "Red", "Magenta", "Green", "Cyan", "Yellow", "White",
    ];
    if project.config.hud_rendering.hud_ink < 8 {
        ui.small(format!(
            "Текущий цвет текста: {}",
            colors[project.config.hud_rendering.hud_ink as usize]
        ));
    }
}
