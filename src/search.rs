use crate::board::Board;
use crate::cannon_move::BitMove;
use crate::cannon_move::MoveWithScore;
use crate::eval::eval;
use rayon;

pub fn alpha_beta_search(
    board: &mut Board,
    mut alpha: i16,
    beta: i16,
    depth: u16,
) -> MoveWithScore {
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
        if m.dst() == board.enemy_castle().to_square() {
            return MoveWithScore::new_with_score(m, 5000);
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

pub fn jamboree(board: &mut Board, mut alpha: i16, beta: i16, depth: u16) -> MoveWithScore {
    assert!(alpha <= beta);
    if depth <= 2 {
        return alpha_beta_search(board, alpha, beta, depth);
    }

    let mut moves = board.generate_moves();

    if moves.is_empty() {
        return MoveWithScore::new_with_score(BitMove::null(), 0);
    }

    let (mvs, _) = moves.moves.split_at_mut(moves.len);
    let amount_seq = 1 + (mvs.len() / 4).min(2);
    let (seq, non_seq) = mvs.split_at_mut(amount_seq);
    let mut best_move = MoveWithScore::new_with_score(BitMove::null(), alpha);

    for m in seq {
        if m.dst() == board.enemy_castle().to_square() {
            return MoveWithScore::new_with_score(*m, 5000);
        }
        board.apply_move(*m);

        let score = -jamboree(board, -beta, -alpha, depth - 1).score();
        let result = MoveWithScore::new_with_score(*m, score);
        board.undo_move();
        if result.score > alpha {
            alpha = result.score;
            if alpha >= beta {
                return result;
            }
            best_move = result;
        }
    }

    parallel_search(non_seq, board, alpha, beta, depth).max(best_move)
}

pub fn parallel_search(
    moves: &mut [BitMove],
    board: &mut Board,
    mut alpha: i16,
    beta: i16,
    depth: u16,
) -> MoveWithScore {
    if moves.len() <= 5 {
        let mut best_move = MoveWithScore::new_with_score(BitMove::null(), alpha);

        for m in moves {
            if m.dst() == board.enemy_castle().to_square() {
                return MoveWithScore::new_with_score(*m, 5000);
            }
            board.apply_move(*m);

            let score = -jamboree(board, -beta, -alpha, depth - 1).score();
            let result = MoveWithScore::new_with_score(*m, score);
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
    } else {
        let mid = moves.len() / 2;
        let (left, right) = moves.split_at_mut(mid);
        let mut board_copy = board.clone();

        let (best_left, best_right) = rayon::join(
            || parallel_search(left, &mut board_copy, alpha, beta, depth),
            || parallel_search(right, board, alpha, beta, depth),
        );

        best_left.max(best_right)
    }
}
