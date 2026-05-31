use super::states::{CustomTab, WizardStep};
use super::ZxIdeApp;
use crate::core::{export_config_h, export_enems_h, save_project_json};
use crate::models::ProjectData;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Ru,
    En,
}

// --- СТРУКТУРЫ ПОЛНОЙ ГЛОБАЛЬНОЙ ЛОКАЛИЗАЦИИ ---

#[derive(Deserialize, Debug, Clone)]
pub struct MenuSection {
    pub project: String,
    pub new_project: String,
    pub open_project: String,
    pub recent_projects: String,
    pub empty_list: String,
    pub save_json: String,
    pub gen_c_code: String,
    pub close_project: String,
    pub exit: String,
    pub utilities: String,
    pub clear_cache: String,
    pub window: String,
    pub reset_layout: String,
    pub lang_select: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TabsSection {
    pub map_canvas: String,
    pub script_editor: String,
    pub configurator: String,
    pub hud_editor: String,
    pub console: String,
    pub project_tree: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WizardSection {
    pub welcome: String,
    pub create_desc: String,
    pub open_desc: String,
    pub select_platform: String,
    pub next_btn: String,
    pub back_btn: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusSection {
    pub init: String,
    pub saved: String,
    pub save_err: String,
    pub c_generated: String,
    pub closed: String,
    pub cache_cleared: String,
    pub layout_reset: String,
    pub loading: String,
    pub script_opened: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppTranslations {
    pub menu: MenuSection,
    pub tabs: TabsSection,
    pub wizard: WizardSection,
    pub status: StatusSection,
}

impl AppTranslations {
    pub fn load(lang: Language) -> Self {
        let json_content = match lang {
            Language::Ru => include_str!("../../locales/ru.json"),
            Language::En => include_str!("../../locales/en.json"),
        };
        serde_json::from_str(json_content).expect("Ошибка парсинга глобального файла локализации")
    }
}

// ------------------------------------------------
fn load_project_file(app: &mut ZxIdeApp, path: PathBuf) -> Result<(), String> {
    // ФИКС ОБЛАСТИ ВИДИМОСТИ: Объявляем строковый путь в самом начале
    let path_string = path.to_string_lossy().to_string();

    let content =
        fs::read_to_string(&path).map_err(|e| format!("Не удалось прочитать файл: {}", e))?;

    let loaded_project: ProjectData = serde_json::from_str(&content)
        .map_err(|e| format!("Ошибка парсинга структуры JSON: {}", e))?;

    let project_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("loaded_game")
        .to_string();
    let project_path = path.parent().unwrap_or(&path).to_string_lossy().to_string();

    app.project = loaded_project;
    app.project_name = project_name;
    app.project_path = project_path;
    app.wizard_active = false;

    app.tileset_texture = None;
    app.sliced_tile_textures.clear();
    app.sprites_texture = None;
    app.hud_frame_texture = None;

    // Теперь переменная path_string гарантированно видна здесь компилятору
    app.recent_projects.retain(|p| p != &path_string);
    app.recent_projects.insert(0, path_string);
    if app.recent_projects.len() > 10 {
        app.recent_projects.truncate(10);
    }
    Ok(())
}

pub fn render_menu_bar(app: &mut ZxIdeApp, ctx: &egui::Context) {
    // ФИКС: Клонируем секции перевода меню и статусов в локальные переменные.
    // Это полностью разрывает заимствование (borrow) от переменной `app`.
    let loc = app.translations.menu.clone();
    let status_loc = app.translations.status.clone();

    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button(&loc.project, |ui| {
                if ui.button(&loc.new_project).clicked() {
                    app.wizard_active = true;
                    app.wizard_step = WizardStep::WelcomeChoice;
                    ui.close_menu();
                }

                if ui.button(&loc.open_project).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("ZX Spectrum Core Project", &["prj", "json"])
                        .pick_file()
                    {
                        match load_project_file(app, path.clone()) {
                            Ok(_) => {
                                app.status_message =
                                    format!("🚀 {}: {:?}", status_loc.loading, path);
                            }
                            Err(err) => {
                                app.status_message = format!("❌ Ошибка: {}", err);
                            }
                        }
                    }
                    ui.close_menu();
                }

                ui.menu_button(&loc.recent_projects, |ui| {
                    if app.recent_projects.is_empty() {
                        ui.label(&loc.empty_list);
                    } else {
                        let projects = app.recent_projects.clone();
                        for path_str in projects {
                            let path = PathBuf::from(&path_str);
                            let file_name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown Project");

                            if ui.button(format!("📄 {}", file_name)).clicked() {
                                match load_project_file(app, path) {
                                    Ok(_) => {
                                        app.status_message =
                                            format!("🚀 {}: {}", status_loc.loading, path_str);
                                    }
                                    Err(err) => {
                                        app.status_message = format!("❌ Ошибка: {}", err);
                                    }
                                }
                                ui.close_menu();
                            }
                        }
                    }
                });

                ui.separator();

                ui.add_enabled_ui(!app.project_path.is_empty(), |ui| {
                    if ui.button(&loc.save_json).clicked() {
                        match save_project_json(&app.project_path, &app.project_name, &app.project)
                        {
                            Ok(_) => {
                                app.status_message = format!(
                                    "{} {}/{}.prj!",
                                    status_loc.saved, &app.project_path, &app.project_name
                                );
                            }
                            Err(err) => {
                                app.status_message = format!("{}: {}", status_loc.save_err, err);
                            }
                        }
                        ui.close_menu();
                    }

                    if ui.button(&loc.gen_c_code).clicked() {
                        let _ = export_config_h(&app.project_path, &app.project);
                        let _ = export_enems_h(&app.project_path, &app.project);
                        app.status_message = status_loc.c_generated.clone();
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button(&loc.close_project).clicked() {
                        app.project = ProjectData::default();
                        app.project_name = "my_retro_game".to_string();
                        app.project_path = String::new();
                        app.wizard_active = true;
                        app.wizard_step = WizardStep::WelcomeChoice;
                        app.status_message = status_loc.closed.clone();
                        ui.close_menu();
                    }
                });

                ui.separator();
                if ui.button(&loc.exit).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button(&loc.utilities, |ui| {
                if ui.button(&loc.clear_cache).clicked() {
                    app.status_message = status_loc.cache_cleared.clone();
                    ui.close_menu();
                }
            });

            ui.menu_button(&loc.window, |ui| {
                if ui.button(&loc.reset_layout).clicked() {
                    let mut default_state = egui_dock::DockState::new(vec![
                        CustomTab::MapCanvas,
                        CustomTab::ScriptEditor,
                        CustomTab::Configurator,
                        CustomTab::HudEditor,
                    ]);
                    let surface = default_state.main_surface_mut();
                    let root_node = egui_dock::NodeIndex::root();
                    let [top_node, _bottom_node] =
                        surface.split_below(root_node, 0.80, vec![CustomTab::Console]);
                    let [_left_node, _main_work_node] =
                        surface.split_left(top_node, 0.18, vec![CustomTab::ProjectTree]);

                    app.dock_state = default_state;
                    app.status_message = status_loc.layout_reset.clone();
                    ui.close_menu();
                }
            });

            ui.menu_button(&loc.lang_select, |ui| {
                if ui
                    .radio_value(&mut app.current_language, Language::Ru, "Русский")
                    .clicked()
                {
                    app.translations = AppTranslations::load(Language::Ru);
                    ui.close_menu();
                }
                if ui
                    .radio_value(&mut app.current_language, Language::En, "English")
                    .clicked()
                {
                    app.translations = AppTranslations::load(Language::En);
                    ui.close_menu();
                }
            });
        });
    });
}
