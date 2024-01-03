use std::ops::{BitOr, BitOrAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CastlingRights(u8);

pub const WHITE_KING_SIDE: u8 = 0b00000001;
pub const WHITE_QUEEN_SIDE: u8 = 0b00000010;
pub const BLACK_KING_SIDE: u8 = 0b00000100;
pub const BLACK_QUEEN_SIDE: u8 = 0b00001000;

pub const NO_CASTLING: u8 = 0b00000000;
pub const WHITE_CASTLING: u8 = WHITE_KING_SIDE | WHITE_QUEEN_SIDE;
pub const BLACK_CASTLING: u8 = BLACK_KING_SIDE | BLACK_QUEEN_SIDE;

impl CastlingRights {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn from_bytes(bytes: u8) -> Self {
        Self(bytes)
    }
}

impl BitOr for CastlingRights {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for CastlingRights {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}