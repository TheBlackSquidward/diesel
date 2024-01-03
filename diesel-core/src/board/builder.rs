use crate::board::Board;
use crate::castling::CastlingRights;
use crate::color::Color;
use crate::piece::Piece;
use crate::square::Square;

pub struct BoardBuilder {
    // Board
    board: [Option<(Piece, Color)>; 64],
    side_to_move: Color,
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    halfmove_clock: u8,
    fullmove_counter: u16,
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self {
            board: [None; 64],
            side_to_move: Color::White,
            castling_rights: CastlingRights::empty(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_counter: 0,
        }
    }

    pub fn get_board(&self) -> [Option<(Piece, Color)>; 64] {
        self.board
    }

    pub fn set_board(mut self, board: [Option<(Piece, Color)>; 64]) -> Self {
        self.board = board;
        self
    }

    pub fn get_side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn set_side_to_move(mut self, side_to_move: Color) -> Self {
        self.side_to_move = side_to_move;
        self
    }

    pub fn get_castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    pub fn set_castling_rights(mut self, castling_rights: CastlingRights) -> Self {
        self.castling_rights = castling_rights;
        self
    }

    pub fn get_en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    pub fn en_passant(mut self, en_passant: Option<Square>) -> Self {
        self.en_passant = en_passant;
        self
    }

    pub fn get_halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    pub fn halfmove_clock(mut self, halfmove_clock: u8) -> Self {
        self.halfmove_clock = halfmove_clock;
        self
    }

    pub fn get_fullmove_counter(&self) -> u16 {
        self.fullmove_counter
    }

    pub fn fullmove_counter(mut self, fullmove_counter: u16) -> Self {
        self.fullmove_counter = fullmove_counter;
        self
    }

    pub fn build(&mut self) -> Board {
        Board::try_from(self).unwrap()
    }
}

impl Default for BoardBuilder {
    fn default() -> Self {
        todo!()
    }
}