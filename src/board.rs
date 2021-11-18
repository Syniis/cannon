use crate::grid::Grid;
use std::fmt;
use vek::{num_integer::Integer, Vec2};

#[derive(Copy, Clone, Debug)]
enum Color {
    White = 0,
    Black = 1,
}

#[derive(Copy, Clone, Debug)]
enum Square {
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

const SIZE: u32 = 10;
const SIZE_VEC: Vec2<u32> = Vec2::new(SIZE, SIZE);

pub struct Board {
    squares: Grid<Square>,
    side_to_move: Color,
    castles: [Vec2<u32>; 2],
}

impl Board {
    pub fn new(castles: &[Vec2<u32>; 2]) -> Self {
        let populate = |pos: Vec2<u32>| -> Square {
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
    pub fn print(&self) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                print!(
                    "{}",
                    self.squares
                        .get(Vec2::new(x, SIZE - y - 1))
                        .unwrap_or(&Square::Empty)
                );
            }
            println!();
        }
    }
}
