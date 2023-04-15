mod board;

use board::Board;
use time::macros::date;

fn main() {
    let board = Board::new(date!(2023 - 02 - 12));
    println!("{}", board);
}
