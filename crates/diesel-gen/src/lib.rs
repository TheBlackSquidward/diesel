use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

mod constants;
mod king;
mod knight;
mod pawn;
mod bishop;
mod rook;
mod magic;

use crate::constants::{generate_constants, write_constants};

use crate::king::{generate_king_moves, write_king_moves};
use crate::knight::{generate_knight_moves, write_knight_moves};
use crate::pawn::{generate_pawn_moves, generate_pawn_attacks, write_pawn_moves, write_pawn_attacks};

use crate::rook::{generate_rook_rays, write_rook_rays};
use crate::bishop::{generate_bishop_rays, write_bishop_rays};

use crate::magic::{generate_magics, write_magics};

pub fn generate_all_tables() {
    // Generate constants
    generate_constants();

    // Generate
    generate_king_moves();
    generate_knight_moves();
    generate_pawn_moves();
    generate_pawn_attacks();

    // Generate sliding piece rays
    generate_rook_rays();
    generate_bishop_rays();

    // Generate magics
    generate_magics();

    let out_dir: String = env::var("OUT_DIR").unwrap_or("./out".to_string());
    let gen_path: PathBuf = Path::new(&out_dir).join("gen.rs");
    let f: &mut File = &mut File::create(gen_path).unwrap();

    // Write constants
    write_constants(f);

    // Write
    write_knight_moves(f);
    write_king_moves(f);
    write_pawn_moves(f);
    write_pawn_attacks(f);

    // Write sliding piece rays
    write_rook_rays(f);
    write_bishop_rays(f);

    // Write magics
    write_magics(f);
}