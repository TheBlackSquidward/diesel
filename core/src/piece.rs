use std::fmt;
use crate::color::Color;

#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Debug, Hash)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const NUM_PIECES: usize = 6;

pub const ALL_PIECES: [Piece; NUM_PIECES] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

impl Piece {
    pub fn to_string(&self, color: Color) -> String {
        let piece = format!("{}", self);
        if color == Color::White {
            piece.to_uppercase()
        } else {
            piece
        }
    }
    pub fn to_fancy_string(&self, color: Color) -> String {
        match color {
            Color::White => match self {
                Piece::Pawn => "♙",
                Piece::Knight => "♘",
                Piece::Bishop => "♗",
                Piece::Rook => "♖",
                Piece::Queen => "♕",
                Piece::King => "♔",
            },
            Color::Black => match self {
                Piece::Pawn => "♟",
                Piece::Knight => "♞",
                Piece::Bishop => "♝",
                Piece::Rook => "♜",
                Piece::Queen => "♛",
                Piece::King => "♚",
            },
        }.to_string()
    }

    pub fn from_char(c: char) -> Result<Self, ()> {
        match c {
            'p' => Ok(Piece::Pawn),
            'n' => Ok(Piece::Knight),
            'b' => Ok(Piece::Bishop),
            'r' => Ok(Piece::Rook),
            'q' => Ok(Piece::Queen),
            'k' => Ok(Piece::King),
            _ => Err(()),
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn => write!(f, "p"),
            Piece::Knight => write!(f, "n"),
            Piece::Bishop => write!(f, "b"),
            Piece::Rook => write!(f, "r"),
            Piece::Queen => write!(f, "q"),
            Piece::King => write!(f, "k"),
        }
    }
}