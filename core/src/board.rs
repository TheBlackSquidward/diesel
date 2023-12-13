use std::fmt;
use std::str::FromStr;

use crate::bitboard::{BitBoard, EMPTY_BITBOARD};
use crate::color::{Color, NUM_COLORS};
use crate::file::File;
use crate::piece::{NUM_PIECES, Piece};
use crate::rank::Rank;
use crate::square::Square;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Board {
    pub bitboards_pieces: [[BitBoard; NUM_PIECES]; NUM_COLORS],
    pub bitboards_side: [BitBoard; NUM_COLORS],
    pub to_move: Color,
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitboards_pieces: [[EMPTY_BITBOARD; NUM_PIECES]; NUM_COLORS],
            bitboards_side: [EMPTY_BITBOARD; NUM_COLORS],
            to_move: Color::White,
        }
    }

    pub fn set_piece_at(&mut self, square: Square, piece: Piece, color: Color) {
        let bitboard_mask = BitBoard::new(1u64 << square.to_int());

        self.bitboards_pieces[color.to_index()][piece.to_index()] |= bitboard_mask;
        self.bitboards_side[color.to_index()] |= bitboard_mask;
    }

    pub fn get_piece_at(&self, square: Square) -> Option<(Piece, Color)> {
        let bitboard_mask = BitBoard::new(1u64 << square.to_int());
        for &color in &[Color::White, Color::Black] {
            for &piece in &[Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King] {
                if self.bitboards_pieces[color.to_index()][piece.to_index()] & bitboard_mask != EMPTY_BITBOARD {
                    return Some((piece, color));
                }
            }
        }
        None
    }

    pub fn get_pawn_bitboard(&self, color: Color) -> BitBoard {
        self.bitboards_pieces[color.to_index()][Piece::Pawn.to_index()]
    }

    pub fn get_white_pawn_bitboard(&self) -> BitBoard {
        self.get_pawn_bitboard(Color::White)
    }

pub fn get_black_pawn_bitboard(&self) -> BitBoard {
        self.get_pawn_bitboard(Color::Black)
    }

    pub fn get_knight_bitboard(&self, color: Color) -> BitBoard {
        self.bitboards_pieces[color.to_index()][Piece::Knight.to_index()]
    }

    pub fn get_white_knight_bitboard(&self) -> BitBoard {
        self.get_knight_bitboard(Color::White)
    }

    pub fn get_black_knight_bitboard(&self) -> BitBoard {
        self.get_knight_bitboard(Color::Black)
    }

    pub fn get_bishop_bitboard(&self, color: Color) -> BitBoard {
        self.bitboards_pieces[color.to_index()][Piece::Bishop.to_index()]
    }

    pub fn get_white_bishop_bitboard(&self) -> BitBoard {
        self.get_bishop_bitboard(Color::White)
    }

    pub fn get_black_bishop_bitboard(&self) -> BitBoard {
        self.get_bishop_bitboard(Color::Black)
    }

    pub fn get_rook_bitboard(&self, color: Color) -> BitBoard {
        self.bitboards_pieces[color.to_index()][Piece::Rook.to_index()]
    }

    pub fn get_white_rook_bitboard(&self) -> BitBoard {
        self.get_rook_bitboard(Color::White)
    }

    pub fn get_black_rook_bitboard(&self) -> BitBoard {
        self.get_rook_bitboard(Color::Black)
    }

    pub fn get_queen_bitboard(&self, color: Color) -> BitBoard {
        self.bitboards_pieces[color.to_index()][Piece::Queen.to_index()]
    }

    pub fn get_white_queen_bitboard(&self) -> BitBoard {
        self.get_queen_bitboard(Color::White)
    }

    pub fn get_black_queen_bitboard(&self) -> BitBoard {
        self.get_queen_bitboard(Color::Black)
    }

    pub fn get_king_bitboard(&self, color: Color) -> BitBoard {
        self.bitboards_pieces[color.to_index()][Piece::King.to_index()]
    }

    pub fn get_white_king_bitboard(&self) -> BitBoard {
        self.get_king_bitboard(Color::White)
    }

    pub fn get_black_king_bitboard(&self) -> BitBoard {
        self.get_king_bitboard(Color::Black)
    }

    pub fn get_pieces_bitboard(&self, color: Color) -> BitBoard {
        self.bitboards_side[color.to_index()]
    }

    pub fn get_white_pieces_bitboard(&self) -> BitBoard {
        self.get_pieces_bitboard(Color::White)
    }

    pub fn get_black_pieces_bitboard(&self) -> BitBoard {
        self.get_pieces_bitboard(Color::Black)
    }

    pub fn get_occupancy_bitboard(&self) -> BitBoard {
        self.bitboards_side[Color::White.to_index()] | self.bitboards_side[Color::Black.to_index()]
    }

    pub fn get_to_move(&self) -> Color {
        self.to_move
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  a b c d e f g h")?;
        writeln!(f, " -----------------")?;

        for rank in (0..8).rev() {
            write!(f, "{}|", rank + 1)?;

            for file in 0..8 {
                let square = Square::create_square(Rank::from_index(rank), File::from_index(file));
                let piece = self.get_piece_at(square);

                write!(f, " ")?;

                match piece {
                    Some((piece, color)) => {
                        let piece_str = piece.to_string(color);
                        write!(f, "{}", piece_str)?;
                    }
                    None => {
                        write!(f, ".")?;
                    }
                }
            }

            writeln!(f, "|")?;
        }

        writeln!(f, " -----------------")?;
        write!(f, "  a b c d e f g h")
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Failed to parse default FEN string")
    }
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        let mut file = 0;
        let mut rank = 7;

        for c in s.chars() {
            match c {
                '1'..='8' => {
                    let empty_squares = c.to_digit(10).unwrap() as usize;
                    file += empty_squares;
                }
                '/' => {
                    file = 0;
                    rank -= 1;
                }
                'r' | 'n' | 'b' | 'q' | 'k' | 'p' | 'R' | 'N' | 'B' | 'Q' | 'K' | 'P' => {
                    let square = Square::create_square(Rank::from_index(rank), File::from_index(file));
                    let color = if c.is_lowercase() { Color::Black } else { Color::White };

                    match c.to_ascii_lowercase() {
                        'r' => board.set_piece_at(square, Piece::Rook, color),
                        'n' => board.set_piece_at(square, Piece::Knight, color),
                        'b' => board.set_piece_at(square, Piece::Bishop, color),
                        'q' => board.set_piece_at(square, Piece::Queen, color),
                        'k' => board.set_piece_at(square, Piece::King, color),
                        'p' => board.set_piece_at(square, Piece::Pawn, color),
                        _ => return Err("Invalid FEN string"),
                    }

                    file += 1;
                }
                ' ' => break,
                _ => return Err("Invalid FEN string"),
            }
        }

        Ok(board)
    }
}