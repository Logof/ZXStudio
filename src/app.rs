use eframe::egui;
use crate::models::{ProjectData, config::EngineViewMode};
use crate::core::{save_project_json, export_config_h, validator::validate_attribute_clash, validator::ClashError};
use crate::ui::{render_map_editor, render_script_editor, render_configurator};

#[derive(PartialEq)]
pub enum Tab { MapEditor, ScriptEditor, Configurator }

enum WizardStep { SelectPlatform, ConfigureWorld }

pub struct ZxIdeApp {
    project: ProjectData,
    current_tab: Tab,
    selected_screen: usize,
    selected_tile: u8,
    script_text: String,
    status_message: String,
    wizard_active: bool,
    wizard_step: WizardStep,
    clash_errors: Vec<ClashError>,
}

impl Default for ZxIdeApp {
    fn default() -> Self {
        Self {
            project: ProjectData::default(),
            current_tab: Tab::MapEditor,
            selected_screen: 0,
            selected_tile: 1,
            script_text: "ENTERING SCREEN 0\nIF FLAG 1 == 0 THEN\n\tSET TILE (5, 5) = 15\nEND".to_string(),
            status_message: "Ожидание создания проекта...".to_string(),
            wizard_active: true,
            wizard_step: WizardStep::SelectPlatform,
            clash_errors: Vec::new(),
        }
    }
}

impl ZxIdeApp {
    pub fn new() -> Self { Self::default() }

    fn setup_custom_styles(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 35);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 50);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 60, 70);
        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(200, 200, 210);
        visuals.selection.bg_fill = egui::Color32::from_rgb(0, 122, 204);
        visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
        visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
        visuals.window_rounding = egui::Rounding::same(6.0);
        ctx.set_visuals(visuals);
    }

    fn render_project_wizard(&mut self, ctx: &egui::Context) {
        egui::Window::new("🧙 Мастер создания нового проекта")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false).resizable(false).default_width(450.0)
            .show(ctx, |ui| {
                match self.wizard_step {
                    WizardStep::SelectPlatform => {
                        ui.label("Выберите целевую платформу:");
                        ui.radio_value(&mut self.project.is_128k, false, "💾 ZX Spectrum 48K");
                        ui.radio_value(&mut self.project.is_128k, true, "🎹 ZX Spectrum 128K (Музыка AY)");
                        ui.add_space(10.0);
                        ui.label("Режим камеры:");
                        ui.radio_value(&mut self.project.view_mode, EngineViewMode::SideView, "🧗 Side View (Платформер)");
                        ui.radio_value(&mut self.project.view_mode, EngineViewMode::TopView, "🗺️ Top View (Вид сверху)");
                        ui.add_space(20.0);
                        if ui.button("Далее ➡️").clicked() { self.wizard_step = WizardStep::ConfigureWorld; }
                    }
                    WizardStep::ConfigureWorld => {
                        ui.label("Размеры карты в экранах:");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut self.project.map_w).clamp_range(1..=16));
                            ui.label("Ширина");
                            ui.add(egui::DragValue::new(&mut self.project.map_h).clamp_range(1..=16));
                            ui.label("Высота");
                        });
                        ui.add_space(20.0);
                        if ui.button("🚀 Создать проект!").clicked() {
                            let total_screens = self.project.map_w * self.project.map_h;
                            self.project.screens.clear();
                            for i in 0..total_screens {
                                self.project.screens.insert(format!("screen_{}", i), crate::models::ScreenData::default());
                            }
                            self.wizard_active = false;
                            self.status_message = "Проект успешно инициализирован".to_string();
                        }
                    }
                }
            });
    }
}

impl eframe::App for ZxIdeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.setup_custom_styles(ctx);

        if self.wizard_active {
            egui::CentralPanel::default().show(ctx, |_ui| {});
            self.render_project_wizard(ctx);
            return;
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("📁 Файл", |ui| {
                    if ui.button("Новый проект...").clicked() {
                        self.wizard_active = true;
                        self.wizard_step = WizardStep::SelectPlatform;
                        ui.close_menu();
                    }
                    if ui.button("Сохранить").clicked() {
                        let _ = save_project_json(&self.project);
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Выход").clicked() { std::process::exit(0); }
                });
            });
        });

        egui::TopBottomPanel::top("toolbar")
            .frame(egui::Frame::none().inner_margin(4.0).fill(egui::Color32::from_rgb(22, 22, 26)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("💾 Сохранить JSON").clicked() {
                        let _ = save_project_json(&self.project);
                        self.status_message = "Проект сохранен".to_string();
                    }
                    if ui.button("⚙️ Экспорт config.h").clicked() {
                        let _ = export_config_h(&self.project);
                        self.status_message = "Заголовки сгенерированы по шаблону".to_string();
                    }
                    if ui.button("🔍 Валидация Attribute Clash").clicked() {
                        match validate_attribute_clash("gfx/title.png") {
                            Ok(errors) => {
                                self.clash_errors = errors;
                                self.status_message = format!("Сканирование завершено. Найдено ошибок: {}", self.clash_errors.len());
                            }
                            Err(_) => self.status_message = "Положите проверочный файл в gfx/title.png".to_string(),
                        }
                    }
                    ui.separator();
                    ui.radio_value(&mut self.current_tab, Tab::MapEditor, "🗺️ Карта");
                    ui.radio_value(&mut self.current_tab, Tab::ScriptEditor, "📜 Скрипты");
                    ui.radio_value(&mut self.current_tab, Tab::Configurator, "⚙️ Баланс");
                });
            });

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

        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(10.0).fill(egui::Color32::from_rgb(28, 28, 33)))
            .show(ctx, |ui| {
                match self.current_tab {
                    Tab::MapEditor => render_map_editor(ui, &mut self.project, &mut self.selected_screen, &mut self.selected_tile, &self.clash_errors),
                    Tab::ScriptEditor => render_script_editor(ui, &mut self.script_text, self.selected_screen),
                    Tab::Configurator => render_configurator(ui, &mut self.project),
                }
            });
    }
                  }
