// font.rs — кастомный шрифт (placeholder).
#[derive(Default)]
pub struct Font {
    pub name: &'static str,
    pub size: f32,
}

impl Font {
    pub fn default() -> Self {
        Self { name: "Inter", size: 14.0 }
    }
}