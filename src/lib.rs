use chrono::NaiveDate;

use crate::board::Board;
use crate::piece::{ALL_PIECES, Piece};

mod board;
mod piece;
mod shape;

pub fn find_all_solutions(date: NaiveDate) {
    let board = Board::new(date);
    let all_solutions = board.find_solutions();
    println!("Found {} solutions", all_solutions.len());
}
