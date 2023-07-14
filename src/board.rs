use std::collections::HashSet;
use std::fmt;

use chrono::{Datelike, Month, NaiveDate};

use num_traits::FromPrimitive;
use rayon::current_thread_index;
use rayon::prelude::*;

use crate::board::BoardError::{OutOfBounds, TileOccupied};
use crate::piece::ALL_PIECES;
use crate::shape::Shape;

#[derive(Eq, PartialEq, Hash, Debug)]
pub(crate) struct Board {
    tiles: [[&'static str; 7]; 8],
}

impl Board {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            tiles: Self::init_tiles(date),
        }
    }

    fn init_tiles(date: NaiveDate) -> [[&'static str; 7]; 8] {
        let mut tiles = [[Self::EMPTY_TILE; 7]; 8]; // rows x columns

        // block top right tiles
        tiles[0][6] = Self::BLOCKED_TILE;
        tiles[1][6] = Self::BLOCKED_TILE;

        // block bottom left tiles
        tiles[7][0] = Self::BLOCKED_TILE;
        tiles[7][1] = Self::BLOCKED_TILE;
        tiles[7][2] = Self::BLOCKED_TILE;
        tiles[7][3] = Self::BLOCKED_TILE;

        let (month_coord, day_coord, weekday_coord) = Self::find_engravings_coordinates(date);
        let(month_row, month_col) = month_coord;
        let(day_row, day_col) = day_coord;
        let(weekday_row, weekday_col) = weekday_coord;

        // Block date tiles
        tiles[month_row][month_col] = Self::ENGRAVINGS[month_row][month_col];
        tiles[day_row][day_col] = Self::ENGRAVINGS[day_row][day_col];
        tiles[weekday_row][weekday_col] = Self::ENGRAVINGS[weekday_row][weekday_col];

        tiles
    }

    fn find_engravings_coordinates(date: NaiveDate) -> ((usize, usize) , (usize, usize), (usize, usize)) {
        let month_name_prefix = &Month::from_u32(date.month()).unwrap().name()[..3];
        let day = date.day() as usize;
        let weekday_prefix = &date.weekday().to_string()[..3];

        let mut month_coordinates = None;
        let mut day_coordinates = None;
        let mut weekday_coordinates =None;

        for  (row_idx, row) in Self::ENGRAVINGS.iter().enumerate() {
            for (col_idx, engraving) in row.iter().enumerate() {
                if engraving == &month_name_prefix {
                    month_coordinates = Some((row_idx, col_idx));
                } else if engraving.trim() == &day.to_string() {
                    day_coordinates = Some((row_idx, col_idx));
                } else if engraving == &weekday_prefix {
                    weekday_coordinates = Some((row_idx, col_idx));
                }
            }
        }


        (month_coordinates.unwrap(), day_coordinates.unwrap(), weekday_coordinates.unwrap())
    }

    const EMPTY_TILE: &'static str = " ▢ ";
    const BLOCKED_TILE: &'static str = "   ";
    const ENGRAVINGS: [[&'static str; 7]; 8] =  {
        [
            ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "   "],
            ["Jul", "Aug", "Sep", "Oct", "Nov", "Dec", "   "],
            ["  1", "  2", "  3", "  4", "  5", "  6", "  7"],
            ["  8", "  9", " 10", " 11", " 12", " 13", " 14"],
            [" 15", " 16", " 17", " 18", " 19", " 20", " 21"],
            [" 22", " 23", " 24", " 25", " 26", " 27", " 28"],
            [" 29", " 30", " 31", "Sun", "Mon", "Tue", "Wed"],
            ["   ", "   ", "   ", "   ", "Thu", "Fri", "Sat"],
        ]
    };

    pub fn find_solutions(self) -> HashSet<Board> {
        return self.do_find_solutions(&[true; 10]);
    }

    fn do_find_solutions(self, available_pieces_idx: &[bool]) -> HashSet<Board> {
        // print!("{esc}c", esc = 27 as char); // clear the screen
        // println!("{}", self);

        if available_pieces_idx.iter().all(|&is_available| is_available == false) {
            let mut solutions = HashSet::new();
            solutions.insert(self); // we were able to place all the pieces. It's solved!
            return solutions;
        }

        let (row_start, col_start) = self.find_next_free_tile().unwrap();

        return available_pieces_idx
            .par_iter()
            .enumerate()
            .filter(|(_, is_available)| **is_available)
            .map(|(idx, _)| (idx, &ALL_PIECES[idx]))
            .flat_map(|(idx, piece)| {
                let mut new_available_pieces_idx: [bool; 10] = [true; 10];
                new_available_pieces_idx.copy_from_slice(available_pieces_idx);
                new_available_pieces_idx[idx] = false;

                piece.distinct_shapes
                    .iter()
                    .map(|shape| (new_available_pieces_idx, shape))
                    .collect::<Vec<([bool; 10], &Shape)>>()
            })
            .filter_map(|(new_available_pieces_idx, shape)| {
                // println!("Thread: {:?}", current_thread_index());
                match self.add_shape_at_position(shape, (row_start, col_start)) {
                    Ok(new_board) => Some(new_board.do_find_solutions(&new_available_pieces_idx)),
                    Err(_) => None,
                }
            })
            .flatten()
            .collect();
    }

    fn find_next_free_tile(&self) -> Option<(usize, usize)> {
        for (row_idx, row) in self.tiles.iter().enumerate() {
            for (col_idx, tile_value) in row.iter().enumerate() {
                if tile_value == &Self::EMPTY_TILE {
                    return Some((row_idx, col_idx));
                }
            }
        }
        None
    }

    pub(crate) fn add_shape_at_position(&self, shape: &Shape, (row_start, col_start): (usize, usize)) -> Result<Board, BoardError> {
        // check bounds
        let shape_max_row_idx = shape.tile_matrix.len() - 1;
        if (row_start + shape_max_row_idx) >= self.tiles.len() {
            return Err(OutOfBounds);
        }

        let col_max_row_idx = shape.tile_matrix[0].len() - 1;
        if (col_start + col_max_row_idx) >= self.tiles[0].len() {
            return Err(OutOfBounds);
        }

        // check that the shape doesn't overlap with any existing tiles
        for (shape_row_idx, shape_row) in shape.tile_matrix.iter().enumerate() {
            for (shape_col_idx, shape_tile_value) in shape_row.iter().enumerate() {
                let shape_tile_is_not_empty = !shape_tile_value.trim().is_empty();
                let board_tile_is_occupied = self.tiles[row_start + shape_row_idx][col_start + shape_col_idx] != Self::EMPTY_TILE;
                if shape_tile_is_not_empty && board_tile_is_occupied {
                    return Err(TileOccupied);
                }
            }
        }

        // add the shape to the board
        let mut new_tiles = self.tiles.clone();
        for (shape_row_idx, shape_row) in shape.tile_matrix.iter().enumerate() {
            for (shape_col_idx, shape_tile_value) in shape_row.iter().enumerate() {
                if !shape_tile_value.trim().is_empty() {
                    new_tiles[row_start + shape_row_idx][col_start + shape_col_idx] = shape_tile_value;
                }
            }
        }

        Ok(
            Board {
                tiles: new_tiles,
            }
        )
    }
}

#[derive(Debug)]
pub enum BoardError {
    OutOfBounds,
    TileOccupied,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for  row in self.tiles.iter() {
            for tile_value in row.iter() {
                write!(f, " {}", tile_value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
