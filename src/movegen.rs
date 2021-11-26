use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::cannon_move::BitMove;
use crate::color::*;
use crate::square::Square;
use crate::tables::*;

pub const MAX_MOVES: usize = 64;
pub struct MoveList {
    moves: [BitMove; MAX_MOVES],
    pub len: usize,
    pub idx: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [BitMove::null(); MAX_MOVES],
            len: 0,
            idx: 0,
        }
    }
}

impl MoveList {
    pub fn push(&mut self, mv: BitMove) {
        if self.len < MAX_MOVES {
            self.moves[self.len] = mv;
            self.len += 1;
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl ExactSizeIterator for MoveList {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Iterator for MoveList {
    type Item = BitMove;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len;
        (len, Some(len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            let m = self.moves[self.idx];
            self.idx += 1;
            Some(m)
        }
    }
}

impl FromIterator<BitMove> for MoveList {
    fn from_iter<T: IntoIterator<Item = BitMove>>(iter: T) -> Self {
        let mut list = MoveList::default();
        for i in iter {
            list.push(i);
        }
        list
    }
}

pub struct MoveGen {
    moves: MoveList,
}

impl MoveGen {
    pub fn generate(board: &Board) -> MoveList {
        let mut move_list = MoveList::default();

        MoveGen::generate_side_moves(&mut move_list, &board);
        MoveGen::generate_cannon_shots(&mut move_list, &board);
        MoveGen::generate_forward_moves(&mut move_list, &board);
        MoveGen::generate_cannon_jumps(&mut move_list, &board);
        MoveGen::generate_retreats(&mut move_list, &board);
        move_list
    }

    fn generate_forward_moves(moves: &mut MoveList, board: &Board) {
        let mut pieces = board.pieces_with_color(board.side_to_move());
        let mask = !pieces;
        // While some piece left (mask not empty), pop lsb = src
        // Get the forward bitboard for that position (TODO precompute the tables)
        // and & it with the initial mask
        while let Some(src) = pieces.pop_some_lsb() {
            let forwards = front(board.side_to_move(), src);
            let forwards = forwards & mask;
            for dst in forwards {
                moves.push(BitMove::make(src, dst));
            }
        }
    }

    fn generate_side_moves(moves: &mut MoveList, board: &Board) {
        let mut pieces = board.pieces_with_color(board.side_to_move());
        let enemy_pieces = board.pieces_with_color(!board.side_to_move());

        while let Some(src) = pieces.pop_some_lsb() {
            let sides = sides(board.side_to_move(), src);
            let sides = sides & enemy_pieces;

            for dst in sides {
                moves.push(BitMove::make(src, dst));
            }
        }
    }

    fn generate_retreats(moves: &mut MoveList, board: &Board) {
        let my_color = board.side_to_move();
        let enemy_color = !my_color;
        let mut my_pieces = board.pieces_with_color(my_color);
        let enemy_pieces = board.pieces_with_color(enemy_color);
        let unoccupied = !board.pieces();

        while let Some(src) = my_pieces.pop_some_lsb() {
            let adj = front(my_color, src) | sides(my_color, src);
            // there is enemy piece adjacent
            if (adj & enemy_pieces).is_not_empty() {
                // non occupied retreats
                let retreats = retreats(my_color, src) & unoccupied;
                for dst in retreats {
                    if (between(src, dst) & unoccupied).is_not_empty() {
                        moves.push(BitMove::make(src, dst));
                    }
                }
            }
        }
    }

    fn generate_cannon_jumps(moves: &mut MoveList, board: &Board) {
        let my_color = board.side_to_move();
        let mut my_pieces = board.pieces_with_color(my_color);
        let my_pieces_original = my_pieces;
        let unoccupied = !board.pieces_with_castles();

        while let Some(src) = my_pieces.pop_some_lsb() {
            let possible_jumps = distance_ring(src, 3) & unoccupied;
            for dst in possible_jumps {
                let between = between(src, dst);
                if between.is_not_empty() && (between & my_pieces_original) == between {
                    moves.push(BitMove::make(src, dst));
                }
            }
        }
    }

    fn generate_cannon_shots(moves: &mut MoveList, board: &Board) {
        let my_color = board.side_to_move();
        let enemy_color = !my_color;
        let mut my_pieces = board.pieces_with_color(my_color);
        let my_pieces_original = my_pieces;
        let enemy_pieces = board.pieces_with_color(enemy_color);
        let unoccupied = !board.pieces();

        while let Some(src) = my_pieces.pop_some_lsb() {
            for d in 0..8 {
                if (shot_blocker(src, d) & unoccupied).is_not_empty() {
                    if shot_body(src, d).is_not_empty()
                        && (shot_body(src, d) & my_pieces_original) == shot_body(src, d)
                    {
                        let shots = shot_targets(src, d) & enemy_pieces;
                        for dst in shots {
                            moves.push(BitMove::make_shot(src, dst));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn move_test() {
    init();
    let board = Board::start_position();
    let rev = board.pieces().reverse();
    println!("{}", rev);
    let moves = board.generate_moves_for(Square::A1);
    //let moves = board.generate_moves();
    for m in moves {
        println!("{} -> {}", m.src(), m.dst());
    }
}
