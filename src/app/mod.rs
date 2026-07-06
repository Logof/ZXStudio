mod app_struct;
mod asset_loader;
pub mod menu_bar;
pub mod states;
pub mod tab_viewer;
mod theme;
pub mod toolbar;
pub mod wizard;

// Экспортируем структуру приложения наружу для всего остального проекта
pub use app_struct::ZxIdeApp;

use crate::core::validator::world_collisions::WorldValidator;
use crate::core::validator::ClashError;
use crate::models::ProjectData;
use eframe::egui;
use egui_dock::{DockArea, DockState, Style};
use menu_bar::{render_menu_bar, Language};
use states::{CustomTab, MapEditMode, Tab, WizardStep};
use tab_viewer::ZxTabViewer;
use toolbar::render_toolbar;

impl ZxIdeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dock_state: Option<DockState<CustomTab>> = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "dock_state"));

        let saved_language: Language = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "current_language"))
            .unwrap_or(Language::Ru);

        let saved_recent: Vec<String> = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "recent_projects"))
            .unwrap_or_else(Vec::new);
        // ============================================================================

        let final_dock_state = dock_state.unwrap_or_else(Self::create_default_layout);
        let translations = menu_bar::AppTranslations::load(saved_language);
        let saved_command: String = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "compile_command"))
            .unwrap_or_else(|| "zcc +zx -vn main.c -o game.tap".to_string());

        // ДОБАВЛЕНО: Восстанавливаем сохраненный путь к Z88DK
        let saved_z88dk_path: String = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "z88dk_path"))
            .unwrap_or_else(String::new);

        let (compiler_tx, compiler_rx) = std::sync::mpsc::channel();

        Self {
            project: ProjectData::default(),
            current_tab: Tab::MapEditor,
            selected_screen: 0,
            selected_tile: 0,
            script_text: "ENTERING SCREEN 0\nIF FLAG 1 = 0\nTHEN\n\tSET TILE (5, 5) = 14\nEND"
                .to_string(),
            status_message: "IDE успешно инициализирована".to_string(),
            wizard_active: true,
            wizard_step: WizardStep::WelcomeChoice,
            clash_errors: Vec::new(),
            dock_state: final_dock_state,
            map_edit_mode: MapEditMode::Tiles,
            cyber_palette_index: 0,
            selected_enemy_type: 0,
            selected_hotspot_type: 1,
            selected_font_char_idx: 0,
            tileset_texture: None,
            sliced_tile_textures: Vec::new(),

            sprites_texture: None,
            hud_frame_texture: None,

            enable_hotspot_items: true,
            enable_hotspot_keys: true,
            enable_hotspot_refills: true,

            project_name: "my_retro_game".to_string(),
            project_path: String::new(),

            configurator_tab: crate::ui::configurator::ConfigTab::General,

            current_language: saved_language,
            recent_projects: saved_recent,
            translations: translations,
            compile_command: saved_command,
            z88dk_path: saved_z88dk_path,
            compiler_log: String::new(),
            compiler_tx,
            compiler_rx,
        }
    }

    pub fn create_default_layout() -> DockState<CustomTab> {
        let mut default_state = DockState::new(vec![
            CustomTab::MapCanvas,
            CustomTab::ScriptEditor,
            CustomTab::Configurator,
            CustomTab::IdeSettings,
        ]);
        let surface = default_state.main_surface_mut();
        let root_node = egui_dock::NodeIndex::root();

        let [top_node, _bottom_node] =
            surface.split_below(root_node, 0.80, vec![CustomTab::Console]);
        let [_left_node, _main_work_node] =
            surface.split_left(top_node, 0.18, vec![CustomTab::ProjectTree]);

        default_state
    }
}

impl eframe::App for ZxIdeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "dock_state", &self.dock_state);
        eframe::set_value(storage, "current_language", &self.current_language);
        eframe::set_value(storage, "recent_projects", &self.recent_projects);
        eframe::set_value(storage, "z88dk_path", &self.z88dk_path);
        eframe::set_value(storage, "compile_command", &self.compile_command);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.15);

        // ============================================================================
        // ИСПРАВЛЕНО: Жёсткая изоляция аудита. На холст карты комнат должны попадать
        // ИСКЛЮЧИТЕЛЬНО критические ошибки застревания сущностей в стенах (World Collisions).
        // Ошибки цветового наложения (Attribute Clash) заставки удаляются из этого среза,
        // что гарантированно ликвидирует три призрачных квадрата в координатах (2,4), (2,8), (10,3).
        // ============================================================================
        if !self.wizard_active {
            let mut current_errors = WorldValidator::validate_world(&self.project);

            // Фильтруем массив: оставляем только те ошибки, которые сгенерированы WorldValidator.
            // Графические ошибки attribute clash (если они содержат слова "цвет", "атрибут" или "clash")
            // безжалостно вырезаются из контекста редактора карт комнат.
            current_errors.retain(|err| {
                !err.message.contains("цвет")
                    && !err.message.contains("атрибут")
                    && !err.message.contains("Clash")
            });

            let scr_key = format!("screen_{}", self.selected_screen);
            let current_level = &self.project.levels[self.project.current_level_index];
            if let Some(screen_data) = current_level.screens.get(&scr_key) {
                // Если все враги на экране стёрты (TypeID == 0) — принудительно очищаем ошибки этого экрана
                let has_active_entities = screen_data.enemies.iter().any(|e| e.type_id != 0)
                    || screen_data.hotspot.type_id != 0;

                if !has_active_entities {
                    current_errors.retain(|err| err.screen_index != self.selected_screen);
                }

                // Формируем диагностический отчет для вкладки «Логи компиляции»
                let mut debug_log = format!(
                    "🔍 ДИАГНОСТИКА ЭКРАНА {} (Ключ: {})\n",
                    self.selected_screen, scr_key
                );
                debug_log.push_str(&format!(
                    "• Активных (живых) врагов: {}\n",
                    screen_data
                        .enemies
                        .iter()
                        .filter(|e| e.type_id != 0)
                        .count()
                ));
                debug_log.push_str(&format!(
                    "• Ошибок коллизий в базе холста: {}\n",
                    current_errors
                        .iter()
                        .filter(|e| e.screen_index == self.selected_screen)
                        .count()
                ));

                self.status_message = debug_log;
            }

            self.clash_errors = current_errors;
        }

        // 1. Применяем шрифты и оформление из модуля темы
        theme::apply_modern_dark_theme(ctx);

        // 2. Запускаем фоновый конвейер ассетов из модуля загрузчика
        asset_loader::process_asset_loading(self, ctx);

        // 3. Рендерим приветственный Мастер (Wizard), если проект ещё не открыт
        if self.wizard_active {
            egui::CentralPanel::default()
                .frame(egui::Frame::none().fill(egui::Color32::from_rgb(14, 14, 17)))
                .show(ctx, |_ui| {
                    self.render_project_wizard(ctx);
                });
            return;
        }

        // 4. Главный рабочий экран IDE (если визард пройден)
        render_menu_bar(self, ctx);
        render_toolbar(self, ctx);

        // Нижний статус-бар
        egui::TopBottomPanel::bottom("status_bar")
            .frame(
                egui::Frame::none()
                    .inner_margin(6.0)
                    .fill(egui::Color32::from_rgb(22, 22, 26)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&self.status_message);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Изменен путь адресации max_bullets под новую структуру данных
                        let numblocks =
                            (16 * 10) + (self.project.config.shooting_boxes.max_bullets * 5);
                        ui.label(format!("NUMBLOCKS: {}", numblocks));
                    });
                });
            });

        // 5. Основное рабочее пространство DockArea
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(14, 14, 17)))
            .show(ctx, |ui| {
                let mut dock_style = Style::from_egui(ui.style());
                dock_style.separator.width = 3.0;
                dock_style.separator.color_idle = egui::Color32::from_rgb(30, 30, 35);
                dock_style.separator.color_hovered = egui::Color32::from_rgb(0, 150, 255);
                dock_style.tab_bar.bg_fill = egui::Color32::from_rgb(20, 20, 25);
                dock_style.tab.active.bg_fill = egui::Color32::from_rgb(14, 14, 17);
                dock_style.tab.active.rounding = egui::Rounding::same(0.0);
                dock_style.tab.inactive.rounding = egui::Rounding::same(0.0);
                dock_style.tab.focused.rounding = egui::Rounding::same(0.0);

                let safe_clash_errors: &[ClashError] = if self
                    .clash_errors
                    .iter()
                    .any(|e| e.screen_index == self.selected_screen)
                {
                    &self.clash_errors
                } else {
                    &[] // Теперь пустой срез идеально совпадает по типу!
                };

                let mut viewer = ZxTabViewer {
                    project: &mut self.project,
                    project_name: &self.project_name,
                    project_path: &self.project_path,
                    configurator_tab: &mut self.configurator_tab,
                    selected_screen: &mut self.selected_screen,
                    selected_tile: &mut self.selected_tile,
                    script_text: &mut self.script_text,
                    clash_errors: safe_clash_errors, // Передаем чистый отфильтрованный срез
                    status_message: &self.status_message,
                    map_edit_mode: &mut self.map_edit_mode,
                    selected_enemy_type: &mut self.selected_enemy_type,
                    sliced_tile_textures: &self.sliced_tile_textures,
                    sprites_texture: &self.sprites_texture,
                    hud_frame_texture: &self.hud_frame_texture,
                    selected_font_char_idx: &mut self.selected_font_char_idx,
                    translations: &self.translations,
                    z88dk_path: &mut self.z88dk_path,
                    compile_command: &mut self.compile_command,
                    compiler_log: &mut self.compiler_log,
                    compiler_tx: self.compiler_tx.clone(),
                };

                DockArea::new(&mut self.dock_state)
                    .style(dock_style)
                    .show_inside(ui, &mut viewer);

                // Метод try_recv() мгновенно проверяет буфер канала, не блокируя поток GUI
                while let Ok(incoming_msg) = self.compiler_rx.try_recv() {
                    // 1. Дублируем краткую версию в нижнюю плашку status_bar
                    self.status_message = incoming_msg.clone();

                    // 2. Рассчитываем текущую системную дату и время через std::time
                    let timestamp = if let Ok(duration) =
                        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                    {
                        let secs = duration.as_secs();
                        let seconds = secs % 60;
                        let minutes = (secs / 60) % 60;
                        let hours = (secs / 3600) % 24;
                        format!("[{:02}:{:02}:{:02}] ", hours, minutes, seconds)
                    } else {
                        "[??:??:??] ".to_string()
                    };

                    // 3. Дописываем полный текст лога с таймстампом
                    self.compiler_log
                        .push_str(&format!("{}{}\n", timestamp, incoming_msg));

                    // ============================================================================
                    // УЛУЧШЕНИЕ: Очистка памяти. Ограничиваем буфер строго до 500 последних строк
                    // ============================================================================
                    let line_count = self.compiler_log.matches('\n').count();
                    if line_count > 500 {
                        // Ищем индекс символа '\n', который отсекает первые (лишние) строки
                        let skip_lines = line_count - 500;
                        if let Some((byte_idx, _)) =
                            self.compiler_log.match_indices('\n').nth(skip_lines - 1)
                        {
                            // Безопасно отрезаем старый кусок текста, оставляя только свежие 500 строк
                            self.compiler_log = self.compiler_log[byte_idx + 1..].to_string();
                        }
                    }
                    // ============================================================================
                }

                // 4. Автоматически выводим вкладку консоли на передний план док-системы
                if let Some(tab_coordinates) = self.dock_state.find_tab(&CustomTab::Console) {
                    self.dock_state.set_active_tab(tab_coordinates);
                }

                // Форсируем мгновенную перерисовку текущего кадра
                ui.ctx().request_repaint();

                if let Some(incoming_status) = ui.ctx().data_mut(|d| {
                    d.remove_temp::<String>(egui::Id::new("global_compiler_status_msg"))
                }) {
                    // 1. Извлекаем текущий накопительный лог из памяти
                    let mut current_log = ui
                        .ctx()
                        .data(|d| d.get_temp::<String>(egui::Id::new("global_compiler_log_buffer")))
                        .unwrap_or_else(String::new);

                    // 2. Формируем красивую строчку с маркером
                    let timestamp = ">".to_string();
                    current_log.push_str(&format!("{} {}\n", timestamp, incoming_status));

                    // 3. Сохраняем обновленный многострочный массив обратно в память egui
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(egui::Id::new("global_compiler_log_buffer"), current_log);
                    });

                    // 4. Принудительно выводим вкладку консоли на передний план док-системы
                    if let Some(tab_coordinates) = self.dock_state.find_tab(&CustomTab::Console) {
                        self.dock_state.set_active_tab(tab_coordinates);
                    }

                    // Форсируем мгновенную перерисовку экрана
                    ui.ctx().request_repaint();
                }
                // ============================================================================

                if let Some(target_tab) = ui
                    .ctx()
                    .data_mut(|d| d.remove_temp::<CustomTab>(egui::Id::new("tab_switch_signal")))
                {
                    if let Some(tab_coordinates) = self.dock_state.find_tab(&target_tab) {
                        self.dock_state.set_active_tab(tab_coordinates);
                    }
                }

                // Перехватываем сигнал из конфигуратора и создаем файл!
                if let Some(true) = ui
                    .ctx()
                    .data_mut(|d| d.remove_temp::<bool>(egui::Id::new("trigger_create_lock_clear")))
                {
                    match create_custom_lock_clear_file(&self.project_path) {
                        Ok(()) => {
                            self.status_message =
                                "✨ Файл dev/custom_lock_clear.h успешно добавлен в проект"
                                    .to_string();
                        }
                        Err(e) => {
                            self.status_message = format!("❌ Ошибка автогенерации скрипта: {}", e);
                        }
                    }
                }

                // Перехватываем сигнал двойного клика из дерева проекта
                if let Some(file_to_load) = ui.ctx().data_mut(|d| {
                    d.remove_temp::<String>(egui::Id::new("trigger_load_script_file"))
                }) {
                    // Безопасно читаем файл с диска и пишем в буфер редактора скриптов
                    match std::fs::read_to_string(&file_to_load) {
                        Ok(content) => {
                            self.script_text = content;

                            // Вырезаем имя файла для красивого вывода в статус-бар
                            if let Some(name) = std::path::Path::new(&file_to_load).file_name() {
                                self.status_message = format!(
                                    "📖 Файл {} успешно открыт в редакторе",
                                    name.to_string_lossy()
                                );
                            }
                        }
                        Err(e) => {
                            self.status_message = format!("❌ Не удалось прочитать файл: {}", e);
                        }
                    }
                }
            });
    }
}

fn create_custom_lock_clear_file(project_path: &str) -> std::io::Result<()> {
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    if project_path.is_empty() {
        return Ok(());
    }

    let base_path = Path::new(project_path);
    let script_dir = base_path.join("dev");

    if !script_dir.exists() {
        fs::create_dir_all(&script_dir)?;
    }

    let file_path = script_dir.join("custom_lock_clear.h");

    if !file_path.exists() {
        let mut file = fs::File::create(file_path)?;
        let template = b"// MTE MK1 (la Churrera)\n// Custom Lock Clear Script\n\n// This code is executed when a lock is removed from the screen.\n// Write your custom C/Assembler code here.\n\n// Example:\n// if (n_pant == 4) {\n//     // Do something special on screen 4\n// }\n";
        file.write_all(template)?;
    }

    Ok(())
}
