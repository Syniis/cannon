use crate::bits::*;
use crate::square::Square;
use std::fmt;
use std::ops::*;

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct BitBoard(pub u64);

pub const EMPTY: BitBoard = BitBoard(0);

impl_bit_ops!(BitBoard, u64);

impl BitBoard {
    #[inline(always)]
    pub const fn new(b: u64) -> Self {
        Self(b)
    }

    #[inline(always)]
    pub const fn from_square(sq: Square) -> Self {
        Self(1u64 << sq.to_u8())
    }

    #[inline(always)]
    pub const fn count_bits(self) -> u32 {
        self.0.count_ones()
    }

    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub const fn is_not_empty(self) -> bool {
        !self.is_empty()
    }

    #[inline(always)]
    pub const fn to_square(&self) -> Square {
        Square(self.lsb_u8())
    }

    #[inline(always)]
    pub const fn reverse(&self) -> Self {
        Self(self.0.swap_bytes())
    }

    #[inline(always)]
    pub const fn lsb_square(self) -> Square {
        Square(self.lsb_u8())
    }

    #[inline(always)]
    pub fn pop_lsb(&mut self) -> Square {
        let square = self.lsb_square();
        *self &= *self - 1;
        square
    }

    #[inline(always)]
    pub fn pop_some_lsb(&mut self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(self.pop_lsb())
        }
    }

    #[inline(always)]
    pub const fn lsb_u8(self) -> u8 {
        bit_scan_forward(self.0)
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_some_lsb()
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &string_u64(self.0.swap_bytes());
        f.pad(s)
    }
}
