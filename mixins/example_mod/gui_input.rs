fn gui_input(game: &mut crate::game::Game<'_>, event: &WindowEvent, id: WindowId) -> core::ops::ControlFlow<()> {
    use winit::keyboard::{KeyCode, PhysicalKey};
    if game.window.as_ref().is_none_or(|window| window.id() != id) {
        return core::ops::ControlFlow::Continue(());
    }
    if let WindowEvent::KeyboardInput { event: key, .. } = event {
        if key.state == ElementState::Pressed && !key.repeat {
            if key.physical_key == PhysicalKey::Code(KeyCode::ShiftRight) {
                if !crate::game::example_mod::is_open() && (game.screen != Screen::Playing || game.player.is_none() || game.title.is_some()) {
                    return core::ops::ControlFlow::Continue(());
                }
                let open = crate::game::example_mod::toggle();
                game.capture(!open);
                return core::ops::ControlFlow::Break(());
            }
            if crate::game::example_mod::is_open() && key.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                crate::game::example_mod::close();
                game.capture(true);
                return core::ops::ControlFlow::Break(());
            }
        }
    }
    if crate::game::example_mod::is_open() {
        if let WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } = event {
            if let Some(window) = &game.window {
                let batch = HudBatch::new(window.inner_size().into());
                let (x, y) = crate::game::example_mod::origin(batch.width, batch.height);
                let (cx, cy) = game.cursor;
                if cx >= x + 10.0 && cx < x + 210.0 && cy >= y + 55.0 && cy < y + 76.0 {
                    game.smooth_camera = !game.smooth_camera;
                }
                if cx >= x + 10.0 && cx < x + 210.0 && cy >= y + 82.0 && cy < y + 106.0 {
                    crate::game::example_mod::close();
                    game.capture(true);
                }
            }
        }
        if matches!(event, WindowEvent::KeyboardInput { .. } | WindowEvent::MouseInput { .. } | WindowEvent::MouseWheel { .. }) {
            return core::ops::ControlFlow::Break(());
        }
    }
    core::ops::ControlFlow::Continue(())
}