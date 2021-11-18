mod board;
mod grid;

use vek::Vec2;

fn main() {
    let board = board::Board::new(&[Vec2::new(0, 0); 2]);
    board.print();
}
