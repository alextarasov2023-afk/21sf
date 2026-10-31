// Put testable mod logic here. This file is both the standalone Cargo library
// and the injected `crate::game::example_mod` module; no generated copy exists.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub const CATEGORIES: [&str; 5] = ["Combat", "Movement", "Visuals", "Player", "Misc"];
pub const MODULES: [[&str; 6]; 5] = [
    [
        "Attack Aura",
        "No Velocity",
        "Trigger Bot",
        "Aim Assist",
        "Auto Explosion",
        "Auto Swap",
    ],
    ["Speed", "Fly", "Sprint", "Step", "No Slow", "Air Jump"],
    [
        "ESP",
        "Chams",
        "Full Bright",
        "Tracers",
        "Name Tags",
        "View Model",
    ],
    [
        "Auto Totem",
        "Auto Eat",
        "Inventory Sort",
        "Fast Use",
        "No Fall",
        "Auto Tool",
    ],
    [
        "Sprint Reset",
        "Tape Mouse",
        "Backtrack",
        "Web Trap",
        "Anti Bot",
        "No Friend Damage",
    ],
];

pub struct GuiState {
    pub category: usize,
    pub enabled: [[bool; 6]; 5],
    pub animation: [[f32; 6]; 5],
    pub binds: [[Option<String>; 6]; 5],
    pub values: [[u8; 6]; 5],
    pub selected: Option<(usize, usize)>,
    pub waiting_bind: bool,
}

pub static GUI: Mutex<GuiState> = Mutex::new(GuiState {
    category: 0,
    enabled: [[false; 6]; 5],
    animation: [[0.0; 6]; 5],
    binds: [const { [const { None }; 6] }; 5],
    values: [[50; 6]; 5],
    selected: None,
    waiting_bind: false,
});

impl GuiState {
    pub fn activate_bind(&mut self, key: &str) -> usize {
        let mut count = 0;
        for category in 0..5 {
            for module in 0..6 {
                if self.binds[category][module].as_deref() == Some(key) {
                    self.enabled[category][module] = !self.enabled[category][module];
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn layout(width: f32, height: f32) -> (f32, f32, f32) {
    let scale = (width / 600.0).min(height / 360.0).clamp(0.1, 1.0);
    (
        (width - 600.0 * scale) / 2.0,
        (height - 360.0 * scale) / 2.0,
        scale,
    )
}

static OPEN: AtomicBool = AtomicBool::new(false);

pub fn is_open() -> bool {
    OPEN.load(Ordering::Relaxed)
}

pub fn toggle() -> bool {
    !OPEN.fetch_xor(true, Ordering::Relaxed)
}

pub fn close() {
    OPEN.store(false, Ordering::Relaxed);
}

pub fn block_break_message(pos: [i32; 3]) -> String {
    format!(
        "[example_mod] Breaking block at {}, {}, {}",
        pos[0], pos[1], pos[2]
    )
}

pub fn before_break(pos: [i32; 3]) {
    eprintln!("{}", block_break_message(pos));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_preserves_signed_block_coordinates() {
        assert_eq!(
            block_break_message([-3, 64, 12]),
            "[example_mod] Breaking block at -3, 64, 12"
        );
    }

    #[test]
    fn layout_centers_and_scales_down() {
        assert_eq!(layout(800.0, 600.0), (100.0, 120.0, 1.0));
        assert_eq!(layout(300.0, 180.0), (0.0, 0.0, 0.5));
    }

    #[test]
    fn bind_toggles_matching_placeholders_only() {
        let mut state = GuiState {
            category: 0,
            enabled: [[false; 6]; 5],
            animation: [[0.0; 6]; 5],
            binds: [const { [const { None }; 6] }; 5],
            values: [[50; 6]; 5],
            selected: None,
            waiting_bind: false,
        };
        state.binds[0][1] = Some("KeyK".into());
        state.binds[2][3] = Some("KeyK".into());
        assert_eq!(state.activate_bind("KeyJ"), 0);
        assert_eq!(state.activate_bind("KeyK"), 2);
        assert!(state.enabled[0][1] && state.enabled[2][3]);
        assert!(!state.enabled[0][0]);
        assert_eq!(state.activate_bind("KeyK"), 2);
        assert!(!state.enabled[0][1] && !state.enabled[2][3]);
    }
}
