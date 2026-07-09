// src/app/menu_bar/mod.rs
pub mod file_menu;
pub mod build_menu;
pub mod lang_menu;

use crate::app::ZxIdeApp;
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Ru,
    En,
}

#[derive(Clone)]
pub struct AppTranslations {
    pub menu: MenuTranslations,
    pub tabs: TabsTranslations,
    pub ide_settings: IdeSettingsTranslations,
}

#[derive(Clone)]
pub struct MenuTranslations {
    pub lang_select: String,
}

#[derive(Clone)]
pub struct TabsTranslations {
    pub console: String,
    pub project_tree: String,
    pub map_canvas: String,
    pub script_editor: String,
    pub configurator: String,
    pub hud_editor: String,
}

#[derive(Clone)]
pub struct IdeSettingsTranslations {
    pub title: String,
    pub compiler_section: String,
    pub compiler_path_label: String,
    pub compiler_path_btn: String,
    pub compiler_label: String,
    pub compiler_hint: String,
    pub test_btn: String,
    pub status_test_start: String,
    pub status_test_ok: String,
    pub status_test_fail: String,
}

impl AppTranslations {
    pub fn load(lang: Language) -> Self {
        match lang {
            Language::Ru => Self {
                menu: MenuTranslations {
                    lang_select: "Язык: Ru".to_string(),
                },
                tabs: TabsTranslations {
                    console: "🖨️ Консоль сборки".to_string(),
                    project_tree: "📁 Дерево проекта".to_string(),
                    map_canvas: "🗺️ Редактор карты".to_string(),
                    script_editor: "📜 Редактор скриптов".to_string(),
                    configurator: "⚙️ Настройки движка".to_string(),
                    hud_editor: "📺 HUD Интерфейс".to_string(),
                },
                ide_settings: IdeSettingsTranslations {
                    title: "⚙️ Настройки IDE".to_string(),
                    compiler_section: "🛠️ Конфигурация тулчейна Z88DK".to_string(),
                    compiler_path_label: "Путь к бинарникам Z88DK (папка bin):".to_string(),
                    compiler_path_btn: "📁 Обзор...".to_string(),
                    compiler_label: "Параметры и строка вызова компилятора:".to_string(),
                    compiler_hint: "Пример: zcc +zx -vn main.c -o game.bin".to_string(),
                    test_btn: "🧪 Тестировать вызов zcc".to_string(),
                    status_test_start: "Запуск теста компилятора...".to_string(),
                    status_test_ok: "Тест Z88DK успешен! Компилятор найден.".to_string(),
                    status_test_fail: "Ошибка теста: Проверьте правильность путей Z88DK!".to_string(),
                },
            },
            Language::En => Self {
                menu: MenuTranslations {
                    lang_select: "Language: En".to_string(),
                },
                tabs: TabsTranslations {
                    console: "🖨️ Build Console".to_string(),
                    project_tree: "📁 Project Tree".to_string(),
                    map_canvas: "🗺️ Map Canvas".to_string(),
                    script_editor: "📜 Script Editor".to_string(),
                    configurator: "⚙️ Engine Configurator".to_string(),
                    hud_editor: "📺 HUD Editor".to_string(),
                },
                ide_settings: IdeSettingsTranslations {
                    title: "⚙️ IDE Settings".to_string(),
                    compiler_section: "🛠️ Z88DK Toolchain Configuration".to_string(),
                    compiler_path_label: "Path to Z88DK binaries (bin folder):".to_string(),
                    compiler_path_btn: "📁 Browse...".to_string(),
                    compiler_label: "Compiler arguments string:".to_string(),
                    compiler_hint: "Example: zcc +zx -vn main.c -o game.bin".to_string(),
                    test_btn: "🧪 Test zcc execution".to_string(),
                    status_test_start: "Starting compiler test...".to_string(),
                    status_test_ok: "Z88DK test successful! Compiler found.".to_string(),
                    status_test_fail: "Test failed: Please verify your Z88DK paths!".to_string(),
                },
            },
        }
    }
}

pub fn render_menu_bar(app: &mut ZxIdeApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("global_menu_bar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(20, 20, 25)).inner_margin(2.0))
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                file_menu::render(ui, app);
                build_menu::render(ui, app);
                lang_menu::render(ui, app);
            });
        });
}
