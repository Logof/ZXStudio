// src/ui/map_editor/map_canvas/mod.rs
pub mod camera;
pub mod entity_layer;
pub mod scroll;
pub mod tile_layer;

use crate::app::states::MapEditMode;
use crate::core::validator::ClashError;
use crate::models::{ProjectData, ScreenData};
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
    sliced_tile_textures: &[egui::TextureHandle],
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
    let is_top_down = project.config.movement_controls.player_genital;

    // Хранилище ID выделенного врага (общий ключ для холста и левой панели)
    let id_context_enemy = egui::Id::new("inspector_selected_enemy_id");
    let selected_enemy_id: Option<u64> = ui.ctx().data(|d| d.get_temp(id_context_enemy));

    ui.horizontal(|ui| {
        ui.label("🔍 Масштаб:");
        ui.add(egui::Slider::new(&mut camera.zoom, 0.15..=2.0).text("x"));
        if ui.button("🔄 Сброс").clicked() {
            camera.zoom = 1.0;
            camera.pan = egui::Vec2::ZERO;
        }
    });

    // Извлекаем изменяемый контекст активного уровня один раз сверху
    let active_idx = project.current_level_index;
    let current_level = &mut project.levels[active_idx];
    let mode = current_level.tile_mode;

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

        // Проверяем, держит ли пользователь мышку в состоянии перетаскивания (тела или ручки)
        let is_dragging_ai_handle = ui
            .ctx()
            .data(|d| d.get_temp::<usize>(active_drag_enemy_id).is_some());

        // ============================================================================
        // ОБРАБОТКА ВВОДА: ВЫДЕЛЕНИЕ (ПКЛ), СПАВН (КЛИК ЛКМ) И УНИВЕРСАЛЬНЫЙ ЛАСТИК
        // ============================================================================
        if let Some((scr_idx, cell_x, cell_y)) = virtual_cell_pos {
            let scr_key = format!("screen_{}", scr_idx);
            
            // Запрашиваем экран из изолированной мапы экранов активного уровня
            let screen_data = current_level
                .screens
                .entry(scr_key.clone())
                .or_insert_with(ScreenData::default);

            // 🖱️ ПКЛ: Просто выделяет врага для Инспектора Свойств в левой панели
            if response.secondary_clicked() {
                if let Some(found_enemy) = screen_data
                    .enemies
                    .iter()
                    .find(|e| e.type_id != 0 && e.x == cell_x && e.y == cell_y)
                {
                    ui.ctx()
                        .data_mut(|d| d.insert_temp(id_context_enemy, found_enemy.id));
                } else {
                    ui.ctx()
                        .data_mut(|d| d.remove_temp::<u64>(id_context_enemy));
                }
            }

            if *selected_screen != scr_idx {
                if ui.ctx().input(|i| i.pointer.primary_clicked()) && !is_dragging_ai_handle {
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
                let is_primary_click = ui.ctx().input(|i| i.pointer.primary_clicked());
                let is_primary_drag = ui.ctx().input(|i| i.pointer.primary_down());

                match map_edit_mode {
                    MapEditMode::Tiles => {
                        if (is_primary_drag || is_primary_click)
                            && !ui.ctx().input(|i| i.modifiers.ctrl)
                        {
                            screen_data.set_tile_and_sync(
                                cell_x,
                                cell_y,
                                *selected_tile,
                                mode,
                            );
                        }
                    }
                    MapEditMode::Enemies => {
                        // Ищем, есть ли живой враг строго в текущей ячейке под курсором
                        let existing_enemy_idx = screen_data
                            .enemies
                            .iter()
                            .position(|e| e.type_id != 0 && e.x == cell_x && e.y == cell_y);

                        // Новый враг создается ТОЛЬКО по одиночному клику (не драгу!),
                        // и только если ячейка абсолютно свободна. Это исключает дублирование при Drag&Drop.
                        if is_primary_click && existing_enemy_idx.is_none() {
                            let id_ai = egui::Id::new("default_enemy_ai_type_ctx");
                            let current_default_ai =
                                ui.ctx().data(|d| d.get_temp::<u8>(id_ai)).unwrap_or(1);
                            let mut forced_ai_by_slot = current_default_ai;

                            if current_default_ai == 1 {
                                forced_ai_by_slot = match selected_enemy_sprite_slot {
                                    0 => 1,
                                    1 => 6,
                                    2 => 9,
                                    3 => 4,
                                    _ => 1,
                                };
                            }

                            use std::time::{SystemTime, UNIX_EPOCH};
                            let unique_id = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_nanos() as u64;

                            let mut new_enemy = crate::models::screen::Enemy {
                                id: unique_id,
                                x: cell_x,
                                y: cell_y,
                                x1: cell_x,
                                y1: cell_y,
                                x2: cell_x,
                                y2: cell_y,
                                type_id: forced_ai_by_slot,
                                sprite_slot: selected_enemy_sprite_slot,
                                speed: 2,
                            };

                            // Рассчитываем дефолтные траектории под выбранный ИИ
                            match forced_ai_by_slot {
                                1..=3 => {
                                    new_enemy.x2 = (cell_x + 2).min(14);
                                }
                                7..=10 => {
                                    new_enemy.x1 = cell_x.saturating_sub(1);
                                    new_enemy.x2 = (cell_x + 2).min(14);
                                    new_enemy.y1 = cell_y.saturating_sub(1);
                                    new_enemy.y2 = (cell_y + 2).min(9);
                                }
                                5 | 6 => {
                                    new_enemy.x1 = 0;
                                    new_enemy.x2 = 0;
                                    new_enemy.y1 = 0;
                                    new_enemy.y2 = 0;
                                }
                                4 => {
                                    if !is_top_down {
                                        new_enemy.y2 = (cell_y + 2).min(9);
                                    }
                                }
                                _ => {
                                    new_enemy.x2 = (cell_x + 2).min(14);
                                    new_enemy.y2 = cell_y;
                                }
                            }

                            // Переиспользуем свободный резерв памяти
                            let dead_slot_idx =
                                screen_data.enemies.iter().position(|e| e.type_id == 0);

                            if let Some(slot_idx) = dead_slot_idx {
                                screen_data.enemies[slot_idx] = new_enemy;
                            } else if screen_data.enemies.len() < 3 {
                                screen_data.enemies.push(new_enemy);
                            }
                        }
                    }
                    MapEditMode::Eraser => {
                        // ============================================================================
                        // ВЫДЕЛЕННЫЙ ИНСТРУМЕНТ ЛАСТИКА (Одиночный клик или протягивание мыши)
                        // ============================================================================
                        if is_primary_click || is_primary_drag {
                            let existing_enemy_idx = screen_data
                                .enemies
                                .iter()
                                .position(|e| e.type_id != 0 && e.x == cell_x && e.y == cell_y);

                            if let Some(idx) = existing_enemy_idx {
                                // 1. Сначала стираем врага из этой ячейки
                                screen_data.enemies[idx].type_id = 0;
                                ui.ctx()
                                    .data_mut(|d| d.remove_temp::<u64>(id_context_enemy));
                            } else {
                                // 2. Если врага в ячейке нет — стираем тайл карты (зануляем в ID 0)
                                screen_data.set_tile_and_sync(cell_x, cell_y, 0, mode);
                            }
                        }
                    }
                }
            }
        }

        // ============================================================================
        // ОТРИСОВКА ВСЕХ СЛОЕВ КАРТЫ МИРА В КОНТЕКСТЕ ТЕКУЩЕГО УРОВНЯ
        // ============================================================================
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
                let mut screen_data = current_level
                    .screens
                    .get(&scr_key)
                    .cloned()
                    .unwrap_or_else(ScreenData::default);

                tile_layer::render_tiles(
                    &painter,
                    view_rect,
                    &screen_data,
                    camera.zoom,
                    sliced_tile_textures,
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

                // Отрисовка сущностей (Drag & Drop ручек и тела теперь работает плавно)
                entity_layer::render_entities(
                    ui,
                    &painter,
                    view_rect,
                    &mut screen_data,
                    current_scr_idx,
                    camera.zoom,
                    &response,
                    map_edit_mode,
                    sprites_texture,
                    is_top_down,
                );

                if let Some(target_id) = selected_enemy_id {
                    if let Some(e) = screen_data.enemies.iter().find(|e| e.id == target_id) {
                        let e_rect = egui::Rect::from_min_size(
                            egui::pos2(
                                view_rect.min.x + e.x as f32 * 32.0 * camera.zoom,
                                view_rect.min.y + e.y as f32 * 32.0 * camera.zoom,
                            ),
                            egui::vec2(32.0 * camera.zoom, 32.0 * camera.zoom),
                        );
                        painter.rect_stroke(
                            e_rect,
                            2.0,
                            egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
                        );
                    }
                }

                // Возвращаем измененный экран обратно в хэшмапу активного уровня
                current_level.screens.insert(scr_key, screen_data);

                if current_scr_idx == *selected_screen {
                    entity_layer::render_clash_errors(
                        &painter,
                        view_rect,
                        clash_errors,
                        current_scr_idx,
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
