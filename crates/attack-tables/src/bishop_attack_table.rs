use core::bitboard::{EMPTY_BITBOARD, BitBoard};
use core::square::Square;

pub fn generate_bishop_attack(square: Square) -> BitBoard {
    let mut attacks = EMPTY_BITBOARD;
    let square_rank = square.get_rank().to_index();
    let square_file = square.get_file().to_index();

    // Iterate diagonally up and to the right
    for (rank, file) in (square_rank + 1..=6).zip(square_file + 1..=6) {
        attacks |= BitBoard::new(1u64 << (rank * 8 + file));
    }

    // Iterate diagonally down and to the right
    for (rank, file) in (1..square_rank).rev().zip(square_file + 1..=6) {
        attacks |= BitBoard::new(1u64 << (rank * 8 + file));
    }

    // Iterate diagonally up and to the left
    for (rank, file) in (square_rank + 1..=6).zip((1..square_file).rev()) {
        attacks |= BitBoard::new(1u64 << (rank * 8 + file));
    }

    // Iterate diagonally down and to the left
    for (rank, file) in (1..square_rank).rev().zip((1..square_file).rev()) {
        attacks |= BitBoard::new(1u64 << (rank * 8 + file));
    }

    attacks
}

pub fn generate_bishop_attacks_on_the_fly(square: Square, block_bitboard: BitBoard) -> BitBoard {
    let mut attacks = EMPTY_BITBOARD;
    let square_rank = square.get_rank().to_index();
    let square_file = square.get_file().to_index();

    // Iterate diagonally up and to the right
    for (rank, file) in (square_rank + 1..=7).zip(square_file + 1..=7) {
        attacks |= BitBoard::new(1u64 << (rank * 8 + file));
        if ((1u64 << (rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
            break;
        }
    }

    // Iterate diagonally down and to the right
    for (rank, file) in (0..square_rank).rev().zip(square_file + 1..=7) {
        attacks |= BitBoard::new(1u64 << (rank * 8 + file));
        if ((1u64 << (rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
            break;
        }
    }

    // Iterate diagonally up and to the left
    for (rank, file) in (square_rank + 1..=7).zip((0..square_file).rev()) {
        attacks |= BitBoard::new(1u64 << (rank * 8 + file));
        if ((1u64 << (rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
            break;
        }
    }

    // Iterate diagonally down and to the left
    for (rank, file) in (0..square_rank).rev().zip((0..square_file).rev()) {
        attacks |= BitBoard::new(1u64 << (rank * 8 + file));
        if ((1u64 << (rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
            break;
        }
    }

    attacks
}