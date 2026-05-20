use eframe::egui;

pub fn render(ui: &mut egui::Ui, script_text: &mut String, _total_height: f32) {
    // 1. Берем всю доступную область панели целиком
    let desired_size = ui.available_size();

    // 2. Конфигурируем TextEdit как полноценный контейнер
    let text_edit = egui::TextEdit::multiline(script_text)
        .font(egui::FontId::monospace(13.0))
        .code_editor() // Включает табуляцию и правильную обработку пробелов
        .lock_focus(true) // Предотвращает потерю фокуса при вводе спецсимволов
        .desired_width(desired_size.x)
        .desired_rows((desired_size.y / 13.0) as usize); // Грубый, но безопасный расчет строк для заполнения space

    // 3. Помещаем его в контейнер с жестким ограничением размера
    ui.allocate_ui_with_layout(
        desired_size,
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            // Заставляем текстовое поле занять 100% выделенного пространства
            ui.add_sized(desired_size, text_edit);
        },
    );
}
