use std::io;
use std::time::Instant;

use chrono::NaiveDate;
use rayon::current_num_threads;

use board::Board;

use crate::piece::Piece;

mod board;
mod piece;
mod shape;

fn main() {
    println!("Tira una fecha wachin! (YYYY/MM/DD)");
    let mut date_input = String::new();
    io::stdin().read_line(&mut date_input).unwrap();
    let date = NaiveDate::parse_from_str(&date_input.trim(), "%Y/%m/%d").unwrap();

    let board = Board::new(date);
    println!("{}", board);

    let all_pieces = Piece::get_all_pieces();

    // TODO use a real benchmarking framework
    let instant = Instant::now();

    // Find single Solution
    // board.solve(all_pieces).unwrap();

    // Find all solutions
    // println!("Number of threads: {:.2?}", current_num_threads());
    let all_solutions = board.find_solutions(all_pieces);
    println!("Found {} solutions", all_solutions.len());

    // print all solutions
    // for solution in all_solutions {
    //     println!("{}", solution);
    // }

    println!("Elapsed time: {:.2?}", instant.elapsed());

    ()
}
