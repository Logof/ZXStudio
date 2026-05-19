// src/ui/configurator/mod.rs
use crate::models::ProjectData;
use eframe::egui;

mod general;
mod hud_rendering;
mod map_goals;
mod mechanics_enemies;
mod movement_controls;
mod player_physics;
mod shooting_boxes;
mod tile_behaviour;

/// Перечисление доступных разделов конфигуратора (7 вкладок + матрица тайлов)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigTab {
    General,
    MapGoals,
    PlayerPhysics,
    MechanicsEnemies,
    ShootingBoxes,
    MovementControls,
    HudRendering,
    TileBehaviour,
}

/// Точка входа для отрисовки конфигуратора
pub fn render_configurator(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    current_tab: &mut ConfigTab,
) {
    ui.heading("⚙️ Настройка конфигурации движка La Churrera (config.h)");
    ui.add_space(8.0);

    // Вкладки строго в ОДНУ горизонтальную строку с уменьшенным шагом (spacing)
    ui.horizontal(|ui| {
        ui.style_mut().spacing.button_padding = egui::vec2(6.0, 4.0); // Компактные отступы внутри кнопок
        ui.style_mut().spacing.item_spacing.x = 4.0; // Расстояние между кнопками-вкладками

        ui.selectable_value(current_tab, ConfigTab::General, "💾 Общие");
        ui.selectable_value(current_tab, ConfigTab::MapGoals, "🗺️ Цели");
        ui.selectable_value(current_tab, ConfigTab::PlayerPhysics, "👤 Физика");
        ui.selectable_value(current_tab, ConfigTab::MechanicsEnemies, "👾 Враги");
        ui.selectable_value(current_tab, ConfigTab::ShootingBoxes, "🔫 Стрельба");
        ui.selectable_value(current_tab, ConfigTab::MovementControls, "🕹️ Управление");
        ui.selectable_value(current_tab, ConfigTab::HudRendering, "📺 HUD");
        ui.selectable_value(current_tab, ConfigTab::TileBehaviour, "🧱 Тайлы");
    });

    ui.add_space(12.0);

    // Главный контейнер текущей вкладки (автоматически растягивается под ширину вкладок)
    egui::Frame::group(ui.style())
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(620.0));

            match current_tab {
                ConfigTab::General => general::render(ui, project),
                ConfigTab::MapGoals => map_goals::render(ui, project),
                ConfigTab::PlayerPhysics => player_physics::render(ui, project),
                ConfigTab::MechanicsEnemies => mechanics_enemies::render(ui, project),
                ConfigTab::ShootingBoxes => shooting_boxes::render(ui, project),
                ConfigTab::MovementControls => movement_controls::render(ui, project),
                ConfigTab::HudRendering => hud_rendering::render(ui, project),
                ConfigTab::TileBehaviour => tile_behaviour::render(ui, project),
            }
        });
}
