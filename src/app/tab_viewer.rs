use eframe::egui;
use egui_dock::TabViewer;
use super::states::{CustomTab, MapEditMode};
use crate::core::validator::ClashError;
use crate::models::{ProjectData, ScreenData};

// Импортируем компоненты UI
use crate::ui::{
    render_map_canvas, render_palette_tiles, render_palette_enemies, 
    render_script_editor, render_configurator, render_project_tree
};

pub struct ZxTabViewer<'a> {
    pub project: &'a mut ProjectData,
    pub selected_screen: &'a mut usize,
    pub selected_tile: &'a mut u8,
    pub script_text: &'a mut String,
    pub clash_errors: &'a [ClashError],
    pub status_message: &'a str,
    pub map_edit_mode: &'a mut MapEditMode,
    pub selected_enemy_type: &'a mut u8,
    pub selected_hotspot_type: &'a mut u8,
    pub tileset_texture: &'a Option<egui::TextureHandle>, 
    pub sprites_texture: &'a Option<egui::TextureHandle>,

    pub enable_hotspot_items: &'a mut bool,
    pub enable_hotspot_keys: &'a mut bool,
    pub enable_hotspot_refills: &'a mut bool,
}

impl<'a> TabViewer for ZxTabViewer<'a> {
    type Tab = CustomTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            CustomTab::ProjectTree => "📁 Проект".into(),
            CustomTab::MapCanvas => "🗺️ Конструктор мира".into(),
            CustomTab::ScriptEditor => "📜 Редактор скриптов".into(),
            CustomTab::Configurator => "⚙️ Настройки движка".into(),
            CustomTab::Console => "💻 Логи компиляции".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(tex) = self.tileset_texture {
            ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("tileset_tex"), tex.clone()));
        }

        match tab {
            // 📑 НОВЫЙ КЕЙС: ДЕРЕВО ПРОЕКТА КАК САМОСТОЯТЕЛЬНАЯ ПАНЕЛЬ ДOК-СИСТЕМЫ
            CustomTab::ProjectTree => {
                if let Some(target_tab) = render_project_tree(ui) {
                    ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("tab_switch_signal"), target_tab));
                }
            }

            // ============================================================================
            // ОБНОВЛЕННЫЙ КОНСТРУКТОР МИРА: ТЕПЕРЬ ЗАНИМАЕТ ВСЮ ШИРИНУ БЕЗ ДЕРЕВА
            // ============================================================================
            CustomTab::MapCanvas => {
                let scr_key = format!("screen_{}", self.selected_screen);

                ui.horizontal(|ui| {
                    // --- SIDEBAR ИНСТРУМЕНТОВ РИСОВАНИЯ СЛЕВА ---
                    ui.vertical(|ui| {
                        ui.set_max_width(160.0);
                        let available_height = ui.available_height();
                        
                        let height_selector = 45.0;
                        let height_tiles = available_height * 0.52;
                        let height_enemies = available_height * 0.28;
                        let height_props = available_height * 0.15;

                        ui.allocate_ui_with_layout(egui::vec2(160.0, height_selector), egui::Layout::top_down(egui::Align::Min), |ui| {
                            ui.group(|ui| {
                                ui.label("Индекс экрана:");
                                ui.add(egui::Slider::new(self.selected_screen, 0..=(self.project.map_w * self.project.map_h - 1)));
                            });
                        });

                        if let Some(new_mode) = render_palette_tiles(ui, self.project, self.selected_tile, *self.map_edit_mode == MapEditMode::Tiles, self.tileset_texture) {
                            *self.map_edit_mode = new_mode;
                        }

                        if let Some(new_mode) = render_palette_enemies(ui, self.project, self.selected_enemy_type, *self.map_edit_mode == MapEditMode::Enemies, self.sprites_texture) {
                            *self.map_edit_mode = new_mode;
                        }

                        ui.allocate_ui_with_layout(egui::vec2(160.0, height_props), egui::Layout::top_down(egui::Align::Min), |ui| {
                            ui.group(|ui| {
                                ui.label("⚙️ Параметры кисти:");
                                if *self.map_edit_mode == MapEditMode::Tiles && *self.selected_tile < 32 {
                                    let t_idx = *self.selected_tile as usize;
                                    let mut current_beh = self.project.tile_behaviours[t_idx];
                                    egui::ComboBox::from_label("")
                                        .selected_text(match current_beh { 0 => "🚶 Проходимый", 1 => "💀 Шипы", 4 => "🧗 Платформа", 8 => "🧱 Стена", _ => "Маска" })
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut current_beh, 0, "🚶 0: Walkable");
                                            ui.selectable_value(&mut current_beh, 1, "💀 1: Kills");
                                            ui.selectable_value(&mut current_beh, 4, "🧗 4: Platform");
                                            ui.selectable_value(&mut current_beh, 8, "🧱 8: Obstacle");
                                        });
                                    self.project.tile_behaviours[t_idx] = current_beh;
                                } else if *self.map_edit_mode == MapEditMode::Enemies {
                                    let screen_data = self.project.screens.entry(scr_key.clone()).or_insert_with(ScreenData::default);
                                    if !screen_data.enemies.is_empty() {
                                        if let Some(enemy) = screen_data.enemies.last_mut() {
                                            if enemy.tp == 1 || enemy.tp == 2 || enemy.tp == 3 || enemy.tp == 4 {
                                                ui.add(egui::DragValue::new(&mut enemy.x1).clamp_range(0..=14).prefix("Мин:"));
                                                ui.add(egui::DragValue::new(&mut enemy.x2).clamp_range(0..=14).prefix("Макс:"));
                                            }
                                        }
                                    } else { ui.small("Нет активных врагов."); }
                                } else { ui.small("Кликните на объект."); }
                            });
                        });
                    });

                    ui.separator();

                    // --- ХОЛСТ СЕТКИ КАРТЫ СТРАНИЦЫ ---
                    render_map_canvas(ui, self.project, self.selected_screen, self.selected_tile, self.clash_errors, self.map_edit_mode, *self.selected_enemy_type);
                });
            }
            CustomTab::ScriptEditor => { render_script_editor(ui, self.script_text, *self.selected_screen); }
            CustomTab::Configurator => { render_configurator(ui, self.project); }
            CustomTab::Console => {
                ui.heading("Логи сборки проекта");
                ui.colored_label(egui::Color32::LIGHT_BLUE, self.status_message);
            }
        }
    }
}
