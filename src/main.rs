mod board;
mod piece;
mod shape;

use board::Board;
use time::macros::date;
use crate::board::BoardError;
use crate::piece::Piece;
use crate::shape::Shape;

fn main() {
    let board = Board::new(date!(2023 - 02 - 12));
    println!("{}", board);

    let all_pieces = piece::Piece::get_all_pieces();
    let piece_i = &all_pieces[0];
    for shape in piece_i.distinct_shapes.iter() {
        match board.add_shape_at_position(shape, (0, 0)) {
            Ok(new_board) => { println!("{}", new_board); },
            Err(board_error) => { dbg!(board_error); },
        }
    }

    ()
}
