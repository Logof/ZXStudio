use eframe::egui;

pub fn apply_modern_dark_theme(ctx: &egui::Context) {
    // 1. НАСТРОЙКА ИДЕАЛЬНОЙ СЕТКИ РАЗМЕРОВ ШРИФТОВ (Баланс пропорций)
    let mut style = (*ctx.style()).clone();

    // Делаем интерфейс более аккуратным и собранным (размеры в пунктах)
    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(11.0));
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(14.0)); // Текст по умолчанию
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(13.5)); // Кнопки и чекбоксы
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(18.0)); // Заголовки страниц
    style
        .text_styles
        .insert(egui::TextStyle::Monospace, egui::FontId::monospace(14.0)); // Код скриптов и логи

    // Добавим кастомный стиль для второстепенных подзаголовков панелей
    style.text_styles.insert(
        egui::TextStyle::Name("Subheading".into()),
        egui::FontId::proportional(15.0),
    );

    // Настройка отступов для более воздушного, но плотного UI
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(10.0);

    ctx.set_style(style);

    // 2. СТИЛИЗАЦИЯ ВИЗУАЛЬНЫХ ЭЛЕМЕНТОВ (Современная темная тема)
    let mut visuals = egui::Visuals::dark();

    // Цветовая палитра: глубокий темно-серый без синевы (Меньше устают глаза)
    visuals.panel_fill = egui::Color32::from_rgb(18, 18, 22); // Задний фон панелей
    visuals.window_fill = egui::Color32::from_rgb(26, 26, 32); // Фон всплывающих окон

    // Настройка неактивных виджетов (Поля ввода, неактивные кнопки)
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(32, 32, 40);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(170, 172, 180);
    visuals.widgets.inactive.rounding = egui::Rounding::same(3.0); // Мягкое, едва заметное скругление кнопок

    // Настройка элементов при наведении мыши (Hover)
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(44, 44, 55);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
    visuals.widgets.hovered.rounding = egui::Rounding::same(3.0);

    // Настройка активных виджетов (Клик, фокус)
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 130, 230);
    visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
    visuals.widgets.active.rounding = egui::Rounding::same(3.0);

    // Цвет выделения текста и активных вкладок (Фирменный Cyan/Blue)
    visuals.selection.bg_fill = egui::Color32::from_rgb(0, 120, 215);

    ctx.set_visuals(visuals);
}
