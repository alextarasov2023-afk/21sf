fn gui_input(game: &mut crate::game::Game<'_>, event: &WindowEvent, id: WindowId) -> core::ops::ControlFlow<()> {
    use winit::keyboard::{KeyCode, PhysicalKey};
    if game.window.as_ref().is_none_or(|window| window.id() != id) {
        return core::ops::ControlFlow::Continue(());
    }
    if let WindowEvent::KeyboardInput { event: key, .. } = event {
        if key.state == ElementState::Pressed && !key.repeat {
            if crate::game::example_mod::is_open() {
                let mut state = crate::game::example_mod::GUI.lock().unwrap_or_else(|e| e.into_inner());
                if state.waiting_bind {
                    if let Some((category, module)) = state.selected {
                        if let PhysicalKey::Code(code) = key.physical_key {
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
                    }
                    return core::ops::ControlFlow::Break(());
                }
                if key.physical_key == PhysicalKey::Code(KeyCode::Escape) && state.selected.is_some() {
                    state.selected = None;
                    return core::ops::ControlFlow::Break(());
                }
            } else if game.screen == Screen::Playing && game.player.is_some() && game.title.is_none() {
                if let PhysicalKey::Code(code) = key.physical_key {
                    if code != KeyCode::ShiftRight && code != KeyCode::Escape {
                        crate::game::example_mod::GUI.lock().unwrap_or_else(|e| e.into_inner()).activate_bind(&format!("{code:?}"));
                    }
                }
            }
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
                let (x, y, scale) = crate::game::example_mod::layout(batch.width, batch.height);
                let (cx, cy) = game.cursor;
                let (cx, cy) = ((cx - x) / scale, (cy - y) / scale);
                let mut state = crate::game::example_mod::GUI.lock().unwrap_or_else(|e| e.into_inner());
                if let Some((category, module)) = state.selected {
                    if (270.0..450.0).contains(&cx) && (120.0..142.0).contains(&cy) {
                        state.waiting_bind = true;
                    } else if (270.0..450.0).contains(&cx) && (167.0..189.0).contains(&cy) {
                        state.values[category][module] = (((cx - 270.0) / 180.0 * 100.0).round() as u8).min(100);
                    } else if (400.0..450.0).contains(&cx) && (220.0..244.0).contains(&cy) {
                        state.selected = None;
                    }
                    return core::ops::ControlFlow::Break(());
                }
                if (10.0..125.0).contains(&cx) {
                    for i in 0..5 {
                        let top = 73.0 + i as f32 * 28.0;
                        if cy >= top && cy < top + 24.0 { state.category = i; }
                    }
                }
                for col in 0..2 {
                    let left = 148.0 + col as f32 * 223.0;
                    for row in 0..3 {
                        let top = 99.0 + row as f32 * 62.0;
                        if cx >= left && cx < left + 211.0 && cy >= top && cy < top + 36.0 {
                            let category = state.category;
                            let idx = col * 3 + row;
                            if cx >= left + 148.0 && cx < left + 174.0 {
                                state.selected = Some((category, idx));
                                state.waiting_bind = false;
                            } else {
                                state.enabled[category][idx] = !state.enabled[category][idx];
                            }
                        }
                    }
                }
            }
        }
        if matches!(event, WindowEvent::KeyboardInput { .. } | WindowEvent::MouseInput { .. } | WindowEvent::MouseWheel { .. }) {
            return core::ops::ControlFlow::Break(());
        }
    }
    core::ops::ControlFlow::Continue(())
}