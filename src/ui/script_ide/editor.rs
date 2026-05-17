use eframe::egui;

pub fn render(ui: &mut egui::Ui, script_text: &mut String, total_height: f32) {
    // Вычисляем реальную доступную высоту (с учетом отступов сверху)
    let available_height = ui.available_height();

    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), available_height),
        egui::Layout::top_down(egui::Align::LEFT),

        |ui| {
            // Ограничиваем ScrollArea строго доступной высотой панели
            egui::ScrollArea::both()
                .id_source("script_editor_scroll")
                .max_height(available_height)
                .show(ui, |ui| {
                    // Задаем минимальный размер текстового поля, равный всей доступной области
                    // Это заставит темно-серое поле ввода растянуться до самого низа без лишних скроллов
                    let desired_size = egui::vec2(
                        ui.available_width() - 8.0, 
                        available_height - 4.0
                    );

                    let text_edit = egui::TextEdit::multiline(script_text)
                        .font(egui::FontId::monospace(13.0))
                        .min_size(desired_size); // ИСПРАВЛЕНИЕ: Вместо desired_rows фиксируем честный размер в px

                    ui.add(text_edit);
                });
        },
    );
}
