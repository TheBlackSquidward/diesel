use chess_core::square::{Square, NUM_SQUARES, ALL_SQUARES};
use chess_core::rank::Rank;
use chess_core::file::File;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_new() {
        let square = Square::new(0);
        assert_eq!(square.to_int(), 0);

        let square = Square::new(63);
        assert_eq!(square.to_int(), 63);
    }

    #[test]
    fn test_default_square() {
        let default_square = Square::default();
        assert_eq!(default_square.to_int(), 0);
    }

    #[test]
    fn test_square_create_square() {
        let square = Square::create_square(Rank::First, File::A);
        assert_eq!(square.to_int(), 0);

        let square = Square::create_square(Rank::Third, File::C);
        assert_eq!(square.to_int(), 18); // (2 << 3) ^ 2 = 16 + 2 = 18

        let square = Square::create_square(Rank::Eighth, File::H);
        assert_eq!(square.to_int(), 63);
    }

    #[test]
    fn test_square_get_rank() {
        assert_eq!(Square::A1.get_rank(), Rank::First);
        assert_eq!(Square::H8.get_rank(), Rank::Eighth);
        assert_eq!(Square::E4.get_rank(), Rank::Fourth);
    }

    #[test]
    fn test_square_get_file() {
        assert_eq!(Square::A1.get_file(), File::A);
        assert_eq!(Square::H8.get_file(), File::H);
        assert_eq!(Square::E4.get_file(), File::E);
    }

    #[test]
    fn test_square_to_int() {
        assert_eq!(Square::A1.to_int(), 0);
        assert_eq!(Square::H8.to_int(), 63);
        assert_eq!(Square::E4.to_int(), 28);
    }

    #[test]
    fn test_square_consts() {
        assert_eq!(Square::A1.to_int(), 0);
        assert_eq!(Square::B1.to_int(), 1);
        assert_eq!(Square::C1.to_int(), 2);
        assert_eq!(Square::D1.to_int(), 3);
        assert_eq!(Square::E1.to_int(), 4);
        assert_eq!(Square::F1.to_int(), 5);
        assert_eq!(Square::G1.to_int(), 6);
        assert_eq!(Square::H1.to_int(), 7);

        assert_eq!(Square::A2.to_int(), 8);
        assert_eq!(Square::B2.to_int(), 9);
        assert_eq!(Square::C2.to_int(), 10);
        assert_eq!(Square::D2.to_int(), 11);
        assert_eq!(Square::E2.to_int(), 12);
        assert_eq!(Square::F2.to_int(), 13);
        assert_eq!(Square::G2.to_int(), 14);
        assert_eq!(Square::H2.to_int(), 15);

        assert_eq!(Square::A3.to_int(), 16);
        assert_eq!(Square::B3.to_int(), 17);
        assert_eq!(Square::C3.to_int(), 18);
        assert_eq!(Square::D3.to_int(), 19);
        assert_eq!(Square::E3.to_int(), 20);
        assert_eq!(Square::F3.to_int(), 21);
        assert_eq!(Square::G3.to_int(), 22);
        assert_eq!(Square::H3.to_int(), 23);

        assert_eq!(Square::A4.to_int(), 24);
        assert_eq!(Square::B4.to_int(), 25);
        assert_eq!(Square::C4.to_int(), 26);
        assert_eq!(Square::D4.to_int(), 27);
        assert_eq!(Square::E4.to_int(), 28);
        assert_eq!(Square::F4.to_int(), 29);
        assert_eq!(Square::G4.to_int(), 30);
        assert_eq!(Square::H4.to_int(), 31);

        assert_eq!(Square::A5.to_int(), 32);
        assert_eq!(Square::B5.to_int(), 33);
        assert_eq!(Square::C5.to_int(), 34);
        assert_eq!(Square::D5.to_int(), 35);
        assert_eq!(Square::E5.to_int(), 36);
        assert_eq!(Square::F5.to_int(), 37);
        assert_eq!(Square::G5.to_int(), 38);
        assert_eq!(Square::H5.to_int(), 39);

        assert_eq!(Square::A6.to_int(), 40);
        assert_eq!(Square::B6.to_int(), 41);
        assert_eq!(Square::C6.to_int(), 42);
        assert_eq!(Square::D6.to_int(), 43);
        assert_eq!(Square::E6.to_int(), 44);
        assert_eq!(Square::F6.to_int(), 45);
        assert_eq!(Square::G6.to_int(), 46);
        assert_eq!(Square::H6.to_int(), 47);

        assert_eq!(Square::A7.to_int(), 48);
        assert_eq!(Square::B7.to_int(), 49);
        assert_eq!(Square::C7.to_int(), 50);
        assert_eq!(Square::D7.to_int(), 51);
        assert_eq!(Square::E7.to_int(), 52);
        assert_eq!(Square::F7.to_int(), 53);
        assert_eq!(Square::G7.to_int(), 54);
        assert_eq!(Square::H7.to_int(), 55);

        assert_eq!(Square::A8.to_int(), 56);
        assert_eq!(Square::B8.to_int(), 57);
        assert_eq!(Square::C8.to_int(), 58);
        assert_eq!(Square::D8.to_int(), 59);
        assert_eq!(Square::E8.to_int(), 60);
        assert_eq!(Square::F8.to_int(), 61);
        assert_eq!(Square::G8.to_int(), 62);
        assert_eq!(Square::H8.to_int(), 63);
    }

    #[test]
    fn test_all_squares() {
        assert_eq!(ALL_SQUARES, [
            Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
            Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
            Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
            Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
            Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
            Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
            Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
            Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
        ]);
    }

    #[test]
    fn test_num_squares() {
        assert_eq!(NUM_SQUARES, 64);
    }
}
