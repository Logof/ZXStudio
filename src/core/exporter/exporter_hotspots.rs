// src/core/exporter/exporter_hotspots.rs
use crate::models::ProjectData;

/// Сборка Си-кода массива hotspots с байтовым сжатием координат XY для конкретного уровня
pub fn build_hotspots_source_for_level(project: &ProjectData, level_idx: usize, total_screens: u32) -> String {
    let mut n_hotspots_type = vec![0; 8]; // Индексы 0..7 под типы хотспотов
    let mut body = String::new();

    // Извлекаем контекст указанного уровня
    let current_level = &project.levels[level_idx];

    body.push_str("typedef struct {\n\tunsigned char xy, tipo, act;\n} HOTSPOT;\n\n");
    body.push_str("HOTSPOT hotspots [] = {\n");

    for i in 0..total_screens {
        let scr_key = format!("screen_{}", i);

        if let Some(screen) = current_level.screens.get(&scr_key) {
            if screen.hotspot.type_id > 0 {
                // Формула сжатия Mojon Twins: xy = (y * 16) + x
                let compressed_xy = (screen.hotspot.y * 16) + screen.hotspot.x;

                body.push_str(&format!(
                    "\t{{{}, {}, 0}}, // Screen {}\n",
                    compressed_xy, screen.hotspot.type_id, i
                ));

                if (screen.hotspot.type_id as usize) < n_hotspots_type.len() {
                    n_hotspots_type[screen.hotspot.type_id as usize] += 1;
                }
            } else {
                // Если хотспота нет — пишем пустую Си-заглушку, увеличивая счетчик типа 0
                body.push_str(&format!("\t{{0, 0, 0}}, // Screen {} пуста\n", i));
                n_hotspots_type[0] += 1;
            }
        } else {
            body.push_str(&format!("\t{{0, 0, 0}}, // Screen {} пуста\n", i));
            n_hotspots_type[0] += 1;
        }
    }

    if body.ends_with(",\n") {
        body.truncate(body.len() - 2);
        body.push_str("\n");
    }
    body.push_str("};\n\n");

    // Дописываем дефайны статистики хотспотов
    for (hp_tp, count) in n_hotspots_type.iter().enumerate() {
        body.push_str(&format!("#define N_HOTSPOTS_TYPE_{} {}\n", hp_tp, count));
    }

    body
}
