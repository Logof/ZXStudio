// src/ui/configurator/mod.rs
use crate::models::project::TileMode;
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

            // ============================================================================
            // НОВОЕ УЛУЧШЕНИЕ: Селектор режимов тайлов прямо над содержимым вкладки Общие
            // ============================================================================
            if *current_tab == ConfigTab::General {
                ui.group(|ui| {
                    ui.strong("⚙️ Глобальный формат тайлсета:");
                    ui.add_space(4.0);

                    let mut selected_mode = project.tile_mode;

                    egui::ComboBox::from_id_source("global_tile_mode_selector")
                        .selected_text(selected_mode.name())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut selected_mode,
                                TileMode::Packed16,
                                TileMode::Packed16.name(),
                            );
                            ui.selectable_value(
                                &mut selected_mode,
                                TileMode::Packed16WithShadows,
                                TileMode::Packed16WithShadows.name(),
                            );
                            ui.selectable_value(
                                &mut selected_mode,
                                TileMode::Extended48,
                                TileMode::Extended48.name(),
                            );
                        });

                    if selected_mode != project.tile_mode {
                        // 1. Фиксируем новый режим в метаданных проекта
                        project.tile_mode = selected_mode;

                        // 2. БЕЗОПАСНАЯ МИГРАЦИЯ ДАННЫХ: Перестраиваем вектор поведений
                        let target_count = selected_mode.behaviours_count();
                        if project.tile_behaviours.len() < target_count {
                            project.tile_behaviours.resize(target_count, 0);
                        } else if project.tile_behaviours.len() > target_count {
                            project.tile_behaviours.truncate(target_count);
                        }

                        // 3. ОТПРАВЛЯЕМ СИГНАЛЫ СБРОСА И ОЧИСТКИ ПАЛИТРЫ
                        ui.ctx().data_mut(|d| {
                            d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                            // Новый флаг: принудительно очистить вектор нарезанных текстур
                            d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
                        });
                    }

                    ui.add_space(2.0);
                    ui.small(selected_mode.description());
                });
                ui.add_space(10.0);
            }

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
