// src/core/compiler_runner/mod.rs
pub mod step1;
pub mod step2;
pub mod step3;
pub mod step4;
pub mod step5;

use crate::models::ProjectData;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread;

pub struct CompilerRunner;

impl CompilerRunner {
    /// Запускает асинхронный процесс полной пошаговой сборки Spectrum-игры в фоне.
    /// Передает состояние ProjectData для динамического контроля упаковщиков и ASM флагов.
    pub fn spawn_build_task(
        project_path: String,
        z88dk_path: String,
        compile_command: String,
        is_multilevel: bool,
        project: ProjectData, // 🔥 Передаем снимок ОЗУ проекта в поток
        log_tx: Sender<String>,
    ) {
        // Уходим в изолированный фоновый поток ОС, полностью освобождая UI egui от фризов
        thread::spawn(move || {
            let _ = log_tx.send("🛠️ ИНИЦИАЛИЗАЦИЯ ФОНОВОГО ОРКЕСТРАТОРА СБОРКИ MK1v4...".to_string());
            
            let base_path = Path::new(&project_path);
            if !base_path.exists() {
                let _ = log_tx.send("❌ Ошибка сборщика: Путь к проекту не существует!".to_string());
                return;
            }

            // ШАГ 1: Нативная нарезка и компиляция сценариев Churscript
            if !step1::run_spt2hp_step(base_path, &log_tx) { return; }

            // ШАГ 2: Компиляция Си-кода через Z88DK + Предварительное сжатие карты через ResourceCompressor
            if !step2::run_z88dk_step(base_path, &z88dk_path, &compile_command, is_multilevel, &project, &log_tx) { return; }

            // ШАГ 3: Нативная токенизация и ассемблирование BASIC-загрузчика
            if !step3::run_bas2tap_step(base_path, &log_tx) { return; }

            // ШАГ 4: Подготовка экрана заставки
            if !step4::run_loading_screen_step(base_path, &log_tx) { return; }

            // ШАГ 5: Финальный контейнерный монтаж релиза ленты game.tap с XOR суммами
            let _ = step5::run_tape_concat_step(base_path, &log_tx);
        });
    }
}
