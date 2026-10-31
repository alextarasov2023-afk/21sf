// The PRISM panel's drawing and geometry, injected as a public support
// module of `horizon_client`. It lives beside `ui_theme` because the game
// module (`crates/horizon-client/src/game.rs`) is compiled into the binary
// crate, while the launcher's `Ui` widgets are crate-private to the library.
//
// Everything here is authored on a 1280x640 design surface. The main panel is
// always centred; when a manager window opens, the panel + window pair stays
// centred as one group. The game module snapshots its state into `View` and
// calls `render`; input hit-tests the same public `Rect`s, so drawing and
// clicking cannot disagree. Icons and the logo are the bundled PNGs in
// `example_mod_icons`, uploaded once into the bottom strip of the glyph atlas.
use std::cell::RefCell;

use crate::example_mod_icons;
use crate::hud::{HudAtlas, HudBatch, rgba};
use crate::ui_theme::{Face, Ui};

/// The design surface the panel is authored on, in design units. The renderer
/// scales this box to the current window.
pub const DESIGN_W: f32 = 1280.0;
pub const DESIGN_H: f32 = 640.0;

pub const PANEL_W: f32 = 804.0;
pub const PANEL_H: f32 = 392.0;
pub const PANEL_Y: f32 = (DESIGN_H - PANEL_H) / 2.0;
pub const SIDE_W: f32 = 272.0;
pub const SIDE_GAP: f32 = 14.0;
pub const SIDEBAR_W: f32 = 176.0;

pub const CONTENT_X: f32 = 196.0;
pub const CONTENT_W: f32 = 592.0;
pub const COLUMN_W: f32 = 290.0;
pub const COLUMN_STEP: f32 = 302.0;
pub const ROW_H: f32 = 56.0;
pub const ROW_STEP: f32 = 62.0;
pub const ROW_TOP: f32 = 96.0;

/// Mirrors `example_mod::SIDE_*` in the game module: 0 closed, then module
/// settings, binds and configs.
pub const SIDE_NONE: usize = 0;
pub const SIDE_MODULE: usize = 1;
pub const SIDE_BINDS: usize = 2;
pub const SIDE_CONFIG: usize = 3;

/// The bundled icon for a category, reused by its rows and its modules.
pub fn category_icon(category: usize) -> &'static str {
    ["sword", "running", "camera", "person", "dots"]
        .get(category)
        .copied()
        .unwrap_or("dots")
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }

    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
}

/// The main panel, centred on its own or together with the right window.
pub fn main_rect(side: usize) -> Rect {
    let width = if side == SIDE_NONE {
        PANEL_W
    } else {
        PANEL_W + SIDE_GAP + SIDE_W
    };
    Rect {
        x: (DESIGN_W - width) / 2.0,
        y: PANEL_Y,
        w: PANEL_W,
        h: PANEL_H,
    }
}

pub fn side_rect(side: usize) -> Rect {
    let main = main_rect(side);
    Rect {
        x: main.x + PANEL_W + SIDE_GAP,
        y: PANEL_Y,
        w: SIDE_W,
        h: PANEL_H,
    }
}

pub fn search_rect(side: usize) -> Rect {
    let main = main_rect(side);
    Rect {
        x: main.x + CONTENT_X,
        y: main.y + 22.0,
        w: CONTENT_W,
        h: 34.0,
    }
}

pub fn search_clear(side: usize) -> Rect {
    let search = search_rect(side);
    Rect {
        x: search.x + search.w - 30.0,
        y: search.y + 5.0,
        w: 24.0,
        h: 24.0,
    }
}

pub fn column_x(side: usize, column: usize) -> f32 {
    main_rect(side).x + CONTENT_X + column as f32 * COLUMN_STEP
}

pub fn card_rect(side: usize, column: usize) -> Rect {
    let main = main_rect(side);
    Rect {
        x: main.x + CONTENT_X + column as f32 * COLUMN_STEP,
        y: main.y + 88.0,
        w: COLUMN_W,
        h: 196.0,
    }
}

pub fn module_row(side: usize, column: usize, row: usize) -> Rect {
    Rect {
        x: column_x(side, column),
        y: main_rect(side).y + ROW_TOP + row as f32 * ROW_STEP,
        w: COLUMN_W,
        h: ROW_H,
    }
}

pub fn toggle_row(side: usize, column: usize, row: usize) -> Rect {
    let module = module_row(side, column, row);
    Rect {
        x: module.x + COLUMN_W - 56.0,
        y: module.y + (ROW_H - 24.0) / 2.0,
        w: 44.0,
        h: 24.0,
    }
}

pub fn dots_hit(side: usize, column: usize, row: usize) -> Rect {
    let module = module_row(side, column, row);
    Rect {
        x: module.x + COLUMN_W - 138.0,
        y: module.y + (ROW_H - 28.0) / 2.0,
        w: 28.0,
        h: 28.0,
    }
}

pub fn keyboard_hit(side: usize, column: usize, row: usize) -> Rect {
    let module = module_row(side, column, row);
    Rect {
        x: module.x + COLUMN_W - 100.0,
        y: module.y + (ROW_H - 28.0) / 2.0,
        w: 28.0,
        h: 28.0,
    }
}

pub fn category_row(side: usize, index: usize) -> Rect {
    let main = main_rect(side);
    Rect {
        x: main.x + 12.0,
        y: main.y + 74.0 + index as f32 * 34.0,
        w: SIDEBAR_W - 24.0,
        h: 30.0,
    }
}

pub fn manager_row(side: usize, index: usize) -> Rect {
    let main = main_rect(side);
    Rect {
        x: main.x + 12.0,
        y: main.y + 260.0 + index as f32 * 34.0,
        w: SIDEBAR_W - 24.0,
        h: 30.0,
    }
}

pub fn side_close(side: usize) -> Rect {
    let frame = side_rect(side);
    Rect {
        x: frame.x + frame.w - 36.0,
        y: frame.y + 14.0,
        w: 22.0,
        h: 22.0,
    }
}

pub fn settings_toggle(side: usize) -> Rect {
    let frame = side_rect(side);
    Rect {
        x: frame.x + frame.w - 18.0 - 46.0,
        y: frame.y + 82.0,
        w: 46.0,
        h: 24.0,
    }
}

pub fn settings_bind(side: usize) -> Rect {
    let frame = side_rect(side);
    Rect {
        x: frame.x + 18.0,
        y: frame.y + 136.0,
        w: frame.w - 36.0,
        h: 40.0,
    }
}

pub fn settings_slider_track(side: usize) -> Rect {
    let frame = side_rect(side);
    Rect {
        x: frame.x + 18.0,
        y: frame.y + 232.0,
        w: frame.w - 36.0,
        h: 6.0,
    }
}

pub fn settings_slider_hit(side: usize) -> Rect {
    let frame = side_rect(side);
    Rect {
        x: frame.x + 12.0,
        y: frame.y + 216.0,
        w: frame.w - 24.0,
        h: 38.0,
    }
}

pub fn settings_done(side: usize) -> Rect {
    let frame = side_rect(side);
    Rect {
        x: frame.x + 18.0,
        y: frame.y + frame.h - 54.0,
        w: frame.w - 36.0,
        h: 38.0,
    }
}

pub fn binds_row(side: usize, index: usize) -> Rect {
    let frame = side_rect(side);
    Rect {
        x: frame.x + 18.0,
        y: frame.y + 82.0 + index as f32 * 40.0,
        w: frame.w - 36.0,
        h: 34.0,
    }
}

pub fn binds_clear(side: usize) -> Rect {
    let frame = side_rect(side);
    Rect {
        x: frame.x + 18.0,
        y: frame.y + frame.h - 54.0,
        w: frame.w - 36.0,
        h: 38.0,
    }
}

pub fn config_row(side: usize, index: usize) -> Rect {
    let frame = side_rect(side);
    Rect {
        x: frame.x + 18.0,
        y: frame.y + 94.0 + index as f32 * 31.0,
        w: frame.w - 36.0,
        h: 27.0,
    }
}

/// `which`: 0 new, 1 save, 2 load, 3 delete; a 2x2 block of buttons.
pub fn config_button(side: usize, which: usize) -> Rect {
    let frame = side_rect(side);
    let column = (which % 2) as f32;
    let row = (which / 2) as f32;
    Rect {
        x: frame.x + 18.0 + column * 118.0,
        y: frame.y + 266.0 + row * 42.0,
        w: 112.0,
        h: 36.0,
    }
}

/// Physical pixels per dp, matching `ui_theme::Ui` exactly.
pub fn density(batch_width: f32, batch_height: f32, gui_scale: f32) -> f32 {
    ((batch_width * gui_scale) / 1040.0)
        .min((batch_height * gui_scale) / 700.0)
        .clamp(1.0, 4.0)
}

/// Centering offset and scale of the design surface inside the dp screen.
pub fn layout(batch_width: f32, batch_height: f32, gui_scale: f32) -> (f32, f32, f32) {
    let density = density(batch_width, batch_height, gui_scale);
    let (width, height) = (
        batch_width * gui_scale / density,
        batch_height * gui_scale / density,
    );
    let scale = (width / DESIGN_W).min(height / DESIGN_H).min(1.0);
    (
        (width - DESIGN_W * scale) / 2.0,
        (height - DESIGN_H * scale) / 2.0,
        scale,
    )
}

/// A GUI-pixel cursor in design units.
pub fn to_design(
    cursor: (f32, f32),
    batch_width: f32,
    batch_height: f32,
    gui_scale: f32,
) -> (f32, f32) {
    let density = density(batch_width, batch_height, gui_scale);
    let (offset_x, offset_y, scale) = layout(batch_width, batch_height, gui_scale);
    (
        (cursor.0 * gui_scale / density - offset_x) / scale,
        (cursor.1 * gui_scale / density - offset_y) / scale,
    )
}

/// One frame of the game's preview state, borrowed from the injected game
/// module. The renderer steps [`View::animation`] toward the enabled flags.
#[allow(clippy::type_complexity)]
pub struct View<'a> {
    pub side: usize,
    pub category: usize,
    pub enabled: [[bool; 6]; 5],
    pub animation: [[f32; 6]; 5],
    pub binds: &'a [[Option<String>; 6]; 5],
    pub values: [[u8; 6]; 5],
    pub selected: Option<(usize, usize)>,
    pub waiting_bind: bool,
    pub query: &'a str,
    pub search_focus: bool,
    /// The modules currently shown in the 2x3 grid, in reading order.
    pub visible: &'a [(usize, usize)],
    /// Bound modules as (category, module, key), in grid order.
    pub bound: &'a [(usize, usize, String)],
    pub categories: &'a [&'a str],
    pub modules: &'a [[&'a str; 6]; 5],
    pub manager: &'a [&'a str; 2],
    pub configs: &'a [String],
    pub config_selected: Option<usize>,
    pub status: &'a str,
    /// Panel open fade/slide, 0..1; stepped by [`render`].
    pub open: &'a mut f32,
    /// Right-hand window slide-in, 0..1; stepped by [`render`].
    pub side_anim: &'a mut f32,
    /// True while the VALUE slider is held, so the cursor drives it.
    pub dragging: bool,
}

pub fn render(atlas: &HudAtlas, batch: &mut HudBatch, cursor: (f32, f32), view: &mut View<'_>) {
    let (ox, oy, s) = layout(batch.width, batch.height, batch.scale);
    let mut ui = Ui::new(atlas, batch, cursor);
    let cursor = ((ui.cursor.0 - ox) / s, (ui.cursor.1 - oy) / s);
    *view.open += (1.0 - *view.open) * 0.24;
    let open = view.open.clamp(0.0, 1.0);
    let side_target = if view.side == SIDE_NONE { 0.0 } else { 1.0 };
    *view.side_anim += (side_target - *view.side_anim) * 0.28;
    for category in 0..5 {
        for module in 0..6 {
            let target = f32::from(view.enabled[category][module]);
            let anim = &mut view.animation[category][module];
            *anim += (target - *anim) * 0.22;
        }
    }
    if view.dragging
        && view.side == SIDE_MODULE
        && let Some((category, module)) = view.selected
    {
        let track = settings_slider_track(view.side);
        let value = ((cursor.0 - track.x) / track.w * 100.0)
            .round()
            .clamp(0.0, 100.0);
        view.values[category][module] = value as u8;
    }
    // Dim the game behind the click GUI; the design box is centred in dp, so
    // the scrim covers the whole surface rather than only the panel.
    ui.rounded(
        0.0,
        0.0,
        ui.w,
        ui.h,
        0.0,
        rgba(4, 5, 8, (128.0 * open) as u8),
    );
    {
        let mut paint = Paint {
            ui: &mut ui,
            // The panel settles a little upward while it fades in.
            ox,
            oy: oy + (1.0 - open) * 18.0,
            s,
            cursor,
            side_anim: view.side_anim.clamp(0.0, 1.0),
            theme: Theme::new(),
        };
        paint.draw_main(view);
    }
    ui.finish();
}

/// Linearly blends two packed colours (`rgba`'s A<<24|B<<16|G<<8|R order).
fn mix(a: u32, b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let channel = |shift: u32| {
        let from = ((a >> shift) & 0xff) as f32;
        let to = ((b >> shift) & 0xff) as f32;
        (from + (to - from) * t).round() as u32
    };
    (channel(24) << 24) | (channel(16) << 16) | (channel(8) << 8) | channel(0)
}

#[derive(Clone, Copy)]
struct Theme {
    page: u32,
    window: u32,
    border: u32,
    divider: u32,
    field: u32,
    raised: u32,
    hover: u32,
    selected_bg: u32,
    selected_border: u32,
    ink: u32,
    strong: u32,
    muted: u32,
    faint: u32,
    icon: u32,
    accent: u32,
    accent_light: u32,
    accent_deep: u32,
    toggle_off: u32,
    toggle_border: u32,
    knob_off: u32,
    knob_on: u32,
}

impl Theme {
    fn new() -> Self {
        Self {
            page: rgba(9, 10, 14, 252),
            window: rgba(13, 15, 20, 252),
            border: rgba(30, 33, 40, 255),
            divider: rgba(25, 28, 34, 255),
            field: rgba(15, 17, 22, 255),
            raised: rgba(18, 20, 26, 255),
            hover: rgba(255, 255, 255, 7),
            selected_bg: rgba(19, 27, 41, 255),
            selected_border: rgba(47, 66, 102, 255),
            ink: rgba(205, 210, 219, 255),
            strong: rgba(235, 238, 245, 255),
            muted: rgba(124, 130, 142, 255),
            faint: rgba(86, 92, 104, 255),
            icon: rgba(151, 157, 169, 255),
            accent: rgba(112, 146, 219, 255),
            accent_light: rgba(126, 162, 236, 255),
            accent_deep: rgba(88, 121, 188, 255),
            toggle_off: rgba(37, 40, 47, 255),
            toggle_border: rgba(60, 65, 75, 255),
            knob_off: rgba(99, 105, 117, 255),
            knob_on: rgba(240, 243, 250, 255),
        }
    }
}

struct Paint<'a, 'b> {
    ui: &'a mut Ui<'b>,
    ox: f32,
    oy: f32,
    s: f32,
    cursor: (f32, f32),
    side_anim: f32,
    theme: Theme,
}

impl Paint<'_, '_> {
    fn has(&self, rect: Rect) -> bool {
        rect.contains(self.cursor.0, self.cursor.1)
    }

    fn fill(&mut self, x: f32, y: f32, w: f32, h: f32, color: u32) {
        self.rounded(x, y, w, h, 0.0, color);
    }

    fn rounded(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: u32) {
        self.ui.rounded(
            self.ox + x * self.s,
            self.oy + y * self.s,
            w * self.s,
            h * self.s,
            radius * self.s,
            color,
        );
    }

    fn border(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: u32) {
        self.ui.border(
            self.ox + x * self.s,
            self.oy + y * self.s,
            w * self.s,
            h * self.s,
            radius * self.s,
            color,
        );
    }

    fn text(&mut self, face: Face, size: f32, x: f32, y: f32, color: u32, value: &str) -> f32 {
        self.ui.text(
            face,
            size * self.s,
            self.ox + x * self.s,
            self.oy + y * self.s,
            color,
            value,
        )
    }

    fn text_centered(&mut self, face: Face, size: f32, cx: f32, y: f32, color: u32, value: &str) {
        self.ui.text_centered(
            face,
            size * self.s,
            self.ox + cx * self.s,
            self.oy + y * self.s,
            color,
            value,
        );
    }

    fn text_right(&mut self, face: Face, size: f32, right: f32, y: f32, color: u32, value: &str) {
        let width = self.ui.measure(face, size * self.s, value) / self.s;
        self.text(face, size, right - width, y, color, value);
    }

    /// Text whose line box is visually centred on `cy`.
    fn text_mid(&mut self, face: Face, size: f32, x: f32, cy: f32, color: u32, value: &str) {
        self.text(face, size, x, cy - size * 0.725 + 0.5, color, value);
    }

    fn text_spaced(
        &mut self,
        face: Face,
        size: f32,
        x: f32,
        y: f32,
        spacing: f32,
        color: u32,
        value: &str,
    ) {
        let mut pen = x;
        for ch in value.chars() {
            let mut buf = [0u8; 4];
            let glyph = ch.encode_utf8(&mut buf);
            let advance = self.ui.measure(face, size * self.s, glyph) / self.s;
            if !ch.is_whitespace() {
                self.text(face, size, pen, y, color, glyph);
            }
            pen += advance + spacing;
        }
    }

    fn icon(&mut self, name: &str, cx: f32, cy: f32, size: f32, color: u32) {
        let (ox, oy, s) = (self.ox, self.oy, self.s);
        let k = self.ui.density / self.ui.batch.scale;
        let atlas = self.ui.atlas;
        upload_icons(atlas, self.ui.batch);
        let rect = ICON_MAP.with(|cell| {
            cell.borrow().as_ref().and_then(|map| {
                map.icons
                    .iter()
                    .find(|(icon, _)| *icon == name)
                    .map(|(_, rect)| *rect)
            })
        });
        if let Some(rect) = rect {
            self.ui.batch.rect_uv(
                (ox + (cx - size / 2.0) * s) * k,
                (oy + (cy - size / 2.0) * s) * k,
                size * s * k,
                size * s * k,
                rect,
                color,
            );
        }
    }

    fn raw_quad(&mut self, corners: [[f32; 2]; 4], color: u32) {
        let k = self.ui.density / self.ui.batch.scale;
        let quad = corners.map(|[x, y]| [x * k, y * k]);
        let atlas = self.ui.atlas;
        self.ui.batch.fill_quad(atlas, quad, color);
    }

    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: u32) {
        let a = [self.ox + x1 * self.s, self.oy + y1 * self.s];
        let b = [self.ox + x2 * self.s, self.oy + y2 * self.s];
        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
        let length = (dx * dx + dy * dy).sqrt();
        if length < 0.001 {
            return;
        }
        let half = thickness * self.s / 2.0;
        let (nx, ny) = (-dy / length * half, dx / length * half);
        self.raw_quad(
            [
                [a[0] + nx, a[1] + ny],
                [b[0] + nx, b[1] + ny],
                [b[0] - nx, b[1] - ny],
                [a[0] - nx, a[1] - ny],
            ],
            color,
        );
    }

    fn disc(&mut self, cx: f32, cy: f32, radius: f32, color: u32) {
        let center = [self.ox + cx * self.s, self.oy + cy * self.s];
        let radius = radius * self.s;
        let segments = 16;
        for i in 0..segments {
            let a0 = std::f32::consts::TAU * i as f32 / segments as f32;
            let a1 = std::f32::consts::TAU * (i + 1) as f32 / segments as f32;
            let p0 = [center[0] + a0.cos() * radius, center[1] + a0.sin() * radius];
            let p1 = [center[0] + a1.cos() * radius, center[1] + a1.sin() * radius];
            self.raw_quad([center, p0, p1, p1], color);
        }
    }

    fn keyboard(&mut self, cx: f32, cy: f32, scale: f32, color: u32) {
        let (hw, hh) = (8.0 * scale, 5.0 * scale);
        self.border(cx - hw, cy - hh, hw * 2.0, hh * 2.0, 2.4 * scale, color);
        for key in 0..3 {
            self.fill(
                cx - 5.4 * scale + key as f32 * 4.2 * scale,
                cy - 2.3 * scale,
                2.0 * scale,
                1.6 * scale,
                color,
            );
        }
        self.fill(
            cx - 3.6 * scale,
            cy + 1.4 * scale,
            7.2 * scale,
            1.5 * scale,
            color,
        );
    }

    fn toggle(&mut self, rect: Rect, anim: f32) {
        let theme = self.theme;
        let t = anim.clamp(0.0, 1.0);
        let track = mix(theme.toggle_off, theme.accent, t);
        self.rounded(rect.x, rect.y, rect.w, rect.h, rect.h / 2.0, track);
        self.border(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            rect.h / 2.0,
            mix(theme.toggle_border, theme.accent, t),
        );
        let knob = rect.h - 4.0;
        let offset = (rect.w - knob - 4.0) * t;
        self.rounded(
            rect.x + 2.0 + offset,
            rect.y + 2.0,
            knob,
            knob,
            knob / 2.0,
            mix(theme.knob_off, theme.knob_on, t),
        );
    }

    fn draw_main(&mut self, view: &View<'_>) {
        let main = main_rect(view.side);
        let theme = self.theme;
        self.rounded(main.x, main.y, main.w, main.h, 16.0, theme.page);
        self.border(main.x, main.y, main.w, main.h, 16.0, theme.border);
        self.fill(
            main.x + SIDEBAR_W,
            main.y + 1.0,
            1.0,
            main.h - 2.0,
            theme.divider,
        );
        self.draw_sidebar(view, main);
        self.draw_content(view, main);
        if view.side != SIDE_NONE {
            let saved = self.ox;
            self.ox += (1.0 - self.side_anim) * 26.0 * self.s;
            self.draw_side(view);
            self.ox = saved;
        }
    }

    fn draw_sidebar(&mut self, view: &View<'_>, main: Rect) {
        let theme = self.theme;
        self.icon("logo", main.x + 30.0, main.y + 34.0, 24.0, u32::MAX);
        self.text_spaced(
            Face::Medium,
            12.5,
            main.x + 48.0,
            main.y + 23.0,
            1.8,
            theme.strong,
            "PRISM",
        );
        self.text_spaced(
            Face::Medium,
            8.0,
            main.x + 30.0,
            main.y + 58.0,
            1.8,
            theme.faint,
            "FEATURES",
        );
        for (index, name) in view.categories.iter().enumerate() {
            let row = category_row(view.side, index);
            let active = view.category == index;
            if active {
                self.rounded(row.x, row.y, row.w, row.h, 9.0, theme.selected_bg);
                self.border(row.x, row.y, row.w, row.h, 9.0, theme.selected_border);
            } else if self.has(row) {
                self.rounded(row.x, row.y, row.w, row.h, 9.0, theme.hover);
            }
            let cy = row.y + row.h / 2.0;
            let icon = category_icon(index);
            self.icon(
                icon,
                row.x + 17.0,
                cy,
                15.0,
                if active { theme.accent } else { theme.icon },
            );
            self.text_mid(
                if active { Face::Medium } else { Face::Regular },
                13.5,
                row.x + 33.0,
                cy,
                if active {
                    theme.accent_light
                } else {
                    rgba(193, 199, 209, 255)
                },
                name,
            );
        }
        self.text_spaced(
            Face::Medium,
            8.0,
            main.x + 30.0,
            main.y + 246.0,
            1.8,
            theme.faint,
            "MANAGER",
        );
        for (index, name) in view.manager.iter().enumerate() {
            let row = manager_row(view.side, index);
            let target = if index == 0 { SIDE_BINDS } else { SIDE_CONFIG };
            let active = view.side == target;
            if active {
                self.rounded(row.x, row.y, row.w, row.h, 9.0, theme.selected_bg);
                self.border(row.x, row.y, row.w, row.h, 9.0, theme.selected_border);
            } else if self.has(row) {
                self.rounded(row.x, row.y, row.w, row.h, 9.0, theme.hover);
            }
            let cy = row.y + row.h / 2.0;
            let color = if active { theme.accent } else { theme.icon };
            if index == 0 {
                self.keyboard(row.x + 17.0, cy, 0.95, color);
            } else {
                self.icon("folder", row.x + 17.0, cy, 15.0, color);
            }
            self.text_mid(
                if active { Face::Medium } else { Face::Regular },
                13.5,
                row.x + 33.0,
                cy,
                if active {
                    theme.accent_light
                } else {
                    rgba(172, 178, 189, 255)
                },
                name,
            );
        }
        self.fill(
            main.x + 16.0,
            main.y + 334.0,
            SIDEBAR_W - 32.0,
            1.0,
            theme.divider,
        );
        self.icon("person", main.x + 32.0, main.y + 358.0, 22.0, theme.muted);
        self.text(
            Face::Medium,
            12.0,
            main.x + 48.0,
            main.y + 342.0,
            theme.strong,
            "Player",
        );
        self.text_spaced(
            Face::Regular,
            7.5,
            main.x + 48.0,
            main.y + 358.0,
            1.2,
            theme.faint,
            "PREVIEW BUILD",
        );
    }

    fn draw_content(&mut self, view: &View<'_>, main: Rect) {
        let theme = self.theme;
        let side = view.side;
        let search = search_rect(side);
        self.rounded(search.x, search.y, search.w, search.h, 10.0, theme.raised);
        self.border(
            search.x,
            search.y,
            search.w,
            search.h,
            10.0,
            if view.search_focus {
                theme.accent_deep
            } else {
                theme.border
            },
        );
        let cy = search.y + search.h / 2.0;
        self.icon("search", search.x + 18.0, cy, 13.0, theme.muted);
        if view.query.is_empty() {
            self.text_mid(
                Face::Regular,
                12.5,
                search.x + 34.0,
                cy,
                theme.faint,
                "Search something",
            );
        } else {
            let shown = if view.search_focus {
                format!("{}|", view.query)
            } else {
                view.query.to_string()
            };
            self.text_mid(Face::Regular, 12.5, search.x + 34.0, cy, theme.ink, &shown);
        }
        if !view.query.is_empty() {
            let clear = search_clear(side);
            let (x, y) = clear.center();
            self.line(x - 4.0, y - 4.0, x + 4.0, y + 4.0, 1.4, theme.icon);
            self.line(x - 4.0, y + 4.0, x + 4.0, y - 4.0, 1.4, theme.icon);
        }

        let header_y = main.y + 64.0;
        if view.query.is_empty() {
            self.text(
                Face::Medium,
                15.5,
                main.x + CONTENT_X,
                header_y,
                theme.strong,
                view.categories[view.category],
            );
            self.text_right(
                Face::Regular,
                7.5,
                main.x + CONTENT_X + CONTENT_W,
                header_y + 6.0,
                theme.faint,
                "CLICK A ROW TO OPEN SETTINGS",
            );
        } else {
            self.text(
                Face::Medium,
                15.5,
                main.x + CONTENT_X,
                header_y,
                theme.accent,
                "Results",
            );
            let count = format!("{} MATCHES", view.visible.len());
            self.text_right(
                Face::Regular,
                7.5,
                main.x + CONTENT_X + CONTENT_W,
                header_y + 6.0,
                theme.faint,
                &count,
            );
        }

        if view.visible.is_empty() {
            let card = card_rect(side, 0);
            let wide = Rect {
                x: card.x,
                y: card.y,
                w: CONTENT_W,
                h: card.h,
            };
            self.rounded(wide.x, wide.y, wide.w, wide.h, 12.0, theme.field);
            self.border(wide.x, wide.y, wide.w, wide.h, 12.0, theme.border);
            self.text_centered(
                Face::Medium,
                11.0,
                wide.x + wide.w / 2.0,
                wide.y + wide.h / 2.0 - 14.0,
                theme.muted,
                "NO MODULE MATCHES",
            );
            self.text_centered(
                Face::Regular,
                8.0,
                wide.x + wide.w / 2.0,
                wide.y + wide.h / 2.0 + 4.0,
                theme.faint,
                "TRY ANOTHER SEARCH",
            );
        } else {
            for column in 0..2 {
                let card = card_rect(side, column);
                self.rounded(card.x, card.y, card.w, card.h, 12.0, theme.field);
                self.border(card.x, card.y, card.w, card.h, 12.0, theme.border);
                for row in 0..3 {
                    let slot = column * 3 + row;
                    let Some(&(category, module)) = view.visible.get(slot) else {
                        continue;
                    };
                    self.draw_module_row(view, side, column, row, category, module);
                }
            }
        }
        self.text_spaced(
            Face::Regular,
            7.5,
            main.x + CONTENT_X,
            main.y + 302.0,
            0.9,
            theme.faint,
            "VISUAL PREVIEW / MODULES HAVE NO GAMEPLAY EFFECT",
        );
    }

    fn draw_module_row(
        &mut self,
        view: &View<'_>,
        side: usize,
        column: usize,
        row: usize,
        category: usize,
        module: usize,
    ) {
        let theme = self.theme;
        let rect = module_row(side, column, row);
        let cy = rect.y + rect.h / 2.0;
        if self.has(rect) {
            self.rounded(
                rect.x + 2.0,
                rect.y + 2.0,
                rect.w - 4.0,
                rect.h - 4.0,
                9.0,
                theme.hover,
            );
        }
        let enabled = view.enabled[category][module];
        self.icon(
            category_icon(category),
            rect.x + 17.0,
            cy,
            15.0,
            if enabled { theme.accent } else { theme.icon },
        );
        let bind = view.binds[category][module].as_deref();
        let mut detail = String::new();
        if !view.query.is_empty() {
            detail.push_str(&view.categories[category].to_uppercase());
        }
        if let Some(key) = bind {
            if !detail.is_empty() {
                detail.push_str("  /  ");
            }
            detail.push_str(&key_label(key));
        }
        let name_color = if enabled { theme.strong } else { theme.ink };
        if detail.is_empty() {
            self.text_mid(
                Face::Regular,
                13.5,
                rect.x + 33.0,
                cy,
                name_color,
                view.modules[category][module],
            );
        } else {
            self.text(
                Face::Regular,
                13.0,
                rect.x + 33.0,
                cy - 12.5,
                name_color,
                view.modules[category][module],
            );
            self.text_spaced(
                Face::Medium,
                7.0,
                rect.x + 33.0,
                cy + 2.0,
                1.0,
                if bind.is_some() {
                    theme.accent
                } else {
                    theme.faint
                },
                &detail,
            );
        }
        let (kx, ky) = keyboard_hit(side, column, row).center();
        self.keyboard(kx, ky, 0.9, theme.muted);
        self.toggle(
            toggle_row(side, column, row),
            view.animation[category][module],
        );
        if row < 2 && view.visible.get(column * 3 + row + 1).is_some() {
            self.fill(
                rect.x + 12.0,
                rect.y + rect.h + 2.5,
                rect.w - 24.0,
                1.0,
                theme.divider,
            );
        }
    }

    fn draw_side(&mut self, view: &View<'_>) {
        let theme = self.theme;
        let side = view.side;
        let frame = side_rect(side);
        self.rounded(frame.x, frame.y, frame.w, frame.h, 14.0, theme.window);
        self.border(frame.x, frame.y, frame.w, frame.h, 14.0, theme.border);

        let close = side_close(side);
        let (x, y) = close.center();
        let color = if self.has(close) {
            theme.strong
        } else {
            theme.faint
        };
        self.line(x - 4.0, y - 4.0, x + 4.0, y + 4.0, 1.4, color);
        self.line(x - 4.0, y + 4.0, x + 4.0, y - 4.0, 1.4, color);

        let (kicker, title, detail) = match side {
            SIDE_MODULE => match view.selected {
                Some((category, module)) => (
                    "MODULE",
                    view.modules[category][module].to_string(),
                    format!(
                        "{} / {} OF 06",
                        view.categories[category].to_uppercase(),
                        module + 1
                    ),
                ),
                None => ("MODULE", "Select a module".to_string(), String::new()),
            },
            SIDE_BINDS => (
                "MANAGER",
                "Key binds".to_string(),
                format!("{} BOUND", view.bound.len()),
            ),
            _ => (
                "MANAGER",
                "Configs".to_string(),
                if view.status.is_empty() {
                    format!("{} SAVED", view.configs.len())
                } else {
                    view.status.to_string()
                },
            ),
        };
        self.text_spaced(
            Face::Medium,
            8.0,
            frame.x + 18.0,
            frame.y + 18.0,
            1.8,
            theme.faint,
            kicker,
        );
        self.text(
            Face::Medium,
            17.0,
            frame.x + 18.0,
            frame.y + 32.0,
            theme.strong,
            &title,
        );
        if !detail.is_empty() {
            self.text_spaced(
                Face::Regular,
                7.5,
                frame.x + 18.0,
                frame.y + 58.0,
                1.0,
                if side == SIDE_CONFIG && !view.status.is_empty() {
                    theme.accent
                } else {
                    theme.faint
                },
                &detail,
            );
        }
        self.fill(
            frame.x + 18.0,
            frame.y + 72.0,
            frame.w - 36.0,
            1.0,
            theme.divider,
        );

        match side {
            SIDE_MODULE => self.draw_module_settings(view, frame),
            SIDE_BINDS => self.draw_binds(view, frame),
            _ => self.draw_config(view, frame),
        }
    }

    fn draw_module_settings(&mut self, view: &View<'_>, frame: Rect) {
        let theme = self.theme;
        let side = SIDE_MODULE;
        let Some((category, module)) = view.selected else {
            self.text_centered(
                Face::Medium,
                9.0,
                frame.x + frame.w / 2.0,
                frame.y + 190.0,
                theme.muted,
                "SELECT A MODULE IN THE LIST",
            );
            return;
        };
        self.text_spaced(
            Face::Medium,
            8.5,
            frame.x + 18.0,
            frame.y + 86.0,
            1.6,
            theme.faint,
            "ENABLED",
        );
        self.toggle(settings_toggle(side), view.animation[category][module]);

        self.text_spaced(
            Face::Medium,
            8.5,
            frame.x + 18.0,
            frame.y + 122.0,
            1.6,
            theme.faint,
            "KEY BIND",
        );
        let bind_box = settings_bind(side);
        self.rounded(
            bind_box.x,
            bind_box.y,
            bind_box.w,
            bind_box.h,
            10.0,
            theme.field,
        );
        self.border(
            bind_box.x,
            bind_box.y,
            bind_box.w,
            bind_box.h,
            10.0,
            if view.waiting_bind {
                theme.accent
            } else {
                theme.border
            },
        );
        let cy = bind_box.y + bind_box.h / 2.0;
        self.keyboard(
            bind_box.x + 22.0,
            cy,
            1.05,
            if view.waiting_bind {
                theme.accent
            } else {
                theme.muted
            },
        );
        if view.waiting_bind {
            self.text(
                Face::Medium,
                12.0,
                bind_box.x + 42.0,
                cy - 12.0 * 0.725,
                theme.accent,
                "PRESS A KEY",
            );
        } else {
            let bind = view.binds[category][module].as_deref();
            let label = bind.map(key_label).unwrap_or_else(|| "UNBOUND".to_string());
            self.text(
                Face::Medium,
                12.0,
                bind_box.x + 42.0,
                cy - 12.0 * 0.725,
                if bind.is_some() {
                    theme.strong
                } else {
                    theme.muted
                },
                &label,
            );
        }
        self.text_spaced(
            Face::Regular,
            7.0,
            frame.x + 18.0,
            frame.y + 180.0,
            0.9,
            theme.faint,
            "BACKSPACE / DELETE CLEARS THE BIND",
        );

        self.text_spaced(
            Face::Medium,
            8.5,
            frame.x + 18.0,
            frame.y + 204.0,
            1.6,
            theme.faint,
            "VALUE",
        );
        let percent = format!("{}%", view.values[category][module]);
        self.text_right(
            Face::Medium,
            10.0,
            frame.x + frame.w - 18.0,
            frame.y + 202.0,
            theme.accent,
            &percent,
        );
        let track = settings_slider_track(side);
        self.rounded(
            track.x,
            track.y,
            track.w,
            track.h,
            track.h / 2.0,
            rgba(31, 35, 43, 255),
        );
        let filled = track.w * f32::from(view.values[category][module]) / 100.0;
        if filled > 1.0 {
            self.rounded(
                track.x,
                track.y,
                filled,
                track.h,
                track.h / 2.0,
                theme.accent,
            );
        }
        self.disc(track.x + filled, track.y + track.h / 2.0, 5.5, theme.strong);
        self.text_spaced(
            Face::Regular,
            7.0,
            frame.x + 18.0,
            frame.y + 244.0,
            0.9,
            theme.faint,
            "DRAG THE TRACK TO SET THE STRENGTH",
        );

        let done = settings_done(side);
        let fill = if self.has(done) {
            theme.accent_light
        } else {
            theme.accent
        };
        self.rounded(done.x, done.y, done.w, done.h, 10.0, fill);
        self.text_centered(
            Face::Medium,
            11.0,
            done.x + done.w / 2.0,
            done.y + (done.h - 11.0 * 1.45) / 2.0,
            rgba(12, 14, 18, 255),
            "DONE",
        );
    }

    fn draw_binds(&mut self, view: &View<'_>, frame: Rect) {
        let theme = self.theme;
        if view.bound.is_empty() {
            self.text_centered(
                Face::Medium,
                9.0,
                frame.x + frame.w / 2.0,
                frame.y + 176.0,
                theme.muted,
                "NO BINDS YET",
            );
            self.text_centered(
                Face::Regular,
                7.5,
                frame.x + frame.w / 2.0,
                frame.y + 196.0,
                theme.faint,
                "OPEN A MODULE AND PRESS A KEY",
            );
        } else {
            for (index, (category, module, key)) in view.bound.iter().take(6).enumerate() {
                let row = binds_row(SIDE_BINDS, index);
                self.rounded(row.x, row.y, row.w, row.h, 9.0, theme.field);
                if self.has(row) {
                    self.rounded(row.x, row.y, row.w, row.h, 9.0, theme.hover);
                }
                self.text(
                    Face::Regular,
                    12.0,
                    row.x + 12.0,
                    row.y + 10.5,
                    theme.ink,
                    view.modules[*category][*module],
                );
                let label = key_label(key);
                self.text_right(
                    Face::Medium,
                    9.5,
                    row.x + row.w - 12.0,
                    row.y + 12.0,
                    theme.accent,
                    &label,
                );
            }
        }
        let clear = binds_clear(SIDE_BINDS);
        let hovered = self.has(clear);
        self.rounded(
            clear.x,
            clear.y,
            clear.w,
            clear.h,
            10.0,
            if hovered { theme.raised } else { theme.field },
        );
        self.border(
            clear.x,
            clear.y,
            clear.w,
            clear.h,
            10.0,
            if hovered {
                theme.accent_deep
            } else {
                theme.border
            },
        );
        self.text_centered(
            Face::Medium,
            10.0,
            clear.x + clear.w / 2.0,
            clear.y + (clear.h - 10.0 * 1.45) / 2.0,
            if view.bound.is_empty() {
                theme.faint
            } else {
                theme.muted
            },
            "CLEAR ALL BINDS",
        );
    }

    fn draw_config(&mut self, view: &View<'_>, frame: Rect) {
        let theme = self.theme;
        if view.configs.is_empty() {
            self.text_centered(
                Face::Medium,
                9.0,
                frame.x + frame.w / 2.0,
                frame.y + 140.0,
                theme.muted,
                "NO CONFIGS YET",
            );
            self.text_centered(
                Face::Regular,
                7.5,
                frame.x + frame.w / 2.0,
                frame.y + 160.0,
                theme.faint,
                "PRESS NEW TO SAVE THE CURRENT SETUP",
            );
        }
        for (index, name) in view.configs.iter().take(5).enumerate() {
            let row = config_row(SIDE_CONFIG, index);
            let selected = view.config_selected == Some(index);
            self.rounded(
                row.x,
                row.y,
                row.w,
                row.h,
                9.0,
                if selected {
                    theme.selected_bg
                } else {
                    theme.field
                },
            );
            if selected {
                self.border(row.x, row.y, row.w, row.h, 9.0, theme.selected_border);
            } else if self.has(row) {
                self.rounded(row.x, row.y, row.w, row.h, 9.0, theme.hover);
            }
            self.text(
                Face::Medium,
                11.5,
                row.x + 12.0,
                row.y + 8.0,
                if selected {
                    theme.accent_light
                } else {
                    theme.ink
                },
                name,
            );
            if selected {
                self.disc(row.x + row.w - 12.0, row.y + row.h / 2.0, 2.2, theme.accent);
            }
        }

        const LABELS: [&str; 4] = ["NEW", "SAVE", "LOAD", "DELETE"];
        for (which, label) in LABELS.iter().enumerate() {
            let rect = config_button(SIDE_CONFIG, which);
            let hovered = self.has(rect);
            if which == 1 {
                self.rounded(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    9.0,
                    if hovered {
                        theme.accent_light
                    } else {
                        theme.accent
                    },
                );
                self.text_centered(
                    Face::Medium,
                    9.5,
                    rect.x + rect.w / 2.0,
                    rect.y + (rect.h - 9.5 * 1.45) / 2.0,
                    rgba(12, 14, 18, 255),
                    label,
                );
            } else {
                self.rounded(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    9.0,
                    if hovered { theme.raised } else { theme.field },
                );
                self.border(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    9.0,
                    if hovered {
                        theme.accent_deep
                    } else {
                        theme.border
                    },
                );
                self.text_centered(
                    Face::Medium,
                    9.5,
                    rect.x + rect.w / 2.0,
                    rect.y + (rect.h - 9.5 * 1.45) / 2.0,
                    if which == 3 { theme.muted } else { theme.ink },
                    label,
                );
            }
        }
    }
}

struct IconMap {
    atlas: usize,
    icons: Vec<(&'static str, crate::hud::Rect)>,
}

thread_local! {
    static ICON_MAP: RefCell<Option<IconMap>> = const { RefCell::new(None) };
}

/// Rows in the glyph atlas region reserved for the bundled icons.
const ICON_STRIP: u32 = 104;

fn upload_icons(atlas: &HudAtlas, batch: &mut HudBatch) {
    let key = atlas as *const HudAtlas as usize;
    ICON_MAP.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.as_ref().is_some_and(|map| map.atlas == key) {
            return;
        }
        let Some(region) = atlas.sprites.get("horizon/ui-text").copied() else {
            return;
        };
        let origin_x = (region.u0 * atlas.width as f32).round() as u32;
        let origin_y =
            (region.v0 * atlas.height as f32).round() as u32 + region.h.saturating_sub(ICON_STRIP);
        let (aw, ah) = (atlas.width as f32, atlas.height as f32);
        let mut icons = Vec::with_capacity(example_mod_icons::ICONS.len());
        let mut pen = 0u32;
        for (name, _, data) in example_mod_icons::ICONS {
            let Ok((width, height, pixels)) =
                crate::hud::decode_bytes(&example_mod_icons::decode_base64(data))
            else {
                continue;
            };
            let (x, y) = (origin_x + pen, origin_y);
            batch.patches.push(crate::font::Patch {
                x,
                y,
                width,
                height,
                rgba: pixels,
            });
            icons.push((
                name,
                crate::hud::Rect {
                    u0: x as f32 / aw,
                    v0: y as f32 / ah,
                    u1: (x + width) as f32 / aw,
                    v1: (y + height) as f32 / ah,
                    w: width,
                    h: height,
                },
            ));
            pen += width + 8;
        }
        *slot = Some(IconMap { atlas: key, icons });
    });
}

/// A short label for a winit `KeyCode` debug name.
pub fn key_label(key: &str) -> String {
    let base = key
        .strip_prefix("Key")
        .or_else(|| key.strip_prefix("Digit"))
        .unwrap_or(key);
    let mapped = match key {
        "ArrowUp" => "Up",
        "ArrowDown" => "Down",
        "ArrowLeft" => "Left",
        "ArrowRight" => "Right",
        "ShiftLeft" => "LShift",
        "ShiftRight" => "RShift",
        "ControlLeft" => "LCtrl",
        "ControlRight" => "RCtrl",
        "AltLeft" => "LAlt",
        "AltRight" => "RAlt",
        "Space" => "Space",
        "Enter" => "Enter",
        "Tab" => "Tab",
        "Escape" => "Esc",
        "Backspace" => "Bksp",
        "Delete" => "Del",
        "Insert" => "Ins",
        "Home" => "Home",
        "End" => "End",
        "PageUp" => "PgUp",
        "PageDown" => "PgDn",
        "Semicolon" => ";",
        "Comma" => ",",
        "Period" => ".",
        "Slash" => "/",
        "Minus" => "-",
        "Equal" => "=",
        "Backslash" => "\\",
        "Quote" => "'",
        "Backquote" => "`",
        "BracketLeft" => "[",
        "BracketRight" => "]",
        "NumpadAdd" => "Num+",
        "NumpadSubtract" => "Num-",
        "NumpadMultiply" => "Num*",
        "NumpadDivide" => "Num/",
        "NumpadEnter" => "NumEnter",
        _ => base,
    };
    mapped.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn panels_are_centred() {
        let closed = main_rect(SIDE_NONE);
        assert!(close(closed.x + closed.w / 2.0, DESIGN_W / 2.0));
        let group = PANEL_W + SIDE_GAP + SIDE_W;
        let open = main_rect(SIDE_BINDS);
        assert!(close(open.x + group / 2.0, DESIGN_W / 2.0));
        let side = side_rect(SIDE_BINDS);
        assert!(close(side.x + side.w, open.x + group));
        assert!(close(side.y, PANEL_Y));
    }

    #[test]
    fn rows_and_hitboxes_agree() {
        let row = module_row(SIDE_NONE, 1, 2);
        assert!(row.contains(row.center().0, row.center().1));
        for hit in [
            toggle_row(SIDE_NONE, 1, 2),
            dots_hit(SIDE_NONE, 1, 2),
            keyboard_hit(SIDE_NONE, 1, 2),
        ] {
            let (x, y) = hit.center();
            assert!(row.contains(x, y), "{hit:?} is outside {row:?}");
        }
        let card = card_rect(SIDE_NONE, 1);
        assert!(card.contains(row.center().0, row.center().1));
        let last = module_row(SIDE_NONE, 1, 2);
        assert!(last.y + last.h <= card.y + card.h);
    }

    #[test]
    fn categories_keep_their_icon_on_every_module() {
        for (category, icon) in ["sword", "running", "camera", "person", "dots"]
            .iter()
            .enumerate()
        {
            assert_eq!(category_icon(category), *icon);
        }
        assert_eq!(category_icon(9), "dots");
    }

    #[test]
    fn slider_hit_covers_the_track() {
        let hit = settings_slider_hit(SIDE_MODULE);
        let track = settings_slider_track(SIDE_MODULE);
        assert!(hit.contains(track.x + track.w / 2.0, track.y + track.h / 2.0));
        assert!(hit.x <= track.x && hit.x + hit.w >= track.x + track.w);
    }

    #[test]
    fn side_controls_stay_inside_the_window() {
        let frame = side_rect(SIDE_MODULE);
        for rect in [
            side_close(SIDE_MODULE),
            settings_toggle(SIDE_MODULE),
            settings_bind(SIDE_MODULE),
            settings_slider_hit(SIDE_MODULE),
            settings_done(SIDE_MODULE),
            binds_clear(SIDE_BINDS),
            config_button(SIDE_CONFIG, 3),
        ] {
            assert!(rect.x >= frame.x && rect.y >= frame.y, "{rect:?}");
            assert!(
                rect.x + rect.w <= frame.x + frame.w && rect.y + rect.h <= frame.y + frame.h,
                "{rect:?}"
            );
        }
    }

    #[test]
    fn cursor_maps_to_design_units() {
        let cursor = (320.0, 180.0);
        let (x, y) = to_design(cursor, 640.0, 360.0, 2.0);
        let (offset_x, offset_y, scale) = layout(640.0, 360.0, 2.0);
        let density = density(640.0, 360.0, 2.0);
        assert!(close(x, (cursor.0 * 2.0 / density - offset_x) / scale));
        assert!(close(y, (cursor.1 * 2.0 / density - offset_y) / scale));
    }

    #[test]
    fn key_labels_are_short() {
        assert_eq!(key_label("KeyK"), "K");
        assert_eq!(key_label("Digit3"), "3");
        assert_eq!(key_label("ArrowUp"), "UP");
        assert_eq!(key_label("ShiftLeft"), "LSHIFT");
        assert_eq!(key_label("Semicolon"), ";");
    }
}
