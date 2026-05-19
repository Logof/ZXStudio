// src/ui/configurator/tile_behaviour.rs
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    ui.strong("🧱 Физика поведения тайлов (behs[])");
    ui.label("Нажмите на кнопку с кодом под тайлом для тонкой настройки коллизий.");
    ui.add_space(8.0);

    let is_compressed = project.config.map_goals.compressed_levels;

    if is_compressed {
        ui.colored_label(
            egui::Color32::GOLD,
            "ℹ️ Внимание: При COMPRESSED_LEVELS массив behs[] переопределяется файлами уровней.",
        );
    }

    ui.add_enabled_ui(!is_compressed, |ui| {
        let tileset_tex: Option<egui::TextureHandle> =
            ui.ctx().data(|d| d.get_temp(egui::Id::new("tileset_tex")));

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Увеличиваем spacing, так как элементы стали шире из-за ID слева
            egui::Grid::new("behs_matrix_grid")
                .spacing([14.0, 10.0])
                .show(ui, |ui| {
                    for tile_id in 0..48 {
                        let current_val = &mut project.config.tile_behaviour.behs[tile_id];

                        // Группируем один элемент в вертикальный контейнер для центрирования кнопки кода снизу
                        ui.vertical(|ui| {
                            // Верхний ряд: ID слева, ТАЙЛ справа
                            ui.horizontal(|ui| {
                                // 1. Выводим ID тайла слева в формате "00 × " с моноширинным выравниванием
                                ui.monospace(format!("{:02} ×", tile_id));

                                // 2. Отрисовка графического мини-тайла
                                if let Some(ref tex) = tileset_tex {
                                    let tex_size = tex.size_vec2();
                                    let tw = 16.0 / tex_size.x;
                                    let th = 16.0 / tex_size.y;
                                    let tx = (tile_id % 16) as f32 * tw;
                                    let ty = (tile_id / 16) as f32 * th;

                                    let uv_rect = egui::Rect::from_min_max(
                                        egui::pos2(tx, ty),
                                        egui::pos2(tx + tw, ty + th),
                                    );

                                    ui.add(
                                        egui::Image::new(egui::load::SizedTexture::new(
                                            tex.id(),
                                            egui::vec2(32.0, 32.0),
                                        ))
                                        .uv(uv_rect),
                                    );
                                } else {
                                    let (rect, _) = ui.allocate_exact_size(
                                        egui::vec2(32.0, 32.0),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter().rect_filled(
                                        rect,
                                        2.0,
                                        egui::Color32::from_gray(40),
                                    );
                                }
                            });

                            // Снизу под тайлом выводим кнопку с кодом поведения (смещаем вправо, чтобы отцентровать под картинкой тайла)
                            ui.horizontal(|ui| {
                                ui.add_space(28.0); // Сдвиг, компенсирующий ширину текста "00 ×"

                                // Кнопка-меню в виде компактной плашки с кодом поведения [ 8 ]
                                ui.menu_button(format!("[ {:^2} ]", current_val), |ui| {
                                    ui.strong(format!("Физика тайла #{}", tile_id));

                                    let mut is_kills = (*current_val & 1) != 0;
                                    let mut is_hides = (*current_val & 2) != 0;
                                    let mut is_platform = (*current_val & 4) != 0;
                                    let mut is_obstacle = (*current_val & 8) != 0;
                                    let mut is_breakable = (*current_val & 16) != 0;

                                    if ui
                                        .checkbox(&mut is_obstacle, "Сплошная Стена (8)")
                                        .changed()
                                    {
                                        toggle_bit(current_val, 8, is_obstacle);
                                    }
                                    if ui.checkbox(&mut is_kills, "Шипы / Убийца (1)").changed()
                                    {
                                        toggle_bit(current_val, 1, is_kills);
                                    }
                                    if ui.checkbox(&mut is_hides, "Скрывает игрока (2)").changed()
                                    {
                                        toggle_bit(current_val, 2, is_hides);
                                    }
                                    if ui.checkbox(&mut is_platform, "Платформа (4)").changed()
                                    {
                                        toggle_bit(current_val, 4, is_platform);
                                    }
                                    if ui
                                        .checkbox(&mut is_breakable, "Разрушаемая стена (16)")
                                        .changed()
                                    {
                                        toggle_bit(current_val, 16, is_breakable);
                                    }

                                    ui.separator();
                                    if ui
                                        .radio_value(
                                            current_val,
                                            10,
                                            "Ящик / Замок (Спец. блок 10)",
                                        )
                                        .clicked()
                                    {}
                                    if ui.radio_value(current_val, 0, "Пусто (0)").clicked() {}
                                });
                            });
                        });

                        // Перенос строки в сетке Grid каждые 8 тайлов (уменьшено с 12, чтобы сетка не растягивалась слишком широко по горизонтали)
                        if (tile_id + 1) % 8 == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    });
}

fn toggle_bit(val: &mut u8, bit: u8, set: bool) {
    if set {
        *val |= bit;
        if bit == 8 {
            *val &= !4;
        }
        if bit == 4 {
            *val &= !8;
        }
    } else {
        *val &= !bit;
    }
}
