fn gui_render(mut batch: HudBatch, game: &mut crate::game::Game<'_>) -> HudBatch {
    if !crate::game::example_mod::is_open() {
        return batch;
    }
    let atlas = &game.gui.atlas;
    let (x, y) = crate::game::example_mod::origin(batch.width, batch.height);
    batch.fill(atlas, x, y, 220.0, 130.0, rgba(18, 20, 29, 245));
    batch.fill(atlas, x, y, 220.0, 25.0, rgba(38, 106, 190, 255));
    batch.text(atlas, "YAYA  /  CLICK GUI", x + 9.0, y + 8.0, rgba(255, 255, 255, 255), true);
    batch.text(atlas, "Horizon native client", x + 10.0, y + 36.0, rgba(220, 225, 235, 255), false);
    batch.fill(atlas, x + 10.0, y + 55.0, 200.0, 21.0, rgba(47, 55, 72, 255));
    let camera = if game.smooth_camera { "Smooth camera: ON" } else { "Smooth camera: OFF" };
    batch.text(atlas, camera, x + 18.0, y + 61.0, rgba(255, 255, 255, 255), false);
    batch.fill(atlas, x + 10.0, y + 82.0, 200.0, 24.0, rgba(47, 55, 72, 255));
    batch.text(atlas, "Close  [Right Shift / Esc]", x + 18.0, y + 90.0, rgba(255, 255, 255, 255), false);
    batch
}