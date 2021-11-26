#![feature(generic_const_exprs)]
#![feature(int_abs_diff)]

#[macro_use]
pub mod macros;
pub mod bitboard;
pub mod bits;
pub mod board;
pub mod cannon_move;
pub mod color;
pub mod defs;
pub mod eval;
pub mod movegen;
pub mod search;
pub mod square;
pub mod tables;
