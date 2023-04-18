mod board;
mod piece;
mod shape; // TODO should this go in board?

use board::Board;
use time::macros::date;
use crate::shape::Shape;

fn main() {
    let board = Board::new(date!(2023 - 02 - 12));
    println!("{}", board);

    // testing transpose
    let m = vec![
        vec!["🟦", "🟦", "🟦"],
        vec!["  ", "🟦", "  "],
        vec!["  ", "🟦", "  "]
    ];
    let shape = Shape { tile_matrix: m };

    let distinct_shapes = shape.generate_all_distinct_variations();
    for shape in distinct_shapes {
        println!("{}", shape);
    }
}
