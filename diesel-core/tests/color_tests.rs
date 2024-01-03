use chess_core::color::{Color, NUM_COLORS, ALL_COLORS};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_white() {
        let white = Color::White;
        let not_white = !white;
        assert_eq!(not_white, Color::Black);
    }

    #[test]
    fn test_not_black() {
        let black = Color::Black;
        let not_black = !black;
        assert_eq!(not_black, Color::White);
    }

    #[test]
    fn test_not_not_white() {
        let white = Color::White;
        let not_not_white = !!white;
        assert_eq!(not_not_white, Color::White);
    }

    #[test]
    fn test_not_not_black() {
        let black = Color::Black;
        let not_not_black = !!black;
        assert_eq!(not_not_black, Color::Black);
    }

    #[test]
    fn test_all_colors() {
        assert_eq!(ALL_COLORS, [Color::White, Color::Black]);
    }

    #[test]
    fn test_num_colors() {
        assert_eq!(NUM_COLORS, 2);
    }

    #[test]
    fn test_to_index_white() {
        let white = Color::White;
        assert_eq!(white.to_index(), 0);
    }

    #[test]
    fn test_to_index_black() {
        let black = Color::Black;
        assert_eq!(black.to_index(), 1);
    }

}
