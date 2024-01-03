#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
}

pub const NUM_RANKS: usize = 8;

pub const ALL_RANKS: [Rank; NUM_RANKS] = [
    Rank::First,
    Rank::Second,
    Rank::Third,
    Rank::Fourth,
    Rank::Fifth,
    Rank::Sixth,
    Rank::Seventh,
    Rank::Eighth,
];

impl Rank {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => unreachable!(),
        }
    }

    pub fn down(&self) -> Rank {
        if *self as usize == 0 {
            *self
        } else {
            Rank::from_index(self.to_index() - 1)
        }
    }

    pub fn up(&self) -> Rank {
        if *self as usize == 7 {
            *self
        } else {
            Rank::from_index(self.to_index() + 1)
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }
}