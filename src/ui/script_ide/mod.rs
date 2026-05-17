use eframe::egui;

mod snippets;
mod editor;

pub fn render_script_editor(ui: &mut egui::Ui, script_text: &mut String, selected_screen: usize) {
    ui.heading("Пространство разработки сценариев (.spt)");
    ui.add_space(4.0);

    // Вычисляем честную доступную высоту вкладки до самого подвала логов
    let total_height = ui.available_height();

    // Принудительно вытягиваем все внутренние панели по вертикали (Align::Max)
    ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Max).with_main_wrap(false),

        |ui| {
            // 1. Вызов левой панели быстрых сниппетов
            snippets::render(ui, script_text, selected_screen, total_height);

            ui.separator();

            // 2. Вызов правой панели текстового редактора
            editor::render(ui, script_text, total_height);
        },
    );
}
