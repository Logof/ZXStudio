use eframe::egui;
use crate::app::states::CustomTab;

/// Рендерит интерактивное дерево проекта и возвращает сигнал переключения вкладки при двойном клике
pub fn render_project_tree(ui: &mut egui::Ui) -> Option<CustomTab> {
    let mut tab_signal = None;

    ui.group(|ui| {
        ui.colored_label(egui::Color32::GOLD, "📁 ПРОЕКТ (MTE MK1)");
        ui.add_space(4.0);

        egui::ScrollArea::vertical().id_source("project_tree_scroll").show(ui, |ui| {
            // Узел 1: Игровой мир и карты
            let map_node = egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                egui::Id::new("node_maps"),
                true
            );
            map_node.show_header(ui, |ui| {
                let response = ui.selectable_label(false, "🗺️ Мир и Экраны");
                if response.double_clicked() { tab_signal = Some(CustomTab::MapCanvas); }
            }).body(|ui| {
                let res = ui.selectable_label(false, "   📄 mapa.prj (Карта)");
                if res.double_clicked() { tab_signal = Some(CustomTab::MapCanvas); }
            });

            // Узел 2: Исходный Си-код и Скрипты
            let src_node = egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                egui::Id::new("node_src"),
                true
            );
            src_node.show_header(ui, |ui| {
                let response = ui.selectable_label(false, "📜 Скрипты и Код");
                if response.double_clicked() { tab_signal = Some(CustomTab::ScriptEditor); }
            }).body(|ui| {
                let res = ui.selectable_label(false, "   📄 level1.spt (Логика)");
                if res.double_clicked() { tab_signal = Some(CustomTab::ScriptEditor); }
            });

            // Узел 3: Графические ассеты (Заставки, спрайты)
            let gfx_node = egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                egui::Id::new("node_gfx"),
                false
            );
            gfx_node.show_header(ui, |ui| {
                ui.label("🖼️ Графика (gfx/)");
            }).body(|ui| {
                ui.small("   • work.png (Тайлы)");
                ui.small("   • sprites.png (ИИ)");
                ui.small("   • title.png (HUD)");
            });

            ui.separator();

            // --- НОВЫЙ ПУНКТ: НАСТРОЙКИ ИГРОВОГО ДВИЖКА (ВЫНЕСЕН ИЗ ФАЙЛОВ) ---
            let config_response = ui.selectable_label(false, "⚙️ Настройки движка (config.h)");
            if config_response.double_clicked() {
                tab_signal = Some(CustomTab::Configurator); // По двойному щелчку открываем Баланс и HUD
            }
        });
    });

    tab_signal
}
