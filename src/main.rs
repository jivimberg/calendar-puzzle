use std::io;
use std::time::Instant;

use chrono::NaiveDate;
use rayon::current_num_threads;

use board::Board;

use crate::piece::{ALL_PIECES, Piece};

mod board;
mod piece;
mod shape;

fn main() {
    count_solutions_for_year();

    ()
}

#[allow(dead_code)]
fn count_solutions_for_year() {
    println!("Tira un año wachin! (YYYY)");
    let mut year_input = String::new();
    io::stdin().read_line(&mut year_input).unwrap();
    let year = year_input.trim().parse::<i32>().unwrap();

    let instant = Instant::now();

    for month in 1..=12 {
        for day in 1..=31 {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let board = Board::new(date);
                let solutions = board.find_solutions().len();
                println!("{}: {}", date, solutions);
            }
        }
    }
    println!("Elapsed time: {:.2?}", instant.elapsed());
}

#[allow(dead_code)]
fn find_solutions_for_date() {
    println!("Tira una fecha wachin! (YYYY/MM/DD)");
    let mut date_input = String::new();
    io::stdin().read_line(&mut date_input).unwrap();
    let date = NaiveDate::parse_from_str(&date_input.trim(), "%Y/%m/%d").unwrap();

    let board = Board::new(date);
    println!("{}", board);

    let instant = Instant::now();

    // Find single Solution
    // board.solve(all_pieces).unwrap();

    // Find all solutions
    // println!("Number of threads: {:.2?}", current_num_threads());
    let all_solutions = board.find_solutions();
    println!("Found {} solutions", all_solutions.len());

    // print all solutions
    // for solution in all_solutions {
    //     println!("{}", solution);
    // }

    println!("Elapsed time: {:.2?}", instant.elapsed());
}
