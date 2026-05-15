use std::fs;
use std::io::Write;
use crate::models::ProjectData;

pub fn save_project_json(project: &ProjectData) -> Result<(), std::io::Error> {
    let mut file = fs::File::create("map/mapa.prj")?;
    let json = serde_json::to_string_pretty(project)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn export_config_h(project: &ProjectData) -> Result<(), std::io::Error> {
    let template_path = "templates/config.h.template";
    let target_path = "dev/config.h";

    let template_content = fs::read_to_string(template_path)?;
    let total_screens = project.map_w * project.map_h;

    let output_content = template_content
        .replace("{{GENERATION_DATE}}", "2026-05-15")
        .replace("{{MAP_W}}", &project.map_w.to_string())
        .replace("{{MAP_H}}", &project.map_h.to_string())
        .replace("{{TOTAL_SCREENS}}", &total_screens.to_string())
        .replace("{{SCR_INICIO}}", &project.scr_inicio.to_string())
        .replace("{{PLAYER_LIFE_INI}}", &project.config.player_life_ini.to_string())
        .replace("{{MAX_BULLETS}}", &project.config.max_bullets.to_string())
        .replace("{{LIFE_X}}", &project.config.hud_life_x.to_string())
        .replace("{{LIFE_Y}}", &project.config.hud_life_y.to_string());

    let mut file = fs::File::create(target_path)?;
    file.write_all(output_content.as_bytes())?;
    Ok(())
}
