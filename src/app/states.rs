use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    MapEditor,
    ScriptEditor,
    Configurator,
    HudEditor,
}

pub enum WizardStep {
    WelcomeChoice,
    NameAndPath,
    SelectPlatform,
    ConfigureWorld,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MapEditMode {
    Tiles,
    Enemies,
    Eraser,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomTab {
    ProjectTree,
    MapCanvas,
    ScriptEditor,
    Configurator,
    HudEditor,
    Console,
    IdeSettings,
}
