fn gui_render(mut batch: HudBatch, game: &mut crate::game::Game<'_>) -> HudBatch {
    fn rounded(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, w: f32, h: f32, radius: f32, color: u32) {
        let r = radius.min(w * 0.5).min(h * 0.5).max(0.0);
        batch.fill(atlas, x, y + r, w, h - 2.0 * r, color);
        let steps = 8;
        for i in 0..steps {
            let a = i as f32 * r / steps as f32;
            let b = (i + 1) as f32 * r / steps as f32;
            let inset = r - (r * r - (r - a).powi(2)).max(0.0).sqrt();
            batch.fill(atlas, x + inset, y + a, w - 2.0 * inset, b - a, color);
            batch.fill(atlas, x + inset, y + h - b, w - 2.0 * inset, b - a, color);
        }
    }
    use crate::game::example_mod::{CATEGORIES, GUI, MODULES, layout};
    if !crate::game::example_mod::is_open() { return batch; }
    let (x, y, s) = layout(batch.width, batch.height);
    let atlas = &game.gui.atlas;
    let start = batch.vertices.len();
    let mut state = GUI.lock().unwrap_or_else(|e| e.into_inner());
    let bg = rgba(10, 11, 14, 248);
    let panel = rgba(16, 17, 21, 255);
    let border = rgba(31, 33, 39, 255);
    let muted = rgba(146, 149, 158, 255);
    let accent = rgba(123, 156, 224, 255);
    rounded(&mut batch, atlas, x, y, 600.0, 360.0, 9.0, bg);
    batch.fill(atlas, x + 135.0, y, 1.0, 360.0, border);
    batch.fill(atlas, x, y + 46.0, 600.0, 1.0, border);
    batch.text(atlas, "P", x + 32.0, y + 16.0, accent, false);
    batch.text(atlas, "PRISM", x + 49.0, y + 16.0, rgba(223, 227, 237, 255), false);
    batch.text(atlas, "FEATURES", x + 18.0, y + 61.0, muted, false);
    for (i, name) in CATEGORIES.iter().enumerate() {
        let yy = y + 78.0 + i as f32 * 28.0;
        if state.category == i { rounded(&mut batch, atlas, x + 10.0, yy - 5.0, 115.0, 24.0, 6.0, rgba(29, 35, 48, 255)); }
        batch.text(atlas, ["+", "<>", "o", "@", "*"][i], x + 20.0, yy + 2.0, accent, false);
        batch.text(atlas, name, x + 40.0, yy + 2.0, if state.category == i { accent } else { muted }, false);
    }
    batch.text(atlas, "MANAGER", x + 18.0, y + 236.0, muted, false);
    batch.text(atlas, "Settings  /  Preview", x + 18.0, y + 258.0, muted, false);
    batch.text(atlas, "Player", x + 18.0, y + 328.0, rgba(220, 224, 234, 255), false);
    rounded(&mut batch, atlas, x + 148.0, y + 12.0, 438.0, 24.0, 5.0, panel);
    batch.text(atlas, "PRISM  /  MODULES", x + 158.0, y + 20.0, muted, false);
    batch.text(atlas, "RSHIFT  /  ESC", x + 495.0, y + 20.0, muted, false);
    let category = state.category;
    for col in 0..2 {
        let px = x + 148.0 + col as f32 * 223.0;
        rounded(&mut batch, atlas, px, y + 57.0, 211.0, 244.0, 7.0, panel);
        batch.fill(atlas, px, y + 88.0, 211.0, 1.0, border);
        batch.text(atlas, if col == 0 { CATEGORIES[category] } else { "Other" }, px + 10.0, y + 68.0, accent, false);
        for row in 0..3 {
            let idx = col * 3 + row;
            let ry = y + 99.0 + row as f32 * 62.0;
            let target = if state.enabled[category][idx] { 1.0 } else { 0.0 };
            state.animation[category][idx] += (target - state.animation[category][idx]) * 0.2;
            let anim = state.animation[category][idx];
            batch.text(atlas, MODULES[category][idx], px + 10.0, ry + 5.0, rgba(212, 214, 220, 255), false);
            let bind = state.binds[category][idx].as_deref().unwrap_or("NONE");
            batch.text(atlas, &format!("BIND: {bind}"), px + 10.0, ry + 19.0, muted, false);
            batch.text(atlas, "...", px + 157.0, ry + 5.0, muted, false);
            rounded(&mut batch, atlas, px + 176.0, ry + 4.0, 25.0, 12.0, 6.0, if target > 0.0 { accent } else { border });
            rounded(&mut batch, atlas, px + 178.0 + anim * 12.0, ry + 6.0, 9.0, 8.0, 4.0, rgba(205, 211, 224, 255));
            batch.fill(atlas, px + 10.0, ry + 43.0, 191.0, 1.0, border);
        }
    }
    batch.text(atlas, "VISUAL PREVIEW  /  MODULES HAVE NO GAMEPLAY EFFECT", x + 148.0, y + 322.0, muted, false);
    if let Some((category, module)) = state.selected {
        rounded(&mut batch, atlas, x + 245.0, y + 67.0, 230.0, 200.0, 8.0, rgba(26, 28, 35, 255));
        batch.text(atlas, MODULES[category][module], x + 260.0, y + 81.0, accent, false);
        batch.text(atlas, "MODULE SETTINGS / PREVIEW ONLY", x + 260.0, y + 98.0, muted, false);
        rounded(&mut batch, atlas, x + 270.0, y + 120.0, 180.0, 22.0, 5.0, rgba(47, 51, 62, 255));
        let bind = if state.waiting_bind { "PRESS A KEY (ESC CANCEL)" } else { state.binds[category][module].as_deref().unwrap_or("NONE") };
        batch.text(atlas, &format!("BIND: {bind}"), x + 278.0, y + 127.0, rgba(225, 228, 236, 255), false);
        batch.text(atlas, "BACKSPACE / DELETE CLEARS BIND", x + 270.0, y + 149.0, muted, false);
        rounded(&mut batch, atlas, x + 270.0, y + 167.0, 180.0, 22.0, 5.0, rgba(47, 51, 62, 255));
        let value = state.values[category][module];
        batch.fill(atlas, x + 270.0, y + 185.0, 180.0 * f32::from(value) / 100.0, 3.0, accent);
        batch.text(atlas, &format!("PREVIEW VALUE: {value}%"), x + 278.0, y + 174.0, rgba(225, 228, 236, 255), false);
        batch.text(atlas, "NO GAMEPLAY EFFECT", x + 270.0, y + 200.0, muted, false);
        rounded(&mut batch, atlas, x + 400.0, y + 220.0, 50.0, 24.0, 5.0, rgba(47, 51, 62, 255));
        batch.text(atlas, "DONE", x + 411.0, y + 228.0, accent, false);
    }
    batch.scale_since(start, x, y, s, s);
    batch
}