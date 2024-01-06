use crate::bitboard::{BitBoard};

#[derive(Clone, Copy, Debug)]
pub struct Magic {
    pub mask: BitBoard,
    pub magic: u64,
    pub shift: u8,
}

impl Magic {
    pub fn get_magic_index(self: &Magic, blocker_bitboard: BitBoard) -> usize {
        let blockers = blocker_bitboard & self.mask;
        let hash = blockers.0.wrapping_mul(self.magic);
        (hash >> self.shift) as usize
    }
}