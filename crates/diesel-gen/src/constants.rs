use std::fs::File;
use std::io::Write;

use diesel_core::bitboard::{BitBoard, EMPTY_BITBOARD};
use diesel_core::file::NUM_FILES;
use diesel_core::rank::NUM_RANKS;

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
    todo!()
}

fn generate_file_masks() {
    todo!()
}

fn generate_adjacent_file_masks() {
    todo!()
}

fn generate_edge_mask() {
    todo!()
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