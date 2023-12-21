use std::str::FromStr;
use crate::board::Board;
use crate::board::builder::BoardBuilder;
use crate::castling::{CastlingRights, WHITE_KING_SIDE, WHITE_QUEEN_SIDE, BLACK_KING_SIDE, BLACK_QUEEN_SIDE};
use crate::color::Color;
use crate::file::File;
use crate::piece::Piece;
use crate::rank::Rank;
use crate::square::Square;

#[derive(Debug)]
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
            return Err(BoardParseError::InvalidFenString);
        }

        // Parse board
        let board = Self::parse_board(fields[0])
            .map_err(|_| BoardParseError::InvalidBoard).unwrap();

        // Parse side to move
        let side_to_move = Self::parse_side_to_move(fields[1])
            .map_err(|_| BoardParseError::InvalidSideToMove).unwrap();

        // Parse castling rights
        let castling_rights = Self::parse_castling_rights(fields[2])
            .map_err(|_| BoardParseError::InvalidCastlingRights).unwrap();

        // Parse en passant target square
        let en_passant_target_square = Self::parse_en_passant_target_square(fields[3])
            .map_err(|_| BoardParseError::InvalidEnPassantTargetSquare).unwrap();

        // Parse halfmove clock
        let halfmove_clock = Self::parse_halfmove_clock(fields[4])
            .map_err(|_| BoardParseError::InvalidHalfmoveClock).unwrap();

        // Parse fullmove counter
        let fullmove_counter = Self::parse_fullmove_counter(fields[5])
            .map_err(|_| BoardParseError::InvalidFullmoveCounter).unwrap();

        let mut board_builder: BoardBuilder = BoardBuilder::new().
            set_board(board)
            .set_side_to_move(side_to_move)
            .set_castling_rights(castling_rights)
            .en_passant(en_passant_target_square)
            .halfmove_clock(halfmove_clock)
            .fullmove_counter(fullmove_counter);
        let board: Board = board_builder.build();
        Ok(board)
    }

    pub fn parse_shredder_fen(fen: &str) -> Result<Self, BoardParseError> {
        todo!()
    }

    fn parse_board(str: &str) -> Result<[Option<(Piece, Color)>; 64], ()> {
        let mut board: [Option<(Piece, Color)>; 64] = [None; 64];
        for (rank, fileStr) in str.rsplit('/').enumerate() {
            let rank = Rank::from_index(rank);
            let mut file = 0;
            for c in fileStr.chars() {
                if let Some(offset) = c.to_digit(10) {
                    file += offset as usize;
                } else {
                    let piece: Piece = Piece::from_char(c.to_ascii_lowercase()).map_err(|_| ())?;
                    let color: Color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let square = Square::create_square(rank, File::from_index(file)).to_index();
                    board[square] = Some((piece, color));
                    file += 1;
                }
            }
        }
        Ok(board)
    }

    fn parse_side_to_move(str: &str) -> Result<Color, ()> {
        str.parse::<Color>()
    }

    fn parse_castling_rights(str: &str) -> Result<CastlingRights, ()> {
        let mut castling_rights = CastlingRights::empty();
        if str.contains('K') {
            castling_rights |= CastlingRights::from_bytes(WHITE_KING_SIDE);
        }
        if str.contains('Q') {
            castling_rights |= CastlingRights::from_bytes(WHITE_QUEEN_SIDE);
        }
        if str.contains('k') {
            castling_rights |= CastlingRights::from_bytes(BLACK_KING_SIDE);
        }
        if str.contains('q') {
            castling_rights |= CastlingRights::from_bytes(BLACK_QUEEN_SIDE);
        }
        Ok(castling_rights)
    }

    fn parse_en_passant_target_square(str: &str) -> Result<Option<Square>, ()> {
        if str != "-" {
            let square = str.parse::<Square>().map_err(|_| ()).unwrap();
            // if square.rank() != 3 && square.rank() != 6 {
            //     return Err(());
            // }
            return Ok(Some(square));
        }
        Ok(None)
    }

    fn parse_halfmove_clock(str: &str) -> Result<u8, ()> {
        let halfmove_clock = str.parse::<u8>().map_err(|_| ()).unwrap();
        if halfmove_clock > 100 {
            return Err(());
        }
        Ok(halfmove_clock)
    }

    fn parse_fullmove_counter(str: &str) -> Result<u16, ()> {
        let fullmove_counter = str.parse::<u16>().map_err(|_| ()).unwrap();
        if fullmove_counter == 0 {
            return Err(());
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