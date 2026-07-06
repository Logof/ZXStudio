// src/ui/map_editor/map_canvas/camera.rs
use eframe::egui;

pub struct CanvasCamera {
    pub zoom: f32,
    pub pan: egui::Vec2,
    pub zoom_id: egui::Id,
    pub pan_id: egui::Id,
}

impl CanvasCamera {
    pub fn load(ctx: &egui::Context) -> Self {
        let zoom_id = egui::Id::new("map_canvas_zoom_factor");
        let pan_id = egui::Id::new("map_canvas_pan_offset");

        let zoom: f32 = ctx.data(|d| d.get_temp(zoom_id)).unwrap_or(1.0);
        let pan: egui::Vec2 = ctx.data(|d| d.get_temp(pan_id)).unwrap_or(egui::Vec2::ZERO);

        Self {
            zoom,
            pan,
            zoom_id,
            pan_id,
        }
    }

    pub fn save(&self, ctx: &egui::Context) {
        ctx.data_mut(|d| {
            d.insert_temp(self.zoom_id, self.zoom);
            d.insert_temp(self.pan_id, self.pan);
        });
    }

    /// Изменение масштаба работает СТРОГО при зажатом Ctrl + колесико
    pub fn handle_zoom(
        &mut self,
        ui: &egui::Ui,
        response: &egui::Response,
        selected_screen: usize,
        map_w: usize,
    ) {
        let scr_w_px = 15.0 * 32.0; // 480.0
        let scr_h_px = 10.0 * 32.0; // 320.0

        // Проверяем, находится ли курсор физически над холстом
        let is_mouse_over_canvas = ui.ctx().input(|i| {
            if let Some(pos) = i.pointer.hover_pos() {
                response.rect.contains(pos)
            } else {
                false
            }
        });

        if is_mouse_over_canvas {
            // Собираем дельту движения колесика мыши напрямую из потока событий кадра
            let (wheel_x, wheel_y) = ui.ctx().input(|i| {
                let mut x_delta = 0.0;
                let mut y_delta = 0.0;
                for event in &i.events {
                    if let egui::Event::MouseWheel { delta, .. } = event {
                        x_delta += delta.x;
                        y_delta += delta.y;
                    }
                }
                (x_delta, y_delta)
            });

            if ui.ctx().input(|i| i.modifiers.ctrl) {
                // НАПРАВЛЕНИЕ 1: Ctrl зажат -> Масштабирование (Зум) с удержанием фокуса
                if wheel_y != 0.0 {
                    // 1. Находим мировые координаты центра ВЫДЕЛЕННОГО экрана (без учета зума)
                    let scr_col = selected_screen % map_w;
                    let scr_row = selected_screen / map_w;
                    let target_world_center_x = (scr_col as f32 * scr_w_px) + (scr_w_px / 2.0);
                    let target_world_center_y = (scr_row as f32 * scr_h_px) + (scr_h_px / 2.0);

                    // 2. Рассчитываем, в каком конкретно месте холста (в экранных пикселях)
                    // этот центр находится прямо сейчас, до изменения масштаба
                    let screen_pos_before_x = target_world_center_x * self.zoom + self.pan.x;
                    let screen_pos_before_y = target_world_center_y * self.zoom + self.pan.y;

                    // 3. Применяем новый масштаб
                    let old_zoom = self.zoom;
                    self.zoom = (self.zoom + wheel_y * 0.05).clamp(0.15, 2.0);

                    // 4. Корректируем pan (смещение), чтобы экранная точка центра осталась строго на месте!
                    if self.zoom != old_zoom {
                        self.pan.x = screen_pos_before_x - (target_world_center_x * self.zoom);
                        self.pan.y = screen_pos_before_y - (target_world_center_y * self.zoom);
                    }
                }
            } else {
                // НАПРАВЛЕНИЕ 2: Ctrl НЕ зажат -> Скролл всей карты колесиком мыши
                if ui.ctx().input(|i| i.modifiers.shift) {
                    self.pan.x += wheel_y * 20.0;
                } else {
                    self.pan.y += wheel_y * 20.0;
                    self.pan.x += wheel_x * 20.0;
                }
            }
        }
    }

    /// Перетаскивание карты теперь работает ТОЛЬКО на среднюю кнопку мыши (колесико),
    /// так как Ctrl + ЛКМ у нас зарезервирован под Зум/Клик
    pub fn handle_pan(&mut self, _ui: &egui::Ui, response: &egui::Response) -> bool {
        let is_pan_drag = response.dragged_by(egui::PointerButton::Middle);
        if is_pan_drag {
            self.pan += response.drag_delta();
        }
        is_pan_drag
    }

    pub fn to_virtual_cell(
        &self,
        mouse_pos: egui::Pos2,
        rect: egui::Rect,
        map_w: usize,
        map_h: usize,
        scr_w: f32,
        scr_h: f32,
    ) -> Option<(usize, u8, u8)> {
        // Учитываем физический сдвиг холста на экране (координаты относительно rect.min)
        let world_x = (mouse_pos.x - rect.min.x - self.pan.x) / self.zoom;
        let world_y = (mouse_pos.y - rect.min.y - self.pan.y) / self.zoom;

        // Отсекаем клики, если они выходят за левую или верхнюю границу виртуального мира
        if world_x >= 0.0 && world_y >= 0.0 {
            let scr_col = (world_x / scr_w) as usize;
            let scr_row = (world_y / scr_h) as usize;

            if scr_col < map_w && scr_row < map_h {
                let current_scr_idx = scr_row * map_w + scr_col;

                // Рассчитываем локальные координаты тайла внутри Spectrum-экрана
                let local_x = ((world_x - (scr_col as f32 * scr_w)) / 32.0) as u8;
                let local_y = ((world_y - (scr_row as f32 * scr_h)) / 32.0) as u8;

                if local_x < 15 && local_y < 10 {
                    return Some((current_scr_idx, local_x, local_y));
                }
            }
        }
        None
    }
}
