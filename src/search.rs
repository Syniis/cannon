use crate::board::Board;
use crate::cannon_move::BitMove;
use crate::cannon_move::MoveWithScore;
use crate::eval::eval;

// TODO find good values
const QUIESCENCE_DEPTH: u16 = 14;
const WINDOW: i16 = 6;
const WINDOW_DEPTH_FACTOR: i16 = 4;
const NEG_INF: i16 = -9999;
const INF: i16 = 9999;
const WIN: i16 = 5000;
const FUTILITY_CUTOFF: [i16; 3] = [8, 15, 25];

// TODO futility pruning
// TODO delta pruning
// TODO static exchange evaluation (this should be quite important for this game)
pub fn search(board: &mut Board, max_depth: u16) -> MoveWithScore {
    let mut current_depth = 5;
    let mut alpha = NEG_INF;
    let mut beta = INF;
    let mut best_move = MoveWithScore::new_with_score(BitMove::null(), alpha);

    while current_depth <= max_depth {
        println!("Searching at {}", current_depth);
        let mut b = board.shallow_clone();
        let (m, n) = alpha_beta_search(&mut b, alpha, beta, current_depth, 0);
        println!("Nodes searched {}", n);
        if m.score <= alpha {
            alpha = NEG_INF;
        } else if m.score >= beta {
            beta = INF;
        } else {
            if m.bitmove() != BitMove::null() {
                alpha =
                    m.score - (WINDOW + (max_depth - current_depth) as i16 * WINDOW_DEPTH_FACTOR);
                beta =
                    m.score + (WINDOW + (max_depth - current_depth) as i16 * WINDOW_DEPTH_FACTOR);
                best_move = m;
                println!(
                    "Best move {}, score {} alpha {}, beta, {}",
                    m.bitmove(),
                    m.score(),
                    alpha,
                    beta
                );
            }
            current_depth += 1;
        }
    }
    best_move
}
pub fn alpha_beta_search(
    board: &mut Board,
    mut alpha: i16,
    beta: i16,
    depth: u16,
    nodes: u64,
) -> (MoveWithScore, u64) {
    if depth == 0 {
        if board.last_capture() {
            return quiescence(board, alpha, beta, QUIESCENCE_DEPTH, 0);
        }
        return (
            MoveWithScore::new_with_score(BitMove::null(), eval(board)),
            1,
        );
    }
    if depth <= 3 && !board.last_capture() {
        assert!(depth != 0);
        let eval = eval(board);
        if eval + FUTILITY_CUTOFF[depth as usize - 1] < alpha {
            return quiescence(board, alpha, beta, QUIESCENCE_DEPTH, 0);
        }
    }
    let moves = board.generate_moves();

    if moves.is_empty() {
        return (MoveWithScore::new_with_score(BitMove::null(), -5000), 1);
    }

    let mut best_move = MoveWithScore::new_with_score(BitMove::null(), alpha);
    let mut new_nodes = nodes;
    for m in moves {
        if m.dst() == board.enemy_castle().to_square() {
            return (MoveWithScore::new_with_score(m, WIN), new_nodes);
        }
        board.apply_move(m);
        let (ret, n) = alpha_beta_search(board, -beta, -alpha, depth - 1, 0);
        new_nodes += n;
        let score = -ret.score();
        let result = MoveWithScore::new_with_score(m, score);
        board.undo_move();
        if result.score > alpha {
            alpha = result.score;
            if alpha >= beta {
                return (result, new_nodes);
            }
            best_move = result;
        }
    }
    (best_move, new_nodes)
}

pub fn quiescence(
    board: &mut Board,
    mut alpha: i16,
    beta: i16,
    depth: u16,
    nodes: u64,
) -> (MoveWithScore, u64) {
    if depth == 0 {
        return (
            MoveWithScore::new_with_score(BitMove::null(), eval(board)),
            1,
        );
    }

    let moves = board.generate_captures();

    if moves.is_empty() {
        return (
            MoveWithScore::new_with_score(BitMove::null(), eval(board)),
            1,
        );
    }

    let mut best_move = MoveWithScore::new_with_score(BitMove::null(), alpha);
    let mut new_nodes = nodes;
    for m in moves {
        if m.dst() == board.enemy_castle().to_square() {
            return (MoveWithScore::new_with_score(m, WIN), new_nodes);
        }
        board.apply_move(m);
        let (ret, n) = quiescence(board, -beta, -alpha, depth - 1, 0);
        new_nodes += n;
        let score = -ret.score();
        let result = MoveWithScore::new_with_score(m, score);
        board.undo_move();
        if result.score > alpha {
            alpha = result.score;
            if alpha >= beta {
                return (result, new_nodes);
            }
            best_move = result;
        }
    }
    (best_move, new_nodes)
}
