use crate::color::Color;
use crate::square::Square;

pub struct BoardBuilder {
    // Board
    pub side_to_move: Color,
    // Castling Rights
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_counter: u16,
}

impl BoardBuilder {

}

impl Default for BoardBuilder {
    fn default() -> Self {
        todo!()
    }
}