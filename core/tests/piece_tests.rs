use chess_core::piece::{Piece, NUM_PIECES, ALL_PIECES};
use chess_core::color::Color;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_display() {
        assert_eq!(format!("{}", Piece::Pawn), "p");
        assert_eq!(format!("{}", Piece::Knight), "n");
        assert_eq!(format!("{}", Piece::Bishop), "b");
        assert_eq!(format!("{}", Piece::Rook), "r");
        assert_eq!(format!("{}", Piece::Queen), "q");
        assert_eq!(format!("{}", Piece::King), "k");
    }

    #[test]
    fn test_piece_to_string_white() {
        assert_eq!(Piece::Pawn.to_string(Color::White), "P");
        assert_eq!(Piece::Knight.to_string(Color::White), "N");
        assert_eq!(Piece::Bishop.to_string(Color::White), "B");
        assert_eq!(Piece::Rook.to_string(Color::White), "R");
        assert_eq!(Piece::Queen.to_string(Color::White), "Q");
        assert_eq!(Piece::King.to_string(Color::White), "K");
    }

    #[test]
    fn test_piece_to_string_black() {
        assert_eq!(Piece::Pawn.to_string(Color::Black), "p");
        assert_eq!(Piece::Knight.to_string(Color::Black), "n");
        assert_eq!(Piece::Bishop.to_string(Color::Black), "b");
        assert_eq!(Piece::Rook.to_string(Color::Black), "r");
        assert_eq!(Piece::Queen.to_string(Color::Black), "q");
        assert_eq!(Piece::King.to_string(Color::Black), "k");
    }

    #[test]
    fn test_all_pieces() {
        assert_eq!(ALL_PIECES, [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ]);
    }

    #[test]
    fn test_num_pieces() {
        assert_eq!(NUM_PIECES, 6);
    }
}
