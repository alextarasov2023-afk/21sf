// The PRISM panel's drawing and geometry, injected as a public support
// module of `horizon_client`. It lives beside `ui_theme` because the game
// module (`crates/horizon-client/src/game.rs`) is compiled into the binary
// crate, while the launcher's `Ui` widgets are crate-private to the library.
//
// Everything here is authored on a 1244x700 design surface that is scaled
// and centred like the launcher's own 1040x700 window. The game module only
// snapshots its state into `View` and calls `render`; input hit-tests the
// same public `Rect`s, so drawing and clicking cannot disagree.
use crate::hud::{HudAtlas, HudBatch, rgba};
use crate::ui_theme::{Face, Ui};

/// The launcher-like surface the panel is authored on, in design units.
/// The renderer scales this box to the current window.
pub const DESIGN_W: f32 = 1244.0;
pub const DESIGN_H: f32 = 700.0;

pub const PANEL_X: f32 = 12.0;
pub const PANEL_Y: f32 = 12.0;
pub const PANEL_W: f32 = 1220.0;
pub const PANEL_H: f32 = 676.0;
pub const SIDEBAR_W: f32 = 270.0;

pub const CONTENT_X: f32 = 310.0;
pub const CONTENT_W: f32 = 898.0;
pub const COLUMN_W: f32 = 437.0;
pub const COLUMN_STEP: f32 = 461.0;
pub const ROW_H: f32 = 62.0;
pub const ROW_TOP: f32 = 148.0;

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

pub const SEARCH: Rect = Rect {
    x: CONTENT_X,
    y: 34.0,
    w: CONTENT_W,
    h: 36.0,
};

pub const CARD: Rect = Rect {
    x: CONTENT_X,
    y: 358.0,
    w: CONTENT_W,
    h: 306.0,
};

pub const BIND_BOX: Rect = Rect {
    x: 334.0,
    y: 452.0,
    w: 240.0,
    h: 42.0,
};

pub const SLIDER: Rect = Rect {
    x: 334.0,
    y: 552.0,
    w: 380.0,
    h: 8.0,
};

pub const SLIDER_HIT: Rect = Rect {
    x: 334.0,
    y: 532.0,
    w: 380.0,
    h: 44.0,
};

pub const DONE: Rect = Rect {
    x: 1080.0,
    y: 452.0,
    w: 104.0,
    h: 42.0,
};

pub fn category_row(index: usize) -> Rect {
    Rect {
        x: 36.0,
        y: 138.0 + index as f32 * 50.0,
        w: 222.0,
        h: 48.0,
    }
}

pub fn manager_row(index: usize) -> Rect {
    Rect {
        x: 36.0,
        y: 436.0 + index as f32 * 50.0,
        w: 222.0,
        h: 48.0,
    }
}

pub fn column_x(column: usize) -> f32 {
    CONTENT_X + column as f32 * COLUMN_STEP
}

pub fn module_row(column: usize, row: usize) -> Rect {
    Rect {
        x: column_x(column),
        y: ROW_TOP + row as f32 * ROW_H,
        w: COLUMN_W,
        h: ROW_H,
    }
}

pub fn toggle_row(column: usize, row: usize) -> Rect {
    let module = module_row(column, row);
    Rect {
        x: module.x + COLUMN_W - 58.0,
        y: module.y + 19.0,
        w: 46.0,
        h: 24.0,
    }
}

pub fn dots_hit(column: usize, row: usize) -> Rect {
    let module = module_row(column, row);
    Rect {
        x: module.x + COLUMN_W - 96.0,
        y: module.y + 17.0,
        w: 32.0,
        h: 28.0,
    }
}

pub fn keyboard_hit(column: usize, row: usize) -> Rect {
    let module = module_row(column, row);
    Rect {
        x: module.x + COLUMN_W - 132.0,
        y: module.y + 17.0,
        w: 28.0,
        h: 28.0,
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
/// module. The renderer only writes back [`View::animation`].
#[allow(clippy::type_complexity)]
pub struct View<'a> {
    pub category: usize,
    pub enabled: [[bool; 6]; 5],
    pub animation: [[f32; 6]; 5],
    pub binds: &'a [[Option<String>; 6]; 5],
    pub values: [[u8; 6]; 5],
    pub selected: Option<(usize, usize)>,
    pub waiting_bind: bool,
    pub categories: &'a [&'a str],
    pub modules: &'a [[&'a str; 6]; 5],
    pub manager: &'a [&'a str; 3],
}

pub fn render(atlas: &HudAtlas, batch: &mut HudBatch, cursor: (f32, f32), view: &mut View<'_>) {
    let (ox, oy, s) = layout(batch.width, batch.height, batch.scale);
    let mut ui = Ui::new(atlas, batch, cursor);
    let (cursor_x, cursor_y) = ((ui.cursor.0 - ox) / s, (ui.cursor.1 - oy) / s);

    let page = rgba(9, 10, 14, 252);
    let border = rgba(30, 33, 40, 255);
    let divider = rgba(25, 28, 34, 255);
    let raised = rgba(16, 18, 23, 255);
    let field = rgba(17, 19, 25, 255);
    let hover = rgba(255, 255, 255, 6);
    let selected_bg = rgba(19, 27, 41, 255);
    let selected_border = rgba(47, 66, 102, 255);
    let ink = rgba(205, 210, 219, 255);
    let strong = rgba(232, 235, 242, 255);
    let muted = rgba(124, 130, 142, 255);
    let faint = rgba(86, 92, 104, 255);
    let icon_idle = rgba(151, 157, 169, 255);
    let accent = rgba(112, 146, 219, 255);
    let accent_light = rgba(126, 162, 236, 255);
    let accent_deep = rgba(88, 121, 188, 255);
    let toggle_off = rgba(37, 40, 47, 255);
    let toggle_off_border = rgba(60, 65, 75, 255);
    let knob_off = rgba(99, 105, 117, 255);
    let knob_on = rgba(240, 243, 250, 255);

    rounded(
        &mut ui, ox, oy, s, PANEL_X, PANEL_Y, PANEL_W, PANEL_H, 16.0, page,
    );
    bordered(
        &mut ui, ox, oy, s, PANEL_X, PANEL_Y, PANEL_W, PANEL_H, 16.0, border,
    );
    fill(
        &mut ui,
        ox,
        oy,
        s,
        PANEL_X + SIDEBAR_W,
        PANEL_Y,
        1.0,
        PANEL_H,
        divider,
    );

    logo(
        &mut ui,
        ox,
        oy,
        s,
        147.0,
        52.0,
        1.0,
        accent_light,
        accent_deep,
    );

    text_spaced(
        &mut ui,
        ox,
        oy,
        s,
        Face::Medium,
        11.5,
        52.0,
        116.0,
        1.7,
        faint,
        "FEATURES",
    );
    for (index, name) in view.categories.iter().enumerate() {
        let row = category_row(index);
        let active = view.category == index;
        if active {
            rounded(
                &mut ui,
                ox,
                oy,
                s,
                row.x,
                row.y,
                row.w,
                row.h,
                10.0,
                selected_bg,
            );
            bordered(
                &mut ui,
                ox,
                oy,
                s,
                row.x,
                row.y,
                row.w,
                row.h,
                10.0,
                selected_border,
            );
        } else if row.contains(cursor_x, cursor_y) {
            rounded(&mut ui, ox, oy, s, row.x, row.y, row.w, row.h, 10.0, hover);
        }
        let (_, row_cy) = row.center();
        let color = if active { accent } else { icon_idle };
        category_icon(&mut ui, ox, oy, s, index, row.x + 36.0, row_cy, 18.0, color);
        let label_color = if active {
            accent
        } else {
            rgba(193, 199, 209, 255)
        };
        text(
            &mut ui,
            ox,
            oy,
            s,
            if active { Face::Medium } else { Face::Regular },
            16.5,
            row.x + 66.0,
            row_cy - 16.5 * 0.725,
            label_color,
            name,
        );
    }

    text_spaced(
        &mut ui,
        ox,
        oy,
        s,
        Face::Medium,
        11.5,
        52.0,
        414.0,
        1.7,
        faint,
        "MANAGER",
    );
    for (index, name) in view.manager.iter().enumerate() {
        let row = manager_row(index);
        if row.contains(cursor_x, cursor_y) {
            rounded(&mut ui, ox, oy, s, row.x, row.y, row.w, row.h, 10.0, hover);
        }
        let (_, row_cy) = row.center();
        manager_icon(
            &mut ui,
            ox,
            oy,
            s,
            index,
            row.x + 36.0,
            row_cy,
            18.0,
            icon_idle,
        );
        text(
            &mut ui,
            ox,
            oy,
            s,
            Face::Regular,
            16.5,
            row.x + 66.0,
            row_cy - 16.5 * 0.725,
            rgba(172, 178, 189, 255),
            name,
        );
    }

    fill(&mut ui, ox, oy, s, 36.0, 610.0, 222.0, 1.0, divider);
    disc(&mut ui, ox, oy, s, 54.0, 646.0, 13.0, rgba(30, 38, 52, 255));
    ring(&mut ui, ox, oy, s, 54.0, 642.6, 3.4, 1.4, muted);
    arc(
        &mut ui,
        ox,
        oy,
        s,
        54.0,
        653.0,
        6.2,
        5.0,
        std::f32::consts::PI,
        std::f32::consts::TAU,
        1.4,
        muted,
    );
    text(
        &mut ui,
        ox,
        oy,
        s,
        Face::Medium,
        14.5,
        78.0,
        634.0,
        strong,
        "Player",
    );
    text(
        &mut ui,
        ox,
        oy,
        s,
        Face::Regular,
        10.5,
        78.0,
        654.0,
        muted,
        "Until 1 January 3928",
    );

    rounded(
        &mut ui, ox, oy, s, SEARCH.x, SEARCH.y, SEARCH.w, SEARCH.h, 10.0, raised,
    );
    bordered(
        &mut ui, ox, oy, s, SEARCH.x, SEARCH.y, SEARCH.w, SEARCH.h, 10.0, border,
    );
    let search_cy = SEARCH.y + SEARCH.h / 2.0;
    search_icon(&mut ui, ox, oy, s, SEARCH.x + 22.0, search_cy, muted);
    text(
        &mut ui,
        ox,
        oy,
        s,
        Face::Regular,
        14.5,
        SEARCH.x + 40.0,
        search_cy - 14.5 * 0.725,
        rgba(104, 110, 122, 255),
        "Search something",
    );
    hamburger_icon(
        &mut ui,
        ox,
        oy,
        s,
        SEARCH.x + SEARCH.w - 26.0,
        search_cy,
        icon_idle,
    );

    let header_y = 104.0;
    text(
        &mut ui,
        ox,
        oy,
        s,
        Face::Medium,
        19.0,
        CONTENT_X,
        header_y,
        accent,
        "Base",
    );
    text(
        &mut ui,
        ox,
        oy,
        s,
        Face::Medium,
        19.0,
        column_x(1),
        header_y,
        strong,
        "Tools",
    );
    for column in 0..2 {
        let cx = column_x(column) + COLUMN_W - 20.0;
        chevron(&mut ui, ox, oy, s, cx, 114.5, 7.0, 4.5, true, 1.6, accent);
        chevron(&mut ui, ox, oy, s, cx, 121.5, 7.0, 4.5, false, 1.6, accent);
    }
    fill(
        &mut ui, ox, oy, s, CONTENT_X, 146.0, CONTENT_W, 1.0, divider,
    );

    let category = view.category;
    for column in 0..2 {
        for row in 0..3 {
            let index = column * 3 + row;
            let rect = module_row(column, row);
            if rect.contains(cursor_x, cursor_y) {
                rounded(
                    &mut ui,
                    ox,
                    oy,
                    s,
                    rect.x + 2.0,
                    rect.y + 2.0,
                    rect.w - 4.0,
                    rect.h - 4.0,
                    9.0,
                    hover,
                );
            }
            let (_, row_cy) = rect.center();
            let label = view.modules[category][index];
            text(
                &mut ui,
                ox,
                oy,
                s,
                Face::Regular,
                17.0,
                rect.x + 16.0,
                row_cy - 17.0 * 0.725,
                ink,
                label,
            );
            keyboard_icon(
                &mut ui,
                ox,
                oy,
                s,
                rect.x + COLUMN_W - 118.0,
                row_cy,
                rgba(117, 123, 135, 255),
            );
            dots_icon(
                &mut ui,
                ox,
                oy,
                s,
                rect.x + COLUMN_W - 80.0,
                row_cy,
                rgba(127, 133, 145, 255),
            );

            let toggle = toggle_row(column, row);
            let target = f32::from(view.enabled[category][index]);
            let anim = &mut view.animation[category][index];
            *anim += (target - *anim) * 0.22;
            let anim = *anim;
            let track = if target > 0.5 { accent } else { toggle_off };
            rounded(
                &mut ui,
                ox,
                oy,
                s,
                toggle.x,
                toggle.y,
                toggle.w,
                toggle.h,
                toggle.h / 2.0,
                track,
            );
            if target <= 0.5 {
                bordered(
                    &mut ui,
                    ox,
                    oy,
                    s,
                    toggle.x,
                    toggle.y,
                    toggle.w,
                    toggle.h,
                    toggle.h / 2.0,
                    toggle_off_border,
                );
            }
            let knob = toggle.h - 4.0;
            rounded(
                &mut ui,
                ox,
                oy,
                s,
                toggle.x + 2.0 + (toggle.w - knob - 4.0) * anim,
                toggle.y + 2.0,
                knob,
                knob,
                knob / 2.0,
                if target > 0.5 { knob_on } else { knob_off },
            );
            if row < 2 {
                fill(
                    &mut ui,
                    ox,
                    oy,
                    s,
                    rect.x + 8.0,
                    rect.y + rect.h - 1.0,
                    rect.w - 16.0,
                    1.0,
                    divider,
                );
            }
        }
    }

    rounded(
        &mut ui,
        ox,
        oy,
        s,
        CARD.x,
        CARD.y,
        CARD.w,
        CARD.h,
        14.0,
        rgba(12, 14, 18, 255),
    );
    bordered(
        &mut ui,
        ox,
        oy,
        s,
        CARD.x,
        CARD.y,
        CARD.w,
        CARD.h,
        14.0,
        rgba(27, 30, 37, 255),
    );
    text_spaced(
        &mut ui,
        ox,
        oy,
        s,
        Face::Medium,
        11.0,
        CARD.x + 24.0,
        378.0,
        1.6,
        faint,
        "MODULE SETTINGS",
    );
    if let Some((selected_category, module)) = view.selected {
        text(
            &mut ui,
            ox,
            oy,
            s,
            Face::Medium,
            20.0,
            CARD.x + 24.0,
            396.0,
            strong,
            view.modules[selected_category][module],
        );
        let breadcrumb = format!(
            "{} / {} OF 6",
            view.categories[selected_category].to_uppercase(),
            module + 1
        );
        let width = ui.measure(Face::Medium, 10.0 * s, &breadcrumb) / s;
        text_spaced(
            &mut ui,
            ox,
            oy,
            s,
            Face::Medium,
            10.0,
            CARD.x + CARD.w - 24.0 - width - breadcrumb.chars().count() as f32 * 1.4,
            402.0,
            1.4,
            faint,
            &breadcrumb,
        );

        rounded(
            &mut ui, ox, oy, s, BIND_BOX.x, BIND_BOX.y, BIND_BOX.w, BIND_BOX.h, 10.0, field,
        );
        bordered(
            &mut ui,
            ox,
            oy,
            s,
            BIND_BOX.x,
            BIND_BOX.y,
            BIND_BOX.w,
            BIND_BOX.h,
            10.0,
            if view.waiting_bind {
                accent
            } else {
                rgba(36, 40, 49, 255)
            },
        );
        let bind_cy = BIND_BOX.y + BIND_BOX.h / 2.0;
        keyboard_icon(&mut ui, ox, oy, s, BIND_BOX.x + 23.0, bind_cy, muted);
        if view.waiting_bind {
            text(
                &mut ui,
                ox,
                oy,
                s,
                Face::Medium,
                13.5,
                BIND_BOX.x + 44.0,
                bind_cy - 13.5 * 0.725,
                accent,
                "PRESS A KEY",
            );
        } else {
            text(
                &mut ui,
                ox,
                oy,
                s,
                Face::Medium,
                10.5,
                BIND_BOX.x + 44.0,
                bind_cy - 10.5 * 0.725,
                muted,
                "BIND",
            );
            let bind = view.binds[selected_category][module]
                .as_deref()
                .unwrap_or("NONE");
            text(
                &mut ui,
                ox,
                oy,
                s,
                Face::Medium,
                13.5,
                BIND_BOX.x + 82.0,
                bind_cy - 13.5 * 0.725,
                if bind == "NONE" { muted } else { strong },
                bind,
            );
        }
        text_spaced(
            &mut ui,
            ox,
            oy,
            s,
            Face::Regular,
            10.0,
            CARD.x + 24.0,
            502.0,
            0.8,
            faint,
            "BACKSPACE / DELETE CLEARS THE BIND",
        );

        text_spaced(
            &mut ui,
            ox,
            oy,
            s,
            Face::Medium,
            10.5,
            CARD.x + 24.0,
            532.0,
            1.4,
            faint,
            "PREVIEW VALUE",
        );
        let value = view.values[selected_category][module];
        let percent = format!("{value}%");
        let width = ui.measure(Face::Medium, 11.0 * s, &percent) / s;
        text(
            &mut ui,
            ox,
            oy,
            s,
            Face::Medium,
            11.0,
            SLIDER.x + SLIDER.w - width,
            531.0,
            accent,
            &percent,
        );
        rounded(
            &mut ui,
            ox,
            oy,
            s,
            SLIDER.x,
            SLIDER.y,
            SLIDER.w,
            SLIDER.h,
            SLIDER.h / 2.0,
            rgba(31, 35, 43, 255),
        );
        let filled = SLIDER.w * f32::from(value) / 100.0;
        if filled > 1.0 {
            rounded(
                &mut ui,
                ox,
                oy,
                s,
                SLIDER.x,
                SLIDER.y,
                filled,
                SLIDER.h,
                SLIDER.h / 2.0,
                accent,
            );
        }
        disc(
            &mut ui,
            ox,
            oy,
            s,
            SLIDER.x + filled,
            SLIDER.y + SLIDER.h / 2.0,
            6.0,
            strong,
        );

        let hovered = DONE.contains(cursor_x, cursor_y);
        rounded(
            &mut ui,
            ox,
            oy,
            s,
            DONE.x,
            DONE.y,
            DONE.w,
            DONE.h,
            11.0,
            if hovered { accent_light } else { accent },
        );
        text_cx(
            &mut ui,
            ox,
            oy,
            s,
            Face::Medium,
            13.0,
            DONE.x + DONE.w / 2.0,
            DONE.y + (DONE.h - 13.0 * 1.45) / 2.0,
            rgba(12, 14, 18, 255),
            "DONE",
        );
    } else {
        logo(
            &mut ui,
            ox,
            oy,
            s,
            CARD.x + CARD.w / 2.0,
            486.0,
            3.0,
            rgba(112, 146, 219, 18),
            rgba(88, 121, 188, 14),
        );
        text_cx(
            &mut ui,
            ox,
            oy,
            s,
            Face::Medium,
            13.0,
            CARD.x + CARD.w / 2.0,
            562.0,
            muted,
            "SELECT A MODULE TO CONFIGURE",
        );
        text_cx(
            &mut ui,
            ox,
            oy,
            s,
            Face::Regular,
            10.5,
            CARD.x + CARD.w / 2.0,
            584.0,
            faint,
            "CLICK ... ON A ROW, THEN SET A BIND OR VALUE",
        );
    }
    text_spaced(
        &mut ui,
        ox,
        oy,
        s,
        Face::Regular,
        10.0,
        CARD.x + 24.0,
        636.0,
        0.8,
        faint,
        "VISUAL PREVIEW / MODULES HAVE NO GAMEPLAY EFFECT",
    );

    ui.finish();
}

#[allow(clippy::too_many_arguments)]
fn fill(ui: &mut Ui<'_>, ox: f32, oy: f32, s: f32, x: f32, y: f32, w: f32, h: f32, color: u32) {
    ui.rounded(ox + x * s, oy + y * s, w * s, h * s, 0.0, color);
}

#[allow(clippy::too_many_arguments)]
fn rounded(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    color: u32,
) {
    ui.rounded(ox + x * s, oy + y * s, w * s, h * s, radius * s, color);
}

#[allow(clippy::too_many_arguments)]
fn bordered(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    color: u32,
) {
    ui.border(ox + x * s, oy + y * s, w * s, h * s, radius * s, color);
}

#[allow(clippy::too_many_arguments)]
fn text(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    face: Face,
    size: f32,
    x: f32,
    y: f32,
    color: u32,
    value: &str,
) -> f32 {
    ui.text(face, size * s, ox + x * s, oy + y * s, color, value)
}

#[allow(clippy::too_many_arguments)]
fn text_cx(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    face: Face,
    size: f32,
    cx: f32,
    y: f32,
    color: u32,
    value: &str,
) {
    let w = ui.measure(face, size * s, value);
    ui.text(
        face,
        size * s,
        ox + cx * s - w / 2.0,
        oy + y * s,
        color,
        value,
    );
}

#[allow(clippy::too_many_arguments)]
fn text_spaced(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    face: Face,
    size: f32,
    x: f32,
    y: f32,
    spacing: f32,
    color: u32,
    value: &str,
) -> f32 {
    let mut pen = x;
    for ch in value.chars() {
        let mut buf = [0u8; 4];
        let glyph = ch.encode_utf8(&mut buf);
        let advance = ui.measure(face, size * s, glyph) / s;
        if !ch.is_whitespace() {
            ui.text(face, size * s, ox + pen * s, oy + y * s, color, glyph);
        }
        pen += advance + spacing;
    }
    pen - x - spacing
}

fn raw_quad(ui: &mut Ui<'_>, corners: [[f32; 2]; 4], color: u32) {
    let k = ui.density / ui.batch.scale;
    let quad = corners.map(|[x, y]| [x * k, y * k]);
    let atlas = ui.atlas;
    ui.batch.fill_quad(atlas, quad, color);
}

#[allow(clippy::too_many_arguments)]
fn line(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    thickness: f32,
    color: u32,
) {
    let a = [ox + x1 * s, oy + y1 * s];
    let b = [ox + x2 * s, oy + y2 * s];
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let length = (dx * dx + dy * dy).sqrt();
    if length < 0.001 {
        return;
    }
    let half = thickness * s / 2.0;
    let nx = -dy / length * half;
    let ny = dx / length * half;
    raw_quad(
        ui,
        [
            [a[0] + nx, a[1] + ny],
            [b[0] + nx, b[1] + ny],
            [b[0] - nx, b[1] - ny],
            [a[0] - nx, a[1] - ny],
        ],
        color,
    );
}

#[allow(clippy::too_many_arguments)]
fn disc(ui: &mut Ui<'_>, ox: f32, oy: f32, s: f32, cx: f32, cy: f32, radius: f32, color: u32) {
    let center = [ox + cx * s, oy + cy * s];
    let radius = radius * s;
    let segments = 16;
    for i in 0..segments {
        let a0 = std::f32::consts::TAU * i as f32 / segments as f32;
        let a1 = std::f32::consts::TAU * (i + 1) as f32 / segments as f32;
        let p0 = [center[0] + a0.cos() * radius, center[1] + a0.sin() * radius];
        let p1 = [center[0] + a1.cos() * radius, center[1] + a1.sin() * radius];
        raw_quad(ui, [center, p0, p1, p1], color);
    }
}

#[allow(clippy::too_many_arguments)]
fn ring(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    cx: f32,
    cy: f32,
    radius: f32,
    thickness: f32,
    color: u32,
) {
    let segments = 20;
    let mut previous = None;
    for i in 0..=segments {
        let angle = std::f32::consts::TAU * i as f32 / segments as f32;
        let point = (cx + angle.cos() * radius, cy + angle.sin() * radius);
        if let Some((px, py)) = previous {
            line(ui, ox, oy, s, px, py, point.0, point.1, thickness, color);
        }
        previous = Some(point);
    }
}

#[allow(clippy::too_many_arguments)]
fn arc(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    start: f32,
    end: f32,
    thickness: f32,
    color: u32,
) {
    let segments = 14;
    let mut previous = None;
    for i in 0..=segments {
        let angle = start + (end - start) * i as f32 / segments as f32;
        let point = (cx + angle.cos() * rx, cy + angle.sin() * ry);
        if let Some((px, py)) = previous {
            line(ui, ox, oy, s, px, py, point.0, point.1, thickness, color);
        }
        previous = Some(point);
    }
}

#[allow(clippy::too_many_arguments)]
fn chevron(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    cx: f32,
    cy: f32,
    width: f32,
    height: f32,
    up: bool,
    thickness: f32,
    color: u32,
) {
    let tip = if up {
        cy - height / 2.0
    } else {
        cy + height / 2.0
    };
    let base = if up {
        cy + height / 2.0
    } else {
        cy - height / 2.0
    };
    line(
        ui,
        ox,
        oy,
        s,
        cx - width / 2.0,
        base,
        cx,
        tip,
        thickness,
        color,
    );
    line(
        ui,
        ox,
        oy,
        s,
        cx,
        tip,
        cx + width / 2.0,
        base,
        thickness,
        color,
    );
}

#[allow(clippy::too_many_arguments)]
fn logo(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    cx: f32,
    cy: f32,
    scale: f32,
    top: u32,
    bottom: u32,
) {
    let point = |x: f32, y: f32| [ox + (cx + x * scale) * s, oy + (cy + y * scale) * s];
    raw_quad(
        ui,
        [
            point(-17.0, -16.0),
            point(-9.0, -16.0),
            point(-13.0, 16.0),
            point(-21.0, 16.0),
        ],
        top,
    );
    raw_quad(
        ui,
        [
            point(-10.0, -16.0),
            point(-2.0, -16.0),
            point(21.0, 16.0),
            point(13.0, 16.0),
        ],
        top,
    );
    raw_quad(
        ui,
        [
            point(9.0, -16.0),
            point(17.0, -16.0),
            point(13.0, 16.0),
            point(5.0, 16.0),
        ],
        bottom,
    );
}

#[allow(clippy::too_many_arguments)]
fn category_icon(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    index: usize,
    cx: f32,
    cy: f32,
    size: f32,
    color: u32,
) {
    match index {
        0 => {
            let h = size * 0.38;
            line(ui, ox, oy, s, cx - h, cy + h, cx + h, cy - h, 1.8, color);
            line(ui, ox, oy, s, cx - h, cy - h, cx + h, cy + h, 1.8, color);
            line(
                ui,
                ox,
                oy,
                s,
                cx - h * 0.86,
                cy + h * 0.34,
                cx - h * 0.34,
                cy + h * 0.86,
                1.6,
                color,
            );
            line(
                ui,
                ox,
                oy,
                s,
                cx + h * 0.86,
                cy + h * 0.34,
                cx + h * 0.34,
                cy + h * 0.86,
                1.6,
                color,
            );
            disc(ui, ox, oy, s, cx - h * 1.08, cy + h * 1.08, 1.5, color);
            disc(ui, ox, oy, s, cx + h * 1.08, cy + h * 1.08, 1.5, color);
        }
        1 => {
            let h = size * 0.36;
            line(ui, ox, oy, s, cx, cy - h, cx, cy + h, 1.8, color);
            line(ui, ox, oy, s, cx - h, cy, cx + h, cy, 1.8, color);
            line(
                ui,
                ox,
                oy,
                s,
                cx - h * 0.44,
                cy - h * 0.52,
                cx,
                cy - h,
                1.5,
                color,
            );
            line(
                ui,
                ox,
                oy,
                s,
                cx + h * 0.44,
                cy - h * 0.52,
                cx,
                cy - h,
                1.5,
                color,
            );
            line(
                ui,
                ox,
                oy,
                s,
                cx - h * 0.44,
                cy + h * 0.52,
                cx,
                cy + h,
                1.5,
                color,
            );
            line(
                ui,
                ox,
                oy,
                s,
                cx + h * 0.44,
                cy + h * 0.52,
                cx,
                cy + h,
                1.5,
                color,
            );
            line(
                ui,
                ox,
                oy,
                s,
                cx - h * 0.52,
                cy - h * 0.44,
                cx - h,
                cy,
                1.5,
                color,
            );
            line(
                ui,
                ox,
                oy,
                s,
                cx - h * 0.52,
                cy + h * 0.44,
                cx - h,
                cy,
                1.5,
                color,
            );
            line(
                ui,
                ox,
                oy,
                s,
                cx + h * 0.52,
                cy - h * 0.44,
                cx + h,
                cy,
                1.5,
                color,
            );
            line(
                ui,
                ox,
                oy,
                s,
                cx + h * 0.52,
                cy + h * 0.44,
                cx + h,
                cy,
                1.5,
                color,
            );
        }
        2 => {
            let rx = size * 0.52;
            let ry = size * 0.33;
            arc(
                ui,
                ox,
                oy,
                s,
                cx,
                cy,
                rx,
                ry,
                std::f32::consts::PI,
                std::f32::consts::TAU,
                1.6,
                color,
            );
            arc(
                ui,
                ox,
                oy,
                s,
                cx,
                cy,
                rx,
                ry,
                0.0,
                std::f32::consts::PI,
                1.6,
                color,
            );
            disc(ui, ox, oy, s, cx, cy, size * 0.13, color);
        }
        3 => {
            ring(ui, ox, oy, s, cx, cy - size * 0.24, size * 0.19, 1.6, color);
            arc(
                ui,
                ox,
                oy,
                s,
                cx,
                cy + size * 0.42,
                size * 0.4,
                size * 0.34,
                std::f32::consts::PI,
                std::f32::consts::TAU,
                1.6,
                color,
            );
        }
        _ => {
            let left = (cx - size * 0.3, cy);
            let top = (cx + size * 0.3, cy - size * 0.32);
            let bottom = (cx + size * 0.3, cy + size * 0.32);
            line(ui, ox, oy, s, left.0, left.1, top.0, top.1, 1.4, color);
            line(
                ui, ox, oy, s, left.0, left.1, bottom.0, bottom.1, 1.4, color,
            );
            ring(ui, ox, oy, s, left.0, left.1, size * 0.16, 1.5, color);
            ring(ui, ox, oy, s, top.0, top.1, size * 0.16, 1.5, color);
            ring(ui, ox, oy, s, bottom.0, bottom.1, size * 0.16, 1.5, color);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn manager_icon(
    ui: &mut Ui<'_>,
    ox: f32,
    oy: f32,
    s: f32,
    index: usize,
    cx: f32,
    cy: f32,
    size: f32,
    color: u32,
) {
    match index {
        0 => {
            ring(ui, ox, oy, s, cx, cy, size * 0.26, 1.5, color);
            for tooth in 0..8 {
                let angle = std::f32::consts::TAU * tooth as f32 / 8.0;
                line(
                    ui,
                    ox,
                    oy,
                    s,
                    cx + angle.cos() * size * 0.35,
                    cy + angle.sin() * size * 0.35,
                    cx + angle.cos() * size * 0.5,
                    cy + angle.sin() * size * 0.5,
                    1.5,
                    color,
                );
            }
        }
        1 => {
            bordered(
                ui,
                ox,
                oy,
                s,
                cx - size * 0.38,
                cy - size * 0.08,
                size * 0.76,
                size * 0.56,
                3.0,
                color,
            );
            arc(
                ui,
                ox,
                oy,
                s,
                cx,
                cy - size * 0.08,
                size * 0.25,
                size * 0.25,
                std::f32::consts::PI,
                std::f32::consts::TAU,
                1.5,
                color,
            );
        }
        _ => {
            ring(ui, ox, oy, s, cx - size * 0.18, cy, size * 0.24, 1.5, color);
            ring(ui, ox, oy, s, cx + size * 0.18, cy, size * 0.24, 1.5, color);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn keyboard_icon(ui: &mut Ui<'_>, ox: f32, oy: f32, s: f32, cx: f32, cy: f32, color: u32) {
    bordered(ui, ox, oy, s, cx - 8.0, cy - 5.0, 16.0, 10.0, 2.4, color);
    for key in 0..3 {
        fill(
            ui,
            ox,
            oy,
            s,
            cx - 5.4 + key as f32 * 4.2,
            cy - 2.3,
            2.0,
            1.6,
            color,
        );
    }
    fill(ui, ox, oy, s, cx - 3.6, cy + 1.4, 7.2, 1.5, color);
}

fn search_icon(ui: &mut Ui<'_>, ox: f32, oy: f32, s: f32, cx: f32, cy: f32, color: u32) {
    ring(ui, ox, oy, s, cx - 1.0, cy - 1.0, 4.4, 1.5, color);
    line(
        ui,
        ox,
        oy,
        s,
        cx + 2.2,
        cy + 2.2,
        cx + 5.6,
        cy + 5.6,
        1.7,
        color,
    );
}

fn dots_icon(ui: &mut Ui<'_>, ox: f32, oy: f32, s: f32, cx: f32, cy: f32, color: u32) {
    for i in -1..=1 {
        disc(ui, ox, oy, s, cx + i as f32 * 5.4, cy, 1.6, color);
    }
}

fn hamburger_icon(ui: &mut Ui<'_>, ox: f32, oy: f32, s: f32, cx: f32, cy: f32, color: u32) {
    for i in -1..=1 {
        let y = cy + i as f32 * 4.6;
        line(ui, ox, oy, s, cx - 8.0, y, cx + 8.0, y, 1.6, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn layout_fills_the_launcher_surface_at_720p() {
        let (x, y, scale) = layout(640.0, 360.0, 2.0);
        assert!(close(scale, 1.0));
        assert!(close(
            x,
            (1280.0 / density(640.0, 360.0, 2.0) - DESIGN_W) / 2.0
        ));
        assert!(close(y, 0.0));
        assert!(close(density(640.0, 360.0, 2.0), 720.0 / 700.0));
    }

    #[test]
    fn layout_scales_down_for_small_windows() {
        let (x, y, scale) = layout(320.0, 240.0, 1.0);
        assert!(close(x, 0.0));
        assert!(close(scale, 320.0 / DESIGN_W));
        assert!(close(y, (240.0 - DESIGN_H * scale) / 2.0));
    }

    #[test]
    fn cursor_maps_to_design_units() {
        let (x, y) = to_design((320.0, 180.0), 640.0, 360.0, 2.0);
        let (offset_x, offset_y, scale) = layout(640.0, 360.0, 2.0);
        let density = density(640.0, 360.0, 2.0);
        assert!(close(x, (320.0 * 2.0 / density - offset_x) / scale));
        assert!(close(y, (180.0 * 2.0 / density - offset_y) / scale));
    }

    #[test]
    fn hitboxes_cover_rows_and_controls() {
        let row = module_row(1, 2);
        assert!(row.contains(column_x(1) + 10.0, ROW_TOP + 2.0 * ROW_H + 10.0));
        let toggle = toggle_row(0, 0);
        assert!(toggle.contains(toggle.center().0, toggle.center().1));
        let dots = dots_hit(0, 0);
        assert!(dots.contains(dots.center().0, dots.center().1));
        assert!(CARD.contains(BIND_BOX.center().0, BIND_BOX.center().1));
        assert!(CARD.contains(DONE.center().0, DONE.center().1));
        assert!(CARD.contains(SLIDER_HIT.center().0, SLIDER_HIT.center().1));
    }
}
