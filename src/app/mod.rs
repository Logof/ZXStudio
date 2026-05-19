pub mod states;
pub mod tab_viewer;
pub mod wizard;
pub mod menu_bar;
pub mod toolbar;
mod app_struct;
mod theme;
mod asset_loader;

// Экспортируем структуру приложения наружу для всего остального проекта
pub use app_struct::ZxIdeApp;

use eframe::egui;
use egui_dock::{DockArea, DockState, Style};
use states::{Tab, WizardStep, MapEditMode, CustomTab};
use tab_viewer::ZxTabViewer;
use crate::models::ProjectData;
use menu_bar::render_menu_bar;
use toolbar::render_toolbar;

impl ZxIdeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dock_state: Option<DockState<CustomTab>> = cc.storage
            .and_then(|storage| eframe::get_value(storage, "dock_state"));

        let final_dock_state = dock_state.unwrap_or_else(Self::create_default_layout);

        Self {
            project: ProjectData::default(),
            current_tab: Tab::MapEditor,
            selected_screen: 0,
            selected_tile: 1,
            script_text: "ENTERING SCREEN 0\nIF FLAG 1 = 0\nTHEN\n\tSET TILE (5, 5) = 14\nEND".to_string(),
            status_message: "IDE успешно инициализирована".to_string(),
            wizard_active: true,
            wizard_step: WizardStep::WelcomeChoice,
            clash_errors: Vec::new(),
            dock_state: final_dock_state,
            map_edit_mode: MapEditMode::Tiles,
            cyber_palette_index: 0,
            selected_enemy_type: 0,
            selected_hotspot_type: 1,

            tileset_texture: None,
            sprites_texture: None,
            hud_frame_texture: None,

            enable_hotspot_items: true,
            enable_hotspot_keys: true,
            enable_hotspot_refills: true,

            project_name: "my_retro_game".to_string(),
            project_path: String::new(),
        }
    }

    pub fn create_default_layout() -> DockState<CustomTab> {
        let mut default_state = DockState::new(vec![
            CustomTab::MapCanvas,
            CustomTab::ScriptEditor,
            CustomTab::Configurator,
        ]);
        let surface = default_state.main_surface_mut();
        let root_node = egui_dock::NodeIndex::root();

        let [top_node, _bottom_node] = surface.split_below(root_node, 0.80, vec![CustomTab::Console]);
        let [_left_node, _main_work_node] = surface.split_left(top_node, 0.18, vec![CustomTab::ProjectTree]);

        default_state
    }
}

impl eframe::App for ZxIdeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "dock_state", &self.dock_state);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.15);

        // 1. Применяем шрифты и оформление из модуля темы
        theme::apply_modern_dark_theme(ctx);

        // 2. Запускаем фоновый конвейер ассетов из модуля загрузчика
        asset_loader::process_asset_loading(self, ctx);

        // 3. Рендерим приветственный Мастер (Wizard), если проект ещё не открыт
        if self.wizard_active {
            egui::CentralPanel::default()
                .frame(egui::Frame::none().fill(egui::Color32::from_rgb(14, 14, 17)))
                .show(ctx, |ui| {
                    self.render_project_wizard(ctx);
                });
            return;
        }

        // 4. Главный рабочий экран IDE (если визард пройден)
        render_menu_bar(self, ctx);
        render_toolbar(self, ctx);

        // Нижний статус-бар
        egui::TopBottomPanel::bottom("status_bar")
            .frame(egui::Frame::none().inner_margin(6.0).fill(egui::Color32::from_rgb(22, 22, 26)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&self.status_message);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let numblocks = (16 * 10) + (self.project.config.engine.max_bullets * 5);
                        ui.label(format!("NUMBLOCKS: {}", numblocks));
                    });
                });
            });

        // Основное рабочее пространство DockArea
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
                    project_name: &self.project_name,
                    project_path: &self.project_path,
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
                    hud_frame_texture: &self.hud_frame_texture,
                    enable_hotspot_items: &mut self.enable_hotspot_items,
                    enable_hotspot_keys: &mut self.enable_hotspot_keys,
                    enable_hotspot_refills: &mut self.enable_hotspot_refills,
                };

                DockArea::new(&mut self.dock_state)
                    .style(dock_style)
                    .show_inside(ui, &mut viewer);

                if let Some(target_tab) = ui.ctx().data_mut(|d| d.remove_temp::<CustomTab>(egui::Id::new("tab_switch_signal"))) {
                    if let Some(tab_coordinates) = self.dock_state.find_tab(&target_tab) {
                        self.dock_state.set_active_tab(tab_coordinates);
                    }
                }
            });
    }
}
