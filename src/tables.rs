use crate::bitboard::*;
use crate::color::*;
use crate::square::Square;
use crate::square::ALL_SQUARES;

// TODO make this safe (remove mut)
static mut FRONT: [[BitBoard; 64]; 2] = [[EMPTY; 64]; 2];
static mut SIDES: [[BitBoard; 64]; 2] = [[EMPTY; 64]; 2];
static mut RETREATS: [[BitBoard; 64]; 2] = [[EMPTY; 64]; 2];
static mut DISTANCES: [[u8; 64]; 64] = [[0; 64]; 64];
static mut DISTANCE_RINGS: [[BitBoard; 64]; 8] = [[EMPTY; 64]; 8];
static mut DIAGONALS: [BitBoard; 64] = [EMPTY; 64];
static mut ORTHOGONALS: [BitBoard; 64] = [EMPTY; 64];
static mut BETWEEN: [[BitBoard; 64]; 64] = [[EMPTY; 64]; 64];
// TODO consider splitting this into 3 different arrays
static mut SHOTS: [[[BitBoard; 64]; 8]; 3] = [[[EMPTY; 64]; 8]; 3];

pub fn generate_front() {
    for c in ALL_COLORS.iter() {
        for src in ALL_SQUARES.iter() {
            let forward = src.forward(*c);
            let forward_left = forward.map_or(EMPTY, |sq| {
                sq.left().map_or(EMPTY, |sq| BitBoard::from_square(sq))
            });
            let forward_right = forward.map_or(EMPTY, |sq| {
                sq.right().map_or(EMPTY, |sq| BitBoard::from_square(sq))
            });
            let forward = forward.map_or(EMPTY, |sq| BitBoard::from_square(sq));

            let combined = forward | forward_left | forward_right;

            unsafe {
                FRONT[c.to_index()][src.to_index()] = combined;
            }
        }
    }
}

pub fn generate_sides() {
    for c in ALL_COLORS.iter() {
        for src in ALL_SQUARES.iter() {
            let left = src.left().map_or(EMPTY, |sq| BitBoard::from_square(sq));
            let right = src.right().map_or(EMPTY, |sq| BitBoard::from_square(sq));
            let combined = left | right;

            unsafe {
                SIDES[c.to_index()][src.to_index()] = combined;
            }
        }
    }
}

pub fn generate_retreats() {
    for c in ALL_COLORS.iter() {
        for src in ALL_SQUARES.iter() {
            let two_down = src.backward(*c).and_then(|sq| sq.backward(*c));

            let left = two_down
                .and_then(|sq| sq.left().and_then(|sq| sq.left()))
                .map_or(EMPTY, |sq| BitBoard::from_square(sq));
            let right = two_down
                .and_then(|sq| sq.right().and_then(|sq| sq.right()))
                .map_or(EMPTY, |sq| BitBoard::from_square(sq));
            let two_down = two_down.map_or(EMPTY, |sq| BitBoard::from_square(sq));
            let combined = two_down | left | right;
            unsafe {
                RETREATS[c.to_index()][src.to_index()] = combined;
            }
        }
    }
}

pub fn generate_distances() {
    for i in 0..64 as u8 {
        for j in 0..64 as u8 {
            unsafe {
                let sqi = Square::new(i);
                let sqj = Square::new(j);
                DISTANCES[i as usize][j as usize] = (sqi.rank_index().abs_diff(sqj.rank_index()))
                    .max(sqi.file_index().abs_diff(sqj.file_index()));
            }
        }
    }
}

pub fn generate_distance_rings() {
    for i in 0..64 {
        for j in 0..64 {
            if i != j {
                let dist = distance_index(i, j) as usize;
                unsafe { DISTANCE_RINGS[dist - 1][i] |= (1 as u64) << (j as usize) }
            }
        }
    }
}

pub fn generate_diagonals() {
    for i in 0..64 {
        let src = Square::new(i);
        let res = ALL_SQUARES
            .iter()
            .filter(|dst| {
                src.rank_index().abs_diff(dst.rank_index())
                    == (src.file_index().abs_diff(dst.file_index()))
                    && src != **dst
            })
            .fold(EMPTY, |b, s| b | BitBoard::from_square(*s));
        unsafe { DIAGONALS[i as usize] = res }
    }
}

pub fn generate_orthogonals() {
    for i in 0..64 {
        let src = Square::new(i);
        let res = ALL_SQUARES
            .iter()
            .filter(|dst| {
                (src.rank_index() == dst.rank_index() || src.file_index() == dst.file_index())
                    && src != **dst
            })
            .fold(EMPTY, |b, s| b | BitBoard::from_square(*s));
        unsafe { ORTHOGONALS[i as usize] = res }
    }
}

pub fn generate_between() {
    let between = |a: u8, t: u8, b: u8| -> bool {
        if a < b {
            a < t && t < b
        } else {
            b < t && t < a
        }
    };

    let between_square = |sqa: Square, sqt: Square, sqb: Square| -> bool {
        between(sqa.file_index(), sqt.file_index(), sqb.file_index())
            || between(sqa.rank_index(), sqt.rank_index(), sqb.rank_index())
    };
    for (i, src) in ALL_SQUARES.iter().enumerate() {
        for (j, dst) in ALL_SQUARES.iter().enumerate() {
            let dst_bb = BitBoard::from_square(*dst);

            if (orthogonals(*src) & dst_bb).is_not_empty() {
                let bb = orthogonals(*src) & orthogonals(*dst);
                let res = bb
                    .into_iter()
                    .filter(|sq| between_square(*src, *sq, *dst))
                    .fold(EMPTY, |b, s| b | BitBoard::from_square(s));
                unsafe { BETWEEN[i as usize][j as usize] = res }
            } else if (diagonals(*src) & dst_bb).is_not_empty() {
                let bb = diagonals(*src) & diagonals(*dst);
                let res = bb
                    .into_iter()
                    .filter(|sq| between_square(*src, *sq, *dst))
                    .fold(EMPTY, |b, s| b | BitBoard::from_square(s));
                unsafe { BETWEEN[i as usize][j as usize] = res }
            } else {
                unsafe { BETWEEN[i as usize][j as usize] = EMPTY }
            }
        }
    }
}

pub fn generate_shots() {
    let direction_to_index = |src: Square, dst: Square| -> usize {
        let x = src.file_index() as i8 - dst.file_index() as i8;
        let y = src.rank_index() as i8 - dst.rank_index() as i8;
        let x = x.clamp(-1, 1);
        let y = y.clamp(-1, 1);
        match x {
            -1 => match y {
                -1 => 0,
                0 => 1,
                1 => 2,
                _ => panic!("Should never happen -1 OOR"),
            },
            0 => match y {
                -1 => 3,
                0 => panic!("Should never happen 0, 0"),
                1 => 4,
                _ => panic!("Should never happen 0 OOR"),
            },
            1 => match y {
                -1 => 5,
                0 => 6,
                1 => 7,
                _ => panic!("Should never happen 1 OOR"),
            },
            _ => panic!("Should never happen"),
        }
    };

    for (idx, sq) in ALL_SQUARES.iter().enumerate() {
        let blocker = distance_dir_ring(*sq, 1);
        for b in blocker {
            let dir = direction_to_index(*sq, b);
            unsafe { SHOTS[0][dir][idx] = BitBoard::from_square(b) }
        }

        let cannon_tail = distance_dir_ring(*sq, 2);
        for tail in cannon_tail {
            let between = between(*sq, tail);
            // find the index of the opposite direction
            let dir = direction_to_index(between.to_square(), *sq);
            let combined = BitBoard::from_square(tail) | between;
            unsafe { SHOTS[1][dir][idx] = combined }
        }

        let dist_2_shots = distance_dir_ring(*sq, 2);
        for shot in dist_2_shots {
            let dir = direction_to_index(*sq, shot);
            unsafe { SHOTS[2][dir][idx] = BitBoard::from_square(shot) }
        }

        let dist_3_shots = distance_dir_ring(*sq, 3);
        for shot in dist_3_shots {
            let dir = direction_to_index(*sq, shot);
            unsafe { SHOTS[2][dir][idx] |= BitBoard::from_square(shot) }
        }
    }
}

pub fn init() {
    generate_front();
    generate_sides();
    generate_retreats();
    generate_distances();
    generate_distance_rings(); // depends on distances
    generate_diagonals();
    generate_orthogonals();
    generate_between(); // depends on diagonals and orthogonals
    generate_shots();
}

#[inline(always)]
pub fn front(color: Color, square: Square) -> BitBoard {
    unsafe { FRONT[color.to_index()][square.to_index()] }
}

#[inline(always)]
pub fn sides(color: Color, square: Square) -> BitBoard {
    unsafe { SIDES[color.to_index()][square.to_index()] }
}

#[inline(always)]
pub fn retreats(color: Color, square: Square) -> BitBoard {
    unsafe { RETREATS[color.to_index()][square.to_index()] }
}

#[inline(always)]
pub fn distance_index(from: usize, to: usize) -> u8 {
    unsafe { DISTANCES[from][to] }
}

#[inline(always)]
pub fn distance_square(from: Square, to: Square) -> u8 {
    unsafe { DISTANCES[from.to_index()][to.to_index()] }
}

#[inline(always)]
pub fn distance_ring(square: Square, dist: usize) -> BitBoard {
    assert!(dist > 0 && dist <= 7);
    unsafe { DISTANCE_RINGS[dist - 1][square.to_index()] }
}

#[inline(always)]
pub fn distance_dir_ring(src: Square, dist: usize) -> BitBoard {
    distance_ring(src, dist) & (diagonals(src) | orthogonals(src))
}

#[inline(always)]
pub fn diagonals(square: Square) -> BitBoard {
    unsafe { DIAGONALS[square.to_index()] }
}

#[inline(always)]
pub fn orthogonals(square: Square) -> BitBoard {
    unsafe { ORTHOGONALS[square.to_index()] }
}

#[inline(always)]
pub fn between(src: Square, dst: Square) -> BitBoard {
    unsafe { BETWEEN[src.to_index()][dst.to_index()] }
}

#[inline(always)]
pub fn shot_blocker(src: Square, dir: usize) -> BitBoard {
    unsafe { SHOTS[0][dir][src.to_index()] }
}

#[inline(always)]
pub fn shot_body(src: Square, dir: usize) -> BitBoard {
    unsafe { SHOTS[1][dir][src.to_index()] }
}

#[inline(always)]
pub fn shot_targets(src: Square, dir: usize) -> BitBoard {
    unsafe { SHOTS[2][dir][src.to_index()] }
}

#[test]
fn tables_test() {
    init();
    let middle = Square::D4;

    let mut all_shots = EMPTY;
    let middle_bb = BitBoard::from_square(middle);
    for i in 0..8 {
        println!(
            "{}",
            shot_targets(middle, i) | shot_body(middle, i) | middle_bb
        );
        //all_shots |= shot_blocker(middle, i);
    }
}
