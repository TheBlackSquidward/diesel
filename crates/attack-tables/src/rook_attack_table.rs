use core::bitboard::{EMPTY_BITBOARD, BitBoard};
use core::square::Square;

pub fn generate_rook_attack(square: Square) -> BitBoard {
    let mut attacks = EMPTY_BITBOARD;
    let square_rank = square.get_rank().to_index();
    let square_file = square.get_file().to_index();

    // Iterate up
    for rank in square_rank + 1..=6 {
        attacks |= BitBoard::new(1u64 << (rank * 8 + square_file));
    }

    // Iterate down
    for rank in (1..square_rank).rev() {
        attacks |= BitBoard::new(1u64 << (rank * 8 + square_file));
    }

    // Iterate left
    for file in (1..square_file).rev() {
        attacks |= BitBoard::new(1u64 << (square_rank * 8 + file));
    }

    // Iterate right
    for file in square_file + 1..=6 {
        attacks |= BitBoard::new(1u64 << (square_rank * 8 + file));
    }

    attacks
}

pub fn rook_attacks_on_the_fly(square: Square, block_bitboard: BitBoard) -> BitBoard {
    let mut attacks = EMPTY_BITBOARD;
    let square_rank = square.get_rank().to_index();
    let square_file = square.get_file().to_index();

    // Iterate up
    for rank in square_rank + 1..=7 {
        attacks |= BitBoard::new(1u64 << (rank * 8 + square_file));

        // Break if blocked
        if ((1u64 << (rank * 8 + square_file)) & block_bitboard.as_u64()) != 0 {
            break;
        }
    }

    // Iterate down
    for rank in (0..square_rank).rev() {
        attacks |= BitBoard::new(1u64 << (rank * 8 + square_file));

        // Break if blocked
        if ((1u64 << (rank * 8 + square_file)) & block_bitboard.as_u64()) != 0 {
            break;
        }
    }

    // Iterate left
    for file in (0..square_file).rev() {
        attacks |= BitBoard::new(1u64 << (square_rank * 8 + file));

        // Break if blocked
        if ((1u64 << (square_rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
            break;
        }
    }

    // Iterate right
    for file in square_file + 1..=7 {
        attacks |= BitBoard::new(1u64 << (square_rank * 8 + file));

        // Break if blocked
        if ((1u64 << (square_rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
            break;
        }
    }

    attacks
}