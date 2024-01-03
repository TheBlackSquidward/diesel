use std::fs::File;
use std::io::Write;

use diesel_core::bitboard::{BitBoard, EMPTY_BITBOARD};
use diesel_core::square::{ALL_SQUARES, Square};

static mut KING_MOVES: [BitBoard; 64] = [EMPTY_BITBOARD; 64];

pub fn generate_king_moves() {
    for src in ALL_SQUARES {
        unsafe {
            KING_MOVES[src.to_index()] = ALL_SQUARES
                .iter()
                .filter(|dst| {
                let src_rank = src.get_rank().to_index() as i8;
                let src_file = src.get_file().to_index() as i8;
                let dst_rank = dst.get_rank().to_index() as i8;
                let dst_file = dst.get_file().to_index() as i8;

                    ((src_rank - dst_rank).abs() == 1 || (src_rank - dst_rank).abs() == 0)
                        && ((src_file - dst_file).abs() == 1 || (src_file - dst_file).abs() == 0)
                        && src != **dst
                })
                .fold(EMPTY_BITBOARD, |bitboard: BitBoard, square: &Square | bitboard | BitBoard::from_square(*square));
        }
    }
}

pub fn write_king_moves(f: &mut File) {
    writeln!(f, "const KING_MOVES: [BitBoard; 64] = [").unwrap();
    unsafe {
        for king_move in KING_MOVES {
            writeln!(f, "    BitBoard({}),", king_move.0).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}