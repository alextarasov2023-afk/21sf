// theme.rs — цветовая схема CS-стиля.
pub struct Theme {
    pub bg: [f32; 4],
    pub panel: [f32; 4],
    pub accent: [f32; 4],
    pub text: [f32; 4],
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: [0.08, 0.08, 0.10, 0.95],
            panel: [0.12, 0.12, 0.15, 0.98],
            accent: [0.20, 0.60, 1.00, 1.00],
            text: [0.95, 0.95, 0.98, 1.00],
        }
    }
}