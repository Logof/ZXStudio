pub mod states;
pub mod tab_viewer;
pub mod wizard;
pub mod menu_bar;
pub mod toolbar;

pub(crate) mod mod_struct {
    use egui_dock::DockState;
    use crate::models::ProjectData;
    use crate::core::validator::ClashError;
    use super::states::{Tab, WizardStep, MapEditMode, CustomTab};

    pub struct ZxIdeApp {
        pub project: ProjectData,
        pub current_tab: Tab,
        pub selected_screen: usize,
        pub selected_tile: u8,
        pub script_text: String,
        pub status_message: String,
        pub wizard_active: bool,
        pub wizard_step: WizardStep,
        pub clash_errors: Vec<ClashError>,
        pub dock_state: DockState<CustomTab>,
        
        pub map_edit_mode: MapEditMode,
        pub cyber_palette_index: usize,
        pub selected_enemy_type: u8,
        pub selected_hotspot_type: u8,
        
        pub tileset_texture: Option<eframe::egui::TextureHandle>,
        pub sprites_texture: Option<eframe::egui::TextureHandle>,

        pub enable_hotspot_items: bool,   // Включить Предметы (Тайл 17)
        pub enable_hotspot_keys: bool,    // Включить Ключи (Тайл 18)
        pub enable_hotspot_refills: bool, // Включить Аптечки (Тайл 16)
    }
}

pub use mod_struct::ZxIdeApp;

use eframe::egui;
use egui_dock::{DockArea, DockState, Style};
use states::{Tab, WizardStep, MapEditMode, CustomTab};
use tab_viewer::ZxTabViewer;
use crate::models::ProjectData;
use menu_bar::render_menu_bar;
use toolbar::render_toolbar;

impl ZxIdeApp {
    // ВАЖНО: Переписываем конструктор для поддержки cc.storage (Шаг 1 улучшения)
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Пытаемся загрузить ранее сохраненную геометрию окон из системного хранилища настроек
        let mut dock_state: Option<DockState<CustomTab>> = cc.storage
            .and_then(|storage| eframe::get_value(storage, "dock_state")); // ИСПРАВЛЕНО НА ПРЯМОЙ ВЫЗОВ


        // Если приложение запускается впервые и настроек нет — генерируем дефолтный макет
        // Пытаемся загрузить ранее сохраненную геометрию окон из системного хранилища настроек
        let mut dock_state: Option<DockState<CustomTab>> = cc.storage
            .and_then(|storage| eframe::get_value(storage, "dock_state")); // ИСПРАВЛЕНО


        let final_dock_state = dock_state.unwrap_or_else(|| {
            let mut default_state = DockState::new(vec![CustomTab::MapCanvas]); // Главное объединенное окно
            let surface = default_state.main_surface_mut();
            let root_node = egui_dock::NodeIndex::root();
            
            // Разделяем экран только на Скрипты, Баланс и Консоль ошибок
            let [center_node, _bottom_node] = surface.split_below(root_node, 0.75, vec![CustomTab::Console]);
            let [_, _right_node] = surface.split_right(center_node, 0.65, vec![CustomTab::ScriptEditor, CustomTab::Configurator]);
            default_state
        });

        Self {
            project: ProjectData::default(),
            current_tab: Tab::MapEditor,
            selected_screen: 0,
            selected_tile: 1,
            script_text: "ENTERING SCREEN 0\nIF FLAG 1 == 0 THEN\n\tSET TILE (5, 5) = 15\nEND".to_string(),
            status_message: "IDE успешно инициализирована".to_string(),
            wizard_active: true,
            wizard_step: WizardStep::SelectPlatform,
            clash_errors: Vec::new(),
            dock_state: final_dock_state, // Применяем восстановленную или дефолтную геометрию
            map_edit_mode: MapEditMode::Tiles,
            cyber_palette_index: 0,
            selected_enemy_type: 1,
            selected_hotspot_type: 1,
            
            tileset_texture: None,
            sprites_texture: None,

            enable_hotspot_items: true,
            enable_hotspot_keys: true,
            enable_hotspot_refills: true,
        }
    }

    fn apply_modern_dark_theme(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(14, 14, 17);
        visuals.window_fill = egui::Color32::from_rgb(20, 20, 25);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(28, 28, 33);
        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(180, 180, 190);
        visuals.widgets.inactive.rounding = egui::Rounding::same(2.0);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(38, 38, 45);
        visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
        visuals.selection.bg_fill = egui::Color32::from_rgb(0, 150, 255);
        ctx.set_visuals(visuals);
    }
}

// Метод Default больше не используется напрямую, но необходим для удовлетворения контрактов
impl Default for ZxIdeApp {
    fn default() -> Self {
        panic!("Используйте ZxIdeApp::new(cc) для корректной инициализации с кэшированием окон");
    }
}

impl eframe::App for ZxIdeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "dock_state", &self.dock_state);
    }


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_modern_dark_theme(ctx);

        if self.wizard_active {
            egui::CentralPanel::default().show(ctx, |_ui| {});
            self.render_project_wizard(ctx);
            return;
        }

        render_menu_bar(self, ctx);
        render_toolbar(self, ctx);

        // Статус-бар
        egui::TopBottomPanel::bottom("status_bar")
            .frame(egui::Frame::none().inner_margin(6.0).fill(egui::Color32::from_rgb(22, 22, 26)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&self.status_message);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let numblocks = (16 * 10) + (self.project.config.max_bullets * 5);
                        ui.label(format!("NUMBLOCKS: {}", numblocks));
                    });
                });
            });

        // Центральная область с восстановленной геометрией окон
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(14, 14, 17)))
            .show(ctx, |ui| {
                let mut dock_style = Style::from_egui(ui.style());
                dock_style.separator.width = 3.0;
                dock_style.separator.color_idle = egui::Color32::from_rgb(30, 30, 35);
                dock_style.separator.color_hovered = egui::Color32::from_rgb(0, 150, 255);
                dock_style.tab_bar.bg_fill = egui::Color32::from_rgb(20, 20, 25);
                dock_style.tab.active.bg_fill = egui::Color32::from_rgb(14, 14, 17);
                dock_style.tab.active.rounding = egui::Rounding::same(0.0);
                dock_style.tab.inactive.rounding = egui::Rounding::same(0.0);
                dock_style.tab.focused.rounding = egui::Rounding::same(0.0);

                let mut viewer = ZxTabViewer {
                    project: &mut self.project,
                    selected_screen: &mut self.selected_screen,
                    selected_tile: &mut self.selected_tile,
                    script_text: &mut self.script_text,
                    clash_errors: &self.clash_errors,
                    status_message: &self.status_message,
                    map_edit_mode: &mut self.map_edit_mode,
                    selected_enemy_type: &mut self.selected_enemy_type,
                    selected_hotspot_type: &mut self.selected_hotspot_type,

                    tileset_texture: &self.tileset_texture,
                    sprites_texture: &self.sprites_texture,
                    
                    enable_hotspot_items: &mut self.enable_hotspot_items,
                    enable_hotspot_keys: &mut self.enable_hotspot_keys,
                    enable_hotspot_refills: &mut self.enable_hotspot_refills,
                };

                DockArea::new(&mut self.dock_state)
                    .style(dock_style)
                    .show_inside(ui, &mut viewer);
            });
    }
}
