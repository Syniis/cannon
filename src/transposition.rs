use crate::board::Board;
use crate::cannon_move::BitMove;

pub mod hash {
    pub static PIECES: [[u64; 64]; 2] = {
        let mut res = [[0; 64]; 2];
        let mut seed = 1070372u64;
        let mut idx = 0;
        let mut c = 0;
        while c < 2 {
            while idx < 64 {
                seed ^= seed >> 12;
                seed ^= seed << 25;
                seed ^= seed >> 27;
                res[c][idx] = seed.wrapping_mul(2685821657736338717u64);
                idx += 1;
            }
            idx = 0;
            c += 1
        }

        res
    };
    pub const SIDE: u64 = 13442441245975073873;
}

pub const TT_SIZE: usize = 10000000;
pub const FLAG_EXACT: u8 = 0x1;
pub const FLAG_UPPER: u8 = 0x2;
pub const FLAG_LOWER: u8 = 0x3;
pub const FLAGS: u8 = 0x3;
pub const AGE_INC: u8 = FLAGS + 1;
pub const AGE_MASK: u8 = !FLAGS;

#[derive(Clone, Default)]
pub struct TTEntry {
    pub hash: u64,
    pub mv: BitMove,
    score: i16,
    pub depth: u8,
    pub flag: u8,
}

impl TTEntry {
    pub fn is_some(&self) -> bool {
        self.mv != BitMove::null()
    }

    pub fn is_hit(&self, hash: u64) -> bool {
        self.hash == hash
    }

    pub fn is_exact(&self) -> bool {
        self.flag & FLAGS == FLAG_EXACT
    }

    pub fn is_lower(&self) -> bool {
        self.flag & FLAGS == FLAG_LOWER
    }

    pub fn is_upper(&self) -> bool {
        self.flag & FLAGS == FLAG_UPPER
    }

    pub fn score(&self) -> i16 {
        self.score
    }
}

#[derive(Default)]
pub struct TTable {
    entries: Vec<TTEntry>,
    index_mask: usize,
    age: u8,
}

impl TTable {
    pub fn increment_age(&mut self) {
        self.age = self.age.wrapping_add(AGE_INC);
    }

    pub fn allocate(&mut self, size: usize) {
        let entries = size.next_power_of_two();
        self.entries = vec![TTEntry::default(); entries];
        self.index_mask = entries - 1;
    }
    // Needs to be mut when we take into account changing age
    pub fn get(&mut self, hash: u64) -> Option<&TTEntry> {
        let entry = &mut self.entries[hash as usize & self.index_mask];
        match entry.is_hit(hash) {
            true => {
                entry.flag = self.age | (entry.flag & FLAGS);
                Some(entry)
            }
            false => None,
        }
    }
    pub fn insert(&mut self, board: &Board, score: i16, mv: BitMove, flag: u8, depth: u8) {
        let age_diff = |current_age: u8, entry_flag: u8| -> u8 {
            ((256 + FLAGS as i32 + current_age as i32 - entry_flag as i32) & AGE_MASK as i32) as u8
        };
        let entry = &mut self.entries[board.hash() as usize & self.index_mask];
        if entry.depth < depth + age_diff(self.age, entry.flag) {
            entry.hash = board.hash();
            entry.mv = mv;
            entry.score = score;
            entry.depth = depth;
            entry.flag = flag;
        }
    }
}
