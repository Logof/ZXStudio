// src/core/validator/world_collisions.rs
use super::{ClashError, ErrorSeverity};
use crate::models::screen::ScreenData;
use crate::models::ProjectData;

pub struct WorldValidator;

impl WorldValidator {
    pub fn validate_world(project: &ProjectData) -> Vec<ClashError> {
        let mut errors = Vec::new();
        let map_w = project.config.map_goals.map_w as usize;
        let map_h = project.config.map_goals.map_h as usize;
        let total_screens = map_w * map_h;

        // Извлекаем контекст активного на данный момент уровня
        let current_level = &project.levels[project.current_level_index];

        for scr_idx in 0..total_screens {
            let scr_key = format!("screen_{}", scr_idx);
            if let Some(screen_data) = current_level.screens.get(&scr_key) {
                Self::check_enemies(scr_idx, screen_data, project, &mut errors);
                Self::check_hotspots(scr_idx, screen_data, project, &mut errors);
            }
        }

        errors
    }

    fn check_enemies(
        scr_idx: usize,
        screen: &ScreenData,
        project: &ProjectData,
        errors: &mut Vec<ClashError>,
    ) {
        let current_level = &project.levels[project.current_level_index];

        for (idx, enemy) in screen.enemies.iter().enumerate() {
            if enemy.type_id == 0 || enemy.x >= 15 || enemy.y >= 10 {
                continue;
            }

            let cell_idx = (enemy.y as usize) * 15 + (enemy.x as usize);
            if let Some(&tile_id) = screen.tiles_matrix.get(cell_idx) { 
                let beh = if (tile_id as usize) < current_level.tile_behaviours.len() {
                    current_level.tile_behaviours[tile_id as usize]
                } else {
                    0
                };

                if (beh & 8) != 0 {
                    errors.push(ClashError {
                        screen_index: scr_idx,
                        cell_x: enemy.x as usize,
                        cell_y: enemy.y as usize,
                        severity: ErrorSeverity::Critical,
                        message: format!(
                            "ВРАГ {} (Ячейка {},{}): Тайл ID={}, Поведение Побитово={:#010b} (Бит 8 взведен!)",
                            idx + 1, enemy.x, enemy.y, tile_id, beh
                        ),
                    });
                }
            }
        }
    }

    fn check_hotspots(
        scr_idx: usize,
        screen: &ScreenData,
        project: &ProjectData,
        errors: &mut Vec<ClashError>,
    ) {
        let current_level = &project.levels[project.current_level_index];
        let hs = &screen.hotspot;
        if hs.type_id == 0 || hs.x >= 15 || hs.y >= 10 {
            return;
        }

        let cell_idx = (hs.y as usize) * 15 + (hs.x as usize);
        if let Some(&tile_id) = screen.tiles_matrix.get(cell_idx) {
            let beh = if (tile_id as usize) < current_level.tile_behaviours.len() {
                current_level.tile_behaviours[tile_id as usize]
            } else {
                0
            };

            if (beh & 8) != 0 {
                errors.push(ClashError {
                    screen_index: scr_idx,
                    cell_x: hs.x as usize,
                    cell_y: hs.y as usize,
                    severity: ErrorSeverity::Critical,
                    message: format!(
                        "ПРЕДМЕТ Тип {} (Ячейка {},{}): Тайл ID={}, Поведение Побитово={:#010b} (Бит 8 взведен!)",
                        hs.type_id, hs.x, hs.y, tile_id, beh
                    ),
                });
            }
        }
    }
}
