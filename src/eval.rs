use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::color::Color;
use crate::square::Square;
use crate::tables::{distance_ring, distance_square};

const PIECE_VALUE: i16 = 8;
// TODO what to evaluate
// cannons -> diagonal > vertical > horizontal
// "aiming" to opponents side
// mobility -> how to evaluate
// overall capture score
#[inline]
pub fn eval(board: &Board) -> i16 {
    let me = board.player_pieces();
    let my_castle = board.castle_with_color(board.side_to_move()).to_square();
    let enemy = board.enemy_pieces();
    let enemy_castle = board.enemy_castle().to_square();
    let pc = piece_count(me) - piece_count(enemy);
    let area = match board.side_to_move() {
        Color::White => area(me) - area(enemy.reverse()),
        Color::Black => area(me.reverse()) - area(enemy),
    };
    // div 2 to reduce double counting a bit
    let surround = surround(me, enemy) / 2;
    let castle_distance = castle_distance(me, my_castle, enemy_castle)
        - castle_distance(enemy, enemy_castle, my_castle);
    // TODO come up with a way to normalize these parameters
    (PIECE_VALUE * pc) + area + surround + castle_distance
}

#[inline(always)]
fn piece_count(mask: BitBoard) -> i16 {
    mask.count_bits() as i16
}

#[inline]
fn surround(me: BitBoard, enemy: BitBoard) -> i16 {
    let mut score = 0;
    let combined = me | enemy;

    for sq in combined {
        let surround = distance_ring(sq, 1);
        score += (me & surround).count_bits() as i16 - (enemy & surround).count_bits() as i16;
    }
    score
}

#[inline]
fn castle_distance(mask: BitBoard, my_castle: Square, enemy_castle: Square) -> i16 {
    // maybe give boni if near castle instead of negative if away from both
    const CUTOFF: i16 = 3;
    const ENEMY_IMPORTANCE: i16 = 1;
    const MY_IMPORTANCE: i16 = 0;
    let mut score = 0;
    for sq in mask {
        let s = (distance_square(sq, my_castle) as i16 - MY_IMPORTANCE)
            .min(distance_square(sq, enemy_castle) as i16 - ENEMY_IMPORTANCE)
            - CUTOFF;
        if s > 0 {
            score -= s;
        }
    }
    score
}

#[inline]
fn area(mask: BitBoard) -> i16 {
    // TODO make this better by having a min area too and then weighting those against the castles positions (this is kinda covered by castle dist already)
    // near own castle min area is more important
    // near enemy castle max area more important
    let mut max_rank_in_file: [u8; 8] = [0; 8];

    for sq in mask {
        let r = sq.rank_index() + 1;
        let f = sq.file_index() as usize;
        max_rank_in_file[f] = max_rank_in_file[f].max(r);
        max_rank_in_file[f.saturating_sub(1)] = max_rank_in_file[f.saturating_sub(1)].max(r);
        max_rank_in_file[(f + 1).min(7)] = max_rank_in_file[(f + 1).min(7)].max(r);
    }

    max_rank_in_file.iter().sum::<u8>() as i16
}
