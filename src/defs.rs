#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
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

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Rank {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

pub const NUM_RANKS: usize = 8;
pub const ALL_RANKS: [Rank; NUM_RANKS] = [
    Rank::One,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
];

impl File {
    pub fn from_index(i: u8) -> Self {
        assert!(i < 8);
        ALL_FILES[i as usize]
    }

    pub fn left(&self) -> Option<Self> {
        if *self == File::A {
            None
        } else {
            Some(File::from_index(self.to_index() - 1))
        }
    }

    pub fn right(&self) -> Option<Self> {
        if *self == File::H {
            None
        } else {
            Some(File::from_index(self.to_index() + 1))
        }
    }

    pub fn to_index(&self) -> u8 {
        *self as u8
    }
}

impl Rank {
    pub fn from_index(i: u8) -> Self {
        assert!(i < 8);
        ALL_RANKS[i as usize]
    }

    pub fn down(&self) -> Option<Self> {
        if *self == Rank::One {
            None
        } else {
            Some(Rank::from_index(self.to_index() - 1))
        }
    }

    pub fn up(&self) -> Option<Self> {
        if *self == Rank::Eight {
            None
        } else {
            Some(Rank::from_index(self.to_index() + 1))
        }
    }

    pub fn to_index(&self) -> u8 {
        *self as u8
    }
}
