use crate::models::ProjectData;
use eframe::egui;

mod canvas;
mod sidebar;
pub mod types;

/// Интерактивный HUD-конструктор экрана ZX Spectrum с динамической синхронизацией
pub fn render_hud_editor(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    hud_frame: &Option<egui::TextureHandle>,
) {
    let drag_id = egui::Id::new("hud_editor_string_drag_state");
    let current_drag = ui.ctx().data(|d| d.get_temp::<String>(drag_id));

    // СБОРКА СТАТИЧЕСКИХ МЕТАДАННЫХ (Больше не заимствует project!)
    let hud_elements = vec![
        types::HudItemMetadata {
            id: "life",
            label: "❤️ Жизнь (LIFE)",
            icon: "❤️",
            width_blocks: 2,
            color: egui::Color32::from_rgb(200, 40, 40),
        },
        types::HudItemMetadata {
            id: "objects",
            label: "📦 Предметы (OBJECTS)",
            icon: "🌟",
            width_blocks: 2,
            color: egui::Color32::from_rgb(40, 120, 200),
        },
        types::HudItemMetadata {
            id: "objects_icon",
            label: "🎨 Иконка предметов (OBJECTS_ICON)",
            icon: "🖼️",
            width_blocks: 2,
            color: egui::Color32::from_rgb(160, 40, 160),
        },
        types::HudItemMetadata {
            id: "keys",
            label: "🔑 Ключи (KEYS)",
            icon: "🔑",
            width_blocks: 2,
            color: egui::Color32::from_rgb(180, 140, 20),
        },
        types::HudItemMetadata {
            id: "killed",
            label: "💀 Убито врагов (KILLED)",
            icon: "💀",
            width_blocks: 2,
            color: egui::Color32::from_rgb(100, 100, 100),
        },
        types::HudItemMetadata {
            id: "ammo",
            label: "🔫 Патроны (AMMO)",
            icon: "🔫",
            width_blocks: 2,
            color: egui::Color32::from_rgb(40, 160, 60),
        },
        types::HudItemMetadata {
            id: "timer",
            label: "⏱️ Таймер (TIMER)",
            icon: "⏱️",
            width_blocks: 2,
            color: egui::Color32::from_rgb(40, 140, 160),
        },
    ];

    ui.horizontal_top(|ui| {
        // Передаем ссылку на project и метаданные раздельно
        sidebar::render_sidebar(ui, &hud_elements, &mut project.config.hud_rendering);

        ui.add_space(16.0);

        // Передаем метаданные и состояние перетаскивания на холст
        canvas::render_canvas(
            ui,
            &hud_elements,
            &mut project.config.hud_rendering,
            hud_frame,
            drag_id,
            current_drag,
        );
    });
}
