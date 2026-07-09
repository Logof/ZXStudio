// src/ui/configurator/general/rendering_settings.rs
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData, is_english: bool) {
    let t_render = if is_english { "⏱️ Rendering & Frame Rate" } else { "⏱️ Рендеринг и частота кадров" };
    let t_fps_limit = if is_english { "MIN_FAPS_PER_FRAME (FPS Limiter: 1=50fps, 2=25fps)" } else { "MIN_FAPS_PER_FRAME (Ограничитель FPS: 1=50fps, 2=25fps)" };

    ui.strong(t_render);
    ui.add(egui::Slider::new(&mut project.config.general.min_faps_per_frame, 1..=4).text(t_fps_limit));
}
