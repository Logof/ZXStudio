// src/ui/configurator/ide_settings.rs
use crate::app::menu_bar::AppTranslations;
use eframe::egui;

pub fn render_ide_settings(
    ui: &mut egui::Ui,
    compile_command: &mut String,
    z88dk_path: &mut String,
    project_path: &str,
    translations: &AppTranslations,
    compiler_tx: std::sync::mpsc::Sender<String>,
) {
    let loc = &translations.ide_settings;
    let is_english = translations.menu.lang_select.contains("Language");

    ui.heading(format!("🛠️ {}", loc.title));
    ui.add_space(8.0);

    // ВАЖНО: Разрешаем клики в этой панели ВСЕГДА, даже если проект еще не создан!
    ui.set_enabled(true);

    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(4.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.strong(&loc.compiler_section);
            ui.add_space(8.0);

            // Ввод пути Z88DK
            ui.label(&loc.compiler_path_label);
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(z88dk_path)
                        .desired_width(ui.available_width() - 90.0)
                        .font(egui::FontId::monospace(12.0)),
                );

                if ui.button(&loc.compiler_path_btn).clicked() {
                    if let Some(folder) = rfd::FileDialog::new()
                        .set_title(&loc.compiler_path_label)
                        .pick_folder()
                    {
                        *z88dk_path = folder.to_string_lossy().to_string();
                    }
                }
            });
            ui.add_space(10.0);

            // Ввод CLI-команды компиляции
            ui.label(&loc.compiler_label);
            ui.add(
                egui::TextEdit::singleline(compile_command)
                    .desired_width(ui.available_width() - 20.0)
                    .font(egui::FontId::monospace(12.0)),
            );

            ui.weak(&loc.compiler_hint);
        });

    ui.add_space(10.0);

    // Кнопка тестирования
    if ui.button(&loc.test_btn).clicked() {
        println!("[IDE DEBUG] Кнопка 'Проверить вызов' была физически НАЖАТА!");

        // Отправляем первичный статус в поток канала связи
        let _ = compiler_tx.send(format!("{} ({})", loc.status_test_start, compile_command));
        ui.ctx().request_repaint();

        let cmd_string = compile_command.clone();
        let z88_string = z88dk_path.clone();
        let prj_string = project_path.to_string();
        let status_ok_template = loc.status_test_ok.clone();
        let status_fail_template = loc.status_test_fail.clone();

        let mut cmd = if cfg!(windows) {
            let mut c = std::process::Command::new("cmd");
            c.arg("/C").arg(&cmd_string);
            c
        } else {
            let mut c = std::process::Command::new("sh");
            c.arg("-c").arg(&cmd_string);
            c
        };

        if !prj_string.is_empty() {
            cmd.current_dir(&prj_string);
        } else if let Ok(current_dir) = std::env::current_dir() {
            cmd.current_dir(current_dir);
        }

        if !z88_string.is_empty() {
            let bin_dir = std::path::Path::new(&z88_string).join("bin");
            cmd.env("Z88DK_ENV", &z88_string);

            if let Ok(current_path) = std::env::var("PATH") {
                let separator = if cfg!(windows) { ";" } else { ":" };
                let new_path =
                    format!("{}{}{}", bin_dir.to_string_lossy(), separator, current_path);
                cmd.env("PATH", new_path);
            }
        }

        let result_msg = match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let exit_code = output.status.code().unwrap_or(-1);

                if output.status.success() {
                    if !stdout.is_empty() {
                        format!("{} | {}", status_ok_template, stdout.trim())
                    } else {
                        status_ok_template
                    }
                } else {
                    let err_details = if !stderr.is_empty() {
                        stderr.to_string()
                    } else if !stdout.is_empty() {
                        stdout.to_string()
                    } else if is_english {
                        format!("Process exited with code: {}", exit_code)
                    } else {
                        format!("Процесс завершился с системным кодом: {}", exit_code)
                    };
                    format!("{}{}", status_fail_template, err_details.trim())
                }
            }
            Err(e) => {
                format!("{}{}", status_fail_template, e)
            }
        };

        // ОТПРАВЛЯЕМ ИТОГОВЫЙ ОТВЕТ В КАНАЛ СВЯЗИ ГЛАВНОГО UI
        let _ = compiler_tx.send(result_msg);

        // Принудительно заставляем egui проснуться и перерисовать экран
        ui.ctx().request_repaint();
    }
}
