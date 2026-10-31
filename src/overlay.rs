// overlay.rs — отдельное окно (overlay) с egui.
// Работает поверх Minecraft как отдельное окно.

use egui::Context;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub struct Overlay {
    pub open: bool,
}

impl Overlay {
    pub fn new() -> Self {
        Self { open: false }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn run(&self) {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title("Yaya Overlay")
            .with_inner_size(winit::dpi::LogicalSize::new(400.0, 300.0))
            .build(&event_loop)
            .unwrap();

        let ctx = Context::default();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Wait;

            match event {
                winit::event::Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::Event::AboutToWait => {
                    let _ = ctx.run(Default::default(), |ctx| {
                        egui::Window::new("Yaya GUI").show(ctx, |ui| {
                            ui.label("Combat");
                            ui.label("Movement");
                            ui.label("Visuals");
                            ui.label("Other");
                        });
                    });
                    window.request_redraw();
                }
                _ => {}
            }
        }).unwrap();
    }
}