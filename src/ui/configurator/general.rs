// src/ui/configurator/general.rs
use crate::app::menu_bar::AppTranslations; // Импортируем нашу глобальную локализацию
use crate::models::project::{TileMode, LevelData}; // Импортируем перечисление режимов и структуру уровня
use crate::models::ProjectData;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, project: &mut ProjectData) {
    // Безопасно извлекаем кэш переводов из временных данных контекста egui
    let translations = ui
        .ctx()
        .data(|d| d.get_temp::<AppTranslations>(egui::Id::new("translations_cache")))
        .unwrap_or_else(|| AppTranslations::load(crate::app::menu_bar::Language::Ru));

    let is_english = translations.menu.lang_select.contains("Language");

    // ЛОКАЛИЗАЦИОННЫЕ МАРКЕРЫ СТРОК
    let t_levels_title = if is_english { "🎛️ Project Levels Management" } else { "🎛️ Управление уровнями проекта" };
    let t_add_level = if is_english { "➕ Add Level" } else { "➕ Добавить уровень" };
    let t_del_level = if is_english { "❌ Delete Current" } else { "❌ Удалить текущий" };
    let t_level_label = if is_english { "Active Level:" } else { "Активный уровень:" };

    let t_platform = if is_english {
        "💾 Platform, Memory & Global Properties"
    } else {
        "💾 Платформа, Память и Глобальные свойства"
    };
    let t_mode_128k = if is_english {
        "MODE_128K — Enable expanded 128K memory mode"
    } else {
        "MODE_128K — Включить расширенный режим памяти 128K"
    };
    let t_veng = if is_english {
        "VENG_SELECTOR (Advanced Engine Selector)"
    } else {
        "VENG_SELECTOR (Расширенный селектор движка)"
    };
    let t_decoder = if is_english {
        "USE_MAP_CUSTOM_DECODER (Custom Map Decoder)"
    } else {
        "USE_MAP_CUSTOM_DECODER (Кастомный декодер карты)"
    };
    let t_arch = if is_english {
        "📊 Graphics Architecture:"
    } else {
        "📊 Архитектура графики:"
    };
    let t_audio = if is_english {
        "🎵 Audio System (Music & Sound Effects)"
    } else {
        "🎵 Аудиосистема (Музыка и Звуковые Эффекты)"
    };
    let t_arkos = if is_english {
        "USE_ARKOS_PLAYER (Use Arkos instead of Wyz)"
    } else {
        "USE_ARKOS_PLAYER (Использовать Arkos вместо Wyz)"
    };
    let t_channel = if is_english {
        "ARKOS_SFX_CHANNEL (SFX Sound Channel)"
    } else {
        "ARKOS_SFX_CHANNEL (Звуковой канал SFX)"
    };
    let t_audio_warn = if is_english {
        "ℹ️ Arkos Player settings require MODE_128K to be active."
    } else {
        "ℹ️ Настройки Arkos Player активны только при включенном MODE_128K."
    };
    let t_render = if is_english {
        "⏱️ Rendering & Frame Rate"
    } else {
        "⏱️ Рендеринг и частота кадров"
    };
    let t_fps_limit = if is_english {
        "MIN_FAPS_PER_FRAME (FPS Limiter: 1=50fps, 2=25fps)"
    } else {
        "MIN_FAPS_PER_FRAME (Ограничитель FPS: 1=50fps, 2=25fps)"
    };

    // ============================================================================
    // НОВОЕ УЛУЧШЕНИЕ: Блок мультилевельного управления (Level Selector)
    // ============================================================================
    ui.strong(t_levels_title);
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        ui.label(t_level_label);
        
        let mut selected = project.current_level_index;
        egui::ComboBox::from_id_source("multilevel_selector")
            .selected_text(format!("[{}] {}", selected + 1, project.levels[selected].name))
            .show_ui(ui, |ui| {
                for i in 0..project.levels.len() {
                    ui.selectable_value(&mut selected, i, format!("[{}] {}", i + 1, project.levels[i].name));
                }
            });

        if selected != project.current_level_index {
            project.current_level_index = selected;
            // Шлем триггеры на горячую перезагрузку графического контекста под TileMode нового уровня
            ui.ctx().data_mut(|d| {
                d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
            });
        }

        // Поле быстрого переименования имени уровня в ОЗУ
        ui.add_space(10.0);
        ui.text_edit_singleline(&mut project.levels[project.current_level_index].name);
    });

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        if ui.button(t_add_level).clicked() {
            let mut new_lvl = LevelData::default();
            new_lvl.name = format!("Level {}", project.levels.len() + 1);
            
            // Наследуем размеры сетки комнат для нового уровня из общих целей
            let total_screens = project.config.map_goals.map_w * project.config.map_goals.map_h;
            new_lvl.screens.clear();
            for i in 0..total_screens {
                new_lvl.screens.insert(format!("screen_{}", i), crate::models::ScreenData::default());
            }
            
            project.levels.push(new_lvl);
            project.current_level_index = project.levels.len() - 1;
            
            ui.ctx().data_mut(|d| {
                d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
            });
        }

        // Кнопка удаления с защитой: нельзя удалить единственный уровень в игре
        ui.add_enabled_ui(project.levels.len() > 1, |ui| {
            if ui.button(t_del_level).clicked() {
                project.levels.remove(project.current_level_index);
                project.current_level_index = 0;
                
                ui.ctx().data_mut(|d| {
                    d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                    d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
                });
            }
        });
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(6.0);

    // Вектор настроек платформы
    ui.strong(t_platform);
    ui.add_space(6.0);

    // Архитектурные флаги платформы и памяти
    ui.checkbox(&mut project.config.general.mode_128k, t_mode_128k);
    ui.checkbox(&mut project.config.general.veng_selector, t_veng);
    ui.checkbox(
        &mut project.config.general.use_map_custom_decoder,
        t_decoder,
    );

    ui.add_space(6.0);

    // ============================================================================
    // ИСПРАВЛЕНИЕ: Выбор архитектуры графики (TileMode) теперь привязан к уровню
    // ============================================================================
    ui.horizontal(|ui| {
        ui.label(t_arch);

        let active_idx = project.current_level_index;
        let old_mode = project.levels[active_idx].tile_mode;

        egui::ComboBox::from_id_source("config_tile_mode_selector")
            .selected_text(project.levels[active_idx].tile_mode.name())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut project.levels[active_idx].tile_mode,
                    TileMode::Packed16,
                    TileMode::Packed16.name(),
                );
                ui.selectable_value(
                    &mut project.levels[active_idx].tile_mode,
                    TileMode::Packed16WithShadows,
                    TileMode::Packed16WithShadows.name(),
                );
                ui.selectable_value(
                    &mut project.levels[active_idx].tile_mode,
                    TileMode::Extended48,
                    TileMode::Extended48.name(),
                );
            });

        // Если пользователь переключил режим уровня — шлем триггеры для горячей перезагрузки PNG
        if project.levels[active_idx].tile_mode != old_mode {
            // Синхронизируем размер массива поведений под новый выбранный режим
            let new_mode = project.levels[active_idx].tile_mode;
            project.levels[active_idx].tile_behaviours = new_mode.default_behaviours();

            ui.ctx().data_mut(|d| {
                d.insert_temp(egui::Id::new("trigger_reset_tileset_graphics"), true);
                d.insert_temp(egui::Id::new("trigger_clear_sliced_textures"), true);
            });
        }
    });
    // ============================================================================

    // Информационная сводка лимитов памяти под выбранный режим активного уровня
    ui.add_space(6.0);
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(4.0)
        .inner_margin(8.0)
        .show(ui, |ui| {
            let map_w = project.config.map_goals.map_w as usize;
            let map_h = project.config.map_goals.map_h as usize;
            let total_screens = map_w * map_h;
            let screen_size_bytes = 15 * 10; // Размер одного экрана в тайлах
            let active_idx = project.current_level_index;

            let total_map_bytes = match project.levels[active_idx].tile_mode {
                TileMode::Packed16 | TileMode::Packed16WithShadows => {
                    (total_screens * screen_size_bytes + 1) / 2
                }
                TileMode::Extended48 => total_screens * screen_size_bytes,
            };

            if is_english {
                ui.small(format!(
                    "World size: {} screens. Map weight in RAM: {} bytes.",
                    total_screens, total_map_bytes
                ));
            } else {
                ui.small(format!(
                    "Размер игрового мира: {} экранов. Вес карты в RAM: {} байт.",
                    total_screens, total_map_bytes
                ));
            }
        });

    ui.separator();
    ui.strong(t_audio);
    ui.add_space(4.0);

    // Взаимосвязь: Arkos Player доступен только в режиме MODE_128K
    ui.add_enabled_ui(project.config.general.mode_128k, |ui| {
        ui.checkbox(&mut project.config.general.use_arkos_player, t_arkos);

        ui.add_enabled_ui(project.config.general.use_arkos_player, |ui| {
            ui.add(
                egui::Slider::new(&mut project.config.general.arkos_sfx_channel, 0..=2)
                    .text(t_channel),
            );
        });
    });

    if !project.config.general.mode_128k {
        ui.colored_label(egui::Color32::GOLD, t_audio_warn);
    }

    ui.separator();
    ui.strong(t_render);
    ui.add(
        egui::Slider::new(&mut project.config.general.min_faps_per_frame, 1..=4).text(t_fps_limit),
    );
}
