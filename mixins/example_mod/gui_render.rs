fn gui_render(mut batch: HudBatch, game: &mut crate::game::Game<'_>) -> HudBatch {
    use crate::game::example_mod as panel;
    use horizon_client::prism_ui;

    if !panel::is_open() {
        return batch;
    }
    let mut state = panel::GUI.lock().unwrap_or_else(|e| e.into_inner());
    let state = &mut *state;
    let mut view = prism_ui::View {
        category: state.category,
        enabled: state.enabled,
        animation: state.animation,
        binds: &state.binds,
        values: state.values,
        selected: state.selected,
        waiting_bind: state.waiting_bind,
        categories: &panel::CATEGORIES,
        modules: &panel::MODULES,
        manager: &panel::MANAGER,
    };
    prism_ui::render(&game.gui.atlas, &mut batch, game.cursor, &mut view);
    state.animation = view.animation;
    batch
}
