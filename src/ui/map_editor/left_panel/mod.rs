// src/ui/map_editor/left_panel/mod.rs
use crate::app::states::MapEditMode;
use crate::models::ProjectData;
use eframe::egui;

mod enemies_palette;
mod tiles_palette;

pub fn render(
    ui: &mut egui::Ui,
    project: &mut ProjectData,
    selected_screen: usize,
    map_edit_mode: &mut MapEditMode,
    selected_tile: &mut u8,
    selected_enemy_sprite_slot: &mut u8,
    sliced_tile_textures: &[egui::TextureHandle],
    sprites_texture: &Option<egui::TextureHandle>,
) {
    let total_height = ui.available_height();

    egui::Frame::group(ui.style())
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.set_height(total_height - 4.0);

            egui::ScrollArea::vertical()
                .id_source("left_editor_scroll")
                .max_height(ui.available_height())
                .show(ui, |ui| {
                    ui.label("💡 Активный инструмент:");
                    ui.radio_value(map_edit_mode, MapEditMode::Tiles, "🧱 Рисовать тайлы");
                    ui.radio_value(map_edit_mode, MapEditMode::Enemies, "👾 Рисовать врагов");

                    // 🔥 ИСПРАВЛЕНО: Добавлена выделенная кнопка ластика, которая включает очистку на холсте
                    ui.radio_value(map_edit_mode, MapEditMode::Eraser, "🧽 Ластик очистки");

                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);

                    match map_edit_mode {
                        MapEditMode::Tiles => {
                            tiles_palette::render(ui, project, selected_tile, sliced_tile_textures);
                        }
                        MapEditMode::Enemies => {
                            enemies_palette::render(
                                ui,
                                project,
                                selected_enemy_sprite_slot,
                                sprites_texture,
                                selected_screen,
                            );
                        }
                        MapEditMode::Eraser => {
                            ui.group(|ui| {
                                ui.label("🧽 Режим ластика активен");
                                ui.small("Кликните ЛКМ на холсте карты, чтобы стереть:");
                                ui.small("• Спрайт врага (если курсор над врагом)");
                                ui.small("• Текущий тайл карты (заменяется на пустой тайл 0)");
                            });
                        }
                    }
                });
        });
}
