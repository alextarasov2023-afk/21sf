# Validation

Checked on 2026-11-01 against the pinned Horizon `ba6344a` and
horizon-mixins `d9c83d7` revisions.

| Check | Result |
|---|---|
| `cargo fmt --all --check` and rustfmt check of the modified GUI files | Pass |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Pass |
| `cargo test --workspace --locked --offline` | 7 runtime tests pass |
| Mixin plan/apply and staged `cargo check` of the client | Pass on WSL Rust 1.95.0 |
| Staged `cargo test -p horizon-client --lib prism_ui` | 11 layout tests pass |
| Staged client binary runtime tests | 7 tests pass |
| `python tools/package.py` | 28,788 bytes, SHA-256 `f189f3314ef106da2411b44f9237ee3973074f795fc052cbba6655ec48c9329e` |
| Launcher `native-build --instance native-mods` | Pass on Windows Rust 1.99.1; build `native-ba6344a-0213ffecd6e7` installed |

The staged layout tests used the same geometry and input changes; a subsequent
one-line sidebar-highlight correction was included in the Windows native build.
The installed package hash matched the project package. The stage was built on
the Linux filesystem because staging under `/mnt/c` can fail with
`E_ATOMIC_STAGE`.

`tools/test_template.py` and `tools/test_stage.py` are template-shaped: they
expect exactly one injection and one module and a tiny fixture without
`Game::hud`/`window_event`, so they stop with `E_TARGET_NOT_FOUND` on this
bundle. The real pinned-source plan/apply/check above replaces those fixtures.

The panel is 950x520 with a logo-only sidebar, a 3x2 grid, inline module
settings, and Binds/Config/Themes inside the same centred window. The square
profile face uses a downloaded remote-player skin when available and falls back
to Steve in singleplayer or before a skin is available. A successful build and
layout tests do not by themselves establish visual appearance or interactive
behaviour in the game; those require a manual check.
