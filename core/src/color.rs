use std::ops::Not;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Color {
    White,
    Black,
}

pub const NUM_COLORS: usize = 2;

pub const ALL_COLORS: [Color; NUM_COLORS] = [Color::White, Color::Black];

impl Color {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
