// src/ui/map_editor/map_canvas/mod.rs
pub mod camera;
pub mod entity_layer;
pub mod scroll;
pub mod tile_layer;

use crate::app::states::MapEditMode;
use crate::core::validator::ClashError;
use crate::models::{screen::Enemy, ProjectData, ScreenData};
use eframe::egui;

use camera::CanvasCamera;

pub fn render_map_canvas(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_screen: &mut usize,
    selected_tile: &mut u8,
    clash_errors: &[ClashError],
    map_edit_mode: &MapEditMode,
    selected_enemy_sprite_slot: u8,
    tileset_texture: &Option<egui::TextureHandle>,
    sprites_texture: &Option<egui::TextureHandle>,
) {
    let map_w = project.config.map_goals.map_w as usize;
    let map_h = project.config.map_goals.map_h as usize;
    if map_w == 0 || map_h == 0 {
        return;
    }

    let scr_w_px = 15.0 * 32.0;
    let scr_h_px = 10.0 * 32.0;
    let total_matrix_w = map_w as f32 * scr_w_px;
    let total_matrix_h = map_h as f32 * scr_h_px;

    let mut camera = CanvasCamera::load(ui.ctx());
    let mut screen_changed_by_click = false;

    let active_drag_enemy_id = egui::Id::new(format!("canvas_drag_enemy_idx_{}", *selected_screen));
    let is_top_down = project.config.movement_controls.player_genital; // 🔥 Признак проекта

    ui.horizontal(|ui| {
        ui.label("🔍 Масштаб:");
        ui.add(egui::Slider::new(&mut camera.zoom, 0.15..=2.0).text("x"));
        if ui.button("🔄 Сброс").clicked() {
            camera.zoom = 1.0;
            camera.pan = egui::Vec2::ZERO;
        }
    });

    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let painter = ui.painter_at(rect);

        camera.handle_zoom(ui, &response, *selected_screen, map_w);
        let is_pan_drag = camera.handle_pan(ui, &response);

        let mut virtual_cell_pos = None;
        if !is_pan_drag {
            if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                if rect.contains(mouse_pos) {
                    virtual_cell_pos =
                        camera.to_virtual_cell(mouse_pos, rect, map_w, map_h, scr_w_px, scr_h_px);
                }
            }
        }

        let is_dragging_ai_handle = ui
            .ctx()
            .data(|d| d.get_temp::<usize>(active_drag_enemy_id).is_some());

        // ============================================================================
        // 🔥 МОЩНЫЙ ГЛОБАЛЬНЫЙ ПЕРЕХВАТ ВВОДА ДЛЯ РУЧЕК И СПАВНА
        // ============================================================================
        if let Some((scr_idx, cell_x, cell_y)) = virtual_cell_pos {
            let scr_key = format!("screen_{}", scr_idx);
            let screen_data = project
                .screens
                .entry(scr_key)
                .or_insert_with(ScreenData::default);

            // 🖱️ ПКЛ РАБОТАЕТ ВСЕГДА: Независимо от выбранного слоя, стирает врага под курсором
            if response.clicked_by(egui::PointerButton::Secondary) {
                screen_data
                    .enemies
                    .retain(|e| e.x != cell_x || e.y != cell_y);
            }

            if *selected_screen != scr_idx {
                if response.clicked_by(egui::PointerButton::Primary) && !is_dragging_ai_handle {
                    let old_col = (*selected_screen % map_w) as isize;
                    let old_row = (*selected_screen / map_w) as isize;
                    let new_col = (scr_idx % map_w) as isize;
                    let new_row = (scr_idx / map_w) as isize;
                    let is_adjacent =
                        (old_col - new_col).abs() <= 1 && (old_row - new_row).abs() <= 1;
                    *selected_screen = scr_idx;
                    if !is_adjacent {
                        screen_changed_by_click = true;
                    }
                }
            } else if !is_dragging_ai_handle {
                let is_primary_click = response.clicked_by(egui::PointerButton::Primary);
                let is_primary_drag = response.dragged_by(egui::PointerButton::Primary);

                match map_edit_mode {
                    MapEditMode::Tiles => {
                        if (is_primary_drag || is_primary_click)
                            && !ui.ctx().input(|i| i.modifiers.ctrl)
                        {
                            let index = (cell_y as usize) * 15 + (cell_x as usize);
                            screen_data.tiles_matrix[index] = *selected_tile;
                        }
                    }
                    MapEditMode::Enemies => {
                        if is_primary_click {
                            let has_enemy_at_cell = screen_data
                                .enemies
                                .iter()
                                .any(|e| e.x == cell_x && e.y == cell_y);

                            if !has_enemy_at_cell && screen_data.enemies.len() < 3 {
                                // Маппинг спавна строго по кнопкам палитры и признаку проекта
                                let forced_ai_by_slot = match selected_enemy_sprite_slot {
                                    0 => 1, // Слот 1 -> Горизонтальный Линейный (Тип 1)
                                    1 => 6, // Слот 2 -> Призрак Фанти (Тип 6)
                                    2 => 9, // Слот 3 -> Куадратор (Тип 9)
                                    3 => 4, // Слот 4 -> Платформа/Лифт или Преследователь (Тип 4)
                                    _ => 1,
                                };

                                // 🔥 ФИКС: Явно передаем выбранный из EnemiesPalette слот графики в структуру!
                                let mut new_enemy = Enemy {
                                    x: cell_x,
                                    y: cell_y,
                                    x1: cell_x,
                                    y1: cell_y,
                                    x2: cell_x,
                                    y2: cell_y,
                                    tp: forced_ai_by_slot,
                                    sprite_slot: selected_enemy_sprite_slot,
                                };

                                match forced_ai_by_slot {
                                    1 => {
                                        new_enemy.x2 = (cell_x + 2).min(14);
                                    }
                                    9 => {
                                        new_enemy.x1 = cell_x.saturating_sub(1);
                                        new_enemy.x2 = (cell_x + 2).min(14);
                                        new_enemy.y1 = cell_y.saturating_sub(1);
                                        new_enemy.y2 = (cell_y + 2).min(9);
                                    }
                                    6 => {
                                        new_enemy.x1 = 0;
                                        new_enemy.x2 = 0;
                                        new_enemy.y1 = 0;
                                        new_enemy.y2 = 0;
                                    }
                                    4 => {
                                        if is_top_down {
                                            new_enemy.x1 = 0;
                                            new_enemy.x2 = 0;
                                            new_enemy.y1 = 0;
                                            new_enemy.y2 = 0;
                                        } else {
                                            new_enemy.y2 = (cell_y + 2).min(9);
                                        } // Лифт едет вверх
                                    }
                                    _ => {}
                                }
                                screen_data.enemies.push(new_enemy);
                            }
                        }
                    }
                }
            }
        }

        // Рендеринг сетки экранов мира
        for scr_row in 0..map_h {
            for scr_col in 0..map_w {
                let current_scr_idx = scr_row * map_w + scr_col;
                let scr_min_x =
                    rect.min.x + camera.pan.x + (scr_col as f32 * scr_w_px * camera.zoom);
                let scr_min_y =
                    rect.min.y + camera.pan.y + (scr_row as f32 * scr_h_px * camera.zoom);
                let view_rect = egui::Rect::from_min_max(
                    egui::pos2(scr_min_x, scr_min_y),
                    egui::pos2(
                        scr_min_x + (scr_w_px * camera.zoom),
                        scr_min_y + (scr_h_px * camera.zoom),
                    ),
                );

                if !rect.intersects(view_rect) {
                    continue;
                }

                let scr_key = format!("screen_{}", current_scr_idx);
                let screen_data = project
                    .screens
                    .entry(scr_key)
                    .or_insert_with(ScreenData::default);

                tile_layer::render_tiles(
                    &painter,
                    view_rect,
                    screen_data,
                    camera.zoom,
                    tileset_texture,
                );
                tile_layer::render_grid(&painter, view_rect, camera.zoom);

                if let Some((hover_scr_idx, hover_x, hover_y)) = virtual_cell_pos {
                    if current_scr_idx == hover_scr_idx {
                        tile_layer::render_hover_frame(
                            &painter,
                            view_rect,
                            hover_x,
                            hover_y,
                            camera.zoom,
                        );
                    }
                }

                let border_stroke = if current_scr_idx == *selected_screen {
                    egui::Stroke::new(2.5, egui::Color32::from_rgb(255, 180, 0))
                } else {
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 60),
                    )
                };
                painter.rect_stroke(view_rect, 0.0, border_stroke);

                // Передаем признак и текстуру
                entity_layer::render_entities(
                    ui,
                    &painter,
                    view_rect,
                    screen_data,
                    current_scr_idx,
                    camera.zoom,
                    &response,
                    map_edit_mode,
                    sprites_texture,
                    is_top_down, // 🔥 Передаем флаг проекта в слой сущностей
                );

                if current_scr_idx == *selected_screen {
                    entity_layer::render_clash_errors(
                        &painter,
                        view_rect,
                        clash_errors,
                        camera.zoom,
                    );
                }
            }
        }

        scroll::render_scrollbars(
            ui,
            rect,
            &mut camera.pan,
            total_matrix_w,
            total_matrix_h,
            camera.zoom,
        );
    });

    if screen_changed_by_click {
        let scr_col = (*selected_screen % map_w) as f32;
        let scr_row = (*selected_screen / map_w) as f32;
        let target_pan_x = -(scr_col * scr_w_px * camera.zoom)
            + (ui.available_width() - scr_w_px * camera.zoom) / 2.0;
        let target_pan_y = -(scr_row * scr_h_px * camera.zoom)
            + (ui.available_height() - scr_h_px * camera.zoom) / 2.0;
        camera.pan = egui::vec2(target_pan_x, target_pan_y);
    }
    camera.save(ui.ctx());
}
