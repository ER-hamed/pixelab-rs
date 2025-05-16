#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}
impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
impl core::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}
