// src/app/mod.rs
pub mod app_struct;
pub mod asset_loader;
pub mod menu_bar;
pub mod states;
pub mod tab_viewer;
pub mod theme;
pub mod toolbar;
pub mod wizard;

// Новые изолированные подмодули декомпозиции
pub mod dock_layout;
pub mod world_inspector;
pub mod compiler_observer;
pub mod signal_handler;

// Экспортируем структуру приложения наружу для всего остального проекта
pub use app_struct::ZxIdeApp;

use crate::models::ProjectData;
use eframe::egui;
use egui_dock::DockState;
use menu_bar::Language;
use states::{CustomTab, Tab, WizardStep};

impl ZxIdeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dock_state: Option<DockState<CustomTab>> = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "dock_state"));

        let saved_language: Language = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "current_language"))
            .unwrap_or(Language::Ru);

        let saved_recent: Vec<String> = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "recent_projects"))
            .unwrap_or_else(Vec::new);

        let final_dock_state = dock_state.unwrap_or_else(dock_layout::create_default_layout);
        let translations = menu_bar::AppTranslations::load(saved_language);
        
        let saved_command: String = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "compile_command"))
            .unwrap_or_else(|| "zcc +zx -vn main.c -o game.tap".to_string());

        let saved_z88dk_path: String = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "z88dk_path"))
            .unwrap_or_else(String::new);

        let (compiler_tx, compiler_rx) = std::sync::mpsc::channel();

        Self {
            project: ProjectData::default(),
            current_tab: Tab::MapEditor,
            selected_screen: 0,
            selected_tile: 0,
            script_text: "ENTERING SCREEN 0\nIF FLAG 1 = 0\nTHEN\n\tSET TILE (5, 5) = 14\nEND".to_string(),
            status_message: "IDE успешно инициализирована".to_string(),
            wizard_active: true,
            wizard_step: WizardStep::WelcomeChoice,
            clash_errors: Vec::new(),
            dock_state: final_dock_state,
            map_edit_mode: states::MapEditMode::Tiles,
            cyber_palette_index: 0,
            selected_enemy_type: 0,
            selected_hotspot_type: 1,
            selected_font_char_idx: 0,
            tileset_texture: None,
            sliced_tile_textures: Vec::new(),
            sprites_texture: None,
            hud_frame_texture: None,
            enable_hotspot_items: true,
            enable_hotspot_keys: true,
            enable_hotspot_refills: true,
            project_name: "my_retro_game".to_string(),
            project_path: String::new(),
            configurator_tab: crate::ui::configurator::ConfigTab::General,
            current_language: saved_language,
            recent_projects: saved_recent,
            translations,
            compile_command: saved_command,
            z88dk_path: saved_z88dk_path,
            compiler_log: String::new(),
            compiler_tx,
            compiler_rx,
        }
    }
}

impl eframe::App for ZxIdeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "dock_state", &self.dock_state);
        eframe::set_value(storage, "current_language", &self.current_language);
        eframe::set_value(storage, "recent_projects", &self.recent_projects);
        eframe::set_value(storage, "z88dk_path", &self.z88dk_path);
        eframe::set_value(storage, "compile_command", &self.compile_command);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.15);

        // 1. АУДИТ КОЛЛИЗИЙ МИРА (Вынесено в world_inspector.rs)
        world_inspector::process_world_validation(self);

        // 2. Оформление темы и конвейер ассетов
        theme::apply_modern_dark_theme(ctx);
        asset_loader::process_asset_loading(self, ctx);

        // 3. Рендерим приветственный Мастер (Wizard)
        if self.wizard_active {
            egui::CentralPanel::default()
                .frame(egui::Frame::none().fill(egui::Color32::from_rgb(14, 14, 17)))
                .show(ctx, |_ui| {
                    self.render_project_wizard(ctx);
                });
            return;
        }

        // 4. Главный рабочий интерфейс
        menu_bar::render_menu_bar(self, ctx);
        toolbar::render_toolbar(self, ctx);

        // 5. Рендеринг статус-бара
        render_bottom_status_bar(self, ctx);

        // 6. Основное рабочее пространство DockArea
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(14, 14, 17)))
            .show(ctx, |ui| {
                let dock_style = dock_layout::configure_dock_style(ui);
                
                // 🔥 КРИТИЧЕСКИЙ ФИКС: Передаем точечные ссылки на поля во избежание Aliasing конфликта
                let safe_clash_errors = world_inspector::get_safe_clash_errors(&self.clash_errors, self.selected_screen);

                let mut viewer = tab_viewer::ZxTabViewer {
                    project: &mut self.project,
                    project_name: &self.project_name,
                    project_path: &self.project_path,
                    configurator_tab: &mut self.configurator_tab,
                    selected_screen: &mut self.selected_screen,
                    selected_tile: &mut self.selected_tile,
                    script_text: &mut self.script_text,
                    clash_errors: safe_clash_errors, // Передаем чистый локальный срез
                    status_message: &self.status_message,
                    map_edit_mode: &mut self.map_edit_mode,
                    selected_enemy_type: &mut self.selected_enemy_type,
                    sliced_tile_textures: &self.sliced_tile_textures,
                    sprites_texture: &self.sprites_texture,
                    hud_frame_texture: &self.hud_frame_texture,
                    selected_font_char_idx: &mut self.selected_font_char_idx,
                    translations: &self.translations,
                    z88dk_path: &mut self.z88dk_path,
                    compile_command: &mut self.compile_command,
                    compiler_log: &mut self.compiler_log,
                    compiler_tx: self.compiler_tx.clone(),
                };

                egui_dock::DockArea::new(&mut self.dock_state)
                    .style(dock_style)
                    .show_inside(ui, &mut viewer);

                // 7. НЕБЛОКИРУЮЩИЙ ОПРОС ЛОГОВ
                compiler_observer::poll_compiler_channels(self, ui);

                // 8. ПЕРЕХВАТ И ОБРАБОТКА UI СИГНАЛОВ
                signal_handler::handle_incoming_signals(self, ui);
            });

    }
}

/// Сервисный рендерер статус-бара подвала
fn render_bottom_status_bar(app: &mut ZxIdeApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("status_bar")
        .frame(egui::Frame::none().inner_margin(6.0).fill(egui::Color32::from_rgb(22, 22, 26)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&app.status_message);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let numblocks = (16 * 10) + (app.project.config.shooting_boxes.max_bullets * 5);
                    ui.label(format!("NUMBLOCKS: {}", numblocks));
                });
            });
        });
}
