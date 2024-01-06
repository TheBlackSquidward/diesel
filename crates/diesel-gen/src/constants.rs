use std::fs::File;
use std::io::Write;

use diesel_core::bitboard::{BitBoard, EMPTY_BITBOARD};
use diesel_core::file::NUM_FILES;
use diesel_core::rank::{NUM_RANKS, Rank};
use diesel_core::square::{ALL_SQUARES, Square};

static mut RANK_MASKS: [BitBoard; NUM_RANKS] = [EMPTY_BITBOARD; NUM_RANKS];
static mut FILE_MASKS: [BitBoard; NUM_FILES] = [EMPTY_BITBOARD; NUM_FILES];
static mut ADJACENT_FILE_MASKS: [BitBoard; NUM_FILES] = [EMPTY_BITBOARD; NUM_FILES];
static mut EDGE_MASK: BitBoard = EMPTY_BITBOARD;

pub fn generate_constants() {
    generate_rank_masks();
    generate_file_masks();
    generate_adjacent_file_masks();
    generate_edge_mask();
}

fn generate_rank_masks() {
    for i in 0..NUM_RANKS {
        unsafe {
            RANK_MASKS[i] = ALL_SQUARES
                .iter()
                .filter(|square| square.get_rank().to_index() == i)
                .fold(EMPTY_BITBOARD, |bitboard: BitBoard, square: &Square| bitboard | BitBoard::from_square(*square));
        }
    }
}

fn generate_file_masks() {
    for i in 0..NUM_FILES {
        unsafe {
            FILE_MASKS[i] = ALL_SQUARES
                .iter()
                .filter(|square| square.get_file().to_index() == i)
                .fold(EMPTY_BITBOARD, |bitboard: BitBoard, square: &Square| bitboard | BitBoard::from_square(*square));
        }
    }
}

fn generate_adjacent_file_masks() {
    for i in 0..NUM_FILES {
        unsafe {
            ADJACENT_FILE_MASKS[i] = ALL_SQUARES
                .iter()
                .filter(|square| {
                    (square.get_file().to_index() as i8) == (i as i8) - 1
                        || (square.get_file().to_index() as i8) == (i as i8) + 1
                })
                .fold(EMPTY_BITBOARD, |bitboard: BitBoard, square: &Square| bitboard | BitBoard::from_square(*square));
        }
    }
}

fn generate_edge_mask() {
    unsafe {
        EDGE_MASK = ALL_SQUARES
            .iter()
            .filter(|square| {
                square.get_rank() == Rank::First
                    || square.get_rank() == Rank::Eighth
                    || square.get_file() == diesel_core::file::File::A
                    || square.get_file() == diesel_core::file::File::H
            })
            .fold(EMPTY_BITBOARD, |bitboard: BitBoard, square: &Square| bitboard | BitBoard::from_square(*square));
    }
}

pub fn write_constants(f: &mut File) {
    write_rank_masks(f);
    write_file_masks(f);
    write_adjacent_file_masks(f);
    write_edge_mask(f);
}

fn write_rank_masks(f: &mut File) {
    writeln!(f, "pub const RANK_MASKS: [BitBoard; NUM_RANKS] = [").unwrap();
    unsafe {
        for rank in RANK_MASKS {
            writeln!(f, "    BitBoard({}), ", rank.0).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}

fn write_file_masks(f: &mut File) {
    writeln!(f, "pub const FILE_MASKS: [BitBoard; NUM_FILES] = [").unwrap();
    unsafe {
        for file in FILE_MASKS {
            writeln!(f, "    BitBoard({}), ", file.0).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}

fn write_adjacent_file_masks(f: &mut File) {
    writeln!(f, "pub const ADJACENT_FILE_MASKS: [BitBoard; NUM_FILES] = [").unwrap();
    unsafe {
        for file in ADJACENT_FILE_MASKS {
            writeln!(f, "    BitBoard({}), ", file.0).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}

fn write_edge_mask(f: &mut File) {
    writeln!(f, "pub const EDGE_MASK: BitBoard = BitBoard({});", unsafe { EDGE_MASK.0 }).unwrap();
}