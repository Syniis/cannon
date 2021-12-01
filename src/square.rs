use crate::color::Color;
use crate::defs::*;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Square(pub u8);

pub const DEFAULT: Square = Square(0);

impl Default for Square {
    fn default() -> Square {
        Square::new(0)
    }
}
pub const NUM_SQUARES: usize = 64;

impl Square {
    #[inline(always)]
    pub fn new(sq: u8) -> Self {
        assert!(sq < 64);
        Self(sq)
    }

    #[inline(always)]
    pub fn make_square(rank: Rank, file: File) -> Self {
        Self((rank.to_index() as u8) << 3 ^ (file.to_index() as u8))
    }

    #[inline(always)]
    pub fn rank(&self) -> Rank {
        assert!(self.is_okay());
        Rank::from_index(self.rank_index())
    }

    #[inline(always)]
    pub fn file(&self) -> File {
        assert!(self.is_okay());
        File::from_index(self.file_index())
    }

    #[inline(always)]
    pub fn rank_index(&self) -> u8 {
        assert!(self.is_okay());
        self.0 >> 3
    }

    #[inline(always)]
    pub fn file_index(&self) -> u8 {
        assert!(self.is_okay());
        self.0 & 7
    }

    #[inline]
    pub fn up(&self) -> Option<Self> {
        self.rank()
            .up()
            .and_then(|r| Some(Square::make_square(r, self.file())))
    }

    #[inline]
    pub fn down(&self) -> Option<Self> {
        self.rank()
            .down()
            .and_then(|r| Some(Square::make_square(r, self.file())))
    }

    #[inline]
    pub fn left(&self) -> Option<Self> {
        self.file()
            .left()
            .and_then(|f| Some(Square::make_square(self.rank(), f)))
    }

    #[inline]
    pub fn right(&self) -> Option<Self> {
        self.file()
            .right()
            .and_then(|f| Some(Square::make_square(self.rank(), f)))
    }

    #[inline]
    pub fn forward(&self, color: Color) -> Option<Self> {
        match color {
            Color::White => self.up(),
            Color::Black => self.down(),
        }
    }

    #[inline]
    pub fn backward(&self, color: Color) -> Option<Self> {
        self.forward(!color)
    }

    #[inline]
    pub fn flip(self) -> Self {
        Self(self.0 ^ 0b111000)
    }

    #[inline(always)]
    pub fn to_u8(&self) -> u8 {
        self.0
    }

    #[inline(always)]
    pub fn to_index(&self) -> usize {
        self.0 as usize
    }

    pub const fn is_okay(self) -> bool {
        self.0 < 64
    }

    pub const A1: Square = Square(0b000000);
    pub const B1: Square = Square(0b000001);
    pub const C1: Square = Square(0b000010);
    pub const D1: Square = Square(0b000011);
    pub const E1: Square = Square(0b000100);
    pub const F1: Square = Square(0b000101);
    pub const G1: Square = Square(0b000110);
    pub const H1: Square = Square(0b000111);
    pub const A2: Square = Square(0b001000);
    pub const B2: Square = Square(0b001001);
    pub const C2: Square = Square(0b001010);
    pub const D2: Square = Square(0b001011);
    pub const E2: Square = Square(0b001100);
    pub const F2: Square = Square(0b001101);
    pub const G2: Square = Square(0b001110);
    pub const H2: Square = Square(0b001111);
    pub const A3: Square = Square(0b010000);
    pub const B3: Square = Square(0b010001);
    pub const C3: Square = Square(0b010010);
    pub const D3: Square = Square(0b010011);
    pub const E3: Square = Square(0b010100);
    pub const F3: Square = Square(0b010101);
    pub const G3: Square = Square(0b010110);
    pub const H3: Square = Square(0b010111);
    pub const A4: Square = Square(0b011000);
    pub const B4: Square = Square(0b011001);
    pub const C4: Square = Square(0b011010);
    pub const D4: Square = Square(0b011011);
    pub const E4: Square = Square(0b011100);
    pub const F4: Square = Square(0b011101);
    pub const G4: Square = Square(0b011110);
    pub const H4: Square = Square(0b011111);
    pub const A5: Square = Square(0b100000);
    pub const B5: Square = Square(0b100001);
    pub const C5: Square = Square(0b100010);
    pub const D5: Square = Square(0b100011);
    pub const E5: Square = Square(0b100100);
    pub const F5: Square = Square(0b100101);
    pub const G5: Square = Square(0b100110);
    pub const H5: Square = Square(0b100111);
    pub const A6: Square = Square(0b101000);
    pub const B6: Square = Square(0b101001);
    pub const C6: Square = Square(0b101010);
    pub const D6: Square = Square(0b101011);
    pub const E6: Square = Square(0b101100);
    pub const F6: Square = Square(0b101101);
    pub const G6: Square = Square(0b101110);
    pub const H6: Square = Square(0b101111);
    pub const A7: Square = Square(0b110000);
    pub const B7: Square = Square(0b110001);
    pub const C7: Square = Square(0b110010);
    pub const D7: Square = Square(0b110011);
    pub const E7: Square = Square(0b110100);
    pub const F7: Square = Square(0b110101);
    pub const G7: Square = Square(0b110110);
    pub const H7: Square = Square(0b110111);
    pub const A8: Square = Square(0b111000);
    pub const B8: Square = Square(0b111001);
    pub const C8: Square = Square(0b111010);
    pub const D8: Square = Square(0b111011);
    pub const E8: Square = Square(0b111100);
    pub const F8: Square = Square(0b111101);
    pub const G8: Square = Square(0b111110);
    pub const H8: Square = Square(0b111111);
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (('a' as u8) + ((self.0 & 7) as u8)) as char,
            (('1' as u8) + ((self.0 >> 3) as u8)) as char
        )
    }
}
pub const ALL_SQUARES: [Square; 64] = [
    Square(0),
    Square(1),
    Square(2),
    Square(3),
    Square(4),
    Square(5),
    Square(6),
    Square(7),
    Square(8),
    Square(9),
    Square(10),
    Square(11),
    Square(12),
    Square(13),
    Square(14),
    Square(15),
    Square(16),
    Square(17),
    Square(18),
    Square(19),
    Square(20),
    Square(21),
    Square(22),
    Square(23),
    Square(24),
    Square(25),
    Square(26),
    Square(27),
    Square(28),
    Square(29),
    Square(30),
    Square(31),
    Square(32),
    Square(33),
    Square(34),
    Square(35),
    Square(36),
    Square(37),
    Square(38),
    Square(39),
    Square(40),
    Square(41),
    Square(42),
    Square(43),
    Square(44),
    Square(45),
    Square(46),
    Square(47),
    Square(48),
    Square(49),
    Square(50),
    Square(51),
    Square(52),
    Square(53),
    Square(54),
    Square(55),
    Square(56),
    Square(57),
    Square(58),
    Square(59),
    Square(60),
    Square(61),
    Square(62),
    Square(63),
];
