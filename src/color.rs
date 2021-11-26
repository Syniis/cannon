use std::fmt;
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

pub const NUM_COLORS: usize = 2;
pub const ALL_COLORS: [Color; 2] = [Color::White, Color::Black];

impl Color {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "{}", "w"),
            Color::Black => write!(f, "{}", "b"),
        }
    }
}
