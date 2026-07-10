// src/core/project_template.rs
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::sync::mpsc::Sender;

/// Вшиваем ZIP-архив со всеми исходниками Си-ядра Churrera v4.8 прямо в EXE/бинарник IDE
const CHURRERA_TEMPLATE_BYTES: &[u8] = include_bytes!("../../assets/churrera_v48_template.zip");

/// Нативно распаковывает структуру шаблона движка в пустую папку нового проекта
pub fn deploy_engine_template(project_path: &str, log_tx: &Sender<String>) -> bool {
    let _ = log_tx.send("📦 [Template] Развертывание Си-ядра движка MTE MK1 v4.8 из ресурсов IDE...".to_string());

    let target_dir = Path::new(project_path);
    if !target_dir.exists() {
        if let Err(e) = fs::create_dir_all(target_dir) {
            let _ = log_tx.send(format!("❌ Ошибка создания папки проекта: {}", e));
            return false;
        }
    }

    // Читаем байты архива прямо из ОЗУ бинарника
    let reader = Cursor::new(CHURRERA_TEMPLATE_BYTES);
    let mut archive = match zip::ZipArchive::new(reader) {
        Ok(arch) => arch,
        Err(e) => {
            let _ = log_tx.send(format!("❌ Ошибка инициализации встроенного архива: {}", e));
            return false;
        }
    };

    // Распаковываем каждый файл и подпапку
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            // Если это папка (например, dev/ или gfx/) — создаем её
            let _ = fs::create_dir_all(&outpath);
        } else {
            // Если это файл (например, main.c) — выжигаем на диск
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    let _ = fs::create_dir_all(p);
                }
            }
            
            let mut outfile = match fs::File::create(&outpath) {
                Ok(f) => f,
                Err(e) => {
                    let _ = log_tx.send(format!("❌ Не удалось создать файл шаблона {:?}: {}", outpath, e));
                    return false;
                }
            };
            
            if let Err(e) = std::io::copy(&mut file, &mut outfile) {
                let _ = log_tx.send(format!("❌ Ошибка извлечения данных в {:?}: {}", outpath, e));
                return false;
            }
        }
    }

    let _ = log_tx.send("   [OK] Все исходные Си-файлы ядра (main.c, churromain.c) успешно развернуты!".to_string());
    true
}
