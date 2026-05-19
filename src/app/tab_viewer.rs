use super::states::{CustomTab, MapEditMode};
use crate::core::validator::ClashError;
use crate::models::{ProjectData, ScreenData};
use eframe::egui;
use egui_dock::TabViewer;

// Импортируем компоненты UI
use crate::ui::{render_configurator, render_project_tree, render_script_editor};
// 🆕 ИСПРАВЛЕНО: Импортируем функцию из нового декомпозированного модуля папки
use crate::ui::hud_editor::render_hud_editor;

use crate::ui::configurator::ConfigTab;
use crate::ui::map_editor::render_map_editor;

pub struct ZxTabViewer<'a> {
    pub project: &'a mut ProjectData,
    pub project_name: &'a str,
    pub project_path: &'a str,

    pub configurator_tab: &'a mut ConfigTab, // Ссылка на вкладку настроек движка

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
    pub hud_frame_texture: &'a Option<egui::TextureHandle>,

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
            CustomTab::HudEditor => "📺 HUD-Конструктор".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(tex) = self.tileset_texture {
            ui.ctx()
                .data_mut(|d| d.insert_temp(egui::Id::new("tileset_tex"), tex.clone()));
        }

        match tab {
            // 📑 ДЕРЕВО ПРОЕКТА КАК САМОСТОЯТЕЛЬНАЯ ПАНЕЛЬ ДOК-СИСТЕМЫ
            CustomTab::ProjectTree => {
                if let Some(target_tab) = render_project_tree(ui, &self.project_path) {
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(egui::Id::new("tab_switch_signal"), target_tab)
                    });
                }
            }

            // ============================================================================
            // КОНСТРУКТОР МИРА: ЗАНИМАЕТ ВСЮ ШИРИНУ БЕЗ ДЕРЕВА
            // ============================================================================
            CustomTab::MapCanvas => {
                crate::ui::map_editor::render_map_editor(
                    ui,
                    self.project,
                    self.selected_screen,
                    self.selected_tile,
                    self.clash_errors,
                    self.map_edit_mode,
                    self.selected_enemy_type,
                    self.tileset_texture,
                    self.sprites_texture,
                );
            }

            CustomTab::ScriptEditor => {
                render_script_editor(ui, self.script_text, *self.selected_screen);
            }
            CustomTab::Configurator => {
                // Передаем третьим параметром ссылку на активную вкладку
                render_configurator(ui, self.project, self.configurator_tab);
            }
            CustomTab::HudEditor => {
                // Вызов декомпозированного HUD-редактора из папки
                render_hud_editor(ui, self.project, &self.hud_frame_texture);
            }
            CustomTab::Console => {
                ui.heading("Логи сборки проекта");
                ui.colored_label(egui::Color32::LIGHT_BLUE, self.status_message);
            }
        }
    }
}
