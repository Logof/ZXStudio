mod app;
mod core;
mod models;
mod ui;

use app::ZxIdeApp;
use std::fs;

fn main() -> Result<(), eframe::Error> {
    // Создаем системные каталоги ТЗ
    let _ = fs::create_dir_all("templates");

    let template_path = "templates/config.h.template";
    if !std::path::Path::new(template_path).exists() {
        let default_template = include_str!("../templates/config.h.template");
        let _ = fs::write(template_path, default_template);
    }

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ZX Spectrum 48K/128K Единая Среда Разработки (Rust + egui)",
        options,
        Box::new(|cc| Box::new(ZxIdeApp::new(cc))),
    )
}
