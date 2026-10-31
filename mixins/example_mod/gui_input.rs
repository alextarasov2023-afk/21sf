fn gui_input(
    game: &mut crate::game::Game<'_>,
    event: &WindowEvent,
    id: WindowId,
) -> core::ops::ControlFlow<()> {
    use crate::game::example_mod as panel;
    use horizon_client::prism_ui;
    use winit::keyboard::{KeyCode, PhysicalKey};
    if game.window.as_ref().is_none_or(|window| window.id() != id) {
        return core::ops::ControlFlow::Continue(());
    }
    if let WindowEvent::KeyboardInput { event: key, .. } = event
        && key.state == ElementState::Pressed
        && !key.repeat
    {
        if panel::is_open() {
            let mut state = panel::GUI.lock().unwrap_or_else(|e| e.into_inner());
            if state.waiting_bind {
                if let Some((category, module)) = state.selected
                    && let PhysicalKey::Code(code) = key.physical_key
                {
                    if code == KeyCode::Escape {
                        state.waiting_bind = false;
                    } else if code == KeyCode::Backspace || code == KeyCode::Delete {
                        state.binds[category][module] = None;
                        state.waiting_bind = false;
                    } else if code != KeyCode::ShiftRight {
                        state.binds[category][module] = Some(format!("{code:?}"));
                        state.waiting_bind = false;
                    }
                }
                return core::ops::ControlFlow::Break(());
            }
            if key.physical_key == PhysicalKey::Code(KeyCode::Escape) && state.selected.is_some() {
                state.selected = None;
                return core::ops::ControlFlow::Break(());
            }
        } else if game.screen == Screen::Playing
            && game.player.is_some()
            && game.title.is_none()
            && let PhysicalKey::Code(code) = key.physical_key
            && code != KeyCode::ShiftRight
            && code != KeyCode::Escape
        {
            panel::GUI
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .activate_bind(&format!("{code:?}"));
        }
        if key.physical_key == PhysicalKey::Code(KeyCode::ShiftRight) {
            if !panel::is_open()
                && (game.screen != Screen::Playing || game.player.is_none() || game.title.is_some())
            {
                return core::ops::ControlFlow::Continue(());
            }
            let open = panel::toggle();
            game.capture(!open);
            return core::ops::ControlFlow::Break(());
        }
        if panel::is_open() && key.physical_key == PhysicalKey::Code(KeyCode::Escape) {
            panel::close();
            game.capture(true);
            return core::ops::ControlFlow::Break(());
        }
    }
    if panel::is_open() {
        if let WindowEvent::MouseInput {
            state: ElementState::Pressed,
            button: MouseButton::Left,
            ..
        } = event
            && let Some(window) = &game.window
        {
            let batch = HudBatch::new(window.inner_size().into());
            let (cx, cy) = prism_ui::to_design(game.cursor, batch.width, batch.height, batch.scale);
            let mut state = panel::GUI.lock().unwrap_or_else(|e| e.into_inner());
            if let Some((category, module)) = state.selected {
                if prism_ui::BIND_BOX.contains(cx, cy) {
                    state.waiting_bind = true;
                } else if prism_ui::SLIDER_HIT.contains(cx, cy) {
                    state.values[category][module] =
                        (((cx - prism_ui::SLIDER.x) / prism_ui::SLIDER.w * 100.0).round() as u8)
                            .min(100);
                } else if prism_ui::DONE.contains(cx, cy) {
                    state.selected = None;
                }
            }
            for index in 0..5 {
                if prism_ui::category_row(index).contains(cx, cy) {
                    state.category = index;
                    state.selected = None;
                    state.waiting_bind = false;
                }
            }
            'rows: for column in 0..2 {
                for row in 0..3 {
                    let rect = prism_ui::module_row(column, row);
                    if !rect.contains(cx, cy) {
                        continue;
                    }
                    let category = state.category;
                    let index = column * 3 + row;
                    if prism_ui::dots_hit(column, row).contains(cx, cy) {
                        state.selected = Some((category, index));
                        state.waiting_bind = false;
                    } else if prism_ui::keyboard_hit(column, row).contains(cx, cy) {
                        state.selected = Some((category, index));
                        state.waiting_bind = true;
                    } else {
                        state.enabled[category][index] = !state.enabled[category][index];
                    }
                    break 'rows;
                }
            }
        }
        if matches!(
            event,
            WindowEvent::KeyboardInput { .. }
                | WindowEvent::MouseInput { .. }
                | WindowEvent::MouseWheel { .. }
        ) {
            return core::ops::ControlFlow::Break(());
        }
    }
    core::ops::ControlFlow::Continue(())
}
