use std::fs::File;
use std::io::Write;

use diesel_core::bitboard::{BitBoard, EMPTY_BITBOARD};

static mut ROOK_RAYS: [BitBoard; 64] = [EMPTY_BITBOARD; 64];

pub fn generate_rook_rays() {
    todo!()
}

pub fn write_rook_rays(f: &mut File) {
    todo!()
}