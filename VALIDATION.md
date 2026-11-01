# Validation

Checked on 2026-11-01 against the pinned Horizon `ba6344a` and
horizon-mixins `d9c83d7` revisions.

| Check | Result |
|---|---|
| rustfmt of every modified file | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --offline` (standalone runtime library) | 16 tests pass |
| Mixin plan/apply + staged `cargo check --bin horizon-client` | Pass on WSL (stage 16), 19s incremental |
| Staged `cargo test -p horizon-client --lib prism_ui` | 13 layout tests pass |
| Staged `cargo test -p horizon-client --bin horizon-client` | 16 module tests pass |
| `python tools/package.py` | 37,764 bytes, SHA-256 `2b41adcaee71b3e6c11dbf1ff06c39c4e1397011e68666078f31da0d7ecccb03` |
| Installed `example_mod.hnmod` MD5 | `6bae5557efcb058aa8e528f0bc6bf8dc`, matches the package |
| Launcher `native-build --instance native-mods` | Pass; build `native-ba6344a-bc998d57569e` in 731s |
| Launch `play --instance native-mods --world "New World"` | Started (PID 7624), first frame presented |

The stage was built on the Linux filesystem because staging under `/mnt/c` can
fail with `E_ATOMIC_STAGE`. The previously installed package was backed up to
`C:\Users\1\Desktop\yaya-mod-backup\example_mod.hnmod.prev` before being
replaced.

`tools/test_template.py` and `tools/test_stage.py` are template-shaped: they
expect exactly one injection and one module and a tiny fixture without
`Game::hud`/`window_event`, so they stop with `E_TARGET_NOT_FOUND` on this
bundle. The real pinned-source plan/apply/check above replaces those fixtures.

## What the panel is

900×520, centred, with a sidebar holding only the recoloured logo, the five
categories and the three manager entries, and a 3×2 module grid that pages six
modules at a time through the twenty each category lists. Clicking a chip opens
a narrow 216×176 drop-down column anchored directly under it (switch, MODE, KEY
BIND and VALUE stacked). Icons and the logo are the bundled raster PNGs in
`icons.rs`, uploaded once into the glyph atlas; the logo uses the current theme
accent and the keyboard glyph is drawn as vectors.

## What actually changes gameplay

The catalog is 100 modules — the list a normal client shows — and each chip is
marked for what it does: a filled corner dot and the `WORKING` legend for the
eleven wired into Horizon's hooks, a hollow dot and `NOT WIRED` for the rest.
`is_supported` in `runtime.rs` is the single source of that label, and the two
engine helpers only act on those same names, so a chip cannot claim more than
the code delivers.

The wired behaviours read the panel state through `enabled_behaviors()` and run
in `mods_tick.rs` (head of `Game::native_tick`, called at `game.rs:5514` before
that tick's movement) and `mods_hud.rs` (head of `Game::native_hud`, called at
`game/extras.rs:252`): Attack Aura (`attack_mob` on the nearest living mob in a
slider-set reach), Trigger Bot (`mob_hit`), Aim Assist (writes `self.yaw`/
`self.pitch` using vanilla's `lookAt` angles), Sprint (holds `ControlLeft`
logically while a movement key is down) and the readouts Coordinates, Direction,
Clock, FPS, CPS, Keystrokes and Session Stats.

The remaining 89 are not switch bugs, they are API limits, verified against the
pinned source:

- Speed, Step, No Slow and friends: `motion.rs:989` rewrites `player.speed` from
  the `minecraft:movement_speed` attribute every tick and `motion.rs:412` clamps
  velocity to `speed * 2.5`, so a tick that sets either is undone in the same
  tick.
- Fly: `motion.rs:897` forces `flying = false` unless `may_fly`, which only the
  server grants.
- Full Bright: `game.rs:12673` recomputes `night_vision` from the light setting
  per frame; there is no field to hold a value.
- ESP, Chams, Tracers, X-ray, mob Name Tags, Freecam view: `native_hud` receives
  a 2-D `HudBatch` and corner stack only. The pinned API exposes no 3-D
  world-render hook, so nothing can be drawn or tinted in the world.
- No Fall, Auto Totem, Fast Break and the rest of Player: no hook reaches the
  damage, offhand or block-breaking paths from `native_tick`.

## Not verified by these checks

A passing build and layout tests do not establish how the panel looks or that a
behaviour feels right in play. Live confirmation still needs a manual session in
the game: that Attack Aura/Trigger Bot really land damage on a mob, that Aim
Assist turns without fighting the mouse, that Sprint kicks in while walking, that
the readouts and the keystrokes caps draw at the right size on every GUI scale,
and that the paged grid, `WORKING`/`NOT WIRED` dots and the footer stepper render
as intended. The Clock readout shows UTC by design — the mod API exposes no
timezone.
