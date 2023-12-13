use core::fmt;
use std::fmt::Formatter;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BitBoard(pub u64);

pub const EMPTY_BITBOARD: BitBoard = BitBoard(0);

// Logical AND Implementation
impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

// Logical OR Implementation
impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

// Logical XOR Implementation
impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

// Logical NOT Implementation
impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

// Logical AND Assignment Implementation
impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

// Logical OR Assignment Implementation
impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

// Logical XOR Assignment Implementation
impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitBoard {
    pub fn new(value: u64) -> Self {
        BitBoard(value)
    }

    pub fn from_square(sq: Square) -> BitBoard {
        BitBoard(1u64 << sq.to_int())
    }

    pub fn set_bit(&mut self, sq: Square) {
        self.0 |= 1u64 << sq.to_int();
    }

    pub fn count_bits(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  a b c d e f g h")?;
        writeln!(f, " -----------------")?;

        for rank in (0..8).rev() {
            write!(f, "{}|", rank + 1)?;

            for file in 0..8 {
                let square = Square::create_square(Rank::from_index(rank), File::from_index(file));

                if self.0 & BitBoard::from_square(square).0 != 0 {
                    write!(f, " 1")?;
                } else {
                    write!(f, " 0")?;
                }
            }

            writeln!(f, "|")?;
        }

        writeln!(f, " -----------------")?;
        write!(f, "  a b c d e f g h")?;
        writeln!(f, "")?;
        write!(f, "Bitboard: {}", self.0)
    }
}

