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
    selected_enemy_type: u8,
    tileset_texture: &Option<egui::TextureHandle>,
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

    // 1. Загрузка состояния камеры из контекста egui
    let mut camera = CanvasCamera::load(ui.ctx());
    let mut screen_changed_by_click = false;

    // Идентификаторы хранения активного драга ручек ИИ врага
    let active_drag_enemy_id = egui::Id::new("canvas_active_drag_enemy_idx");
    let active_drag_handle_id = egui::Id::new("canvas_active_drag_handle_type");

    // Компактная верхняя панель управления
    ui.horizontal(|ui| {
        ui.label("🔍 Масштаб (Ctrl + Колесико):");
        if ui
            .add(egui::Slider::new(&mut camera.zoom, 0.15..=2.0).text("x"))
            .changed()
        {}

        if ui.button("🔄 Сброс камеры").clicked() {
            camera.zoom = 1.0;
            camera.pan = egui::Vec2::ZERO;
        }
    });

    // 2. Рендеринг основного Canvas игрового поля
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let painter = ui.painter_at(rect);

        // Ввод: Изменение зума (Ctrl + скролл) и панорамирование (обычный скролл или средняя кнопка)
        camera.handle_zoom(ui, &response, *selected_screen, map_w);
        let is_pan_drag = camera.handle_pan(ui, &response);

        // Ввод: Вычисление точной виртуальной ячейки под курсором мыши
        let mut virtual_cell_pos = None;
        if !is_pan_drag {
            if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                if rect.contains(mouse_pos) {
                    virtual_cell_pos =
                        camera.to_virtual_cell(mouse_pos, rect, map_w, map_h, scr_w_px, scr_h_px);
                }
            }
        }

        // --- ВЫСОКОПРИОРИТЕТНЫЙ ПЕРЕХВАТ НАЖАТИЯ МЫШИ ДЛЯ ЗАХВАТА РУЧЕК ВРАГОВ ---
        let is_lkm_pressed = ui.ctx().input(|i| i.pointer.primary_pressed());
        let is_lkm_down = ui.ctx().input(|i| i.pointer.primary_down());

        let mut current_dragged_enemy_idx: Option<usize> =
            ui.ctx().data(|d| d.get_temp(active_drag_enemy_id));

        if is_lkm_pressed
            && current_dragged_enemy_idx.is_none()
            && *map_edit_mode == MapEditMode::Enemies
        {
            if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                // Сканируем врагов только на ТЕКУЩЕМ выбранном экране во избежание ложных кликов
                let scr_key = format!("screen_{}", selected_screen);
                if let Some(screen_data) = project.screens.get_mut(&scr_key) {
                    let scr_col = *selected_screen % map_w;
                    let scr_row = *selected_screen / map_w;
                    let scr_min_x =
                        rect.min.x + camera.pan.x + (scr_col as f32 * scr_w_px * camera.zoom);
                    let scr_min_y =
                        rect.min.y + camera.pan.y + (scr_row as f32 * scr_h_px * camera.zoom);

                    for (idx, enemy) in screen_data.enemies.iter().enumerate() {
                        // Точно вычисляем, где сейчас физически нарисованы плитки окон L/R/U/D
                        let b1_rect = egui::Rect::from_min_size(
                            egui::pos2(
                                scr_min_x + enemy.x1 as f32 * 32.0 * camera.zoom,
                                scr_min_y + enemy.y1 as f32 * 32.0 * camera.zoom,
                            ),
                            egui::vec2(32.0 * camera.zoom, 32.0 * camera.zoom),
                        );
                        let b2_rect = egui::Rect::from_min_size(
                            egui::pos2(
                                scr_min_x + enemy.x2 as f32 * 32.0 * camera.zoom,
                                scr_min_y + enemy.y2 as f32 * 32.0 * camera.zoom,
                            ),
                            egui::vec2(32.0 * camera.zoom, 32.0 * camera.zoom),
                        );

                        if b1_rect.contains(mouse_pos) {
                            ui.ctx().data_mut(|d| {
                                d.insert_temp(active_drag_enemy_id, idx);
                                d.insert_temp(active_drag_handle_id, 1u8);
                            });
                            break;
                        } else if b2_rect.contains(mouse_pos) {
                            ui.ctx().data_mut(|d| {
                                d.insert_temp(active_drag_enemy_id, idx);
                                d.insert_temp(active_drag_handle_id, 2u8);
                            });
                            break;
                        }
                    }
                }
            }
        }

        // Перепроверяем статус драга после возможного захвата выше
        let is_dragging_ai_handle = ui
            .ctx()
            .data(|d| d.get_temp::<usize>(active_drag_enemy_id).is_some());

        // ЛКМ: Обычное рисование тайлов / Выбор экранов / Спавн сущностей (только если мы НЕ заняты перетаскиванием ручки)
        if let Some((scr_idx, cell_x, cell_y)) = virtual_cell_pos {
            if *selected_screen != scr_idx {
                // НАПРАВЛЕНИЕ 1: Клик по НЕАКТИВНОМУ экрану
                if response.clicked_by(egui::PointerButton::Primary) && !is_dragging_ai_handle {
                    let old_col = (*selected_screen % map_w) as isize;
                    let old_row = (*selected_screen / map_w) as isize;
                    let new_col = (scr_idx % map_w) as isize;
                    let new_row = (scr_idx / map_w) as isize;

                    let col_diff = (old_col - new_col).abs();
                    let row_diff = (old_row - new_row).abs();
                    let is_adjacent = col_diff <= 1 && row_diff <= 1;

                    *selected_screen = scr_idx;

                    if !is_adjacent {
                        screen_changed_by_click = true;
                    }
                }
            } else if !is_dragging_ai_handle {
                // НАПРАВЛЕНИЕ 2: Модификация данных внутри активного экрана
                if (response.dragged_by(egui::PointerButton::Primary)
                    || response.clicked_by(egui::PointerButton::Primary))
                    && !ui.ctx().input(|i| i.modifiers.ctrl)
                {
                    let scr_key = format!("screen_{}", scr_idx);
                    let screen_data = project
                        .screens
                        .entry(scr_key)
                        .or_insert_with(ScreenData::default);

                    match map_edit_mode {
                        MapEditMode::Tiles => {
                            let index = (cell_y as usize) * 15 + (cell_x as usize);
                            screen_data.tiles_matrix[index] = *selected_tile;
                        }
                        MapEditMode::Enemies => {
                            if response.clicked_by(egui::PointerButton::Primary)
                                && screen_data.enemies.len() < 3
                            {
                                if !screen_data
                                    .enemies
                                    .iter()
                                    .any(|e| e.x == cell_x && e.y == cell_y)
                                {
                                    let id_ai = egui::Id::new("default_enemy_ai_type_ctx");
                                    let default_ai =
                                        ui.ctx().data(|d| d.get_temp::<u8>(id_ai)).unwrap_or(1);

                                    let (x1, x2, y1, y2) = match default_ai {
                                        1..=4 => (cell_x, (cell_x + 2).min(14), cell_y, cell_y),
                                        5 | 6 => (0, 0, 0, 0),
                                        7..=10 => (
                                            cell_x.saturating_sub(1),
                                            (cell_x + 2).min(14),
                                            cell_y.saturating_sub(1),
                                            (cell_y + 2).min(9),
                                        ),
                                        11..=14 => (cell_x, cell_x, cell_y, cell_y),
                                        _ => (cell_x, cell_x, cell_y, cell_y),
                                    };

                                    screen_data.enemies.push(Enemy {
                                        tp: selected_enemy_type,
                                        x: cell_x,
                                        y: cell_y,
                                        x1,
                                        x2,
                                        y1,
                                        y2,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Вызов триггера центрирования камеры при прыжке на отдаленный экран
        if screen_changed_by_click {
            camera.center_on_screen(*selected_screen, map_w, scr_w_px, scr_h_px, rect);
        }

        // ПКМ: Стирание
        if response.secondary_clicked() {
            if let Some((scr_idx, cell_x, cell_y)) = virtual_cell_pos {
                let scr_key = format!("screen_{}", scr_idx);
                if let Some(screen_data) = project.screens.get_mut(&scr_key) {
                    match map_edit_mode {
                        MapEditMode::Tiles => {
                            let index = (cell_y as usize) * 15 + (cell_x as usize);
                            screen_data.tiles_matrix[index] = 0;
                        }
                        MapEditMode::Enemies => {
                            screen_data
                                .enemies
                                .retain(|e| e.x != cell_x || e.y != cell_y);
                        }
                    }
                }
            }
        }

        // 3. ЦИКЛ РЕНДЕРИНГА МАТРИЦЫ ЭКРАНОВ + OPTIMIZED FRUSTUM CULLING
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

                painter.text(
                    view_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    current_scr_idx.to_string(),
                    egui::FontId::proportional(64.0 * camera.zoom),
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 25),
                );

                // Отрисовка врагов и их интерактивных ручек
                entity_layer::render_entities(
                    ui,
                    &painter,
                    view_rect,
                    screen_data,
                    camera.zoom,
                    &response,
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

        // 4. Отрисовка кастомных парящих скроллбаров поверх карты
        scroll::render_scrollbars(
            ui,
            rect,
            &mut camera.pan,
            total_matrix_w,
            total_matrix_h,
            camera.zoom,
        );
    });

    // 5. Сохранение обновленного состояния камеры в кэш контекста
    camera.save(ui.ctx());
}
