// panel.rs
use super::module::Module;
use super::animation::Animation;

pub struct Panel {
    pub name: &'static str,
    pub modules: Vec<Module>,
    pub anim: Animation,
    pub open: bool,
}

impl Panel {
    pub fn new(name: &'static str, modules: Vec<Module>) -> Self {
        Self {
            name,
            modules,
            anim: Animation::new(0.2),
            open: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.anim.update(dt);
        for m in &mut self.modules {
            m.update(dt);
        }
    }

    pub fn render(&self) {
        // Рендер панели с анимацией (placeholder)
        let _ = self.anim.value();
        for m in &self.modules {
            m.render();
        }
    }
}