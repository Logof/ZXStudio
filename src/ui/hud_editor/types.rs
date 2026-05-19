pub struct HudItemMetadata<'a> {
    pub id: &'a str,                  // Уникальный ID для Drag & Drop
    pub label: &'a str,               // Имя для отображения в левой панели
    pub icon: &'a str,                // Иконка на холсте
    pub width_blocks: u32,            // Ширина элемента в знакоместах (2 или 4)
    pub color: eframe::egui::Color32, // Цвет плашки индикатора
}
