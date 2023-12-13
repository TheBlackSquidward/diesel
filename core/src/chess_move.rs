use crate::square::Square;
use crate::piece::Piece;

#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Default, Debug, Hash)]
pub struct ChessMove {
    from: Square,
    to: Square,
    promotion: Option<Piece>,
}

impl ChessMove {
    pub fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        Self { from, to, promotion, }
    }

    pub fn get_from(&self) -> Square {
        self.from
    }

    pub fn get_to(&self) -> Square {
        self.to
    }

    pub fn get_promotion(&self) -> Option<Piece> {
        self.promotion
    }

}