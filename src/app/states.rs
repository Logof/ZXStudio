use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy)]
pub enum Tab { MapEditor, ScriptEditor, Configurator }

pub enum WizardStep { SelectPlatform, ConfigureWorld }

#[derive(Clone, Copy, PartialEq)]
pub enum MapEditMode {
    Tiles,
    Enemies,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomTab {
    ProjectTree,
    MapCanvas,
    ScriptEditor,
    Configurator,
    Console,
}
