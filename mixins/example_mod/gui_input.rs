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
                if key.physical_key == PhysicalKey::Code(KeyCode::ShiftRight) {
                    state.save_session();
                    state.reset_ui();
                    drop(state);
                    panel::close();
                    game.capture(true);
                }
                return core::ops::ControlFlow::Break(());
            }
            if key.physical_key == PhysicalKey::Code(KeyCode::ShiftRight) {
                state.save_session();
                state.reset_ui();
                drop(state);
                panel::close();
                game.capture(true);
                return core::ops::ControlFlow::Break(());
            }
            if state.search_focus {
                match key.physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => {
                        if state.query.is_empty() {
                            state.search_focus = false;
                        } else {
                            state.query.clear();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Enter) => state.search_focus = false,
                    PhysicalKey::Code(KeyCode::Backspace) => {
                        state.query.pop();
                    }
                    _ => {
                        if let Some(text) = key.text.as_deref() {
                            for ch in text.chars() {
                                if !ch.is_control() && state.query.chars().count() < 24 {
                                    state.query.push(ch);
                                }
                            }
                        }
                    }
                }
                state.selected = None;
                state.mode_open = false;
                state.dragging = false;
                state.page = 0;
                return core::ops::ControlFlow::Break(());
            }
            if key.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                if state.mode_open {
                    state.mode_open = false;
                } else if state.side != panel::SIDE_NONE {
                    state.close_side();
                } else if !state.query.is_empty() {
                    state.query.clear();
                    state.page = 0;
                } else if state.selected.is_some() {
                    state.selected = None;
                } else {
                    state.save_session();
                    state.reset_ui();
                    drop(state);
                    panel::close();
                    game.capture(true);
                }
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
            let mut state = panel::GUI.lock().unwrap_or_else(|e| e.into_inner());
            if !panel::is_open()
                && (game.screen != Screen::Playing || game.player.is_none() || game.title.is_some())
            {
                return core::ops::ControlFlow::Continue(());
            }
            let open = panel::toggle();
            if open {
                state.ensure_loaded();
            }
            drop(state);
            game.capture(!open);
            return core::ops::ControlFlow::Break(());
        }
    }
    if panel::is_open() {
        if let WindowEvent::MouseInput {
            state: ElementState::Released,
            button: MouseButton::Left,
            ..
        } = event
        {
            panel::GUI
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .dragging = false;
        }
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
            let side = state.side;
            state.search_focus = false;
            state.dragging = false;
            let mut handled = false;
            if side != panel::SIDE_NONE && prism_ui::side_close(side).contains(cx, cy) {
                state.close_side();
                handled = true;
            } else if side == panel::SIDE_BINDS {
                if prism_ui::binds_clear(side).contains(cx, cy) {
                    for row in &mut state.binds {
                        for bind in row.iter_mut() {
                            *bind = None;
                        }
                    }
                    handled = true;
                } else {
                    let bound = state.bound();
                    for (index, (category, module, _)) in bound.iter().take(8).enumerate() {
                        if prism_ui::binds_row(side, index).contains(cx, cy) {
                            state.goto_category(*category);
                            state.query.clear();
                            state.open_module(*category, *module);
                            state.close_side();
                            handled = true;
                            break;
                        }
                    }
                }
            } else if side == panel::SIDE_CONFIG {
                for index in 0..state.configs.len().min(7) {
                    if prism_ui::config_row(side, index).contains(cx, cy) {
                        state.config_selected = Some(index);
                        handled = true;
                        break;
                    }
                }
                if !handled {
                    for which in 0..4 {
                        if prism_ui::config_button(side, which).contains(cx, cy) {
                            state.config_action(which);
                            handled = true;
                            break;
                        }
                    }
                }
            } else if side == panel::SIDE_THEMES {
                for index in 0..panel::THEMES.len() {
                    if prism_ui::theme_row(side, index).contains(cx, cy) {
                        state.theme = index as u8;
                        handled = true;
                        break;
                    }
                }
            }
            if !handled && side == panel::SIDE_NONE {
                if state.mode_open {
                    if let Some((category, module)) = state.selected {
                        if let Some(slot) = state
                            .visible()
                            .iter()
                            .position(|&(c, m)| c == category && m == module)
                        {
                            let column = slot % prism_ui::COLUMNS;
                            let row = slot / prism_ui::COLUMNS;
                            for option in 0..panel::MODES.len() {
                                if prism_ui::dropdown_mode_option(side, column, row, option)
                                    .contains(cx, cy)
                                {
                                    state.modes[category][module] = option as u8;
                                    break;
                                }
                            }
                        }
                    }
                    state.mode_open = false;
                    handled = true;
                } else if let Some((category, module)) = state.selected {
                    if let Some(slot) = state
                        .visible()
                        .iter()
                        .position(|&(c, m)| c == category && m == module)
                    {
                        let column = slot % prism_ui::COLUMNS;
                        let row = slot / prism_ui::COLUMNS;
                        if prism_ui::dropdown_toggle(side, column, row).contains(cx, cy) {
                            state.enabled[category][module] = !state.enabled[category][module];
                            handled = true;
                        } else if prism_ui::dropdown_mode(side, column, row).contains(cx, cy) {
                            state.mode_open = true;
                            handled = true;
                        } else if prism_ui::dropdown_bind(side, column, row).contains(cx, cy) {
                            state.waiting_bind = true;
                            handled = true;
                        } else if prism_ui::dropdown_slider_hit(side, column, row).contains(cx, cy)
                        {
                            let track = prism_ui::dropdown_slider_track(side, column, row);
                            let value =
                                ((cx - track.x) / track.w * 100.0).round().clamp(0.0, 100.0);
                            state.values[category][module] = value as u8;
                            state.dragging = true;
                            handled = true;
                        } else if prism_ui::dropdown_rect(side, column, row).contains(cx, cy) {
                            handled = true;
                        }
                    }
                }
            }
            if !handled && side == panel::SIDE_NONE {
                let search = prism_ui::search_rect(side);
                if prism_ui::search_clear(side).contains(cx, cy) {
                    state.query.clear();
                    state.page = 0;
                    state.search_focus = true;
                    state.selected = None;
                    state.mode_open = false;
                    handled = true;
                } else if search.contains(cx, cy) {
                    state.search_focus = true;
                    state.selected = None;
                    state.mode_open = false;
                    handled = true;
                }
            }
            // The footer stepper walks the current view six chips at a time.
            if !handled && side == panel::SIDE_NONE {
                if prism_ui::page_prev(side).contains(cx, cy) {
                    state.step_page(-1);
                    handled = true;
                } else if prism_ui::page_next(side).contains(cx, cy) {
                    state.step_page(1);
                    handled = true;
                }
            }
            // Sidebar navigation stays live on every page.
            if !handled {
                for index in 0..5 {
                    if prism_ui::category_row(side, index).contains(cx, cy) {
                        state.goto_category(index);
                        state.query.clear();
                        state.selected = None;
                        state.close_side();
                        handled = true;
                        break;
                    }
                }
            }
            if !handled {
                for index in 0..panel::MANAGER.len() {
                    if prism_ui::manager_row(side, index).contains(cx, cy) {
                        let target = panel::SIDE_BINDS + index;
                        if state.side == target {
                            state.close_side();
                        } else {
                            if state.side == panel::SIDE_NONE {
                                state.side_anim = 0.0;
                            }
                            state.side = target;
                            state.selected = None;
                            state.waiting_bind = false;
                            state.mode_open = false;
                            state.dragging = false;
                            if target == panel::SIDE_CONFIG {
                                state.refresh_configs();
                                if state.config_selected.is_none() && !state.configs.is_empty() {
                                    state.config_selected = Some(0);
                                }
                            }
                        }
                        handled = true;
                        break;
                    }
                }
            }
            if !handled && side == panel::SIDE_NONE {
                let visible = state.visible();
                for (slot, (category, module)) in
                    visible.iter().take(prism_ui::PER_PAGE).enumerate()
                {
                    let column = slot % prism_ui::COLUMNS;
                    let row = slot / prism_ui::COLUMNS;
                    if !prism_ui::module_row(side, column, row).contains(cx, cy) {
                        continue;
                    }
                    if prism_ui::toggle_row(side, column, row).contains(cx, cy) {
                        state.enabled[*category][*module] = !state.enabled[*category][*module];
                    } else if state.selected == Some((*category, *module)) {
                        state.selected = None;
                        state.mode_open = false;
                    } else {
                        state.open_module(*category, *module);
                    }
                    handled = true;
                    break;
                }
            }
            if !handled && side == panel::SIDE_NONE {
                // Clicking empty panel space dismisses the settings drop-down.
                state.selected = None;
                state.mode_open = false;
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
