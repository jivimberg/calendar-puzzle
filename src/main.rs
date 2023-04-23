use std::io;

use chrono::NaiveDate;

use board::Board;

use crate::piece::Piece;

mod board;
mod piece;
mod shape;

fn main() {
    println!("Tira una fecha wachin! (YYYY/MM/DD)");
    let mut date_input = String::new();
    io::stdin().read_line(&mut date_input).unwrap();
    println!("{}", &date_input);
    let date = NaiveDate::parse_from_str(&date_input.trim(), "%Y/%m/%d").unwrap();
    println!("{}", date);

    let board = Board::new(date);
    println!("{}", board);

    let all_pieces = Piece::get_all_pieces();
    board.solve(all_pieces).unwrap();

    ()
}
