#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Area {
    pub width: u16,
    pub height: u16,
}
impl Area {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}
impl Default for Area {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}
impl core::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {} height: {}", self.width, self.height)
    }
}
