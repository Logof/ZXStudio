// src/app/world_inspector.rs
use crate::app::ZxIdeApp;
use crate::core::validator::world_collisions::WorldValidator;
use crate::core::validator::ClashError;

/// Проводит аудит коллизий комнат и чистит память, если враги были стерты
pub fn process_world_validation(app: &mut ZxIdeApp) {
    if app.wizard_active {
        return;
    }

    let mut current_errors = WorldValidator::validate_world(&app.project);

    // Удаляем из этого среза графические ошибки attribute clash заставки
    current_errors.retain(|err| {
        !err.message.contains("цвет")
            && !err.message.contains("атрибут")
            && !err.message.contains("Clash")
    });

    let scr_key = format!("screen_{}", app.selected_screen);
    let current_level = &app.project.levels[app.project.current_level_index];
    
    if let Some(screen_data) = current_level.screens.get(&scr_key) {
        let has_active_entities = screen_data.enemies.iter().any(|e| e.type_id != 0)
            || screen_data.hotspot.type_id != 0;

        if !has_active_entities {
            current_errors.retain(|err| err.screen_index != app.selected_screen);
        }

        // Диагностический отчет для статус-бара
        let mut debug_log = format!(
            "🔍 ДИАГНОСТИКА ЭКРАНА {} (Ключ: {})\n",
            app.selected_screen, scr_key
        );
        debug_log.push_str(&format!(
            "• Активных (живых) врагов: {}\n",
            screen_data.enemies.iter().filter(|e| e.type_id != 0).count()
        ));
        debug_log.push_str(&format!(
            "• Ошибок коллизий в базе холста: {}\n",
            current_errors.iter().filter(|e| e.screen_index == app.selected_screen).count()
        ));

        app.status_message = debug_log;
    }

    app.clash_errors = current_errors;
}

/// Безопасно формирует срез ошибок для текущего активного экрана
pub fn get_safe_clash_errors(clash_errors: &[ClashError], selected_screen: usize) -> &[ClashError] {
    if clash_errors.iter().any(|e| e.screen_index == selected_screen) {
        clash_errors
    } else {
        &[]
    }
}