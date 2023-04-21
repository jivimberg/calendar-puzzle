use time::macros::date;

use board::Board;

use crate::board::BoardError;
use crate::piece::Piece;
use crate::shape::Shape;

mod board;
mod piece;
mod shape;

fn main() {
    let board = Board::new(date!(2023 - 02 - 12));
    println!("{}", board);

    let all_pieces = Piece::get_all_pieces();
    let solved_board = board.solve(all_pieces).unwrap();
    println!("{}", solved_board);

    ()
}
