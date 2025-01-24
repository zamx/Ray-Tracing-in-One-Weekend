use std::fmt::Debug;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }

    pub fn red() -> Color {
        Color { r: 255, g: 0, b: 0 }
    }

    pub fn green() -> Color {
        Color { r: 0, g: 255, b: 0 }
    }

    pub fn blue() -> Color {
        Color { r: 0, g: 0, b: 255 }
    }
}