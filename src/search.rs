use crate::board::Board;
use crate::cannon_move::BitMove;
use crate::cannon_move::MoveWithScore;
use crate::eval::eval;

pub fn best_move(board: Board, depth: u16) -> MoveWithScore {
    let alpha = -9999;
    let beta = 9999;
    alpha_beta_search(&mut board.clone(), alpha, beta, depth)
}
fn alpha_beta_search(board: &mut Board, mut alpha: i16, beta: i16, depth: u16) -> MoveWithScore {
    if depth == 0 {
        let score = eval(board);
        //println!("{}", score);
        return MoveWithScore::new_with_score(BitMove::null(), score);
    }

    let moves = board.generate_moves();

    if moves.is_empty() {
        return MoveWithScore::new_with_score(BitMove::null(), 0);
    }

    let mut best_move = MoveWithScore::new_with_score(BitMove::null(), alpha);

    for m in moves {
        if m == BitMove::null() {
            panic!(":()");
        }
        board.apply_move(m);
        let score = -alpha_beta_search(board, -beta, -alpha, depth - 1).score();
        let result = MoveWithScore::new_with_score(m, score);
        board.undo_move();
        if result.score > alpha {
            alpha = result.score;
            if alpha >= beta {
                return result;
            }
            best_move = result;
        }
    }
    best_move
}
