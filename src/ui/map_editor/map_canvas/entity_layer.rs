// src/ui/map_editor/map_canvas/entity_layer.rs
use crate::app::states::MapEditMode;
use crate::core::validator::ClashError;
use crate::models::{screen::Enemy, ScreenData};
use eframe::egui;

pub fn render_entities(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    scr_rect: egui::Rect,
    screen_data: &mut ScreenData,
    screen_idx: usize,
    zoom: f32,
    _canvas_response: &egui::Response,
    map_edit_mode: &MapEditMode,
    sprites_texture: &Option<egui::TextureHandle>,
    _is_top_down: bool,
) {
    let mut enemy_to_update: Option<(usize, Enemy)> = None;
    let mouse_pos = ui.ctx().input(|i| i.pointer.hover_pos());
    let is_lkm_down = ui.ctx().input(|i| i.pointer.primary_down());

    let active_drag_enemy_id = egui::Id::new(format!("canvas_drag_enemy_idx_{}", screen_idx));
    let active_drag_handle_id = egui::Id::new(format!("canvas_drag_handle_type_{}", screen_idx));

    let mut current_dragged_enemy_idx: Option<usize> =
        ui.ctx().data(|d| d.get_temp(active_drag_enemy_id));
    let mut current_dragged_handle_type: Option<u8> =
        ui.ctx().data(|d| d.get_temp(active_drag_handle_id));

    if !is_lkm_down {
        current_dragged_enemy_idx = None;
        current_dragged_handle_type = None;
        ui.ctx().data_mut(|d| {
            d.remove_temp::<usize>(active_drag_enemy_id);
            d.remove_temp::<u8>(active_drag_handle_id);
        });
    }

    let tile_side = 32.0 * zoom;
    let id_context_enemy = egui::Id::new("inspector_selected_enemy_id");

    for (idx, enemy) in screen_data.enemies.iter().enumerate() {
        if enemy.type_id == 0 {
            continue;
        }

        // Стартовый квадрат тела врага
        let e_rect = egui::Rect::from_min_size(
            egui::pos2(
                scr_rect.min.x + enemy.x as f32 * tile_side,
                scr_rect.min.y + enemy.y as f32 * tile_side,
            ),
            egui::vec2(tile_side, tile_side),
        );

        let is_ghost_fanti = enemy.x1 == 0 && enemy.x2 == 0 && enemy.y1 == 0 && enemy.y2 == 0;
        let is_quadrator = enemy.type_id == 9 || enemy.type_id == 10;
        let is_marruler = enemy.type_id >= 11 && enemy.type_id <= 14;
        let is_linear = !is_ghost_fanti && !is_quadrator && !is_marruler;

        if zoom > 0.15 {
            painter.rect_stroke(
                e_rect,
                1.0 * zoom,
                egui::Stroke::new(
                    1.0 * zoom,
                    egui::Color32::from_rgba_unmultiplied(255, 0, 100, 150),
                ),
            );

            if let Some(tex) = sprites_texture {
                let display_slot = if enemy.sprite_slot == 0 {
                    match enemy.type_id {
                        5 | 6 => 1,
                        7..=10 => 2,
                        _ => 0,
                    }
                } else {
                    enemy.sprite_slot
                };

                let raw_sprite_id = 8 + display_slot * 2;
                let local_index = raw_sprite_id - 8;
                let sprite_x = (local_index as f32) * 32.0;
                let sprite_y = 16.0;

                let tex_w = 256.0;
                let tex_h = 32.0;
                let pad = 0.05;

                let uv_min = egui::pos2((sprite_x + pad) / tex_w, (sprite_y + pad) / tex_h);
                let uv_max = egui::pos2(
                    (sprite_x + 16.0 - pad) / tex_w,
                    (sprite_y + 16.0 - pad) / tex_h,
                );

                painter.image(
                    tex.id(),
                    e_rect,
                    egui::Rect::from_min_max(uv_min, uv_max),
                    egui::Color32::WHITE,
                );
            }
        }

        // Финишный квадрат траектории
        let b2_rect = egui::Rect::from_min_size(
            egui::pos2(
                scr_rect.min.x + enemy.x2 as f32 * tile_side,
                scr_rect.min.y + enemy.y2 as f32 * tile_side,
            ),
            egui::vec2(tile_side, tile_side),
        );

        let handle_size = 12.0 * zoom;

        // ============================================================================
        // ИСПРАВЛЕНО: Умная ховер-зона (Магнит).
        // Физический размер интерактивной зоны захвата h2_click_rect расширен до
        // целого тайла 32x32, чтобы по нему было легко попасть мышкой на любом зуме!
        // Визуальный маркер h2_view_rect при этом остается компактным и аккуратным.
        // ============================================================================
        let h2_click_rect = b2_rect;
        let h2_view_rect =
            egui::Rect::from_center_size(b2_rect.center(), egui::vec2(handle_size, handle_size));

        let color_trajectory = if is_quadrator {
            egui::Color32::from_rgb(0, 255, 150)
        } else if is_marruler {
            egui::Color32::from_rgb(255, 100, 255)
        } else {
            egui::Color32::from_rgb(0, 150, 255)
        };

        if is_linear {
            painter.line_segment(
                [e_rect.center(), b2_rect.center()],
                egui::Stroke::new(2.0 * zoom, color_trajectory),
            );
            painter.circle_stroke(
                b2_rect.center(),
                handle_size / 2.0,
                egui::Stroke::new(2.0 * zoom, color_trajectory),
            );
        } else if is_quadrator || is_marruler {
            let box_rect = egui::Rect::from_min_max(e_rect.min, b2_rect.max);
            painter.rect_stroke(
                box_rect,
                0.0,
                egui::Stroke::new(1.5 * zoom, color_trajectory),
            );
            painter.rect_filled(h2_view_rect, 1.0, color_trajectory);
        }

        // ============================================================================
        // ИСПРАВЛЕНО: Изменен приоритет перехвата клика мыши.
        // Сначала проверяем наведение на финишную ручку (h2_click_rect с магнитом),
        // и только если мышь не над ней — разрешаем тащить само тело врага (e_rect).
        // Это полностью решает проблему заклинивания ручек при совпадении координат!
        // ============================================================================
        if is_lkm_down
            && current_dragged_enemy_idx.is_none()
            && *map_edit_mode == MapEditMode::Enemies
        {
            if let Some(m_pos) = mouse_pos {
                if h2_click_rect.contains(m_pos) && !is_ghost_fanti {
                    current_dragged_enemy_idx = Some(idx);
                    current_dragged_handle_type = Some(2);
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(active_drag_enemy_id, idx);
                        d.insert_temp(active_drag_handle_id, 2u8);
                    });
                } else if e_rect.contains(m_pos) {
                    current_dragged_enemy_idx = Some(idx);
                    current_dragged_handle_type = Some(0);
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(active_drag_enemy_id, idx);
                        d.insert_temp(active_drag_handle_id, 0u8);
                    });
                }
            }
        }
    }

    if let (Some(drag_idx), Some(handle_type)) =
        (current_dragged_enemy_idx, current_dragged_handle_type)
    {
        if let Some(pos) = mouse_pos {
            if let Some(enemy) = screen_data.enemies.get(drag_idx) {
                if enemy.type_id != 0 {
                    let mut updated_enemy = enemy.clone();
                    let world_mouse_x = pos.x - scr_rect.min.x;
                    let world_mouse_y = pos.y - scr_rect.min.y;
                    let cell_x = ((world_mouse_x / tile_side).floor() as i32).clamp(0, 14) as u8;
                    let cell_y = ((world_mouse_y / tile_side).floor() as i32).clamp(0, 9) as u8;

                    match handle_type {
                        0 => {
                            updated_enemy.x = cell_x;
                            updated_enemy.y = cell_y;
                            updated_enemy.x1 = cell_x;
                            updated_enemy.y1 = cell_y;
                        }
                        2 => {
                            updated_enemy.x2 = cell_x;
                            updated_enemy.y2 = cell_y;
                        }
                        _ => {}
                    }

                    sanitize_enemy_boundaries_interactive(&mut updated_enemy);
                    enemy_to_update = Some((drag_idx, updated_enemy));
                }
            }
        }
    }

    if let Some((idx, updated_enemy)) = enemy_to_update {
        if idx < screen_data.enemies.len() {
            screen_data.enemies[idx] = updated_enemy;
        }
    }
}
fn sanitize_enemy_boundaries_interactive(enemy: &mut Enemy) {
    if enemy.type_id == 0 {
        return;
    }
    // Синхронизируем базовые Си-привязки движка Mojon Twins
    enemy.x1 = enemy.x;
    enemy.y1 = enemy.y;

    let is_ghost = enemy.type_id == 5 || enemy.type_id == 6;
    if is_ghost {
        enemy.x1 = 0;
        enemy.x2 = 0;
        enemy.y1 = 0;
        enemy.y2 = 0;
    }
}

// ============================================================================
// ГРАФИЧЕСКИЙ РЕНДЕРИНГ И ПОДСВЕТКА ДИАГНОСТИЧЕСКИХ РАМОК ОШИБОК
// ============================================================================
pub fn render_clash_errors(
    painter: &egui::Painter,
    scr_rect: egui::Rect,
    clash_errors: &[ClashError],
    current_screen: usize,
    zoom: f32,
) {
    if clash_errors.is_empty() {
        return;
    }

    for error in clash_errors {
        // Рисуем ошибку только если она принадлежит именно этому экрану карты!
        if error.screen_index != current_screen {
            continue;
        }

        let is_attribute_clash =
            error.message.contains("цвет") || error.message.contains("атрибут");

        let block_step = if is_attribute_clash {
            8.0 * zoom
        } else {
            32.0 * zoom
        };

        let err_rect = egui::Rect::from_min_size(
            egui::pos2(
                scr_rect.min.x + (error.cell_x as f32 * block_step),
                scr_rect.min.y + (error.cell_y as f32 * block_step),
            ),
            egui::vec2(block_step, block_step),
        );

        painter.rect_filled(
            err_rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(255, 0, 0, 80),
        );

        painter.rect_stroke(
            err_rect,
            0.0,
            egui::Stroke::new(1.2 * zoom, egui::Color32::from_rgb(255, 50, 50)),
        );
    }
}
