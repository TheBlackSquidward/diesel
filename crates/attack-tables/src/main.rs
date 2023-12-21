use std::str::FromStr;
use rand::Rng;

use core::board::Board;
use core::bitboard::{BitBoard, EMPTY_BITBOARD};
use core::square::{ALL_SQUARES, Square};

mod pawn_attack_tables;
mod knight_attack_table;
mod king_attack_table;
mod bishop_attack_table;
mod rook_attack_table;

fn generate_random_u64(rng: &mut impl Rng) -> u64 {
    rng.gen()
}

fn generate_magic_number(rng: &mut impl Rng) -> u64 {
    generate_random_u64(rng) & generate_random_u64(rng) & generate_random_u64(rng)
}

struct MagicEntry {
    mask: BitBoard,
    magic: u64,
    shift: u8,
}

// const ROOK_MAGICS: [MagicEntry; NUM_SQUARES] = todo!();
// const BISHOP_MAGICS: [MagicEntry; NUM_SQUARES] = todo!();

// const ROOK_MOVES: [BitBoard; 4096] = todo!();
// const BISHOP_MOVES: [BitBoard; 4096] = todo!();

fn get_magic_index(magic_entry: &MagicEntry, blocker_bitboard: BitBoard) -> usize {
    let blockers = blocker_bitboard & magic_entry.mask;
    let hash = blockers.0.wrapping_mul(magic_entry.magic);
    let index = (hash >> magic_entry.shift) as usize;
    index
}

// fn get_rook_moves(square: Square, blocker_bitboard: BitBoard) -> BitBoard {
//     let magic = &ROOK_MAGICS[square.to_index()];
//     ROOK_MOVES[get_magic_index(magic, blocker_bitboard)]
// }
//
// fn get_bishop_moves(square: Square, blocker_bitboard: BitBoard) -> BitBoard {
//     let magic = &BISHOP_MAGICS[square.to_index()];
//     BISHOP_MOVES[get_magic_index(magic, blocker_bitboard)]
// }

pub fn generate_rook_mask(square: Square) -> BitBoard {
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

pub fn generate_rook_attack(square: Square, block_bitboard: BitBoard) -> BitBoard {
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

pub fn generate_rook_magics() {
    println!(
        "pub const ROOK_MAGICS: &[MagicEntry; Square::NUM] = &[",
    );
    let mut total_table_size = 0;
    for square in ALL_SQUARES {
        let mask = generate_rook_mask(square);
        let (magic_entry, table) = generate_rook_magic(square, mask);
        println!(
            "    MagicEntry {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, offset: {} }},",
            magic_entry.mask.0, magic_entry.magic, magic_entry.shift, total_table_size
        );
        total_table_size += table.len();
    }
    println!("];");
    println!(
        "pub const ROOK_TABLE_SIZE: usize = {};",
        total_table_size
    );
}

pub fn generate_rook_magic(square: Square, mask: BitBoard) -> (MagicEntry, Vec<BitBoard>) {
    let index_bits = mask.count_bits();
    let shift = 64 - index_bits as u8;
    loop {
        let magic = generate_magic_number(&mut rand::thread_rng());
        let magic_entry = MagicEntry {
            mask,
            magic,
            shift,
        };
        if let Ok(table) = try_make_rook_table(square, &magic_entry) {
            return (magic_entry, table);
        }
    }
}

pub fn try_make_rook_table(square: Square, magic_entry: &MagicEntry) -> Result<Vec<BitBoard>, ()> {
    let index_bits = 64 - magic_entry.shift;
    let mut table = vec![EMPTY_BITBOARD; 1 << index_bits];

    let mut blockers = EMPTY_BITBOARD;
    loop {
        let moves = generate_rook_attack(square, blockers);
        let table_entry = &mut table[get_magic_index(magic_entry, blockers)];
        if table_entry.is_empty() {
            *table_entry = moves;
        } else if *table_entry != moves {
            return Err(());
        }
        blockers.0 = blockers.0.wrapping_sub(magic_entry.mask.0) & magic_entry.mask.0;
        if blockers.is_empty() {
            break;
        }
    }
    Ok(table)
}

pub fn generate_bishop_mask(square: Square) -> BitBoard {
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

fn generate_bishop_magics() {
    println!(
        "pub const BISHOP_MAGICS: &[MagicEntry; Square::NUM] = &[",
    );
    let mut total_table_size = 0;
    for square in ALL_SQUARES {
        let mask = generate_bishop_mask(square);
        let (magic_entry, table) = generate_bishop_magic(square, mask);
        println!(
            "    MagicEntry {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, offset: {} }},",
            magic_entry.mask.0, magic_entry.magic, magic_entry.shift, total_table_size
        );
        total_table_size += table.len();
    }
    println!("];");
    println!(
        "pub const BISHOP_TABLE_SIZE: usize = {};",
        total_table_size
    );
}

fn generate_bishop_magic(square: Square, mask: BitBoard) -> (MagicEntry, Vec<BitBoard>) {
    let index_bits = mask.count_bits();
    let shift = 64 - index_bits as u8;
    loop {
        let magic = generate_magic_number(&mut rand::thread_rng());
        let magic_entry = MagicEntry {
            mask,
            magic,
            shift,
        };
        if let Ok(table) = try_make_bishop_table(square, &magic_entry) {
            return (magic_entry, table);
        }
    }
}

fn try_make_bishop_table(square: Square, magic_entry: &MagicEntry) -> Result<Vec<BitBoard>, ()> {
    let index_bits = 64 - magic_entry.shift;
    let mut table = vec![EMPTY_BITBOARD; 1 << index_bits];

    let mut blockers = EMPTY_BITBOARD;
    loop {
        let moves = generate_bishop_attacks_on_the_fly(square, blockers);
        let table_entry = &mut table[get_magic_index(magic_entry, blockers)];
        if table_entry.is_empty() {
            *table_entry = moves;
        } else if *table_entry != moves {
            return Err(());
        }
        blockers.0 = blockers.0.wrapping_sub(magic_entry.mask.0) & magic_entry.mask.0;
        if blockers.is_empty() {
            break;
        }
    }
    Ok(table)
}

fn main() {
    // generate_rook_magics();
    // generate_bishop_magics();
    let board = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    println!("{}", board)
}
