use std::fs::File;
use std::io::Write;

use diesel_core::bitboard::{BitBoard, EMPTY_BITBOARD};
use diesel_core::square::{ALL_SQUARES, Square};

static mut KNIGHT_MOVES: [BitBoard; 64] = [EMPTY_BITBOARD; 64];

pub fn generate_knight_moves() {
    for src in ALL_SQUARES {
        unsafe {
            KNIGHT_MOVES[src.to_index()] = ALL_SQUARES
                .iter()
                .filter(|dst| {
                let src_rank = src.get_rank().to_index() as i8;
                let src_file = src.get_file().to_index() as i8;
                let dst_rank = dst.get_rank().to_index() as i8;
                let dst_file = dst.get_file().to_index() as i8;

                ((src_rank - dst_rank).abs() == 2 && (src_file - dst_file).abs() == 1)
                    || ((src_rank - dst_rank).abs() == 1 && (src_file - dst_file).abs() == 2)
            })
                .fold(EMPTY_BITBOARD, |bitboard: BitBoard, square: &Square| bitboard | BitBoard::from_square(*square));
        }
    }
}

pub fn write_knight_moves(f: &mut File) {
    writeln!(f, "const KNIGHT_MOVES: [BitBoard; 64] = [").unwrap();
    unsafe {
        for knight_move in KNIGHT_MOVES {
            writeln!(f, "    BitBoard({}),", knight_move.0).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}