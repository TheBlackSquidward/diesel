mod pawn_attack_tables;
mod knight_attack_table;
mod king_attack_table;
mod bishop_attack_table;
mod rook_attack_table;

use core::bitboard::BitBoard;
use core::color::NUM_COLORS;
use core::file::File;
use core::rank::Rank;
use core::square::Square;

use crate::pawn_attack_tables::{generate_pawn_attack_tables, output_pawn_attack_tables};
use crate::knight_attack_table::{generate_knight_attack_table, output_knight_attack_table};
use crate::king_attack_table::{generate_king_attack_table, output_king_attack_table};

fn main() {
    // let pawn_attack_tables: [[BitBoard; NUM_SQUARES]; NUM_COLORS] = generate_pawn_attack_tables();
    // output_pawn_attack_tables(pawn_attack_tables);
    // println!("Pawn attack tables have been generated and written to attack_tables/pawn_attack_tables.rs!");
    //
    // let knight_attack_tables: [BitBoard; NUM_SQUARES] = generate_knight_attack_table();
    // output_knight_attack_table(knight_attack_tables);
    // println!("Knight attack tables have been generated and written to attack_tables/knight_attack_table!");
    //
    // let king_attack_tables: [BitBoard; NUM_SQUARES] = generate_king_attack_table();
    // output_king_attack_table(king_attack_tables);
    // println!("King attack tables have been generated and written to attack_tables/king_attack_table!");

    let mut block_bitboard = BitBoard::new(0);
    // Pawn
    // block_bitboard.set_bit(Square::create_square(Rank::Sixth, File::B));
    // block_bitboard.set_bit(Square::create_square(Rank::Seventh, File::G));
    // block_bitboard.set_bit(Square::create_square(Rank::Third, File::E));
    // block_bitboard.set_bit(Square::create_square(Rank::Second, File::B));

    // Rook
    block_bitboard.set_bit(Square::create_square(Rank::Seventh, File::D));
    block_bitboard.set_bit(Square::create_square(Rank::Second, File::D));
    block_bitboard.set_bit(Square::create_square(Rank::Fourth, File::B));
    block_bitboard.set_bit(Square::create_square(Rank::Fourth, File::G));
    println!("{}", block_bitboard);
    println!("{}", block_bitboard.count_bits());

    let test = rook_attack_table::rook_attacks_on_the_fly(Square::create_square(Rank::Fourth, File::D), block_bitboard);
    println!("{}", test)
}
