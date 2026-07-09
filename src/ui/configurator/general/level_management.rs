// src/ui/configurator/general/level_management.rs
use crate::models::project::LevelData;
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData, is_english: bool) {
    let t_levels_title = if is_english { "🎛️ Project Levels Management" } else { "🎛️ Управление уровнями проекта" };
    let t_add_level = if is_english { "➕ Add Level" } else { "➕ Добавить уровень" };
    let t_del_level = if is_english { "❌ Delete Current" } else { "❌ Удалить текущий" };
    let t_level_label = if is_english { "Active Level:" } else { "Активный уровень:" };

    ui.strong(t_levels_title);
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        ui.label(t_level_label);
        
        let mut selected = project.current_level_index;
        egui::ComboBox::from_id_source("multilevel_selector")
            .selected_text(format!("[{}] {}", selected + 1, project.levels[selected].name))
            .show_ui(ui, |ui| {
                for i in 0..project.levels.len() {
                    ui.selectable_value(&mut selected, i, format!("[{}] {}", i + 1, project.levels[i].name));
                }
            });

        if selected != project.current_level_index {
            project.current_level_index = selected;
            super::trigger_graphics_reset(ui);
        }

        ui.add_space(10.0);
        ui.text_edit_singleline(&mut project.levels[project.current_level_index].name);
    });

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        if ui.button(t_add_level).clicked() {
            let mut new_lvl = LevelData::default();
            new_lvl.name = format!("Level {}", project.levels.len() + 1);
            
            let total_screens = project.config.map_goals.map_w * project.config.map_goals.map_h;
            new_lvl.screens.clear();
            for i in 0..total_screens {
                new_lvl.screens.insert(format!("screen_{}", i), crate::models::ScreenData::default());
            }
            
            project.levels.push(new_lvl);
            project.current_level_index = project.levels.len() - 1;
            super::trigger_graphics_reset(ui);
        }

        ui.add_enabled_ui(project.levels.len() > 1, |ui| {
            if ui.button(t_del_level).clicked() {
                project.levels.remove(project.current_level_index);
                project.current_level_index = 0;
                super::trigger_graphics_reset(ui);
            }
        });
    });
}
