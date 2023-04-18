mod board;
mod piece;
mod shape; // TODO should this go in board?

use board::Board;
use time::macros::date;
use crate::shape::Shape;

fn main() {
    let board = Board::new(date!(2023 - 02 - 12));
    println!("{}", board);

    for piece in piece::Piece::get_all_pieces().iter() {
        println!("{}", piece.name);
        for shape in piece.distinct_shapes.iter() {
            println!("{}", shape);
        }
    }
}
