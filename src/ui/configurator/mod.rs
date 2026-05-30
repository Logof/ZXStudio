// src/ui/configurator/mod.rs
pub mod font_editor;
pub mod general;
pub mod hud_rendering;
pub mod map_goals;
pub mod mechanics_enemies;
pub mod movement_controls;
pub mod player_physics;
pub mod shooting_boxes;
pub mod tile_behaviour;

use crate::models::ProjectData;
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigTab {
    General,
    ShootingBoxes,
    TileBehaviour,
    PlayerPhysics,
    HudRendering,
    MovementControls,
    MechanicsEnemies,
    MapGoals,
    FontEditor, // 🔥 НОВОЕ СОСТОЯНИЕ ВКЛАДКИ
}

pub fn render_configurator(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    current_tab: &mut ConfigTab,
    selected_font_char_idx: &mut usize, // 🔥 ПРИНИМАЕМ ИНДЕКС ДЛЯ РЕДАКТОРА ШРИФТОВ
) {
    egui::SidePanel::left("config_menu_panel")
        .resizable(false)
        .default_width(180.0)
        .show_inside(ui, |ui| {
            ui.vertical(|ui| {
                ui.heading("⚙ Разделы движка");
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                ui.selectable_value(current_tab, ConfigTab::General, "📋 Общие настройки");
                ui.selectable_value(current_tab, ConfigTab::TileBehaviour, "🧱 Поведение тайлов");
                ui.selectable_value(current_tab, ConfigTab::PlayerPhysics, "🏃 Физика игрока");
                ui.selectable_value(
                    current_tab,
                    ConfigTab::MovementControls,
                    "🕹 Управление / Режим",
                );
                ui.selectable_value(
                    current_tab,
                    ConfigTab::MechanicsEnemies,
                    "👾 Механика врагов",
                );
                ui.selectable_value(current_tab, ConfigTab::ShootingBoxes, "🏹 Стрельба / Боксы");
                ui.selectable_value(current_tab, ConfigTab::HudRendering, "📺 HUD / Отрисовка");
                ui.selectable_value(current_tab, ConfigTab::MapGoals, "🏆 Цели / Карта");

                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);

                // 🔥 НОВАЯ КНОПКА В САЙДБАРЕ
                ui.selectable_value(current_tab, ConfigTab::FontEditor, "🔤 Редактор шрифта");
            });
        });

    egui::CentralPanel::default().show_inside(ui, |ui| match current_tab {
        ConfigTab::General => {
            general::render(ui, project);
        }
        ConfigTab::TileBehaviour => {
            tile_behaviour::render(ui, project);
        }
        ConfigTab::PlayerPhysics => {
            player_physics::render(ui, project);
        }
        ConfigTab::MovementControls => {
            movement_controls::render(ui, project);
        }
        ConfigTab::MechanicsEnemies => {
            mechanics_enemies::render(ui, project);
        }
        ConfigTab::ShootingBoxes => {
            shooting_boxes::render(ui, project);
        }
        ConfigTab::HudRendering => {
            hud_rendering::render(ui, project);
        }
        ConfigTab::MapGoals => {
            map_goals::render(ui, project);
        }
        ConfigTab::FontEditor => {
            font_editor::render_font_editor(ui, project, selected_font_char_idx);
        }
    });
}
