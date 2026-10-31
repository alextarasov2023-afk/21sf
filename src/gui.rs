// gui.rs — Click GUI каркас в стиле CS.
mod panel;
mod module;
mod animation;
mod font;
mod theme;
mod icon;
mod keybind;
mod render;

pub use panel::Panel;
pub use module::Module;
pub use animation::Animation;
pub use font::Font;
pub use theme::Theme;
pub use icon::Icon;
pub use keybind::Keybind;
pub use render::{RenderContext, render_panel, render_module};

pub struct ClickGui {
    pub panels: Vec<Panel>,
    pub font: Font,
    pub theme: Theme,
    pub open: bool,
    pub keybind: Keybind,
    pub ctx: RenderContext,
}

impl ClickGui {
    pub fn new() -> Self {
        Self {
            panels: vec![
                Panel::new("Combat", vec![Module::new("KillAura"), Module::new("AutoTotem")]),
                Panel::new("Movement", vec![Module::new("Speed"), Module::new("Fly")]),
                Panel::new("Visuals", vec![Module::new("ESP"), Module::new("Chams")]),
                Panel::new("Other", vec![Module::new("ClickGui"), Module::new("Settings")]),
            ],
            font: Font::default(),
            theme: Theme::default(),
            open: false,
            keybind: Keybind::new("RightShift"),
            ctx: RenderContext::new(1920.0, 1080.0),
        }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn update(&mut self, dt: f32) {
        for p in &mut self.panels {
            p.update(dt);
        }
    }

    pub fn render(&self) {
        if !self.open { return; }
        let mut y = 20.0;
        for p in &self.panels {
            render_panel(&self.ctx, p.name, 20.0, y, 200.0, 200.0, self.theme.accent);
            let mut my = y + 30.0;
            for m in &p.modules {
                render_module(&self.ctx, m.name, 28.0, my, 184.0, 20.0, m.enabled);
                my += 24.0;
            }
            y += 220.0;
        }
    }
}