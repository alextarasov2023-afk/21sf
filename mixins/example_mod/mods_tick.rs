// Inlined at the head of `Game::native_tick`, which the window's tick calls
// just before this tick's movement simulation. Reads the modules the panel has
// switched on and mutates `Game` state so they actually do something. Only the
// window (never a server loop) and only while playing drive a module.
fn mod_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::example_mod as panel;
    let live = game.native_window() && game.native_playing() && game.player.is_some();
    // The Session Stats readout counts the time spent inside a world, so the
    // tick that leaves one closes the session.
    panel::note_session(live);
    if !live {
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

    // Sprint: keep Left Ctrl logically held whenever the player pushes a
    // movement key, so the sim's `held(ControlLeft)` branch sprints every tick
    // instead of needing the double-tap or Ctrl held.
    if on("Sprint")
        && (game.keys.contains(&KeyCode::KeyW)
            || game.keys.contains(&KeyCode::KeyA)
            || game.keys.contains(&KeyCode::KeyD)
            || game.keys.contains(&KeyCode::KeyS))
    {
        game.keys.insert(KeyCode::ControlLeft);
    }

    // Attack Aura (kill aura): hit the nearest living mob inside the reach the
    // slider sets. `attack_mob` runs the real melee path (damage, knockback,
    // sweep, sound), so the target actually takes the hit.
    if let Some((_, value, _)) = module("Attack Aura") {
        let range = panel::aura_range(value);
        let target: Option<usize> = {
            let mut nearest: Option<(usize, f64)> = None;
            if let Some(eye) = game.eye_position() {
                for (index, mob) in game.mobs.iter().enumerate() {
                    if mob.health <= 0.0 {
                        continue;
                    }
                    let distance = ((mob.position.x - eye[0]).powi(2)
                        + (mob.position.y - eye[1]).powi(2)
                        + (mob.position.z - eye[2]).powi(2))
                    .sqrt();
                    if distance <= range && nearest.is_none_or(|(_, best)| distance < best) {
                        nearest = Some((index, distance));
                    }
                }
            }
            nearest.map(|(index, _)| index)
        };
        if let Some(index) = target {
            game.attack_mob(index);
            game.swing_arm();
        }
    }

    // Aim Assist: turn the player's own look onto the nearest living mob within
    // Horizon's melee reach. The slider is how much of the gap one tick closes,
    // and `Smooth`/`Instant` ease or snap that follow.
    if let Some((_, value, mode)) = module("Aim Assist") {
        let strength = panel::aim_strength(value, mode);
        let range = game.reach();
        let target: Option<[f64; 3]> = {
            let mut nearest: Option<(usize, f64)> = None;
            if let Some(eye) = game.eye_position() {
                for (index, mob) in game.mobs.iter().enumerate() {
                    if mob.health <= 0.0 {
                        continue;
                    }
                    let (dx, dy, dz) = (
                        mob.position.x - eye[0],
                        mob.position.y + mob.kind.eye_height - eye[1],
                        mob.position.z - eye[2],
                    );
                    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                    if distance <= range && nearest.is_none_or(|(_, best)| distance < best) {
                        nearest = Some((index, distance));
                    }
                }
            }
            nearest.map(|(index, _)| {
                let mob = &game.mobs[index];
                [
                    mob.position.x,
                    mob.position.y + mob.kind.eye_height,
                    mob.position.z,
                ]
            })
        };
        if let (Some(point), Some(eye)) = (target, game.eye_position()) {
            let (want_yaw, want_pitch) = panel::look_rotation(eye, point);
            game.yaw =
                panel::wrap_degrees(game.yaw + panel::turn_delta(game.yaw, want_yaw) * strength);
            game.pitch = (game.pitch + panel::turn_delta(game.pitch, want_pitch) * strength)
                .clamp(-90.0, 90.0);
        }
    }

    // Trigger Bot: attack whatever the crosshair is already on, every tick, so
    // the player only has to aim. `mob_hit` is Horizon's own targeting result.
    if on("Trigger Bot")
        && let Some((index, _)) = game.mob_hit()
    {
        game.attack_mob(index);
        game.swing_arm();
    }
}
