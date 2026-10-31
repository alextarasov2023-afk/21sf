// module.rs
use super::animation::Animation;

pub struct Module {
    pub name: &'static str,
    pub enabled: bool,
    pub anim: Animation,
}

impl Module {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            enabled: false,
            anim: Animation::new(0.15),
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn update(&mut self, dt: f32) {
        self.anim.update(dt);
    }

    pub fn render(&self) {
        let _ = self.anim.value();
        // Рендер модуля (placeholder)
    }
}