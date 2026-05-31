use super::states::{CustomTab, MapEditMode, Tab, WizardStep};
use crate::app::menu_bar::{AppTranslations, Language};
use crate::core::validator::ClashError;
use crate::models::ProjectData;
use crate::ui::configurator::ConfigTab;
use egui_dock::DockState;

pub struct ZxIdeApp {
    pub project: ProjectData,
    pub current_tab: Tab,
    pub selected_screen: usize,
    pub selected_tile: u8,
    pub script_text: String,
    pub status_message: String,
    pub wizard_active: bool,
    pub wizard_step: WizardStep,
    pub clash_errors: Vec<ClashError>,
    pub dock_state: DockState<CustomTab>,

    pub map_edit_mode: MapEditMode,
    pub cyber_palette_index: usize,
    pub selected_enemy_type: u8,
    pub selected_hotspot_type: u8,

    pub tileset_texture: Option<eframe::egui::TextureHandle>,
    pub sliced_tile_textures: Vec<eframe::egui::TextureHandle>,

    pub sprites_texture: Option<eframe::egui::TextureHandle>,
    pub hud_frame_texture: Option<eframe::egui::TextureHandle>,

    pub enable_hotspot_items: bool,
    pub enable_hotspot_keys: bool,
    pub enable_hotspot_refills: bool,

    pub project_name: String,
    pub project_path: String,

    pub configurator_tab: ConfigTab,
    pub selected_font_char_idx: usize,
    pub current_language: Language,
    pub recent_projects: Vec<String>,
    pub translations: AppTranslations,
}
