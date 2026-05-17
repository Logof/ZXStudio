use std::fs;
use std::io::Write;
use crate::models::ProjectData;

// Импортируем наши декомпозированные атомарные Си-экспортеры
use super::exporter_config::build_and_write_config_h;
use super::exporter_enemies::build_enemies_source;
use super::exporter_hotspots::build_hotspots_source;

pub fn save_project_json(project: &ProjectData) -> Result<(), std::io::Error> {
    let mut file = fs::File::create("map/mapa.prj")?;
    let json = serde_json::to_string_pretty(project)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Чистый диспетчер сборки config.h на основе вызова изолированного сервиса
pub fn export_config_h(project: &ProjectData) -> Result<(), std::io::Error> {
    build_and_write_config_h(project)
}

/// Чистый диспетчер сборки enems.h на основе вызова изолированных сущностей
pub fn export_enems_h(project: &ProjectData) -> Result<(), std::io::Error> {
    let target_path = "dev/enems.h";
    let total_screens = project.map_w * project.map_h;

    let mut final_source = String::new();
    final_source.push_str("// MTE MK1 (la Churrera) v4\n// Generated автоматически из декомпозированных модулей Rust IDE\n\n");

    let enemies_code = build_enemies_source(project, total_screens);
    final_source.push_str(&enemies_code);
    final_source.push_str("\n// ----------------------------------------------------------------------------\n\n");

    let hotspots_code = build_hotspots_source(project, total_screens);
    final_source.push_str(&hotspots_code);

    let mut file = fs::File::create(target_path)?;
    file.write_all(final_source.as_bytes())?;
    Ok(())
}
