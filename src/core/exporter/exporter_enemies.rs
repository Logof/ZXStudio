use crate::models::ProjectData;

/// Сборка Си-кода массива malotes и дефайнов количества врагов по типам ИИ
pub fn build_enemies_source(project: &ProjectData, total_screens: u32) -> String {
    let mut n_enems_type = vec![0; 15]; // Индексы 0..14 под типы ИИ врагов
    let mut body = String::new();

    body.push_str("typedef struct {\n\tunsigned char x, y;\n\tunsigned char x1, y1, x2, y2;\n\tsigned char mx, my;\n\tsigned char t;\n} MALOTE;\n\n");
    body.push_str("MALOTE malotes [] = {\n");

    for i in 0..total_screens {
        let scr_key = format!("screen_{}", i);
        body.push_str(&format!("\t// Screen {}\n", i));

        if let Some(screen) = project.screens.get(&scr_key) {
            for enemy in &screen.enemies {
                // Переводим индексы сетки IDE (32x32) в нативные пиксели Спектрума (шаг 16)
                let x_px = enemy.x * 16;
                let y_px = enemy.y * 16;
                let x1_px = enemy.x1 * 16;
                let y1_px = enemy.y1 * 16;
                let x2_px = enemy.x2 * 16;
                let y2_px = enemy.y2 * 16;

                // Авто-вычисление стартовых векторов mx/my на основе оси движения
                let is_horizontal =
                    enemy.x1 != enemy.x2 || (enemy.y1 == enemy.y2 && enemy.x1 == enemy.x);
                let (mx, my) = if is_horizontal { (1, 0) } else { (0, 1) };

                body.push_str(&format!(
                    "\t {{{}, {}, {}, {}, {}, {}, {}, {}, {}}},\n",
                    x_px, y_px, x1_px, y1_px, x2_px, y2_px, mx, my, enemy.tp
                ));

                if (enemy.tp as usize) < n_enems_type.len() {
                    n_enems_type[enemy.tp as usize] += 1;
                }
            }
        }
    }

    // Удаляем лишнюю финальную запятую для чистоты Си-синтаксиса
    if body.ends_with(",\n") {
        body.truncate(body.len() - 2);
        body.push_str("\n");
    }
    body.push_str("};\n\n");

    // Дописываем дефайны глобальной статистики для аллокации буферов ОЗУ Z80
    for (tp, count) in n_enems_type.iter().enumerate() {
        body.push_str(&format!("#define N_ENEMS_TYPE_{} {}\n", tp, count));
    }

    // Суммарный лимит BADDIES_COUNT по правилу Mojon Twins (типы 1 + 2 + 3)
    let baddies_count = n_enems_type[1] + n_enems_type[2] + n_enems_type[3];
    body.push_str(&format!("\n#define BADDIES_COUNT {}\n", baddies_count));

    body
}
