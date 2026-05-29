use crate::app::states::WizardStep;
use crate::app::ZxIdeApp;
use crate::models::project::TileMode;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, app: &mut ZxIdeApp) {
    ui.label("Размеры карты в экранах:");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut app.project.config.map_goals.map_w).clamp_range(1..=16));
        ui.label("Ширина");
        ui.add(egui::DragValue::new(&mut app.project.config.map_goals.map_h).clamp_range(1..=16));
        ui.label("Высота");
    });
    ui.add_space(15.0);
    ui.separator();
    ui.add_space(10.0);

    // ============================================================================
    // НОВОЕ УЛУЧШЕНИЕ: Интерактивный выбор режима тайлов для La Churrera (MTE MK1)
    // ============================================================================
    ui.label("Режим работы с тайлами (Тайлсет):");
    ui.add_space(5.0);

    let modes = [
        TileMode::Packed16,
        TileMode::Packed16WithShadows,
        TileMode::Extended48,
    ];
    for mode in &modes {
        ui.radio_value(&mut app.project.tile_mode, *mode, mode.name());
    }

    ui.add_space(10.0);

    // Выводим подсказку и технические требования к файлу work.png в реальном времени
    egui::Frame::canvas(ui.style())
        .fill(ui.visuals().faint_bg_color)
        .rounding(4.0)
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.strong("Описание режима:");
            ui.label(app.project.tile_mode.description());
            ui.add_space(4.0);
            let (w, h) = app.project.tile_mode.expected_dimensions();
            ui.colored_label(
                ui.visuals().weak_text_color(),
                format!(
                    "⚠️ Требуемый ассет 'gfx/work.png' должен быть строго: {}x{} px",
                    w, h
                ),
            );
        });

    ui.add_space(20.0);

    ui.horizontal(|ui| {
        if ui.button("◀ Назад").clicked() {
            app.wizard_step = WizardStep::SelectPlatform;
        }

        if ui.button("🚀 Создать проект!").clicked() {
            // 1. Синхронизируем размер и дефолтные значения массива поведений под выбранный режим тайлов
            app.project.tile_behaviours = app.project.tile_mode.default_behaviours();

            // 2. Инициализируем пустую сетку комнат в памяти приложения на основе размеров мира
            let total_screens =
                app.project.config.map_goals.map_w * app.project.config.map_goals.map_h;
            app.project.screens.clear();
            for i in 0..total_screens {
                app.project.screens.insert(
                    format!("screen_{}", i),
                    crate::models::ScreenData::default(),
                );
            }

            // 3. Вызываем сервис дисковой автоматизации ядра
            match crate::core::io::create_project_structure(
                &app.project_path,
                &app.project_name,
                &app.project,
            ) {
                Ok(saved_path) => {
                    // Закрываем мастер приветствия, открывая основную рабочую среду IDE
                    app.wizard_active = false;

                    app.status_message = format!(
                        "🎉 Проект '{}' успешно развернут! Сейв: {:?}",
                        app.project_name, saved_path
                    );
                }
                Err(err_msg) => {
                    // Если диск защищен от записи или указан неверный путь — выводим ошибку в статус-бар и не закрываем визард
                    app.status_message = format!("❌ Ошибка развертывания на диск: {}", err_msg);
                }
            }
        }
    });
}
