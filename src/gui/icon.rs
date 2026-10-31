// icon.rs — иконка (placeholder).
pub struct Icon {
    pub name: &'static str,
    pub data: &'static [u8],
}

impl Icon {
    pub fn new(name: &'static str) -> Self {
        Self { name, data: &[] }
    }
}