#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    LightBrown,
    White,
}

#[inline(always)]
pub const fn entry_color(fg: Color, bg: Color) -> u8 {
    return fg as u8 | (bg as u8) << 4;
}

#[inline(always)]
pub fn entry(uc: u8, color: u8) -> u16 {
    (uc as u16) | ((color as u16) << 8)
}

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 25;
