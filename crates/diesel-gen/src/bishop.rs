use std::fs::File;
use std::io::Write;

use diesel_core::bitboard::{BitBoard, EMPTY_BITBOARD};
use diesel_core::square::{ALL_SQUARES, Square};

static mut BISHOP_RAYS: [BitBoard; 64] = [EMPTY_BITBOARD; 64];

pub fn generate_bishop_rays() {
    for src in ALL_SQUARES {
        unsafe {
            BISHOP_RAYS[src.to_index()] = ALL_SQUARES
                .iter()
                .filter(|dst| {
                let src_rank = src.get_rank().to_index() as i8;
                let src_file = src.get_file().to_index() as i8;
                let dst_rank = dst.get_rank().to_index() as i8;
                let dst_file = dst.get_file().to_index() as i8;

                    (src_rank == dst_rank || src_file == dst_file) && src != **dst
            })
                .fold(EMPTY_BITBOARD, |bitboard: BitBoard, square: &Square| bitboard | BitBoard::from_square(*square));
        }
    }
}

pub fn write_bishop_rays(f: &mut File) {
    writeln!(f, "const BISHOP_RAYS: [BitBoard; 64] = [").unwrap();
    unsafe {
        for bishop_ray in BISHOP_RAYS {
            writeln!(f, "    BitBoard({}),", bishop_ray.0).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}