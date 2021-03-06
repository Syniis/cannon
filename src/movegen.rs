use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::cannon_move::BitMove;
use crate::color::*;
use crate::square::Square;
use crate::tables::*;

pub const MAX_MOVES: usize = 62;
pub struct MoveList {
    pub moves: [BitMove; MAX_MOVES],
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
        } else {
            println!("overflow");
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

pub struct MoveGen {}

impl MoveGen {
    #[inline]
    pub fn generate(board: &Board) -> MoveList {
        let mut move_list = MoveList::default();

        // order is important (captures first)
        MoveGen::generate_side_moves(&mut move_list, &board);
        MoveGen::generate_cannon_shots(&mut move_list, &board);
        MoveGen::generate_cannon_jumps(&mut move_list, &board);
        // TODO putting retreats above forwards results in endless loop even in winning position (maybe just fixed with repition rule)
        MoveGen::generate_forward_moves(&mut move_list, &board);
        MoveGen::generate_retreats(&mut move_list, &board);
        move_list
    }

    pub fn generate_captures(board: &Board) -> MoveList {
        let mut move_list = MoveList::default();
        // We know that retreats and jumps can never be captures
        MoveGen::generate_side_moves(&mut move_list, &board);
        MoveGen::generate_cannon_shots(&mut move_list, &board);
        MoveGen::generate_forward_moves(&mut move_list, &board);
        let enemies = board.enemy_pieces() | board.enemy_castle();
        let is_capture = |m: &BitMove| (BitBoard::from_square(m.dst()) & enemies).is_not_empty();
        move_list.filter(move |m| is_capture(m)).collect()
    }

    #[inline]
    fn generate_forward_moves(moves: &mut MoveList, board: &Board) {
        let my_pieces = board.player_pieces();
        let unoccupied = !my_pieces;
        for src in my_pieces {
            let forwards = front(board.side_to_move(), src);
            let forwards = forwards & unoccupied;
            for dst in forwards {
                moves.push(BitMove::make(src, dst));
            }
        }
    }

    #[inline]
    fn generate_side_moves(moves: &mut MoveList, board: &Board) {
        let my_pieces = board.player_pieces();
        let enemy_pieces = board.enemy_pieces() | board.enemy_castle();

        for src in my_pieces {
            let sides = sides(board.side_to_move(), src);
            let sides = sides & enemy_pieces;

            for dst in sides {
                moves.push(BitMove::make(src, dst));
            }
        }
    }

    #[inline]
    fn generate_retreats(moves: &mut MoveList, board: &Board) {
        let my_color = board.side_to_move();
        let my_pieces = board.player_pieces();
        let enemy_pieces = board.enemy_pieces();
        let unoccupied = !board.pieces_with_castles();

        for src in my_pieces {
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

    #[inline]
    fn generate_cannon_jumps(moves: &mut MoveList, board: &Board) {
        let my_pieces = board.player_pieces();
        let my_pieces_original = my_pieces;
        let unoccupied = !board.pieces_with_castles();

        for src in my_pieces {
            let possible_jumps = distance_ring(src, 3) & unoccupied;
            for dst in possible_jumps {
                let between = between(src, dst);
                if between.is_not_empty() && (between & my_pieces_original) == between {
                    moves.push(BitMove::make(src, dst));
                }
            }
        }
    }

    #[inline]
    fn generate_cannon_shots(moves: &mut MoveList, board: &Board) {
        let my_pieces = board.player_pieces();
        let my_pieces_original = my_pieces;
        let enemy_pieces = board.enemy_pieces() | board.enemy_castle();
        let unoccupied = !board.pieces_with_castles();

        for src in my_pieces {
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
    println!("{}", board.pieces_with_castles());
    let moves = board.generate_moves_for(Square::A1);
    //let moves = board.generate_moves();
    for m in moves {
        println!("{} -> {}", m.src(), m.dst());
    }
}
