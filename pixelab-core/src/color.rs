#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 0,
        }
    }
    pub fn red() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }
    pub fn green() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 255,
        }
    }
    pub fn blue() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255,
        }
    }
    pub fn white() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
        }
    }
    pub fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }
    pub fn gray() -> Self {
        Self {
            red: 128,
            green: 128,
            blue: 128,
            alpha: 255,
        }
    }
    pub fn orange() -> Self {
        Self {
            red: 255,
            green: 160,
            blue: 0,
            alpha: 255,
        }
    }
    pub fn yellow() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 0,
            alpha: 255,
        }
    }
    pub fn to_u16_color(&self) -> u16 {
        ((self.red as u16) << 11) | ((self.green as u16) << 5) | ((self.blue as u16) << 0)
    }
    pub fn to_u32_color(&self) -> u32 {
        ((self.alpha as u32) << 24)
            | ((self.red as u32) << 16)
            | ((self.green as u32) << 8)
            | (self.blue as u32)
    }
}
