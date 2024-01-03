use chess_core::bitboard::{BitBoard, EMPTY_BITBOARD};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitand() {
        let board1 = BitBoard(0b1010);
        let board2 = BitBoard(0b1100);

        let result = board1 & board2;

        assert_eq!(result.0, 0b1000);
    }

    #[test]
    fn test_bitor() {
        let board1 = BitBoard(0b1010);
        let board2 = BitBoard(0b1100);

        let result = board1 | board2;

        assert_eq!(result.0, 0b1110);
    }

    #[test]
    fn test_bitxor() {
        let board1 = BitBoard(0b1010);
        let board2 = BitBoard(0b1100);

        let result = board1 ^ board2;

        assert_eq!(result.0, 0b0110);
    }

    #[test]
    fn test_not() {
        let board = BitBoard(0b1010);

        let result = !board;

        assert_eq!(result.0, 0b1111111111111111111111111111111111111111111111111111111111110101);
    }

    #[test]
    fn test_bitand_assign() {
        let mut board1 = BitBoard(0b1010);
        let board2 = BitBoard(0b1100);

        board1 &= board2;

        assert_eq!(board1.0, 0b1000);
    }

    #[test]
    fn test_bitor_assign() {
        let mut board1 = BitBoard(0b1010);
        let board2 = BitBoard(0b1100);

        board1 |= board2;

        assert_eq!(board1.0, 0b1110);
    }

    #[test]
    fn test_bitxor_assign() {
        let mut board1 = BitBoard(0b1010);
        let board2 = BitBoard(0b1100);

        board1 ^= board2;

        assert_eq!(board1.0, 0b0110);
    }

    #[test]
    fn test_empty_bitboard() {
        assert_eq!(EMPTY_BITBOARD.0, 0);
    }

}
