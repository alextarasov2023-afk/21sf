// render.rs — рендер-каркас для Click GUI.
// Абстрактные примитивы: rect, text, icon.
// Реальный Horizon API подставляется позже.

pub struct RenderContext {
    pub width: f32,
    pub height: f32,
}

impl RenderContext {
    pub fn new(w: f32, h: f32) -> Self {
        Self { width: w, height: h }
    }

    /// Отрисовка прямоугольника (фон панели/модуля).
    pub fn draw_rect(&self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        // TODO: Horizon GUI API: draw_rect(x, y, w, h, color)
        let _ = (x, y, w, h, color);
    }

    /// Отрисовка текста (название панели/модуля).
    pub fn draw_text(&self, text: &str, x: f32, y: f32, size: f32, color: [f32; 4]) {
        // TODO: Horizon GUI API: draw_text(text, x, y, size, color)
        let _ = (text, x, y, size, color);
    }

    /// Отрисовка иконки.
    pub fn draw_icon(&self, name: &str, x: f32, y: f32, size: f32) {
        // TODO: Horizon GUI API: draw_icon(name, x, y, size)
        let _ = (name, x, y, size);
    }
}

/// Рендер панели.
pub fn render_panel(ctx: &RenderContext, name: &str, x: f32, y: f32, w: f32, h: f32, accent: [f32; 4]) {
    // Фон панели
    ctx.draw_rect(x, y, w, h, [0.12, 0.12, 0.15, 0.98]);
    // Заголовок
    ctx.draw_rect(x, y, w, 24.0, accent);
    ctx.draw_text(name, x + 8.0, y + 6.0, 14.0, [1.0, 1.0, 1.0, 1.0]);
}

/// Рендер модуля.
pub fn render_module(ctx: &RenderContext, name: &str, x: f32, y: f32, w: f32, h: f32, enabled: bool) {
    let bg = if enabled { [0.2, 0.6, 1.0, 0.3] } else { [0.15, 0.15, 0.18, 0.9] };
    ctx.draw_rect(x, y, w, h, bg);
    ctx.draw_text(name, x + 8.0, y + 4.0, 12.0, [0.95, 0.95, 0.98, 1.0]);
    // Toggle indicator
    let toggle_color = if enabled { [0.2, 0.8, 0.4, 1.0] } else { [0.4, 0.4, 0.4, 1.0] };
    ctx.draw_rect(x + w - 20.0, y + 4.0, 12.0, 12.0, toggle_color);
}