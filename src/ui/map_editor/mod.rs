// src/ui/map_editor/mod.rs
use crate::app::states::MapEditMode;
use crate::core::validator::ClashError;
use crate::models::ProjectData;
use eframe::egui;

mod left_panel;
pub mod map_canvas;
mod right_panel;

pub fn render_map_editor(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_screen: &mut usize,
    selected_tile: &mut u8,
    clash_errors: &[ClashError],
    map_edit_mode: &mut MapEditMode,
    selected_enemy_sprite_slot: &mut u8,
    tileset_texture: &Option<egui::TextureHandle>,
    sprites_texture: &Option<egui::TextureHandle>,
) {
    // ЖЕСТКОЕ РЕШЕНИЕ: Отключаем встроенный скролл текущего ui-контейнера,
    // чтобы он не перехватывал MouseWheel и не создавал боковую полосу!
    ui.set_min_height(ui.available_height());

    egui::Frame::none().inner_margin(8.0).show(ui, |ui| {
        ui.heading("Редактирование игрового мира");
        ui.add_space(6.0);

        // Фиксируем размеры строго под доступную область экрана БЕЗ учета скроллбаров
        let available_space = ui.available_size();

        ui.allocate_ui_with_layout(
            available_space,
            egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(false),
            |ui| {
                // 1. СНАЧАЛА выделяем 240 пикселей под левый сайдбар
                let left_panel_width = 240.0;
                ui.allocate_ui_with_layout(
                    egui::vec2(left_panel_width, ui.available_height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        left_panel::render(
                            ui,
                            project,
                            *selected_screen,
                            map_edit_mode,
                            selected_tile,
                            selected_enemy_sprite_slot,
                            tileset_texture,
                            sprites_texture,
                        );
                    },
                );

                // Рисуем вертикальный разделитель
                ui.separator();

                // 2. ЗАТЕМ отдаем ВСЁ оставшееся свободное место правой панели с холстом
                // Вычитаем 1.0 пиксель во избежание микро-округлений egui, вызывающих дребезг
                let right_width = ui.available_width() - 1.0;
                let right_height = ui.available_height() - 1.0;

                ui.allocate_ui_with_layout(
                    egui::vec2(right_width, right_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        right_panel::render(
                            ui,
                            project,
                            selected_screen,
                            selected_tile,
                            clash_errors,
                            map_edit_mode,
                            selected_enemy_sprite_slot,
                            tileset_texture,
                        );
                    },
                );
            },
        );
    });
}
