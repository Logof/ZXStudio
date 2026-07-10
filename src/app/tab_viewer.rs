// src/app/tab_viewer.rs
use super::menu_bar::AppTranslations; // ... Импортируем глобальную структуру локализации
use super::states::{CustomTab, MapEditMode};
use crate::core::validator::ClashError;
use crate::models::ProjectData;
use eframe::egui;
use egui_dock::TabViewer;

// Импортируем компоненты UI
use crate::ui::{render_configurator, render_project_tree, render_script_editor};
// ИСПРАВЛЕНО: Импортируем функцию из нового декомпозированного модуля папки
use crate::ui::hud_editor::render_hud_editor;

use crate::ui::configurator::ConfigTab;

pub struct ZxTabViewer<'a> {
    pub project: &'a mut ProjectData,
    pub project_name: &'a str,
    pub project_path: &'a str,

    pub configurator_tab: &'a mut ConfigTab,

    pub selected_screen: &'a mut usize,
    pub selected_tile: &'a mut u8,
    pub script_text: &'a mut String,
    pub clash_errors: &'a [ClashError],
    pub status_message: &'a str, // Оставляем без изменений, как в исходной версии
    pub map_edit_mode: &'a mut MapEditMode,
    pub selected_enemy_type: &'a mut u8,
    pub sliced_tile_textures: &'a [egui::TextureHandle],
    pub sprites_texture: &'a Option<egui::TextureHandle>,
    pub hud_frame_texture: &'a Option<egui::TextureHandle>,
    pub selected_font_char_idx: &'a mut usize,

    pub translations: &'a AppTranslations,
    pub z88dk_path: &'a mut String,
    pub compile_command: &'a mut String,
    pub compiler_log: &'a mut String,

    pub compiler_tx: std::sync::mpsc::Sender<String>,
}

impl<'a> TabViewer for ZxTabViewer<'a> {
    type Tab = CustomTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let loc = &self.translations.tabs;
        match tab {
            CustomTab::ProjectTree => (&loc.project_tree).into(),
            CustomTab::MapCanvas => (&loc.map_canvas).into(),
            CustomTab::ScriptEditor => (&loc.script_editor).into(),
            CustomTab::Configurator => (&loc.configurator).into(),
            CustomTab::Console => (&loc.console).into(),
            CustomTab::HudEditor => (&loc.hud_editor).into(),
            CustomTab::IdeSettings => (&self.translations.ide_settings.title).into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(first_tile_tex) = self.sliced_tile_textures.first() {
            ui.ctx()
                .data_mut(|d| d.insert_temp(egui::Id::new("tileset_tex"), first_tile_tex.clone()));
        }

        match tab {
            CustomTab::ProjectTree => {
                ui.ctx().data_mut(|d| {
                    d.insert_temp(
                        egui::Id::new("translations_cache"),
                        self.translations.clone(),
                    );
                });

                // ФИКС СИГНАТУРЫ: Передаем изменяемую ссылку на project для отрисовки виртуальных нод ОЗУ
                if let Some(target_tab) = render_project_tree(ui, &self.project_path, self.project) {
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(egui::Id::new("tab_switch_signal"), target_tab)
                    });
                }
            }

            CustomTab::MapCanvas => {
                // --------------------------------------------------------------------
                // ВЕЛИКОЕ УЛУЧШЕНИЕ GUI: Выводим интерактивный бейдж активного уровня
                // --------------------------------------------------------------------
                let active_idx = self.project.current_level_index;
                let level_name = &self.project.levels[active_idx].name;
                
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.colored_label(
                        egui::Color32::from_rgb(0, 255, 255),
                        format!("▶ ТЕКУЩИЙ УРОВЕНЬ: [{}] {}", active_idx + 1, level_name)
                    );
                });
                ui.add_space(4.0);

                let max_size = ui.available_size();
                let child_rect = egui::Rect::from_min_size(ui.cursor().min, max_size);
                let mut child_ui =
                    ui.child_ui(child_rect, egui::Layout::top_down(egui::Align::LEFT));

                crate::ui::map_editor::render_map_editor(
                    &mut child_ui,
                    self.project,
                    self.selected_screen,
                    self.selected_tile,
                    self.clash_errors,
                    self.map_edit_mode,
                    self.selected_enemy_type,
                    self.sliced_tile_textures,
                    self.sprites_texture,
                );

                ui.advance_cursor_after_rect(egui::Rect::from_min_size(
                    ui.cursor().min,
                    egui::Vec2::ZERO,
                ));
            }

            CustomTab::ScriptEditor => {
                render_script_editor(ui, self.script_text, *self.selected_screen);
            }

            CustomTab::Configurator => {
                ui.ctx().data_mut(|d| {
                    d.insert_temp(
                        egui::Id::new("translations_cache"),
                        self.translations.clone(),
                    );
                });

                render_configurator(
                    ui,
                    self.project,
                    self.configurator_tab,
                    self.selected_font_char_idx,
                );
            }

            CustomTab::HudEditor => {
                render_hud_editor(ui, self.project, &self.hud_frame_texture);
            }

                        CustomTab::Console => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading(&self.translations.tabs.console);
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🗑 Очистить").clicked() {
                                self.compiler_log.clear();
                            }
                        });
                    });
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    // Скролл-зона для вывода логов в реальном времени с автопрокруткой вниз
                    egui::ScrollArea::vertical()
                        .id_source("compiler_live_console_scroll")
                        .max_width(ui.available_width())
                        .stick_to_bottom(true) // Автопрокрутка к новым ошибкам
                        .show(ui, |ui| {
                            if self.compiler_log.is_empty() {
                                ui.colored_label(
                                    egui::Color32::from_rgb(120, 120, 130),
                                    "   Консоль пуста. Нажмите кнопку сборки на панели инструментов для запуска тулчейна..."
                                );
                            } else {
                                // 🔥 МОДЕРНИЗАЦИЯ: Интерактивное многострочное поле только для чтения (Доступно выделение и копирование)
                                egui::TextEdit::multiline(self.compiler_log)
                                    .font(egui::TextStyle::Monospace) // Сохраняем ретро-шрифт терминала
                                    .id_source("compiler_log_selectable_text")
                                    .desired_width(ui.available_width())
                                    .desired_rows(10)
                                    .lock_focus(false)
                                    .interactive(true) // Разрешаем клики и выделение мыслью
                                    .frame(false)       // Прячем рамки поля ввода, оставляя вид чистого терминала
                                    .show(ui);
                            }
                        });
                });
            }


            // ============================================================================
            // ОПТИМИЗИРОВАННАЯ ОТРИСОВКА ОКНА НАСТРОЕК КОМПИЛЯТОРА С СИГНАЛЬНОЙ СИСТЕМОЙ
            // ============================================================================
            CustomTab::IdeSettings => {
                crate::ui::configurator::ide_settings::render_ide_settings(
                    ui,
                    self.compile_command,
                    self.z88dk_path,
                    self.project_path,
                    self.translations,
                    self.compiler_tx.clone(),
                );
            }
        }
    }
}
