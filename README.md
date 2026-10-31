# Horizon native mod template

## Experimental Yaya click GUI

The native bundle includes an experimental in-game panel styled after the
supplied dark launcher mockup. Text uses Horizon's bundled Geist Regular/Medium
faces. The logo and the module icons are the supplied raster assets (P-logo,
sword, running, camera, person, cog, open folder, three dots, magnifier),
embedded as base64 PNGs in `icons.rs` and generated from `tools/icons/` by
`tools/gen_icons.mjs`; the keyboard glyph is still drawn from vector shapes.
Every module row reuses the icon of its own category, so Combat rows show the
sword, Movement the runner and so on, and Config uses the open-folder asset.
They are decoded once and uploaded into the HUD glyph atlas, so the package
stays source-only.

The panel is centred on a 1280×640 design surface: a sidebar with the PRISM
logo and wordmark, FEATURES (Combat, Movement, Visuals, Player, Misc), MANAGER
(Binds, Config) and a player row, plus a content area with a search field and
two columns of three module cards. Rows and windows are sized to their text so
the panel stays compact. When a settings, binds or config window opens, the
panel slides left so the panel and window are centred together. The panel fades
and lifts in when opened, side windows slide in, and switches/slider animate
between states.

While in a world, press **Right Shift** to open the panel and **Escape** or
**Right Shift** to close it. Click a category to change the listed modules;
click a module's switch to toggle it; click anywhere else on a module row to
open its settings in the right-hand window (ENABLED switch, KEY BIND field,
VALUE slider, DONE). The VALUE slider follows the cursor while the mouse button
is held, so it can be dragged smoothly. Clicking the keyboard icon or the key box starts a key
capture; Escape cancels it, Backspace/Delete clears the bind. A bound key
toggles the placeholder while playing, without consuming the game's own key
action. Right Shift and Escape are reserved.

The search field filters every category live by module name (case-insensitive)
and shows RESULTS instead of a category title; click the field or press a
character to type, Escape unfocuses or clears it, and the X clears it. MANAGER
holds Binds and Config: Binds lists the assigned keys in grid order and jumps to
the module you click, with CLEAR ALL BINDS; Config lists the saved presets and
offers NEW, SAVE, LOAD and DELETE with a status line.

Settings persist as dependency-free text files under `config/prism/`
(`# prism config v1`, `category=N`, `enabled=c,m`, `bind=c,m,key`,
`value=c,m,v`). `session.cfg` is written when the panel closes and restored on
open; named configs are only touched by the Config buttons. Module switches,
values and binds have no gameplay effect — binds only consume their own key
press. The panel is drawn by `prism_ui.rs`, a public support module injected
into `horizon_client`'s library crate: Horizon's design-system widgets are
crate-private there, while `game.rs` is compiled into the binary crate. The
game-side helpers only snapshot state and hit-test the shared rectangles. The
old `src/overlay.rs` scaffold is not part of this native bundle.

A minimal **horizon-mixins** mod template based on
[Horizon Veinminer](https://github.com/levzzz5154/horizon-veinminer).
It declares one HEAD hook that logs a block-break attempt, then lets Horizon's
original method run unchanged. Runtime logic lives in one Rust file that also
builds as a standalone testable Cargo library.

This is the native source-mixin path: it builds a custom Horizon executable.
For content mods (data only, `.hmod`), use Horizon's `cargo horizon-mod new`
instead. This template does not install into a running game or the launcher's
ordinary `.hmod` folder.

## Start a mod

Use GitHub's **Use this template → Create a new repository**, then clone your
new repository. Requirements: Linux, Rust 1.95.0, Python 3.12+, Git and Cargo.
The staging script uses Linux file locks; Windows/macOS are not validated.

```sh
cargo fetch --locked
cargo test --workspace --locked --offline
```

The exact compatible revisions are in [compatibility.json](compatibility.json):

- Horizon: `ba6344a59cb9e755b730dcf02285fc0d33b97d3c` (Nightly 2026-10-30).
- Horizon Mixins: `d9c83d7` (0.6.0, plan schema 6).

Clone the tools beside your mod if you do not already have suitable checkouts:

```sh
git clone https://github.com/horizon-rust/horizon ../horizon
git clone https://github.com/horizon-rust/horizon-mixins ../horizon-mixins
git -C ../horizon-mixins checkout "$(python3 -c 'import json; print(json.load(open("compatibility.json"))["horizon_mixins_revision"])')"
cargo fetch --locked --manifest-path ../horizon-mixins/Cargo.toml
```

Horizon can stay on its existing branch: staging exports the committed pin with
`git archive` and never checks out or edits the original source. Its Git history
must contain the pinned commit. A shallow clone may need a fetch first.
The Mixins checkout must be at the exact recorded revision with clean tracked
engine sources. For an existing development checkout at another revision, use
a separate clone/worktree and set `HORIZON_MIXINS=/path/to/pinned/horizon-mixins`.

## Layout and customization

```text
Cargo.toml                         Standalone runtime library + source audit workspace
compatibility.json                 Exact upstream/tool revisions and source hashes
mixins/example_mod/
  horizon-mixin.toml                Modules, hooks and compatibility pin
  break.rs                         Small typed hook adapter
  runtime.rs                       Panel state, binds and unit tests (standalone library)
  gui_render.rs                    Snapshots state and calls the panel renderer
  gui_input.rs                     Right Shift, keyboard, search and mouse handling
  prism_ui.rs                      Centred layout, Geist text, right-side settings window
  icons.rs                         Generated base64 logo/icon payloads and decoder
tools/gen_icons.mjs                Regenerates icons.rs from tools/icons/ sources
tools/icons/                       Supplied logo and icon assets (P-logo, SVG, PNG)
tools/horizon.py                   Export → plan → apply → check/build/test
tools/test_template.py             Tiny compiled fixture; no Horizon checkout/assets needed
tools/test_stage.py                Real pinned source audit and repeatability test
tools/audit/                      Rust AST comparison against the pristine export
```

To rename the example:

1. Change Cargo package name/description and the manifest's mod ID/version.
2. Rename `mixins/example_mod/` and update the Cargo `[lib].path`.
3. Change the manifest module name `example_mod` and its reference in `break.rs`.
4. Change the runtime log prefix, unit-test expectation and the native test filter
   `game::example_mod::tests` in `tools/horizon.py`.
5. Run `cargo generate-lockfile --offline` after renaming the Cargo package and
   run the checks below.

Add behavior to `runtime.rs`, then expose it through small typed helpers and
TOML declarations. No payload generation is necessary: Cargo and the injected
module read the same file. Module payloads cannot use inner file attributes
(including `//!` comments), external `mod` files or `include!`; use ordinary
comments and inline modules. Hooks compile in their target's crate/module, so
native-specific imports can be placed in the helper. To keep runtime tests
standalone, use ordinary Rust types or a small host interface there.

This example passes only `pos`. To access the native game, declare an explicit
helper binding for `&mut crate::game::Game<'_>` with value `&mut *self`, as
Veinminer does. Existing crate dependencies/privacy/borrowing remain enforced;
Cargo dependencies from this template are not automatically injected into Horizon.
See [Mixin contracts](https://github.com/horizon-rust/horizon-mixins/blob/main/README.md)
and [effects/state](https://github.com/horizon-rust/horizon-mixins/blob/main/docs/effects-and-state.md)
for cancellation, captures, value changes, bridges and state.

## Stage, compile and run

```sh
python3 tools/horizon.py stage --horizon ../horizon
```

This prints the generated stage path. It uses only committed pinned sources,
records original hashes, runs the real Mixins `plan`/`apply` CLI, and preserves
unchanged bytes/mtimes on repeated runs. The original checkout and player data
stay untouched. The plan is saved at `target/mixins/plan.json`.

If Horizon dependencies are not cached, fetch the staged lockfile once:

```sh
cargo fetch --locked --manifest-path target/mixins/modded/Cargo.toml
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py test --horizon ../horizon
python3 tools/horizon.py build --horizon ../horizon
```

`check` checks the actual client binary. `test` compiles it and runs the embedded
runtime test module. `build` uses Horizon's `fast` profile and
`--no-default-features` (audio/controllers disabled), and prints its binary path.
These commands use offline, locked builds; raw logs are in `target/horizon-*.log`.
`--target-dir PATH` selects a reusable external Cargo target directory.

Launch the built client with your own **prepared resources for the pinned
Horizon version**, and explicitly pass their path:

```sh
./target/horizon-cargo/fast/horizon-client --resources /absolute/path/to/prepared/resources
```

Prepare those resources using the compatible Horizon launcher or
`horizon-prepare --prepare`; game resources are not supplied by this repository.
Use the binary's `--help` for world/server arguments. Breaking a block should
print `[example_mod] Breaking block at ...` to the host's stderr. The HEAD log
reports attempts, including attempts the original game later rejects.
Compilation tests do not establish interactive gameplay/resource compatibility.

## Pin to a Nightly, package, publish

A native mod builds on one exact Horizon commit, and every mod of a player's build must pin the same one, so
pin a **Nightly**: its release notes name the commit, and the tag `nightly-source/<date>-<short>` keeps that
commit's source downloadable after the Nightly is replaced. This template pins Nightly 2026-10-30
(`ba6344a`). The launcher also builds every mod of a build with one horizon-mixins revision, the one
they all name (`horizon_mixins_revision`), so keep it the same as the other mods your players combine yours with.
To move to a newer Nightly:

1. Put its commit in `compatibility.json` (`horizon_revision`) and in the manifest's
   `[compatibility] horizon_revision`.
2. Update `source_sha256` for each target file (`git -C ../horizon show <commit>:<path> | sha256sum`).
3. Run the checks below. When a target moved (the game is being split into crates), `plan` names the
   missing function: find its new path in that commit and fix `target` in the manifest.

Then `python3 tools/package.py` writes `target/<id>-<version>.hnmod`: `horizon-native.json` plus your
`mixins/<id>/` sources, byte-for-byte reproducible. Submit it to
[horizon-mods](https://github.com/horizon-rust/horizon-mods) as a `native-mod`. A maintainer reads the
source before it is listed, and the Horizon launcher compiles it with its own Rust on the pinned commit, in
an instance of its own. The mod id must be lowercase letters, digits and `_` (the registry's rule), and
the bundle folder is named after it.

## Validation

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
python3 tools/test_template.py
# With a Horizon Git clone containing the pin:
python3 tools/test_stage.py --horizon ../horizon
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py test --horizon ../horizon
```

`tools/test_template.py` and `tools/test_stage.py` still assert the template's
original shape (exactly one injection and one module, and a tiny fixture without
`Game::hud`/`window_event`), so they stop with `E_TARGET_NOT_FOUND` on this
expanded bundle. Use the pinned plan/apply path instead:
`horizon-mixins apply` on the pristine export, then
`cargo check -p horizon-client --bin horizon-client --no-default-features` in the
stage; `tools/horizon.py` fails to stage on `/mnt/c` with `E_ATOMIC_STAGE`, so run
the CLI with `--out` on the Linux filesystem. The staging tools still reject
wrong revisions and missing targets before replacing a stage. See
[VALIDATION.md](VALIDATION.md).

Build/staging tooling is adapted from the MIT-licensed Veinminer port. This
repository includes no Veinminer gameplay, Minecraft assets, game binaries or
exported Horizon sources. License: [MIT](LICENSE).
