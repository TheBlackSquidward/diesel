use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use crate::file::{File, NUM_FILES};
use crate::rank::{NUM_RANKS, Rank};

#[derive(PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
pub struct Square(u8);

pub const NUM_SQUARES: usize = 64;

impl Default for Square {
    fn default() -> Self {
        Square(0)
    }
}

impl Square {
    pub fn new(square: u8) -> Self {
        Square(square & 63)
    }

    pub fn create_square(rank: Rank, file: File) -> Self {
        Square::new((rank.to_index() as u8) << 3 ^ (file.to_index() as u8))
    }

    pub fn get_rank(&self) -> Rank {
        Rank::from_index((self.0 >> 3) as usize)
    }

    pub fn get_file(&self) -> File {
        File::from_index((self.0 & 7) as usize)
    }

    pub fn up(&self) -> Option<Square> {
        if self.get_rank() != Rank::Eighth {
            Some(Square::new(self.to_int() + 8))
        } else {
            None
        }
    }

    pub fn down(&self) -> Option<Square> {
        if self.get_rank() != Rank::First {
            Some(Square::new(self.to_int() - 8))
        } else {
            None
        }
    }

    pub fn offset(&self, file_offset: i32, rank_offset: i32) -> Option<Square> {
        let new_file = (self.get_file() as i32 + file_offset) as usize;
        let new_rank = (self.get_rank() as i32 + rank_offset) as usize;

        if new_file < NUM_FILES && new_rank < NUM_RANKS {
            Some(Square::create_square(Rank::from_index(new_rank), File::from_index(new_file)))
        } else {
            None
        }
    }

    pub fn to_int(&self) -> u8 {
        self.0
    }

    pub fn to_index(&self) -> usize {
        self.0 as usize
    }
}

impl FromStr for Square {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(());
        }
        let chars: Vec<char> = s.chars().collect::<Vec<char>>();

        let file = File::from_char(chars[0]).unwrap();
        let rank = Rank::from_char(chars[1]).unwrap();
        Ok(Square::create_square(rank, file))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (('a' as u8) + (self.0 & 7)) as char,
            (('1' as u8) + (self.0 >> 3)) as char
        )
    }
}

pub const ALL_SQUARES: [Square; NUM_SQUARES] = [
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
];

pub const A1 : Square = Square(0);
pub const B1 : Square = Square(1);
pub const C1 : Square = Square(2);
pub const D1 : Square = Square(3);
pub const E1 : Square = Square(4);
pub const F1 : Square = Square(5);
pub const G1 : Square = Square(6);
pub const H1 : Square = Square(7);

pub const A2 : Square = Square(8);
pub const B2 : Square = Square(9);
pub const C2 : Square = Square(10);
pub const D2 : Square = Square(11);
pub const E2 : Square = Square(12);
pub const F2 : Square = Square(13);
pub const G2 : Square = Square(14);
pub const H2 : Square = Square(15);

pub const A3 : Square = Square(16);
pub const B3 : Square = Square(17);
pub const C3 : Square = Square(18);
pub const D3 : Square = Square(19);
pub const E3 : Square = Square(20);
pub const F3 : Square = Square(21);
pub const G3 : Square = Square(22);
pub const H3 : Square = Square(23);

pub const A4 : Square = Square(24);
pub const B4 : Square = Square(25);
pub const C4 : Square = Square(26);
pub const D4 : Square = Square(27);
pub const E4 : Square = Square(28);
pub const F4 : Square = Square(29);
pub const G4 : Square = Square(30);
pub const H4 : Square = Square(31);

pub const A5 : Square = Square(32);
pub const B5 : Square = Square(33);
pub const C5 : Square = Square(34);
pub const D5 : Square = Square(35);
pub const E5 : Square = Square(36);
pub const F5 : Square = Square(37);
pub const G5 : Square = Square(38);
pub const H5 : Square = Square(39);

pub const A6 : Square = Square(40);
pub const B6 : Square = Square(41);
pub const C6 : Square = Square(42);
pub const D6 : Square = Square(43);
pub const E6 : Square = Square(44);
pub const F6 : Square = Square(45);
pub const G6 : Square = Square(46);
pub const H6 : Square = Square(47);

pub const A7 : Square = Square(48);
pub const B7 : Square = Square(49);
pub const C7 : Square = Square(50);
pub const D7 : Square = Square(51);
pub const E7 : Square = Square(52);
pub const F7 : Square = Square(53);
pub const G7 : Square = Square(54);
pub const H7 : Square = Square(55);

pub const A8 : Square = Square(56);
pub const B8 : Square = Square(57);
pub const C8 : Square = Square(58);
pub const D8 : Square = Square(59);
pub const E8 : Square = Square(60);
pub const F8 : Square = Square(61);
pub const G8 : Square = Square(62);
pub const H8 : Square = Square(63);

