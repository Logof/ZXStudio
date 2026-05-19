use eframe::egui;
use font_kit::source::SystemSource;
use font_kit::properties::Properties;
use font_kit::family_name::FamilyName;

pub fn apply_modern_dark_theme(ctx: &egui::Context) {
    let mut font_definitions = egui::FontDefinitions::default();
    let mut font_loaded = false;

    if let Ok(font) = SystemSource::new().select_best_match(
        &[FamilyName::SansSerif, FamilyName::Title("Arial".to_string())],
        &Properties::new()
    ) {
        if let Ok(handle) = font.load() {
            if let Some(raw_data) = handle.copy_font_data() {
                let font_data = egui::FontData::from_owned((*raw_data).clone());
                font_definitions.font_data.insert("system_sans".to_owned(), font_data);
                font_definitions.families.get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "system_sans".to_owned());
                font_loaded = true;
            }
        }
    }

    if font_loaded {
        ctx.set_fonts(font_definitions);
    }

    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(egui::TextStyle::Small, egui::FontId::proportional(13.0));
    style.text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(16.0));
    style.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(15.0));
    style.text_styles.insert(egui::TextStyle::Heading, egui::FontId::proportional(22.0));
    style.text_styles.insert(egui::TextStyle::Monospace, egui::FontId::monospace(15.5));
    ctx.set_style(style);

    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(14, 14, 17);
    visuals.window_fill = egui::Color32::from_rgb(20, 20, 25);
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(28, 28, 33);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(180, 180, 190);
    visuals.widgets.inactive.rounding = egui::Rounding::same(0.0);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(38, 38, 45);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
    visuals.selection.bg_fill = egui::Color32::from_rgb(0, 150, 255);
    ctx.set_visuals(visuals);
}
