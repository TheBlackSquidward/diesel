use std::str::FromStr;
use crate::board::Board;
use crate::color::Color;
use crate::square::Square;

pub enum BoardParseError {
    InvalidFenString,
    InvalidBoard,
    InvalidSideToMove,
    InvalidCastlingRights,
    InvalidEnPassantTargetSquare,
    InvalidHalfmoveClock,
    InvalidFullmoveCounter,
}

impl Board {
    pub fn parse_fen(fen: &str) -> Result<Self, BoardParseError> {
        let fields: Vec<&str> = fen.split(' ').collect();
        if fields.len() != 6 {
            Err(BoardParseError::InvalidFenString)
        }

        // Parse board

        // Parse side to move
        let side_to_move = Self::parse_side_to_move(fields[1])
            .map_err(|_| BoardParseError::InvalidSideToMove).unwrap();

        // Parse casting availability

        // Parse en passant target square

        // Parse halfmove clock
        let halfmove_clock = Self::parse_halfmove_clock(fields[4])
            .map_err(|_| BoardParseError::InvalidHalfmoveClock).unwrap();

        // Parse fullmove counter
        let fullmove_counter = Self::parse_fullmove_counter(fields[5])
            .map_err(|_| BoardParseError::InvalidFullmoveCounter).unwrap();

        let board = Board::new();
        Ok(board)
    }

    pub fn parse_shredder_fen(fen: &str) -> Result<Self, BoardParseError> {
        todo!()
    }

    fn parse_board(str: &str) -> Result<(), ()> {

    }

    fn parse_side_to_move(str: &str) -> Result<Color, ()> {
        str.parse::<Color>()
    }

    fn parse_castling_availability(str: &str) -> Result<(), ()> {

    }

    fn parse_en_passant_target_square(str: &str) -> Result<Option<Square>, ()> {
        if str != '-' {

        }
        Ok(None)
    }

    fn parse_halfmove_clock(str: &str) -> Result<u8, ()> {
        let halfmove_clock = str.parse::<u8>().map_err(|_| ()).unwrap();
        if halfmove_clock > 100 {
            Err(())
        }
        Ok(halfmove_clock)
    }

    fn parse_fullmove_counter(str: &str) -> Result<u16, ()> {
        let fullmove_counter = str.parse::<u16>().map_err(|_| ()).unwrap();
        if fullmove_counter == 0 {
            Err(())
        }
        Ok(fullmove_counter)
    }
}

impl FromStr for Board {
    type Err = BoardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse_fen(s) {
            Ok(board) => Ok(board),
            Err(BoardParseError::InvalidCastlingRights) => Self::parse_shredder_fen(s),
            Err(error) => Err(error),
        }
    }
}