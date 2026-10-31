# Validation

Checked on 2026-10-31 with Rust 1.99.1 in WSL against the exact revisions in
`compatibility.json` (Horizon `ba6344a`, horizon-mixins `d9c83d7`) and the bundle
committed here.

| Check | Result |
|---|---|
| `rustfmt --edition 2024 --check` on `prism_ui.rs`, `icons.rs`, `gui_render.rs`, `gui_input.rs`, `break.rs` | Pass |
| `cargo fmt --all --check` | Pass |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Pass |
| `cargo test --workspace --locked --offline` | 7 runtime unit tests pass |
| `horizon-mixins apply` on the pristine pinned export | 3 injections + 3 modules staged, no errors |
| `cargo check -p horizon-client --bin horizon-client --no-default-features` in the stage | Pass, 1m47s with a cached target dir |
| `python3 tools/package.py` | Writes `target/example_mod-0.1.0.hnmod`, 26,014 bytes, sha256 `c99cffad…084010` |

The staged build is the real gate: it injects `runtime.rs` into
`crate::game::example_mod`, and `prism_ui.rs` plus `icons.rs` into
`horizon_client`'s library crate, then compiles the actual client binary. The
stage was produced on the Linux filesystem (`~/prism-stage7`) from the pristine
export; `tools/horizon.py` cannot stage on `/mnt/c` (`E_ATOMIC_STAGE`), and
pointing `--workspace` at the live checkout fails `E_INVALID_PATH`.

`tools/test_template.py` and `tools/test_stage.py` are template-shaped: they
expect exactly one injection and one module and a tiny fixture without
`Game::hud`/`window_event`, so they stop with `E_TARGET_NOT_FOUND` on this
bundle. That is a limitation of those fixtures, not a staging failure; the
plan/apply/check run above replaced them for this mod.

The package was installed into the `native-mods` instance, rebuilt with
`native-build` (485 s, `native-ba6344a-2517c193e775`) and launched into
`New World`. The game window opening does not establish that the panel looks or
behaves as intended; that remains a manual check. The HUD drawing uses only the
public `HudBatch`/`HudAtlas` data and the crate-private `ui_theme::Ui` widgets;
icon bitmaps are uploaded into the unused bottom strip of the `horizon/ui-text`
atlas region the first time the atlas pointer changes.

CI reproduces formatting, clippy and the standalone runtime tests. The full
native checks additionally need a Horizon Git history containing the pin and its
cached dependencies; use the commands in README.md to reproduce them.
