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
    #[inline(always)]
    pub const fn from_index(i: u8) -> Self {
        assert!(i < 8);
        ALL_FILES[i as usize]
    }

    #[inline(always)]
    pub fn left(&self) -> Option<Self> {
        if *self == File::A {
            None
        } else {
            Some(File::from_index(self.to_index() - 1))
        }
    }
    pub const fn const_left(&self) -> Option<Self> {
        LEFT_TABLE[self.to_index() as usize]
    }

    #[inline(always)]
    pub fn right(&self) -> Option<Self> {
        if *self == File::H {
            None
        } else {
            Some(File::from_index(self.to_index() + 1))
        }
    }
    pub const fn const_right(&self) -> Option<Self> {
        RIGHT_TABLE[self.to_index() as usize]
    }

    #[inline(always)]
    pub const fn to_index(&self) -> u8 {
        *self as u8
    }
}

impl Rank {
    #[inline(always)]
    pub const fn from_index(i: u8) -> Self {
        assert!(i < 8);
        ALL_RANKS[i as usize]
    }

    #[inline(always)]
    pub fn down(&self) -> Option<Self> {
        if *self == Rank::One {
            None
        } else {
            Some(Rank::from_index(self.to_index() - 1))
        }
    }

    pub const fn const_down(&self) -> Option<Self> {
        DOWN_TABLE[self.to_index() as usize]
    }

    #[inline(always)]
    pub fn up(&self) -> Option<Self> {
        if *self == Rank::Eight {
            None
        } else {
            Some(Rank::from_index(self.to_index() + 1))
        }
    }

    pub const fn const_up(&self) -> Option<Self> {
        UP_TABLE[self.to_index() as usize]
    }

    #[inline(always)]
    pub const fn to_index(&self) -> u8 {
        *self as u8
    }
}

pub const LEFT_TABLE: [Option<File>; 8] = [
    Some(File::B),
    Some(File::C),
    Some(File::D),
    Some(File::E),
    Some(File::F),
    Some(File::G),
    Some(File::H),
    None,
];

pub const RIGHT_TABLE: [Option<File>; 8] = [
    None,
    Some(File::B),
    Some(File::C),
    Some(File::D),
    Some(File::E),
    Some(File::F),
    Some(File::G),
    Some(File::H),
];

pub const DOWN_TABLE: [Option<Rank>; 8] = [
    Some(Rank::Two),
    Some(Rank::Three),
    Some(Rank::Four),
    Some(Rank::Five),
    Some(Rank::Six),
    Some(Rank::Seven),
    Some(Rank::Eight),
    None,
];

pub const UP_TABLE: [Option<Rank>; 8] = [
    None,
    Some(Rank::One),
    Some(Rank::Two),
    Some(Rank::Three),
    Some(Rank::Four),
    Some(Rank::Five),
    Some(Rank::Six),
    Some(Rank::Seven),
];
