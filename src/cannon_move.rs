use crate::defs::*;
use crate::square::Square;
use core::cmp::Ordering;
use std::fmt;

const SRC_MASK: u16 = 0b0000_000000_111111;
const DST_MASK: u16 = 0b0000_111111_000000;
const SHOT_MASK: u16 = 0b0001_000000_000000;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct BitMove {
    pub data: u16,
}

impl BitMove {
    pub const fn new(m: u16) -> Self {
        Self { data: m }
    }

    pub const fn null() -> Self {
        Self { data: 0 }
    }

    pub const fn make(src: Square, dst: Square) -> Self {
        Self {
            data: src.0 as u16 | ((dst.0 as u16) << 6),
        }
    }

    pub const fn make_shot(src: Square, dst: Square) -> Self {
        Self {
            data: src.0 as u16 | ((dst.0 as u16) << 6) | (1 << 12),
        }
    }

    pub const fn dst(self) -> Square {
        Square(self.dst_u8())
    }

    pub const fn dst_u8(self) -> u8 {
        ((self.data & DST_MASK) >> 6) as u8
    }
    pub const fn src(self) -> Square {
        Square(self.src_u8())
    }

    pub const fn src_u8(self) -> u8 {
        (self.data & SRC_MASK) as u8
    }

    pub fn dst_rank(self) -> Rank {
        self.dst().rank()
    }

    pub fn dst_file(self) -> File {
        self.dst().file()
    }

    pub fn src_rank(self) -> Rank {
        self.src().rank()
    }

    pub fn src_file(self) -> File {
        self.src().file()
    }

    pub fn is_shot(self) -> bool {
        (self.data & SHOT_MASK) != 0
    }
}

impl fmt::Display for BitMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.src(), self.dst())
    }
}

#[derive(Eq, Copy, Clone)]
pub struct MoveWithScore {
    pub bit_move: BitMove,
    pub score: i16,
}

impl MoveWithScore {
    pub fn new(m: BitMove) -> Self {
        Self {
            bit_move: m,
            score: 0,
        }
    }

    pub fn new_with_score(m: BitMove, score: i16) -> Self {
        Self { bit_move: m, score }
    }

    pub fn bitmove(self) -> BitMove {
        self.bit_move
    }

    pub fn score(self) -> i16 {
        self.score
    }

    pub fn negate(mut self) -> Self {
        self.score = self.score.wrapping_neg();
        self
    }

    pub fn fill_move(mut self, m: BitMove) -> Self {
        self.bit_move = m;
        self
    }
}

impl Ord for MoveWithScore {
    fn cmp(&self, other: &MoveWithScore) -> Ordering {
        self.score.cmp(&other.score())
    }
}

impl PartialOrd for MoveWithScore {
    fn partial_cmp(&self, other: &MoveWithScore) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MoveWithScore {
    fn eq(&self, other: &MoveWithScore) -> bool {
        self.score == other.score()
    }
}
