fn gui_render(mut batch: HudBatch, game: &mut crate::game::Game<'_>) -> HudBatch {
    use crate::game::example_mod as panel;
    use horizon_client::prism_ui;

    if !panel::is_open() {
        return batch;
    }
    let mut state = panel::GUI.lock().unwrap_or_else(|e| e.into_inner());
    let state = &mut *state;
    let visible = state.visible();
    let bound = state.bound();
    let supported = visible
        .iter()
        .map(|(category, module)| panel::is_supported(panel::MODULES[*category][*module]))
        .collect::<Vec<_>>();
    let page_count = state.page_count();
    let total = state.matching().len();
    let mut view = prism_ui::View {
        side: state.side,
        category: state.category,
        page: state.page,
        page_count,
        total,
        enabled: state.enabled,
        animation: state.animation,
        binds: &state.binds,
        values: state.values,
        modes: &state.modes,
        selected: state.selected,
        waiting_bind: state.waiting_bind,
        mode_open: state.mode_open,
        query: &state.query,
        search_focus: state.search_focus,
        visible: &visible,
        supported: &supported,
        bound: &bound,
        categories: &panel::CATEGORIES,
        modules: &panel::MODULES,
        manager: &panel::MANAGER,
        mode_names: &panel::MODES,
        themes: &panel::THEMES,
        theme: usize::from(state.theme),
        configs: &state.configs,
        config_selected: state.config_selected,
        status: &state.status,
        open: &mut state.open,
        side_anim: &mut state.side_anim,
        dragging: state.dragging,
    };
    prism_ui::render(&game.gui.atlas, &mut batch, game.cursor, &mut view);
    state.animation = view.animation;
    state.values = view.values;
    batch
}
