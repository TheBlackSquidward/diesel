use chess_core::file::{File, NUM_FILES, ALL_FILES};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_from_index_valid() {
        assert_eq!(File::from_index(0), File::A);
        assert_eq!(File::from_index(1), File::B);
        assert_eq!(File::from_index(2), File::C);
        assert_eq!(File::from_index(3), File::D);
        assert_eq!(File::from_index(4), File::E);
        assert_eq!(File::from_index(5), File::F);
        assert_eq!(File::from_index(6), File::G);
        assert_eq!(File::from_index(7), File::H);
    }

    #[test]
    fn test_file_to_index() {
        assert_eq!(File::A.to_index(), 0);
        assert_eq!(File::B.to_index(), 1);
        assert_eq!(File::C.to_index(), 2);
        assert_eq!(File::D.to_index(), 3);
        assert_eq!(File::E.to_index(), 4);
        assert_eq!(File::F.to_index(), 5);
        assert_eq!(File::G.to_index(), 6);
        assert_eq!(File::H.to_index(), 7);
    }

    #[test]
    fn test_file_left_valid() {
        assert_eq!(File::H.left(), File::G);
        assert_eq!(File::G.left(), File::F);
        assert_eq!(File::F.left(), File::E);
        assert_eq!(File::E.left(), File::D);
        assert_eq!(File::D.left(), File::C);
        assert_eq!(File::C.left(), File::B);
        assert_eq!(File::B.left(), File::A);
        assert_eq!(File::A.left(), File::A);
    }

    #[test]
    fn test_file_right_valid() {
        assert_eq!(File::A.right(), File::B);
        assert_eq!(File::B.right(), File::C);
        assert_eq!(File::C.right(), File::D);
        assert_eq!(File::D.right(), File::E);
        assert_eq!(File::E.right(), File::F);
        assert_eq!(File::F.right(), File::G);
        assert_eq!(File::G.right(), File::H);
        assert_eq!(File::H.right(), File::H);
    }

    #[test]
    fn test_all_files() {
        assert_eq!(ALL_FILES, [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H]);
    }

    #[test]
    fn test_num_files() {
        assert_eq!(NUM_FILES, 8);
    }
}
