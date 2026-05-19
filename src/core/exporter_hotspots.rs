use crate::models::ProjectData;

/// Сборка Си-кода массива hotspots с байтовым сжатием координат XY
pub fn build_hotspots_source(project: &ProjectData, total_screens: usize) -> String {
    let mut n_hotspots_type = vec![0; 8]; // Индексы 0..7 под типы хотспотов
    let mut body = String::new();

    body.push_str("typedef struct {\n\tunsigned char xy, tipo, act;\n} HOTSPOT;\n\n");
    body.push_str("HOTSPOT hotspots [] = {\n");

    for i in 0..total_screens {
        let scr_key = format!("screen_{}", i);

        if let Some(screen) = project.screens.get(&scr_key) {
            if screen.hotspot.tp > 0 {
                // Формула сжатия Mojon Twins: xy = (y * 16) + x
                let compressed_xy = (screen.hotspot.y * 16) + screen.hotspot.x;

                body.push_str(&format!(
                    "\t{{{}, {}, 0}}, // Pantalla {}\n",
                    compressed_xy, screen.hotspot.tp, i
                ));

                if (screen.hotspot.tp as usize) < n_hotspots_type.len() {
                    n_hotspots_type[screen.hotspot.tp as usize] += 1;
                }
            } else {
                // Если хотспота нет — пишем пустую Си-заглушку, увеличивая счетчик типа 0
                body.push_str(&format!("\t{{0, 0, 0}}, // Pantalla {} пуста\n", i));
                n_hotspots_type[0] += 1;
            }
        } else {
            body.push_str(&format!("\t{{0, 0, 0}}, // Pantalla {} пуста\n", i));
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
