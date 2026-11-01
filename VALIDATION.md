# Validation

Checked on 2026-11-01 against the pinned Horizon `ba6344a` and
horizon-mixins `d9c83d7` revisions.

| Check | Result |
|---|---|
| rustfmt of the modified GUI files | Pass |
| Mixin apply and staged `cargo check` of the client (stage 14) | Pass on WSL; finished in 1m43s |
| Staged `cargo test -p horizon-client --lib prism_ui` | 11 layout tests pass |
| Staged `cargo test -p horizon-client --bin horizon-client` | 213 client tests pass |
| `python tools/package.py` | 27,954 bytes, SHA-256 `de585025defadfdb072bfb1dac057da330c7f3a6e2b3479cb6b95a4d19f7284d` |
| Installed `example_mod.hnmod` MD5 | `e009e27324c43203eebf69cf89e5bf76`, matches the package |
| Launcher `native-build --instance native-mods` | Pass; build `native-ba6344a-eb1580e9caa1` in 544s |
| Launch `play --instance native-mods --world "New World"` | Started (PID 25260) |

The stage was built on the Linux filesystem because staging under `/mnt/c` can
fail with `E_ATOMIC_STAGE`. The previous installed package was backed up to
`C:\Users\1\Desktop\yaya-mod-backup\example_mod.hnmod.prev` before replacing it.

`tools/test_template.py` and `tools/test_stage.py` are template-shaped: they
expect exactly one injection and one module and a tiny fixture without
`Game::hud`/`window_event`, so they stop with `E_TARGET_NOT_FOUND` on this
bundle. The real pinned-source plan/apply/check above replaces those fixtures.

The panel is 900x520 with a sidebar holding only the recoloured logo, the five
categories and the three manager entries, and a 3x2 module grid. The player
face, nick and profile code are gone. Clicking a chip opens a narrow 216x176
drop-down column anchored directly under that chip (switch, MODE, KEY BIND and
VALUE stacked), so settings are no longer a half-panel block and there is less
dead space. Icons and the logo are the bundled raster PNGs in `icons.rs` again,
uploaded once into the glyph atlas; the logo uses the current theme accent and
the keyboard glyph is still drawn as vectors. A successful build and layout
tests do not by themselves establish visual appearance or interactive behaviour
in the game; those require a manual check.
