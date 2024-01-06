use std::fs::File;
use std::io::Write;
use rand::Rng;

use rand::rngs::ThreadRng;

use diesel_core::bitboard::{BitBoard, EMPTY_BITBOARD};
use diesel_core::magic::{Magic};
use diesel_core::piece::Piece;
use diesel_core::rank::Rank;
use diesel_core::square::{ALL_SQUARES, NUM_SQUARES, Square};
use crate::bishop::get_bishop_ray;
use crate::rook::get_rook_ray;

// static mut SLIDING_PIECE_ATTACKS: [BitBoard; todo!()] = [EMPTY_BITBOARD; todo!()];

static mut ROOK_MAGICS: [Magic; NUM_SQUARES] = [Magic{
    mask: EMPTY_BITBOARD,
    magic: 0,
    shift: 0,
}; NUM_SQUARES];
static mut BISHOP_MAGICS: [Magic; NUM_SQUARES] = [Magic{
    mask: EMPTY_BITBOARD,
    magic: 0,
    shift: 0,
}; NUM_SQUARES];

const ROOK_RELEVANT_INDEX_BITS: [u8; NUM_SQUARES] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12
];

const BISHOP_RELEVANT_INDEX_BITS: [u8; NUM_SQUARES] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

pub fn generate_magics() {
    for piece in [Piece::Rook, Piece::Bishop] {
        for square in ALL_SQUARES {
            generate_magic(piece, square);
        }
    }
}

fn generate_magic(piece: Piece, square: Square) {
    let magic_mask: BitBoard = match piece {
        Piece::Rook => generate_magic_rook_mask(square),
        Piece::Bishop => generate_magic_bishop_mask(square),
        _ => panic!("Invalid piece"),
    };
    let index_bits: u8 = match piece {
        Piece::Rook => ROOK_RELEVANT_INDEX_BITS[square.to_index()],
        Piece::Bishop => BISHOP_RELEVANT_INDEX_BITS[square.to_index()],
        _ => panic!("Invalid piece"),
    };
    let shift: u8 = 64 - index_bits;

    let mut rng: ThreadRng = rand::thread_rng();

    loop {
        let magic_number: u64 = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
        let magic = Magic {
            mask: magic_mask,
            magic: magic_number,
            shift,
        };
        if let Ok(_table) = try_make_table(piece, square, magic) {
            unsafe {
                match piece {
                    Piece::Rook => ROOK_MAGICS[square.to_index()] = magic,
                    Piece::Bishop => BISHOP_MAGICS[square.to_index()] = magic,
                    _ => panic!("Invalid piece"),
                }
            }
            return;
        }
    }
}

fn generate_magic_rook_mask(square: Square) -> BitBoard {
    get_rook_ray(square) & !ALL_SQUARES
        .iter()
        .filter(|edge| {
            (square.get_rank() == edge.get_rank()
                && (edge.get_file() == diesel_core::file::File::A || edge.get_file() == diesel_core::file::File::H))
                || (square.get_file() == edge.get_file()
                && (edge.get_rank() == Rank::First || edge.get_rank() == Rank::Eighth))
        })
        .fold(EMPTY_BITBOARD, |b, s| b | BitBoard::from_square(*s))
}

fn generate_magic_bishop_mask(square: Square) -> BitBoard {
    get_bishop_ray(square) & !ALL_SQUARES
        .iter()
        .filter(|sq| {
            sq.get_rank() == Rank::First
                || sq.get_rank() == Rank::Eighth
                || sq.get_file() == diesel_core::file::File::A
                || sq.get_file() == diesel_core::file::File::H
        })
        .fold(EMPTY_BITBOARD, |b, s| b | BitBoard::from_square(*s))
}

fn try_make_table(piece: Piece, square: Square, magic: Magic) -> Result<Vec<BitBoard>, ()> {
    let index_bits = 64 - magic.shift;
    let mut table = vec![EMPTY_BITBOARD; 1 << index_bits];

    let mut blockers = EMPTY_BITBOARD;
    loop {
        let attack: BitBoard = match piece {
            Piece::Rook => todo!(),
            Piece::Bishop => todo!(),
            _ => panic!("Invalid piece"),
        };
        let table_entry = &mut table[magic.get_magic_index(blockers)];
        if table_entry.is_empty() {
            *table_entry = attack;
        } else if *table_entry != attack {
            return Err(());
        }
        blockers.0 = blockers.0.wrapping_sub(magic.mask.0) & magic.mask.0;
        if blockers.is_empty() {
            break;
        }
    }
    Ok(table)
}

pub fn write_magics(f: &mut File) {
    todo!()
}


//
// use std::str::FromStr;
// use rand::Rng;
//
// use core::board::Board;
// use core::bitboard::{BitBoard, EMPTY_BITBOARD};
// use core::square::{ALL_SQUARES, Square};
//
// mod pawn_attack_tables;
// mod knight_attack_table;
// mod king_attack_table;
// mod bishop_attack_table;
// mod rook_attack_table;
//
// fn generate_random_u64(rng: &mut impl Rng) -> u64 {
//     rng.gen()
// }
//
// fn generate_magic_number(rng: &mut impl Rng) -> u64 {
//     generate_random_u64(rng) & generate_random_u64(rng) & generate_random_u64(rng)
// }
//
// struct MagicEntry {
//     mask: BitBoard,
//     magic: u64,
//     shift: u8,
// }
//
// // const ROOK_MAGICS: [MagicEntry; NUM_SQUARES] = todo!();
// // const BISHOP_MAGICS: [MagicEntry; NUM_SQUARES] = todo!();
//
// // const ROOK_MOVES: [BitBoard; 4096] = todo!();
// // const BISHOP_MOVES: [BitBoard; 4096] = todo!();
//
// fn get_magic_index(magic_entry: &MagicEntry, blocker_bitboard: BitBoard) -> usize {
//     let blockers = blocker_bitboard & magic_entry.mask;
//     let hash = blockers.0.wrapping_mul(magic_entry.magic);
//     let index = (hash >> magic_entry.shift) as usize;
//     index
// }
//
// // fn get_rook_moves(square: Square, blocker_bitboard: BitBoard) -> BitBoard {
// //     let magic = &ROOK_MAGICS[square.to_index()];
// //     ROOK_MOVES[get_magic_index(magic, blocker_bitboard)]
// // }
// //
// // fn get_bishop_moves(square: Square, blocker_bitboard: BitBoard) -> BitBoard {
// //     let magic = &BISHOP_MAGICS[square.to_index()];
// //     BISHOP_MOVES[get_magic_index(magic, blocker_bitboard)]
// // }
//
// pub fn generate_rook_mask(square: Square) -> BitBoard {
//     let mut attacks = EMPTY_BITBOARD;
//     let square_rank = square.get_rank().to_index();
//     let square_file = square.get_file().to_index();
//
//     // Iterate up
//     for rank in square_rank + 1..=6 {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + square_file));
//     }
//
//     // Iterate down
//     for rank in (1..square_rank).rev() {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + square_file));
//     }
//
//     // Iterate left
//     for file in (1..square_file).rev() {
//         attacks |= BitBoard::new(1u64 << (square_rank * 8 + file));
//     }
//
//     // Iterate right
//     for file in square_file + 1..=6 {
//         attacks |= BitBoard::new(1u64 << (square_rank * 8 + file));
//     }
//
//     attacks
// }
//
// pub fn generate_rook_attack(square: Square, block_bitboard: BitBoard) -> BitBoard {
//     let mut attacks = EMPTY_BITBOARD;
//     let square_rank = square.get_rank().to_index();
//     let square_file = square.get_file().to_index();
//
//     // Iterate up
//     for rank in square_rank + 1..=7 {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + square_file));
//
//         // Break if blocked
//         if ((1u64 << (rank * 8 + square_file)) & block_bitboard.as_u64()) != 0 {
//             break;
//         }
//     }
//
//     // Iterate down
//     for rank in (0..square_rank).rev() {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + square_file));
//
//         // Break if blocked
//         if ((1u64 << (rank * 8 + square_file)) & block_bitboard.as_u64()) != 0 {
//             break;
//         }
//     }
//
//     // Iterate left
//     for file in (0..square_file).rev() {
//         attacks |= BitBoard::new(1u64 << (square_rank * 8 + file));
//
//         // Break if blocked
//         if ((1u64 << (square_rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
//             break;
//         }
//     }
//
//     // Iterate right
//     for file in square_file + 1..=7 {
//         attacks |= BitBoard::new(1u64 << (square_rank * 8 + file));
//
//         // Break if blocked
//         if ((1u64 << (square_rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
//             break;
//         }
//     }
//
//     attacks
// }
//
// pub fn generate_rook_magics() {
//     println!(
//         "pub const ROOK_MAGICS: &[MagicEntry; Square::NUM] = &[",
//     );
//     let mut total_table_size = 0;
//     for square in ALL_SQUARES {
//         let mask = generate_rook_mask(square);
//         let (magic_entry, table) = generate_rook_magic(square, mask);
//         println!(
//             "    MagicEntry {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, offset: {} }},",
//             magic_entry.mask.0, magic_entry.magic, magic_entry.shift, total_table_size
//         );
//         total_table_size += table.len();
//     }
//     println!("];");
//     println!(
//         "pub const ROOK_TABLE_SIZE: usize = {};",
//         total_table_size
//     );
// }
//
// pub fn generate_rook_magic(square: Square, mask: BitBoard) -> (MagicEntry, Vec<BitBoard>) {
//     let index_bits = mask.count_bits();
//     let shift = 64 - index_bits as u8;
//     loop {
//         let magic = generate_magic_number(&mut rand::thread_rng());
//         let magic_entry = MagicEntry {
//             mask,
//             magic,
//             shift,
//         };
//         if let Ok(table) = try_make_rook_table(square, &magic_entry) {
//             return (magic_entry, table);
//         }
//     }
// }
//
// pub fn try_make_rook_table(square: Square, magic_entry: &MagicEntry) -> Result<Vec<BitBoard>, ()> {
//     let index_bits = 64 - magic_entry.shift;
//     let mut table = vec![EMPTY_BITBOARD; 1 << index_bits];
//
//     let mut blockers = EMPTY_BITBOARD;
//     loop {
//         let moves = generate_rook_attack(square, blockers);
//         let table_entry = &mut table[get_magic_index(magic_entry, blockers)];
//         if table_entry.is_empty() {
//             *table_entry = moves;
//         } else if *table_entry != moves {
//             return Err(());
//         }
//         blockers.0 = blockers.0.wrapping_sub(magic_entry.mask.0) & magic_entry.mask.0;
//         if blockers.is_empty() {
//             break;
//         }
//     }
//     Ok(table)
// }
//
// pub fn generate_bishop_mask(square: Square) -> BitBoard {
//     let mut attacks = EMPTY_BITBOARD;
//     let square_rank = square.get_rank().to_index();
//     let square_file = square.get_file().to_index();
//
//     // Iterate diagonally up and to the right
//     for (rank, file) in (square_rank + 1..=6).zip(square_file + 1..=6) {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + file));
//     }
//
//     // Iterate diagonally down and to the right
//     for (rank, file) in (1..square_rank).rev().zip(square_file + 1..=6) {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + file));
//     }
//
//     // Iterate diagonally up and to the left
//     for (rank, file) in (square_rank + 1..=6).zip((1..square_file).rev()) {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + file));
//     }
//
//     // Iterate diagonally down and to the left
//     for (rank, file) in (1..square_rank).rev().zip((1..square_file).rev()) {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + file));
//     }
//
//     attacks
// }
//
// pub fn generate_bishop_attacks_on_the_fly(square: Square, block_bitboard: BitBoard) -> BitBoard {
//     let mut attacks = EMPTY_BITBOARD;
//     let square_rank = square.get_rank().to_index();
//     let square_file = square.get_file().to_index();
//
//     // Iterate diagonally up and to the right
//     for (rank, file) in (square_rank + 1..=7).zip(square_file + 1..=7) {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + file));
//         if ((1u64 << (rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
//             break;
//         }
//     }
//
//     // Iterate diagonally down and to the right
//     for (rank, file) in (0..square_rank).rev().zip(square_file + 1..=7) {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + file));
//         if ((1u64 << (rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
//             break;
//         }
//     }
//
//     // Iterate diagonally up and to the left
//     for (rank, file) in (square_rank + 1..=7).zip((0..square_file).rev()) {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + file));
//         if ((1u64 << (rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
//             break;
//         }
//     }
//
//     // Iterate diagonally down and to the left
//     for (rank, file) in (0..square_rank).rev().zip((0..square_file).rev()) {
//         attacks |= BitBoard::new(1u64 << (rank * 8 + file));
//         if ((1u64 << (rank * 8 + file)) & block_bitboard.as_u64()) != 0 {
//             break;
//         }
//     }
//
//     attacks
// }
//
// fn generate_bishop_magics() {
//     println!(
//         "pub const BISHOP_MAGICS: &[MagicEntry; Square::NUM] = &[",
//     );
//     let mut total_table_size = 0;
//     for square in ALL_SQUARES {
//         let mask = generate_bishop_mask(square);
//         let (magic_entry, table) = generate_bishop_magic(square, mask);
//         println!(
//             "    MagicEntry {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, offset: {} }},",
//             magic_entry.mask.0, magic_entry.magic, magic_entry.shift, total_table_size
//         );
//         total_table_size += table.len();
//     }
//     println!("];");
//     println!(
//         "pub const BISHOP_TABLE_SIZE: usize = {};",
//         total_table_size
//     );
// }
//
// fn generate_bishop_magic(square: Square, mask: BitBoard) -> (MagicEntry, Vec<BitBoard>) {
//     let index_bits = mask.count_bits();
//     let shift = 64 - index_bits as u8;
//     loop {
//         let magic = generate_magic_number(&mut rand::thread_rng());
//         let magic_entry = MagicEntry {
//             mask,
//             magic,
//             shift,
//         };
//         if let Ok(table) = try_make_bishop_table(square, &magic_entry) {
//             return (magic_entry, table);
//         }
//     }
// }
//
// fn try_make_bishop_table(square: Square, magic_entry: &MagicEntry) -> Result<Vec<BitBoard>, ()> {
//     let index_bits = 64 - magic_entry.shift;
//     let mut table = vec![EMPTY_BITBOARD; 1 << index_bits];
//
//     let mut blockers = EMPTY_BITBOARD;
//     loop {
//         let moves = generate_bishop_attacks_on_the_fly(square, blockers);
//         let table_entry = &mut table[get_magic_index(magic_entry, blockers)];
//         if table_entry.is_empty() {
//             *table_entry = moves;
//         } else if *table_entry != moves {
//             return Err(());
//         }
//         blockers.0 = blockers.0.wrapping_sub(magic_entry.mask.0) & magic_entry.mask.0;
//         if blockers.is_empty() {
//             break;
//         }
//     }
//     Ok(table)
// }
//
// fn main() {
//     // generate_rook_magics();
//     // generate_bishop_magics();
//     let board = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
//     println!("{}", board)
// }
