// src/app/menu_bar/lang_menu.rs
use crate::app::ZxIdeApp;
use crate::app::menu_bar::{Language, AppTranslations};
use eframe::egui;

pub fn render(ui: &mut egui::Ui, app: &mut ZxIdeApp) {
    // 🔥 КРИТИЧЕСКИЙ ФИКС: Клонируем строку, чтобы не блокировать структуру app для замыкания
    let current_lang_label = app.translations.menu.lang_select.clone();

    ui.menu_button(&current_lang_label, |ui| {
        if ui.selectable_label(app.current_language == Language::Ru, "Русский").clicked() {
            ui.close_menu();
            app.current_language = Language::Ru;
            app.translations = AppTranslations::load(Language::Ru);
            trigger_translations_cache_reset(ui, Language::Ru);
        }

        if ui.selectable_label(app.current_language == Language::En, "English").clicked() {
            ui.close_menu();
            app.current_language = Language::En;
            app.translations = AppTranslations::load(Language::En);
            trigger_translations_cache_reset(ui, Language::En);
        }
    });
}

fn trigger_translations_cache_reset(ui: &mut egui::Ui, lang: Language) {
    ui.ctx().data_mut(|d| {
        d.insert_temp(egui::Id::new("translations_cache"), AppTranslations::load(lang));
    });
}
