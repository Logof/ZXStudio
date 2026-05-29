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
        // Извлекаем срез индивидуально нарезанных текстур 16x16 из контекста приложения
        let sliced_textures: Option<Vec<egui::TextureHandle>> = ui
            .ctx()
            .data(|d| d.get_temp(egui::Id::new("sliced_tile_textures_ctx")));

        // Динамически определяем лимит ячеек в зависимости от выбранного режима тайлов (16 или 48)
        let mode = project.tile_mode;
        let behaviours_count = mode.behaviours_count();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Увеличиваем spacing, так как элементы стали шире из-за ID слева
            egui::Grid::new("behs_matrix_grid")
                .spacing([14.0, 10.0])
                .show(ui, |ui| {
                    for tile_id in 0..behaviours_count {
                        // БЕЗОПАСНОСТЬ ПАМЯТИ: Защита от выхода за границы массива поведений проекта
                        if tile_id >= project.tile_behaviours.len() {
                            break;
                        }

                        // Связываем логику с синхронизированным вектором поведений ядра модели
                        let current_val = &mut project.tile_behaviours[tile_id];

                        // Группируем один элемент в вертикальный контейнер для центрирования кнопки кода снизу
                        ui.vertical(|ui| {
                            // Верхний ряд: ID слева, ТАЙЛ справа
                            ui.horizontal(|ui| {
                                // 1. Выводим ID тайла слева в формате "00 × " с моноширинным выравниванием
                                ui.monospace(format!("{:02} ×", tile_id));

                                // 2. Отрисовка графического мини-тайла на основе нарезанных текстур
                                if let Some(ref tex_list) = sliced_textures {
                                    if let Some(tex) = tex_list.get(tile_id) {
                                        // Используем полные UV-координаты [0..1], так как тайл уже изолирован слайсером
                                        let uv_rect = egui::Rect::from_min_max(
                                            egui::pos2(0.0, 0.0),
                                            egui::pos2(1.0, 1.0),
                                        );

                                        ui.add(
                                            egui::Image::new(egui::load::SizedTexture::new(
                                                tex.id(),
                                                egui::vec2(32.0, 32.0),
                                            ))
                                            .uv(uv_rect),
                                        );
                                    } else {
                                        // Отрисовка цветного плейсхолдера, если индекс за рамками файла графики (например, служебный ряд)
                                        let (rect, _) = ui.allocate_exact_size(
                                            egui::vec2(32.0, 32.0),
                                            egui::Sense::hover(),
                                        );
                                        let fill_color = match tile_id {
                                            14 => egui::Color32::from_rgb(40, 70, 150),
                                            15 => egui::Color32::from_rgb(150, 40, 40),
                                            16..=18 => egui::Color32::from_rgb(40, 120, 40),
                                            _ => egui::Color32::from_gray(40),
                                        };
                                        ui.painter().rect_filled(rect, 2.0, fill_color);
                                    }
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
                                    let mut mut_val = *current_val;
                                    let mut is_hides = (mut_val & 2) != 0;
                                    let mut is_platform = (mut_val & 4) != 0;
                                    let mut is_obstacle = (mut_val & 8) != 0;
                                    let mut is_breakable = (mut_val & 16) != 0;

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
