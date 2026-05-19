// src/ui/configurator/mechanics_enemies.rs
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.strong("🧱 Физика интерактивного окружения");
    ui.add_space(4.0);

    ui.checkbox(
        &mut project.config.mechanics_enemies.player_check_map_boundaries,
        "PLAYER_CHECK_MAP_BOUNDARIES (Запретить выход за границы карты)",
    );
    ui.checkbox(
        &mut project.config.mechanics_enemies.direct_to_play,
        "DIRECT_TO_PLAY (Стартовый экран совмещен с игровой рамкой)",
    );
    ui.checkbox(
        &mut project.config.mechanics_enemies.map_bottom_kills,
        "MAP_BOTTOM_KILLS (Падение за нижнюю границу экрана убивает)",
    );
    ui.checkbox(
        &mut project.config.mechanics_enemies.walls_stop_enemies,
        "WALLS_STOP_ENEMIES (Враги разворачиваются перед стенами)",
    );
    ui.checkbox(
        &mut project.config.mechanics_enemies.everything_is_a_wall,
        "EVERYTHING_IS_A_WALL (Любой тайл кроме пустой ячейки #0 — стена)",
    );

    ui.separator();
    ui.strong("🧲 Предметы, Ключи и Триггеры");
    ui.add_space(4.0);

    // Двухколоночный макет с обращением по индексам columns[0] и columns[1]
    ui.columns(2, |columns| {
        columns[0].checkbox(
            &mut project.config.mechanics_enemies.deactivate_keys,
            "DEACTIVATE_KEYS (Убрать ключи)",
        );
        columns[0].checkbox(
            &mut project.config.mechanics_enemies.deactivate_objects,
            "DEACTIVATE_OBJECTS (Убрать предметы)",
        );
        columns[0].checkbox(
            &mut project.config.mechanics_enemies.deactivate_refills,
            "DEACTIVATE_REFILLS (Убрать аптечки)",
        );

        columns[1].checkbox(
            &mut project.config.mechanics_enemies.only_one_object,
            "ONLY_ONE_OBJECT (Носить только 1 предмет)",
        );
        columns[1].checkbox(
            &mut project.config.mechanics_enemies.reverse_objects_count,
            "REVERSE_OBJECTS_COUNT (Обратный отсчет)",
        );

        // Реактивный чекбокс CUSTOM_LOCK_CLEAR
        let mut lock_clear = project.config.mechanics_enemies.custom_lock_clear;
        if columns[1]
            .checkbox(&mut lock_clear, "CUSTOM_LOCK_CLEAR (Кастомный скрипт)")
            .changed()
        {
            project.config.mechanics_enemies.custom_lock_clear = lock_clear;

            if lock_clear {
                // ИСПРАВЛЕНО [E0502]: Берем контекст у изолированной колонки columns[1], а не у родительского ui!
                columns[1].ctx().data_mut(|d| {
                    d.insert_temp(egui::Id::new("trigger_create_lock_clear"), true);
                });
            }
        }
    });

    ui.add_space(4.0);
    egui::Grid::new("items_flags_grid").show(ui, |ui| {
        ui.label("Индекс флага для счетчика предметов (OBJECT_COUNT):");
        ui.add(
            egui::DragValue::new(&mut project.config.mechanics_enemies.object_count)
                .clamp_range(0..=31),
        );
        ui.end_row();

        ui.label("Записывать счетчик убитых в флаг (BODY_COUNT_ON):");
        ui.add(
            egui::DragValue::new(&mut project.config.mechanics_enemies.body_count_on)
                .clamp_range(0..=31),
        );
        if project.config.mechanics_enemies.body_count_on == 0 {
            ui.small("(Выключен)");
        }
        ui.end_row();
    });

    ui.separator();
    ui.strong("🛸 Летающие враги: Особый тип Fanties (Тип 6)");
    ui.checkbox(
        &mut project.config.mechanics_enemies.enable_fanties,
        "ENABLE_FANTIES (Активировать летающих преследователей)",
    );

    ui.add_enabled_ui(project.config.mechanics_enemies.enable_fanties, |ui| {
        egui::Grid::new("fanties_grid").show(ui, |ui| {
            ui.label("Базовый фрейм спрайта врага:");
            ui.add(
                egui::DragValue::new(&mut project.config.mechanics_enemies.fanties_base_cell)
                    .clamp_range(0..=3),
            );
            ui.end_row();

            ui.label("Дистанция обнаружения (пиксели):");
            ui.add(
                egui::DragValue::new(&mut project.config.mechanics_enemies.fanties_sight_distance)
                    .clamp_range(16..=255),
            );
            ui.end_row();

            ui.label("Максимальная скорость полета (V):");
            ui.add(
                egui::DragValue::new(&mut project.config.mechanics_enemies.fanties_max_v)
                    .clamp_range(32..=512),
            );
            ui.end_row();

            ui.label("Ускорение полета (A):");
            ui.add(
                egui::DragValue::new(&mut project.config.mechanics_enemies.fanties_a)
                    .clamp_range(1..=64),
            );
            ui.end_row();

            ui.label("Прочность летуна (HP / выстрелы):");
            ui.add(
                egui::DragValue::new(&mut project.config.mechanics_enemies.fanties_life_gauge)
                    .clamp_range(1..=20),
            );
            ui.end_row();
        });
        ui.checkbox(
            &mut project.config.mechanics_enemies.fanties_type_homing,
            "FANTIES_TYPE_HOMING (Включить векторное самонаведение на игрока)",
        );
    });

    ui.separator();
    ui.strong("🏃 Преследователи: Особый тип Pursuers (Тип 7)");
    ui.checkbox(
        &mut project.config.mechanics_enemies.enable_pursuers,
        "ENABLE_PURSUERS (Активировать врагов типа 7)",
    );

    ui.add_enabled_ui(project.config.mechanics_enemies.enable_pursuers, |ui| {
        egui::Grid::new("pursuers_grid").show(ui, |ui| {
            ui.label("Макс. скорость (1, 2, 4):");
            ui.add(egui::Slider::new(
                &mut project.config.mechanics_enemies.pursuers_max_v,
                1..=4,
            ));
            ui.end_row();

            ui.label("Базовый фрейм спрайта:");
            ui.add(
                egui::DragValue::new(&mut project.config.mechanics_enemies.pursuers_base_cell)
                    .clamp_range(0..=3),
            );
            ui.end_row();

            ui.label("Задержка респавна (DEATH_COUNT_ADD):");
            ui.add(egui::DragValue::new(
                &mut project.config.mechanics_enemies.death_count_add,
            ));
            ui.end_row();
        });
        ui.checkbox(
            &mut project
                .config
                .mechanics_enemies
                .pursuers_dont_spawn_in_obstacle,
            "Запретить спавн преследователя внутри препятствий",
        );
    });

    ui.separator();
    ui.strong("🔫 Стационарные турели: Orthoshooters");
    ui.checkbox(
        &mut project.config.mechanics_enemies.enable_orthoshooters,
        "ENABLE_ORTHOSHOOTERS (Активировать турели)",
    );
    ui.add_enabled_ui(
        project.config.mechanics_enemies.enable_orthoshooters,
        |ui| {
            ui.horizontal(|ui| {
                ui.label("Базовый фрейм спрайта (99 — скрыть графику):");
                ui.add(
                    egui::DragValue::new(
                        &mut project.config.mechanics_enemies.orthoshooters_base_cell,
                    )
                    .clamp_range(0..=99),
                );
            });
        },
    );
}
