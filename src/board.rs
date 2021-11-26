use crate::bitboard::BitBoard;
use crate::bitboard::EMPTY;
use crate::cannon_move::BitMove;
use crate::color::Color;
use crate::color::NUM_COLORS;
use crate::movegen::MoveGen;
use crate::movegen::MoveList;
use crate::square::Square;

#[derive(Clone, Copy)]
pub struct Board {
    pieces: BitBoard,
    pieces_with_color: [BitBoard; NUM_COLORS],
    side_to_move: Color,
    castles: [BitBoard; NUM_COLORS],
    pub prev_move: BitMove,
    pub prev_capture: bool,
}

impl Board {
    pub fn new(color: Color) -> Self {
        Self {
            pieces: EMPTY,
            pieces_with_color: [EMPTY; NUM_COLORS],
            side_to_move: color,
            castles: [EMPTY; 2],
            prev_move: BitMove::null(),
            prev_capture: false,
        }
    }

    pub fn start_position() -> Self {
        let mut board = Board::new(Color::White);
        board.set(Color::White, Square::A1);
        board.set(Color::White, Square::A2);
        board.set(Color::White, Square::A3);
        board.set(Color::White, Square::C1);
        board.set(Color::White, Square::C2);
        board.set(Color::White, Square::C3);
        board.set(Color::White, Square::E1);
        board.set(Color::White, Square::E2);
        board.set(Color::White, Square::E3);
        board.set(Color::White, Square::G1);
        board.set(Color::White, Square::G2);
        board.set(Color::White, Square::G3);

        board.set(Color::Black, Square::B8);
        board.set(Color::Black, Square::B7);
        board.set(Color::Black, Square::B6);
        board.set(Color::Black, Square::D8);
        board.set(Color::Black, Square::D7);
        board.set(Color::Black, Square::D6);
        board.set(Color::Black, Square::F8);
        board.set(Color::Black, Square::F7);
        board.set(Color::Black, Square::F6);
        board.set(Color::Black, Square::H8);
        board.set(Color::Black, Square::H7);
        board.set(Color::Black, Square::H6);
        board.castles[0] |= BitBoard::from_square(Square::H1);
        board.castles[1] |= BitBoard::from_square(Square::A8);
        board
    }
    pub fn pieces(&self) -> BitBoard {
        self.pieces
    }

    pub fn pieces_with_castles(&self) -> BitBoard {
        self.pieces | self.castles[0] | self.castles[1]
    }

    pub fn pieces_with_color(&self, color: Color) -> BitBoard {
        self.pieces_with_color[color.to_index()]
    }

    pub fn castle_with_color(&self, color: Color) -> BitBoard {
        self.castles[color.to_index()]
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn player_pieces(&self) -> BitBoard {
        self.pieces_with_color(self.side_to_move())
    }

    pub fn enemy_pieces(&self) -> BitBoard {
        self.pieces_with_color(!self.side_to_move())
    }

    pub fn color_on(&self, square: Square) -> Option<Color> {
        if (self.pieces_with_color(Color::White) & BitBoard::from_square(square)) != EMPTY {
            Some(Color::White)
        } else if (self.pieces_with_color(Color::Black) & BitBoard::from_square(square)) != EMPTY {
            Some(Color::Black)
        } else {
            None
        }
    }

    pub fn set(&mut self, color: Color, square: Square) {
        let square_bb = BitBoard::from_square(square);
        self.pieces_with_color[color.to_index()] ^= square_bb;
        self.pieces_with_color[(!color).to_index()] &= !square_bb;
        self.pieces |= square_bb;
    }

    pub fn apply_move(&mut self, m: BitMove) {
        assert_ne!(m.src(), m.dst());
        let src_bb = BitBoard::from_square(m.src());
        let dst_bb = BitBoard::from_square(m.dst());
        if m.is_shot() {
            self.pieces ^= dst_bb;
            self.pieces_with_color[(!self.side_to_move).to_index()] ^= dst_bb;
        } else {
            self.pieces ^= src_bb;
            self.pieces |= dst_bb;
            self.pieces_with_color[self.side_to_move.to_index()] ^= src_bb | dst_bb;
            self.prev_capture =
                (self.pieces_with_color[(!self.side_to_move).to_index()] & dst_bb).is_not_empty();
            self.pieces_with_color[(!self.side_to_move).to_index()] &= !dst_bb;
        }
        self.prev_move = m;
        self.side_to_move = !self.side_to_move;
    }

    pub fn undo_move(&mut self) {
        self.side_to_move = !self.side_to_move;
        let undo_move = self.prev_move;
        let src_bb = BitBoard::from_square(undo_move.src());
        let dst_bb = BitBoard::from_square(undo_move.dst());

        if undo_move.is_shot() {
            self.pieces ^= dst_bb;
            self.pieces_with_color[(!self.side_to_move).to_index()] ^= dst_bb;
        } else {
            self.pieces ^= src_bb;
            self.pieces &= !dst_bb;
            self.pieces_with_color[self.side_to_move.to_index()] ^= src_bb | dst_bb;
            if self.prev_capture {
                self.pieces |= dst_bb;
                self.pieces_with_color[(!self.side_to_move).to_index()] |= dst_bb;
            }
        }
    }

    pub fn generate_moves(&self) -> MoveList {
        MoveGen::generate(self)
    }

    pub fn generate_moves_for(&self, sq: Square) -> Vec<BitMove> {
        MoveGen::generate(self).filter(|m| m.src() == sq).collect()
    }
}
