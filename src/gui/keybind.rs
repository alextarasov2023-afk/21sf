// keybind.rs — бинд клавиши.
pub struct Keybind {
    pub key: &'static str,
    pub pressed: bool,
}

impl Keybind {
    pub fn new(key: &'static str) -> Self {
        Self { key, pressed: false }
    }
}