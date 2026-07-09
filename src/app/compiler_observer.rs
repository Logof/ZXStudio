// src/app/compiler_observer.rs
use crate::app::ZxIdeApp;
use crate::app::states::CustomTab;
use eframe::egui;

/// Опрашивает каналы mpsc без просадки фреймрейта основного UI
pub fn poll_compiler_channels(app: &mut ZxIdeApp, ui: &mut egui::Ui) {
    while let Ok(incoming_msg) = app.compiler_rx.try_recv() {
        app.status_message = incoming_msg.clone();

        // Рассчитываем текущее системное время на чистом Rust
        let timestamp = if let Ok(duration) =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        {
            let secs = duration.as_secs();
            format!("[{:02}:{:02}:{:02}] ", (secs / 3600) % 24, (secs / 60) % 60, secs % 60)
        } else {
            "[??:??:??] ".to_string()
        };

        app.compiler_log.push_str(&format!("{}{}\n", timestamp, incoming_msg));

        // Ротация ОЗУ логов: держим строго последние 500 строк
        let line_count = app.compiler_log.matches('\n').count();
        if line_count > 500 {
            let skip_lines = line_count - 500;
            if let Some((byte_idx, _)) = app.compiler_log.match_indices('\n').nth(skip_lines - 1) {
                app.compiler_log = app.compiler_log[byte_idx + 1..].to_string();
            }
        }
    }

    // Обрабатываем дополнительные реактивные сообщения из памяти контекста egui
    if let Some(incoming_status) = ui.ctx().data_mut(|d| {
        d.remove_temp::<String>(egui::Id::new("global_compiler_status_msg"))
    }) {
        let mut current_log = ui.ctx().data(|d| d.get_temp::<String>(egui::Id::new("global_compiler_log_buffer"))).unwrap_or_else(String::new);
        current_log.push_str(&format!("> {}\n", incoming_status));

        ui.ctx().data_mut(|d| {
            d.insert_temp(egui::Id::new("global_compiler_log_buffer"), current_log);
        });

        // Переводим фокус на консоль
        force_focus_console(app);
    }
}

pub fn force_focus_console(app: &mut ZxIdeApp) {
    if let Some(coords) = app.dock_state.find_tab(&CustomTab::Console) {
        app.dock_state.set_active_tab(coords);
    }
}
