use eframe::egui;

pub fn render_script_editor(ui: &mut egui::Ui, script_text: &mut String, selected_screen: usize) {
    ui.heading("Пространство разработки сценариев (.spt)");

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label("Быстрые сниппеты:");
                if ui.button("Вставить SET TILE").clicked() {
                    script_text.push_str("\nSET TILE (0, 0) = 14");
                }
                if ui.button("Вставить WIN GAME").clicked() {
                    script_text.push_str("\nWIN GAME");
                }
                if ui.button("Добавить ENTERING SCREEN").clicked() {
                    script_text.push_str(&format!("\nENTERING SCREEN {}", selected_screen));
                }
            });
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(script_text)
                    .font(egui::FontId::monospace(13.0))
                    .desired_width(f32::INFINITY)
                    .desired_rows(20)
            );
        });
    });
}
