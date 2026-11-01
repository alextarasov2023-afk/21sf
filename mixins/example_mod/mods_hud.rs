// Inlined at the head of `Game::native_hud`, which Horizon calls every frame
// for its own HUD extras (and skips under F1). Every Visuals readout the panel
// has switched on becomes one line in the top-left stack, drawn with the same
// primitives Horizon's extras use, so it never fights the debug screen. The
// Keystrokes module adds its own cap cluster in the bottom-left corner.
fn mod_hud(game: &mut crate::game::Game<'_>, batch: &mut HudBatch, corners: &mut Corners) {
    use crate::game::example_mod as panel;
    if !(game.native_window() && game.native_playing()) {
        return;
    }
    let behaviors = panel::enabled_behaviors();
    let module = |name: &str| {
        behaviors
            .iter()
            .find(|(module, _, _)| *module == name)
            .copied()
    };
    let on = |name: &str| module(name).is_some();

    let mut lines: Vec<String> = Vec::new();
    if [
        "Coordinates",
        "Direction",
        "Clock",
        "FPS",
        "CPS",
        "Session Stats",
    ]
    .iter()
    .any(|name| on(name))
    {
        let player = game.native_player();
        let frame_rate = game.native_frame_rate();
        let clicks = game.native_clicks();
        if let Some((_, _, mode)) = module("Coordinates")
            && let Some(player) = &player
        {
            lines.push(if mode == 0 {
                format!(
                    "XYZ {}, {}, {}",
                    player.x as i32, player.y as i32, player.z as i32
                )
            } else {
                format!("{:.2}, {:.2}, {:.2}", player.x, player.y, player.z)
            });
        }
        if let Some((_, _, mode)) = module("Direction")
            && let Some(player) = &player
        {
            let facing = panel::facing_label(player.yaw);
            lines.push(if mode == 0 {
                format!("Facing {facing}")
            } else {
                format!(
                    "{facing} | {:.0} | {}",
                    player.yaw,
                    player.dimension.trim_start_matches("minecraft:")
                )
            });
        }
        if on("Clock") {
            lines.push(panel::utc_clock());
        }
        if let Some((_, _, mode)) = module("FPS") {
            lines.push(match mode {
                0 => format!("{} fps", frame_rate.0),
                1 => format!("{} fps | 1% low {}", frame_rate.0, frame_rate.2),
                _ => format!(
                    "{} fps | slowest {} | 1% low {}",
                    frame_rate.0, frame_rate.1, frame_rate.2
                ),
            });
        }
        if let Some((_, _, mode)) = module("CPS") {
            lines.push(match mode {
                0 => format!("{} cps", clicks.0),
                1 => format!("{} cps (use)", clicks.1),
                _ => format!("{} cps | {} use", clicks.0, clicks.1),
            });
        }
        if on("Session Stats") {
            let seconds = panel::session_seconds();
            lines.push(format!("Session {}:{:02}", seconds / 60, seconds % 60));
        }
    }

    if !lines.is_empty() {
        let height = 12.0 * lines.len() as f32 + 8.0;
        let width = lines
            .iter()
            .map(|line| game.native_text_width(line))
            .fold(96.0_f32, f32::max)
            + 12.0;
        let (x, y) = corners.place(Corner::TopLeft, width, height);
        game.native_fill(batch, [x, y, width, height], 0xA000_0000);
        for (index, line) in lines.iter().enumerate() {
            game.native_text(
                batch,
                line,
                [x + 6.0, y + 5.0 + index as f32 * 12.0],
                0x00_FFAA,
                true,
            );
        }
    }

    if on("Keystrokes") {
        // The caps follow the player's own keybinds: `native_mapping` reports
        // both whether a vanilla action is held and the key that drives it.
        const CAP: f32 = 20.0;
        const GAP: f32 = 2.0;
        const COLUMNS: f32 = 4.0;
        // (mapping, column, row, width in caps); the space bar spans three.
        let layout = [
            ("key.forward", 1.0, 0.0, 1.0),
            ("key.sneak", 2.0, 0.0, 1.0),
            ("key.sprint", 3.0, 0.0, 1.0),
            ("key.left", 0.0, 1.0, 1.0),
            ("key.back", 1.0, 1.0, 1.0),
            ("key.right", 2.0, 1.0, 1.0),
            ("key.jump", 0.0, 2.0, 3.0),
        ];
        let (x, y) = corners.place(
            Corner::BottomLeft,
            COLUMNS * CAP + (COLUMNS - 1.0) * GAP,
            3.0 * CAP + 2.0 * GAP,
        );
        for (name, column, row, span) in layout {
            let Some((held, label)) = game.native_mapping(name) else {
                continue;
            };
            let width = span * CAP + (span - 1.0) * GAP;
            let (at_x, at_y) = (x + column * (CAP + GAP), y + row * (CAP + GAP));
            game.native_fill(
                batch,
                [at_x, at_y, width, CAP],
                if held { 0xE01E5C40 } else { 0x9000_0000 },
            );
            let text: String = label.chars().take(2).collect();
            let text_width = game.native_text_width(&text);
            game.native_text(
                batch,
                &text,
                [at_x + (width - text_width) / 2.0, at_y + 6.0],
                if held { 0xFFFF_66 } else { 0xCCCC_CC },
                true,
            );
        }
    }
}
