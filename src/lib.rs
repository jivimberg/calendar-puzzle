use std::time::Instant;

use chrono::NaiveDate;

use crate::board::Board;
use crate::piece::Piece;

mod board;
mod piece;
mod shape;

pub fn find_all_solutions(date: NaiveDate) {
    let board = Board::new(date);
    let all_pieces = Piece::get_all_pieces();
    let all_solutions = board.find_solutions(all_pieces, None);
    println!("Found {} solutions", all_solutions.len());
}
