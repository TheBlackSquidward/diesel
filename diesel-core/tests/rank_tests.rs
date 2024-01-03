use chess_core::rank::{Rank, NUM_RANKS, ALL_RANKS};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_from_index_valid() {
        assert_eq!(Rank::from_index(0), Rank::First);
        assert_eq!(Rank::from_index(1), Rank::Second);
        assert_eq!(Rank::from_index(2), Rank::Third);
        assert_eq!(Rank::from_index(3), Rank::Fourth);
        assert_eq!(Rank::from_index(4), Rank::Fifth);
        assert_eq!(Rank::from_index(5), Rank::Sixth);
        assert_eq!(Rank::from_index(6), Rank::Seventh);
        assert_eq!(Rank::from_index(7), Rank::Eighth);
    }

    #[test]
    fn test_rank_to_index() {
        assert_eq!(Rank::First.to_index(), 0);
        assert_eq!(Rank::Second.to_index(), 1);
        assert_eq!(Rank::Third.to_index(), 2);
        assert_eq!(Rank::Fourth.to_index(), 3);
        assert_eq!(Rank::Fifth.to_index(), 4);
        assert_eq!(Rank::Sixth.to_index(), 5);
        assert_eq!(Rank::Seventh.to_index(), 6);
        assert_eq!(Rank::Eighth.to_index(), 7);
    }

    #[test]
    fn test_rank_down_valid() {
        assert_eq!(Rank::First.down(), Rank::First);
        assert_eq!(Rank::Second.down(), Rank::First);
        assert_eq!(Rank::Third.down(), Rank::Second);
        assert_eq!(Rank::Fourth.down(), Rank::Third);
        assert_eq!(Rank::Fifth.down(), Rank::Fourth);
        assert_eq!(Rank::Sixth.down(), Rank::Fifth);
        assert_eq!(Rank::Seventh.down(), Rank::Sixth);
        assert_eq!(Rank::Eighth.down(), Rank::Seventh);
    }

    #[test]
    fn test_rank_up_valid() {
        assert_eq!(Rank::First.up(), Rank::Second);
        assert_eq!(Rank::Second.up(), Rank::Third);
        assert_eq!(Rank::Third.up(), Rank::Fourth);
        assert_eq!(Rank::Fourth.up(), Rank::Fifth);
        assert_eq!(Rank::Fifth.up(), Rank::Sixth);
        assert_eq!(Rank::Sixth.up(), Rank::Seventh);
        assert_eq!(Rank::Seventh.up(), Rank::Eighth);
        assert_eq!(Rank::Eighth.up(), Rank::Eighth);
    }

    #[test]
    fn test_all_ranks() {
        assert_eq!(ALL_RANKS, [Rank::First, Rank::Second, Rank::Third, Rank::Fourth, Rank::Fifth, Rank::Sixth, Rank::Seventh, Rank::Eighth]);
    }

    #[test]
    fn test_num_ranks() {
        assert_eq!(NUM_RANKS, 8);
    }
}
