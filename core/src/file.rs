#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

pub const NUM_FILES: usize = 8;

pub const ALL_FILES: [File; NUM_FILES] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl File {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }

    pub fn left(&self) -> File {
        if *self as usize == 0 {
            *self
        } else {
            File::from_index(self.to_index() - 1)
        }
    }

    pub fn right(&self) -> File {
        if *self as usize == 7 {
            *self
        } else {
            File::from_index(self.to_index() + 1)
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }
}
