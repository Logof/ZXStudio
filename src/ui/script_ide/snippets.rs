use eframe::egui;

pub fn render(ui: &mut egui::Ui, script_text: &mut String, selected_screen: usize, total_height: f32) {
    let left_panel_width = 200.0;

    ui.allocate_ui_with_layout(
        egui::vec2(left_panel_width, total_height),
        egui::Layout::top_down(egui::Align::LEFT),

        |ui| {
            ui.set_height(total_height);

            egui::Frame::group(ui.style())
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.set_width(left_panel_width - 16.0);
                    
                    // Константный размер кнопок для идеальной Cyberpunk-эстетики
                    let btn_size = egui::vec2(ui.available_width(), 22.0);

                    // Включаем скроллбар только для зоны сниппетов, чтобы категории не улетали под экран
                    egui::ScrollArea::vertical()
                        .id_source("snippets_categories_scroll")
                        .max_height(ui.available_height() - 10.0)
                        .show(ui, |ui| {
                            
                            // ============================================================================
                            // НОВОЕ УЛУЧШЕНИЕ: Секция макрокоманд Мультилевела (Глава 12)
                            // ============================================================================
                            ui.colored_label(egui::Color32::from_rgb(255, 180, 0), "🗺️ Мультилевел (Кампании):");
                            ui.add_space(2.0);

                            if ui.add_sized(btn_size, egui::Button::new("LEVEL x (Секция)")).clicked() {
                                script_text.push_str("\n\nLEVEL 0\nBEGIN\nEND");
                            }
                            if ui.add_sized(btn_size, egui::Button::new("IF LEVEL = n (Условие)")).clicked() {
                                script_text.push_str("\n\tIF LEVEL = 0\n\tTHEN\n\t\t\n\tEND");
                            }
                            if ui.add_sized(btn_size, egui::Button::new("NEXT_LEVEL (Команда)")).clicked() {
                                script_text.push_str("\n\t\tNEXT_LEVEL");
                            }

                            ui.add_space(6.0);
                            ui.separator();
                            ui.add_space(4.0);

                            ui.colored_label(egui::Color32::from_rgb(0, 255, 255), "🎯 Разделы (Secciones):");
                            ui.add_space(2.0);

                            if ui.add_sized(btn_size, egui::Button::new("ENTERING SCREEN x")).clicked() {
                                script_text.push_str(&format!("\n\nENTERING SCREEN {}\nBEGIN\nEND", selected_screen));
                            }
                            if ui.add_sized(btn_size, egui::Button::new("ENTERING GAME")).clicked() {
                                script_text.push_str("\n\nENTERING GAME\nBEGIN\nEND");
                            }
                            if ui.add_sized(btn_size, egui::Button::new("ENTERING ANY")).clicked() {
                                script_text.push_str("\n\nENTERING ANY\nBEGIN\nEND");
                            }
                            if ui.add_sized(btn_size, egui::Button::new("PRESS_FIRE SCREEN x")).clicked() {
                                script_text.push_str(&format!("\n\nPRESS_FIRE AT SCREEN {}\nBEGIN\nEND", selected_screen));
                            }
                            if ui.add_sized(btn_size, egui::Button::new("PRESS_FIRE ANY")).clicked() {
                                script_text.push_str("\n\nPRESS_FIRE AT ANY\nBEGIN\nEND");
                            }
                            if ui.add_sized(btn_size, egui::Button::new("PLAYER_KILLS_ENEMY")).clicked() {
                                script_text.push_str("\n\nPLAYER_KILLS_ENEMY\nBEGIN\nEND");
                            }

                            ui.add_space(6.0);
                            ui.separator();
                            ui.add_space(4.0);

                            // --- КАТЕГОРИЯ: КЛАУЗУЛЫ И ШАБЛОНЫ ---
                            egui::CollapsingHeader::new("⚖️ Логика Cláusulas")
                                .default_open(true)
                                .show(ui, |ui| {
                                    if ui.add_sized(btn_size, egui::Button::new("IF ... THEN ... END")).clicked() {
                                        script_text.push_str("\n\tIF TRUE\n\tTHEN\n\t\t\n\tEND");
                                    }
                                    if ui.add_sized(btn_size, egui::Button::new("IF FLAG Compare")).clicked() {
                                        script_text.push_str("\n\tIF FLAG 1 = 0\n\tTHEN\n\t\t\n\tEND");
                                    }
                                });

                            // --- КАТЕГОРИЯ: ПРОВЕРКИ И КОМАНДЫ ФЛАГОВ ---
                            egui::CollapsingHeader::new("🚩 Работа с Flags")
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.small("Проверки (IF):");
                                    if ui.add_sized(btn_size, egui::Button::new("IF FLAG = n")).clicked() { script_text.push_str("\n\tIF FLAG 1 = 0"); }
                                    if ui.add_sized(btn_size, egui::Button::new("IF FLAG < n")).clicked() { script_text.push_str("\n\tIF FLAG 1 < 5"); }
                                    if ui.add_sized(btn_size, egui::Button::new("IF FLAG > n")).clicked() { script_text.push_str("\n\tIF FLAG 1 > 0"); }
                                    if ui.add_sized(btn_size, egui::Button::new("IF FLAG <> n")).clicked() { script_text.push_str("\n\tIF FLAG 1 <> 10"); }
                                    if ui.add_sized(btn_size, egui::Button::new("IF FLAG = #n (флаг)")).clicked() { script_text.push_str("\n\tIF FLAG 5 = #3"); }
                                    
                                    ui.add_space(2.0);
                                    ui.small("Команды (THEN):");
                                    if ui.add_sized(btn_size, egui::Button::new("SET FLAG x = n")).clicked() { script_text.push_str("\n\t\tSET FLAG 1 = 1"); }
                                    if ui.add_sized(btn_size, egui::Button::new("INC FLAG x, n")).clicked() { script_text.push_str("\n\t\tINC FLAG 1, 1"); }
                                    if ui.add_sized(btn_size, egui::Button::new("DEC FLAG x, n")).clicked() { script_text.push_str("\n\t\tDEC FLAG 1, 1"); }
                                    if ui.add_sized(btn_size, egui::Button::new("FLIPFLOP x")).clicked() { script_text.push_str("\n\t\tFLIPFLOP 1"); }
                                    if ui.add_sized(btn_size, egui::Button::new("SWAP x, y")).clicked() { script_text.push_str("\n\t\tSWAP 1, 2"); }
                                });

                            // --- КАТЕГОРИЯ: КООРДИНАТЫ И НАВИГАЦИЯ ---
                            egui::CollapsingHeader::new("🚶 Позиция и Мир")
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.small("Проверки (IF):");
                                    if ui.add_sized(btn_size, egui::Button::new("PLAYER_TOUCHES")).clicked() { script_text.push_str("\n\tIF PLAYER_TOUCHES (7, 5)"); }
                                    if ui.add_sized(btn_size, egui::Button::new("PLAYER_IN_X (px)")).clicked() { script_text.push_str("\n\tIF PLAYER_IN_X 32, 64"); }
                                    if ui.add_sized(btn_size, egui::Button::new("PLAYER_IN_Y (px)")).clicked() { script_text.push_str("\n\tIF PLAYER_IN_Y 16, 48"); }
                                    if ui.add_sized(btn_size, egui::Button::new("PLAYER_IN_X_TILES")).clicked() { script_text.push_str("\n\tIF PLAYER_IN_X_TILES 0, 5"); }
                                    if ui.add_sized(btn_size, egui::Button::new("PLAYER_IN_Y_TILES")).clicked() { script_text.push_str("\n\tIF PLAYER_IN_Y_TILES 0, 4"); }
                                    if ui.add_sized(btn_size, egui::Button::new("IF NPANT n (экран)")).clicked() { script_text.push_str("\n\tIF NPANT 5"); }
                                    if ui.add_sized(btn_size, egui::Button::new("IF NPANT_NOT n")).clicked() { script_text.push_str("\n\tIF NPANT_NOT 0"); }

                                    ui.add_space(2.0);
                                    ui.small("Команды (THEN):");
                                    if ui.add_sized(btn_size, egui::Button::new("SETX x (tile)")).clicked() { script_text.push_str("\n\t\tSETX 7"); }
                                    if ui.add_sized(btn_size, egui::Button::new("SETY y (tile)")).clicked() { script_text.push_str("\n\t\tSETY 5"); }
                                    if ui.add_sized(btn_size, egui::Button::new("WARP_TO n, x, y")).clicked() { script_text.push_str("\n\t\tWARP_TO 2, 7, 5"); }
                                });

                            // --- КАТЕГОРИЯ: БАЗОВЫЕ И КРИТИЧЕСКИЕ КОМАНДЫ ДВИЖКА ---
                            egui::CollapsingHeader::new("⚙️ Системные ядра")
                                .default_open(false)
                                .show(ui, |ui| {
                                    if ui.add_sized(btn_size, egui::Button::new("SET TILE")).clicked() { script_text.push_str("\n\t\tSET TILE (0, 0) = 14"); }
                                    if ui.add_sized(btn_size, egui::Button::new("WIN GAME")).clicked() { script_text.push_str("\n\t\tWIN GAME"); }
                                    if ui.add_sized(btn_size, egui::Button::new("BREAK")).clicked() { script_text.push_str("\n\t\tBREAK"); }
                                    if ui.add_sized(btn_size, egui::Button::new("REENTER SCREEN")).clicked() { script_text.push_str("\n\t\tREENTER"); }
                                });
                        });
                });
        },
    );
}
