// Put testable mod logic here. This file is both the standalone Cargo library
// and the injected `crate::game::example_mod` module; no generated copy exists.

use std::sync::atomic::{AtomicBool, Ordering};

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

pub fn origin(width: f32, height: f32) -> (f32, f32) {
    (
        (width - 220.0).max(0.0) / 2.0,
        (height - 130.0).max(0.0) / 2.0,
    )
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
    fn panel_origin_centers_and_clamps_to_screen() {
        assert_eq!(origin(420.0, 330.0), (100.0, 100.0));
        assert_eq!(origin(100.0, 50.0), (0.0, 0.0));
    }
}
