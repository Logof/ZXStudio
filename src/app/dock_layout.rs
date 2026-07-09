// src/app/dock_layout.rs
use crate::app::states::CustomTab;
use eframe::egui;
use egui_dock::{DockState, Style};

/// Собирает дефолтную геометрию рабочих зон
pub fn create_default_layout() -> DockState<CustomTab> {
    let mut default_state = DockState::new(vec![
        CustomTab::MapCanvas,
        CustomTab::ScriptEditor,
        CustomTab::Configurator,
        CustomTab::IdeSettings,
    ]);
    let surface = default_state.main_surface_mut();
    let root_node = egui_dock::NodeIndex::root();

    let [top_node, _bottom_node] =
        surface.split_below(root_node, 0.80, vec![CustomTab::Console]);
    let [_left_node, _main_work_node] =
        surface.split_left(top_node, 0.18, vec![CustomTab::ProjectTree]);

    default_state
}

/// Настраивает Cyberpunk-эстетику (скругления в ноль, матовые Neon-разделители)
pub fn configure_dock_style(ui: &egui::Ui) -> Style {
    let mut dock_style = Style::from_egui(ui.style());
    dock_style.separator.width = 3.0;
    dock_style.separator.color_idle = egui::Color32::from_rgb(30, 30, 35);
    dock_style.separator.color_hovered = egui::Color32::from_rgb(0, 150, 255);
    dock_style.tab_bar.bg_fill = egui::Color32::from_rgb(20, 20, 25);
    dock_style.tab.active.bg_fill = egui::Color32::from_rgb(14, 14, 17);
    dock_style.tab.active.rounding = egui::Rounding::same(0.0);
    dock_style.tab.inactive.rounding = egui::Rounding::same(0.0);
    dock_style.tab.focused.rounding = egui::Rounding::same(0.0);
    dock_style
}
