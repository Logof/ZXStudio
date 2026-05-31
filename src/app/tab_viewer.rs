// src/app/tab_viewer.rs
use super::menu_bar::AppTranslations; // Импортируем глобальную структуру локализации
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
    pub status_message: &'a str,
    pub map_edit_mode: &'a mut MapEditMode,
    pub selected_enemy_type: &'a mut u8,
    pub sliced_tile_textures: &'a [egui::TextureHandle],
    pub sprites_texture: &'a Option<egui::TextureHandle>,
    pub hud_frame_texture: &'a Option<egui::TextureHandle>,

    // ============================================================================
    // НОВОЕ УЛУЧШЕНИЕ: Изменяемая ссылка на индекс выбранного ASCII-символа.
    // Позволяет сквозным образом связать состояние приложения и редактор шрифтов.
    // ============================================================================
    pub selected_font_char_idx: &'a mut usize,

    // ============================================================================
    // ГЛОБАЛЬНАЯ ЛОКАЛИЗАЦИЯ: Ссылка на активную языковую матрицу
    // ============================================================================
    pub translations: &'a AppTranslations,
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
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        // Проверяем первый нарезанный тайл (если он есть), чтобы прокинуть в контекст
        if let Some(first_tile_tex) = self.sliced_tile_textures.first() {
            ui.ctx()
                .data_mut(|d| d.insert_temp(egui::Id::new("tileset_tex"), first_tile_tex.clone()));
        }

        match tab {
            // 📑 ДЕРЕВО ПРОЕКТА КАК САМОСТОЯТЕЛЬНАЯ ПАНЕЛЬ ДOК-СИСТЕМЫ
            CustomTab::ProjectTree => {
                // Вшиваем перевод текущего кадра в контекст egui
                ui.ctx().data_mut(|d| {
                    d.insert_temp(
                        egui::Id::new("translations_cache"),
                        self.translations.clone(),
                    );
                });

                if let Some(target_tab) = render_project_tree(ui, &self.project_path) {
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(egui::Id::new("tab_switch_signal"), target_tab)
                    });
                }
            }

            // ============================================================================
            // КОНСТРУКТОР МИРА: УНИЧТОЖАЕМ СКРОЛЛ ВКЛАДКИ ДОК-СИСТЕМЫ
            // ============================================================================
            CustomTab::MapCanvas => {
                // 1. Измеряем точные физические габариты окна, выделенного док-системой
                let max_size = ui.available_size();

                // 2. Рассчитываем жесткий прямоугольник (Rect) для отрисовки, начиная от текущего курсора
                let child_rect = egui::Rect::from_min_size(ui.cursor().min, max_size);

                // 3. Создаем изолированную область (Child Ui) с абсолютным позиционированием.
                // Находясь внутри child_ui, наш редактор карт не может вызвать появление внешних скроллов!
                let mut child_ui =
                    ui.child_ui(child_rect, egui::Layout::top_down(egui::Align::LEFT));

                // ИСПРАВЛЕНО: Пробрасываем срез нарезанных текстур тайлов в редактор карт
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

                // 4. Обманываем калькулятор размеров ScrollArea док-системы.
                // Говорим родителю, что мы якобы вообще не заняли места, чтобы он скрыл полосы прокрутки.
                ui.advance_cursor_after_rect(egui::Rect::from_min_size(
                    ui.cursor().min,
                    egui::Vec2::ZERO,
                ));
            }

            CustomTab::ScriptEditor => {
                render_script_editor(ui, self.script_text, *self.selected_screen);
            }

            CustomTab::Configurator => {
                // Вшиваем перевод для подмодулей конфигуратора
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
                // Вызов декомпозированного HUD-редактора из папки
                render_hud_editor(ui, self.project, &self.hud_frame_texture);
            }

            CustomTab::Console => {
                // Использование строк из JSON-локализации для заголовка консоли логов сборки
                ui.heading(&self.translations.tabs.console);
                ui.add_space(6.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.status_message.to_string())
                            .font(egui::FontId::monospace(11.0))
                            .desired_rows(15)
                            .lock_focus(true)
                            .desired_width(ui.available_width()),
                    );
                });
            }
        }
    }
}
