use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::color::Color;

// TODO what to evaluate
// cannons -> diagonal > vertical > horizontal
// "aiming" to opponents side
// mobility -> how to evaluate
// overall capture score
pub fn eval(board: &Board) -> i16 {
    let me = board.player_pieces();
    let enemy = board.enemy_pieces();
    let pc = piece_count(me) - piece_count(enemy);
    let area = match board.side_to_move() {
        Color::White => area(me) - area(enemy.reverse()),
        Color::Black => area(me.reverse()) - area(enemy),
    };
    area + 4 * pc
}

fn piece_count(mask: BitBoard) -> i16 {
    mask.count_bits() as i16
}

fn area(mask: BitBoard) -> i16 {
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

#[test]
fn area_test() {
    let board = Board::start_position();
    println!("{}", area(board.player_pieces()));
}
