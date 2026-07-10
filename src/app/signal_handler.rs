// src/app/signal_handler.rs
use crate::app::ZxIdeApp;
use crate::app::states::CustomTab;
use eframe::egui;

/// Диспетчер межпотоковых и внутренних сигналов компонентов
pub fn handle_incoming_signals(app: &mut ZxIdeApp, ui: &mut egui::Ui) {
    // 1. Сигнал на асинхронную сборку релиза .TAP с упаковщиками MK1v4
    if let Some(true) = ui.ctx().data_mut(|d| d.remove_temp::<bool>(egui::Id::new("trigger_async_compile_and_build"))) {
        app.compiler_log.clear();
        
        // 🔥 КРИТИЧЕСКИЙ ФИКС: Впечатываем стартовые строки напрямую в ОЗУ, чтобы консоль мгновенно ожила!
        app.compiler_log.push_str("⏳ [IDE] Инициализация конвейера сборки ZX Spectrum...\n");
        app.compiler_log.push_str("👉 [IDE] Передача управления кроссплатформенному оркестратору CompilerRunner...\n");

        super::compiler_observer::force_focus_console(app);

        crate::core::compiler_runner::CompilerRunner::spawn_build_task(
            app.project_path.clone(),
            app.z88dk_path.clone(),
            app.compile_command.clone(),
            app.project.levels.len() > 1,
            app.project.clone(),
            app.project_name.clone(), // 🔥 ПЕРЕДАЕМ ИМЯ ИГРЫ (например, "cheril")
            app.compiler_tx.clone(),
        );
        
        // Запрашиваем немедленное обновление экрана egui
        ui.ctx().request_repaint();
    }

    // 2. Сигнал автогенерации скрипта снятия замков dev/custom_lock_clear.h
    if let Some(true) = ui.ctx().data_mut(|d| d.remove_temp::<bool>(egui::Id::new("trigger_create_lock_clear"))) {
        match create_custom_lock_clear_file(&app.project_path) {
            Ok(()) => {
                app.status_message = "✨ Файл dev/custom_lock_clear.h успешно добавлен в проект".to_string();
            }
            Err(e) => {
                app.status_message = format!("❌ Ошибка автогенерации скрипта: {}", e);
            }
        }
    }

    // 3. Сигнал ленивой подгрузки файлов по двойному клику из дерева Проекта
    if let Some(file_to_load) = ui.ctx().data_mut(|d| d.remove_temp::<String>(egui::Id::new("trigger_load_script_file"))) {
        match std::fs::read_to_string(&file_to_load) {
            Ok(content) => {
                app.script_text = content;
                if let Some(name) = std::path::Path::new(&file_to_load).file_name() {
                    app.status_message = format!("📖 Файл {} успешно открыт в редакторе", name.to_string_lossy());
                }
            }
            Err(e) => {
                app.status_message = format!("❌ Не удалось прочитать файл: {}", e);
            }
        }
    }

    // 4. Сигнал принудительного фокуса вкладок
    if let Some(target_tab) = ui.ctx().data_mut(|d| d.remove_temp::<CustomTab>(egui::Id::new("tab_switch_signal"))) {
        if let Some(tab_coordinates) = app.dock_state.find_tab(&target_tab) {
            app.dock_state.set_active_tab(tab_coordinates);
        }
    }

    // Форсируем мгновенную перерисовку текущего кадра
    ui.ctx().request_repaint();
}

fn create_custom_lock_clear_file(project_path: &str) -> std::io::Result<()> {
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    if project_path.is_empty() { return Ok(()); }
    let base_path = Path::new(project_path);
    let script_dir = base_path.join("dev");

    if !script_dir.exists() { fs::create_dir_all(&script_dir)?; }
    let file_path = script_dir.join("custom_lock_clear.h");

    if !file_path.exists() {
        let mut file = fs::File::create(file_path)?;
        let template = b"// MTE MK1 (la Churrera)\n// Custom Lock Clear Script\n\n// This code is executed when a lock is removed from the screen.\n// Write your custom C/Assembler code here.\n";
        file.write_all(template)?;
    }
    Ok(())
}
