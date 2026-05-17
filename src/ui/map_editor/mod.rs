use eframe::egui;
use crate::models::ProjectData;
use crate::core::validator::ClashError;
use crate::app::states::MapEditMode;

mod left_panel;
mod right_panel;
pub mod map_canvas;

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
    egui::Frame::none()
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.heading("Редактирование игрового мира");
            ui.add_space(6.0);

            // Главный горизонтальный контейнер с принудительным выравниванием по верхнему краю
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(false),

                |ui| {
                    
                    // 1. СНАЧАЛА жестко выделяем 240 пикселей под левый сайдбар
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
                        }
                    );

                    // Рисуем вертикальный разделитель
                    ui.separator();

                    // 2. ЗАТЕМ отдаем ВСЁ оставшееся свободное место правой панели с холстом
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), ui.available_height()),
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
                        }
                    );
                },
            );
        });
}
