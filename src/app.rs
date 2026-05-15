use eframe::egui;
use egui_dock::{DockArea, DockState, Style, TabViewer};
use crate::models::ProjectData;
use crate::core::{save_project_json, export_config_h, validator::ClashError};
use crate::ui::{render_map_editor, render_script_editor, render_configurator};

// Все доступные типы окон в нашей IDE
pub enum CustomTab {
    MapCanvas,    // Главный холст 15x10
    TilePalette,  // Палитра тайлов 16x3
    ScriptEditor, // IDE скриптов .spt
    Configurator, // Настройки физики и баланса
    Console,      // Вывод ошибок и валидатора ОЗУ
}

// Структура, которая знает, как отрисовать каждую вкладку
struct ZxTabViewer<'a> {
    project: &'a mut ProjectData,
    selected_screen: &'a mut usize,
    selected_tile: &'a mut u8,
    script_text: &'a mut String,
    clash_errors: &'a [ClashError],
    status_message: &'a str,
}

impl<'a> TabViewer for ZxTabViewer<'a> {
    type Tab = CustomTab;

    // Текст на заголовке вкладки
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            CustomTab::MapCanvas => "🗺️ Холст карты".into(),
            CustomTab::TilePalette => "🎨 Палитра тайлов".into(),
            CustomTab::ScriptEditor => "📜 Редактор скриптов".into(),
            CustomTab::Configurator => "⚙️ Баланс и HUD".into(),
            CustomTab::Console => "💻 Консоль и Ошибки".into(),
        }
    }

    // Отрисовка контента внутри конкретного окна
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            CustomTab::MapCanvas => {
                // Отрисовываем холст (выделено из старого map_editor)
                let scr_key = format!("screen_{}", self.selected_screen);
                let screen_data = self.project.screens.entry(scr_key).or_insert_with(crate::models::ScreenData::default);
                
                ui.label(format!("Экран №{}", self.selected_screen));
                // Здесь рендерится интерактивный холст 15x10 (код из прошлых шагов)
                // ... [Рендеринг холста] ...
            }
            CustomTab::TilePalette => {
                ui.add(egui::Slider::new(self.selected_screen, 0..=(self.project.map_w * self.project.map_h - 1)).text("Индекс экрана"));
                ui.separator();
                // Здесь рендерится сетка палитры
                // ... [Рендеринг палитры 16x3] ...
            }
            CustomTab::ScriptEditor => {
                render_script_editor(ui, self.script_text, *self.selected_screen);
            }
            CustomTab::Configurator => {
                render_configurator(ui, self.project);
            }
            CustomTab::Console => {
                ui.heading("Логи сборки проекта");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.colored_label(egui::Color32::LIGHT_BLUE, self.status_message);
                    if !self.clash_errors.is_empty() {
                        ui.colored_label(egui::Color32::LIGHT_RED, format!("⚠️ Обнаружено коллизий цвета: {}", self.clash_errors.len()));
                    }
                });
            }
        }
    }
}

pub struct ZxIdeApp {
    project: ProjectData,
    selected_screen: usize,
    selected_tile: u8,
    script_text: String,
    status_message: String,
    wizard_active: bool,
    clash_errors: Vec<ClashError>,
    
    // Хранилище состояния геометрии окон
    dock_state: DockState<CustomTab>,
}

impl Default for ZxIdeApp {
    fn default() -> Self {
        // Задаем красивое дефолтное распределение окон на старте
        let mut dock_state = DockState::new(vec![CustomTab::MapCanvas]);
        
        // Делим экран: палитру кидаем влево, скрипты и баланс — в правые панели
        let [center, left] = dock_state.main_surface_mut().split_left(center, 0.25, vec![CustomTab::TilePalette]);
        let [_, bottom] = dock_state.main_surface_mut().split_below(center, 0.70, vec![CustomTab::Console]);
        let [_, right] = dock_state.main_surface_mut().split_right(center, 0.60, vec![CustomTab::ScriptEditor, CustomTab::Configurator]);

        Self {
            project: ProjectData::default(),
            selected_screen: 0,
            selected_tile: 1,
            script_text: "ENTERING SCREEN 0\nIF FLAG 1 == 0 THEN\n\tSET TILE (5, 5) = 15\nEND".to_string(),
            status_message: "IDE инициализирована".to_string(),
            wizard_active: false, // Отключим пока для предпросмотра темы
            clash_errors: Vec::new(),
            dock_state,
        }
    }
}
