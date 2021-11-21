use crate::cannon_move::CannonMove;
use crate::grid::Grid;
use arrayvec::ArrayVec;
use nodrop::NoDrop;
use std::fmt;
use vek::{num_integer::Integer, Vec2};

const FRONT: [Vec2<i32>; 3] = [Vec2::new(-1, 1), Vec2::new(0, 1), Vec2::new(1, 1)];
const SIDE: [Vec2<i32>; 2] = [Vec2::new(-1, 0), Vec2::new(1, 0)];
const RETREAT: [Vec2<i32>; 3] = [Vec2::new(-2, -2), Vec2::new(0, -2), Vec2::new(2, -2)];
const DIRS: [Vec2<i32>; 8] = [
    Vec2::new(0, 1),  // Forward
    Vec2::new(1, 0),  // Right
    Vec2::new(0, -1), // Backwards
    Vec2::new(-1, 0), // Left
    Vec2::new(1, -1),
    Vec2::new(1, 1),
    Vec2::new(-1, 1),
    Vec2::new(-1, -1),
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Square {
    Empty,
    Piece(Color),
    Castle(Color),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "{}", "w"),
            Color::Black => write!(f, "{}", "b"),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Empty => write!(f, "{}", "-"),
            Square::Piece(color) => write!(f, "{}", color),
            Square::Castle(color) => write!(f, "{}", color),
        }
    }
}

const SIZE: i32 = 10;
const SIZE_VEC: Vec2<i32> = Vec2::new(SIZE, SIZE);

pub struct Board {
    pub squares: Grid<Square>,
    side_to_move: Color,
    castles: [Vec2<i32>; 2],
}

impl Board {
    pub fn new(castles: &[Vec2<i32>; 2]) -> Self {
        let populate = |pos: Vec2<i32>| -> Square {
            if pos == castles[0] {
                return Square::Castle(Color::Black);
            }
            if pos == castles[1] {
                return Square::Castle(Color::White);
            }
            if (1..=3).contains(&pos.y) {
                if pos.x.is_even() {
                    return Square::Piece(Color::Black);
                }
            } else if (6..=8).contains(&pos.y) {
                if pos.x.is_odd() {
                    return Square::Piece(Color::White);
                }
            }
            return Square::Empty;
        };
        Self {
            squares: Grid::populate_with(SIZE_VEC, populate),
            side_to_move: Color::Black,
            castles: *castles,
        }
    }

    pub fn make_move(&mut self, cannon_move: CannonMove) -> bool {
        let square = self.squares.get_cloned(cannon_move.source());
        if let Some(square) = square {
            if cannon_move.is_shot() {
                self.squares.set(cannon_move.dest(), Square::Empty);
            } else {
                self.squares.set(cannon_move.source(), Square::Empty);
                self.squares.set(cannon_move.dest(), square);
            }
            self.flip_side();
            true
        } else {
            false
        }
    }

    // For now only check for forwards moves
    pub fn valid_move(&self, from: Vec2<i32>, to: Vec2<i32>) -> Option<CannonMove> {
        self.enumerate_moves_for(from)
            .iter()
            .find(|m| m.dest() == to)
            .map_or(None, |r| Some(*r))
    }

    pub fn enumerate_moves_for(&self, pos: Vec2<i32>) -> NoDrop<ArrayVec<CannonMove, 8>> {
        let mut moves = NoDrop::new(ArrayVec::<CannonMove, 8>::new());
        let side_to_move = self.side_to_move;
        let side = match side_to_move {
            Color::White => -1,
            Color::Black => 1,
        };
        let enemy_color = match side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        let move_list = FRONT
            .iter()
            .map(move |dir| pos + (dir * side))
            .filter(move |newpos| {
                if let Some(sq) = self.squares.get_cloned(*newpos) {
                    match sq {
                        Square::Piece(color) | Square::Castle(color) => color != side_to_move,
                        Square::Empty => true,
                        _ => false,
                    }
                } else {
                    false
                }
            })
            .chain(
                SIDE.iter()
                    .map(move |dir| pos + (dir * side))
                    .filter(move |newpos| {
                        if let Some(color) = self.color_at(*newpos) {
                            return color != self.side_to_move;
                        }
                        false
                    }),
            );
        if FRONT
            .iter()
            .chain(SIDE.iter())
            .map(move |dir| pos + (dir * side))
            .any(move |newpos| {
                if let Some(color) = self.color_at(newpos) {
                    return color != self.side_to_move;
                }
                false
            })
        {
            let retreats = RETREAT
                .iter()
                .map(move |dir| pos + (dir * side))
                .filter(move |newpos| self.is_empty(*newpos));
            for r in retreats {
                moves.push(CannonMove::new(pos, r, false));
            }
        }

        let jumps = self
            .list_cannons_with_head_at(pos)
            .map(move |dir| pos + 3 * dir)
            .filter(move |newpos| self.is_empty(*newpos));
        let (two_dist_shots, three_dist_shots): (Vec<Vec2<i32>>, Vec<Vec2<i32>>) = self
            .list_cannons_with_head_at(pos)
            .filter(move |dir| {
                let newpos = pos + -1 * *dir;
                self.is_empty(newpos)
            })
            .map(move |dir| (pos + -2 * dir, pos + -3 * dir))
            .unzip();

        let shots = two_dist_shots
            .iter()
            .chain(three_dist_shots.iter())
            .filter(move |pos| {
                if let Some(sq) = self.squares.get_cloned(**pos) {
                    match sq {
                        Square::Piece(color) | Square::Castle(color) => {
                            return color != side_to_move
                        }
                        _ => return false,
                    }
                }
                false
            });
        for p in shots {
            moves.push(CannonMove::new(pos, *p, true));
        }
        for p in move_list.chain(jumps) {
            moves.push(CannonMove::new(pos, p, false));
        }
        return moves;
    }

    fn list_cannons_with_head_at(&self, head: Vec2<i32>) -> impl Iterator<Item = Vec2<i32>> + '_ {
        let side = match self.side_to_move {
            Color::White => -1,
            Color::Black => 1,
        };
        return DIRS
            .iter()
            .map(move |dir| (head + (dir * side), dir * side))
            .filter(move |(newpos, _)| {
                if let Some(color) = self.color_at(*newpos) {
                    return color == self.side_to_move;
                }
                false
            })
            .map(move |(newpos, dir)| (newpos + dir, dir))
            .filter(move |(newpos, _)| {
                if let Some(color) = self.color_at(*newpos) {
                    return color == self.side_to_move;
                }
                false
            })
            .map(move |(_, dir)| dir);
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn flip_side(&mut self) -> Color {
        match self.side_to_move {
            Color::White => {
                self.side_to_move = Color::Black;
                Color::Black
            }
            Color::Black => {
                self.side_to_move = Color::White;
                Color::White
            }
        }
    }

    pub fn color_at(&self, pos: Vec2<i32>) -> Option<Color> {
        if let Some(Square::Piece(color)) = self.squares.get(pos) {
            Some(*color)
        } else {
            None
        }
    }

    pub fn is_empty(&self, pos: Vec2<i32>) -> bool {
        matches!(self.squares.get_cloned(pos), Some(Square::Empty))
    }

    pub fn black_castle(&self) -> Vec2<i32> {
        self.castles[0]
    }

    pub fn white_castle(&self) -> Vec2<i32> {
        self.castles[1]
    }

    pub fn castle_with(&self, c: Color) -> Vec2<i32> {
        match c {
            Color::Black => self.black_castle(),
            Color::White => self.white_castle(),
        }
    }

    pub fn castle_dead(&self, c: Color) -> bool {
        match c {
            Color::Black => !matches!(
                self.squares.get(self.black_castle()),
                Some(Square::Castle(_))
            ),
            Color::White => !matches!(
                self.squares.get(self.white_castle()),
                Some(Square::Castle(_))
            ),
        }
    }
}
