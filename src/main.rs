mod app;
mod core;
mod models;
mod ui;

use app::ZxIdeApp;
use std::fs;

fn main() -> Result<(), eframe::Error> {
    // Настраиваем конфигурацию нативного окна с внутренним размером 800x600
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // Создаем системные каталоги ТЗ
    let _ = fs::create_dir_all("templates");

    let template_path = "templates/config.h.template";
    if !std::path::Path::new(template_path).exists() {
        let default_template = include_str!("../templates/config.h.template");
        let _ = fs::write(template_path, default_template);
    }

    eframe::run_native(
        "ZX Spectrum 48K/128K Единая Среда Разработки (Rust + egui)",
        native_options, // Передаем корректно настроенные опции
        Box::new(|cc| {
            // ФИКС: Проблемный вызов egui_extras удален во избежание конфликта версий egui.
            // Загрузка ресурсов теперь полностью делегирована вашему модулю asset_loader.rs.

            // ФИКС АДРЕСАЦИИ: Создаем экземпляр напрямую через импортированный модуль app
            Box::new(ZxIdeApp::new(cc))
        }),
    )
}
