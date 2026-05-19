use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.strong("🗺️ Размеры карты и условия завершения игры");
    ui.add_space(6.0);

    // Размеры карты мира в экранах (из MapGoalsConfig)
    egui::Grid::new("map_dimensions_grid").show(ui, |ui| {
        ui.label("MAP_W (Ширина мира в экранах):");
        ui.add(egui::DragValue::new(&mut project.config.map_goals.map_w).clamp_range(1..=100));
        ui.end_row();

        ui.label("MAP_H (Высота мира в экранах):");
        ui.add(egui::DragValue::new(&mut project.config.map_goals.map_h).clamp_range(1..=100));
        ui.end_row();
    });

    ui.add_space(6.0);
    ui.separator();

    // Параметры завершения игры (Раздел I из config.h)
    ui.strong("🏁 Условия финала игры");
    ui.add_space(4.0);
    egui::Grid::new("map_goals_end_conditions").show(ui, |ui| {
        ui.label("Экран финала игры (SCR_FIN):");
        ui.add(egui::DragValue::new(&mut project.config.map_goals.scr_fin).clamp_range(0..=99));
        if project.config.map_goals.scr_fin == 99 {
            ui.small("(Деактивирован)");
        }
        ui.end_row();

        ui.label("Координаты финала (X/Y):");
        ui.horizontal(|ui| {
            ui.add(
                egui::DragValue::new(&mut project.config.map_goals.player_fin_x)
                    .clamp_range(0..=99),
            );
            ui.label("X");
            ui.add(
                egui::DragValue::new(&mut project.config.map_goals.player_fin_y)
                    .clamp_range(0..=99),
            );
            ui.label("Y");
        });
        ui.end_row();

        ui.label("Квест: сколько собрать предметов (PLAYER_NUM_OBJETOS):");
        ui.add(
            egui::DragValue::new(&mut project.config.map_goals.player_num_objetos)
                .clamp_range(0..=99),
        );
        ui.end_row();
    });

    ui.separator();
    ui.strong("🚀 Точка спавна и старт");
    ui.add_space(4.0);
    ui.add(
        egui::Slider::new(&mut project.config.map_goals.scr_inicio, 0..=99)
            .text("SCR_INICIO (Стартовый экран)"),
    );

    ui.label("Координаты спавна игрока на стартовом экране (в тайлах 16х16):");
    ui.horizontal(|ui| {
        ui.add(
            egui::DragValue::new(&mut project.config.map_goals.player_ini_x).clamp_range(0..=14),
        );
        ui.label("X");
        ui.add(egui::DragValue::new(&mut project.config.map_goals.player_ini_y).clamp_range(0..=9));
        ui.label("Y");
    });

    ui.separator();
    ui.strong("🏆 Здоровье и Жизненный цикл");

    // Синхронизировано: player_life_ini изменен на player_life в соответствии с вашей моделью
    ui.add(
        egui::Slider::new(&mut project.config.map_goals.player_life, 1..=100)
            .text("PLAYER_LIFE (Стартовое HP)"),
    );
    ui.add(
        egui::Slider::new(&mut project.config.map_goals.player_refill, 1..=10)
            .text("PLAYER_REFILL (Лечение от аптечки)"),
    );

    ui.separator();
    ui.strong("📦 Система уровней игры");
    ui.checkbox(
        &mut project.config.map_goals.compressed_levels,
        "COMPRESSED_LEVELS (Включить многоуровневость)",
    );

    // Взаимосвязь: Настройки под-уровней доступны только при активном COMPRESSED_LEVELS
    ui.add_enabled_ui(project.config.map_goals.compressed_levels, |ui| {
        ui.add(
            egui::Slider::new(&mut project.config.map_goals.max_levels, 1..=8)
                .text("MAX_LEVELS (Всего уровней)"),
        );
        ui.checkbox(
            &mut project.config.map_goals.per_level_spriteset,
            "PER_LEVEL_SPRITESET (Кастомные спрайты для каждого уровня)",
        );
        ui.checkbox(
            &mut project.config.map_goals.refill_me,
            "REFILL_ME (Полное лечение при смене уровня)",
        );
        ui.checkbox(
            &mut project.config.map_goals.no_reset_stats,
            "NO_RESET_STATS (Не сбрасывать очки при переходе)",
        );
    });
}
