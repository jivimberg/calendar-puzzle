use std::fmt;
use time::Date;
use piece::Piece;
use crate::board::BoardError::{OutOfBounds, TileOccupied};
use crate::piece;
use crate::shape::Shape;

pub(crate) struct Board {
    date: Date,
    tiles: [[&'static str; 7]; 8],
}

impl Board {
    pub fn new(date: Date) -> Self {
        Self {
            date,
            tiles: Self::init_tiles(date),
        }
    }

    fn init_tiles(date: Date) -> [[&'static str; 7]; 8] {
        let mut tiles = [[Self::EMPTY_TILE; 7]; 8]; // rows x columns

        // block top right tiles
        tiles[0][6] = Self::BLOCKED_TILE;
        tiles[1][6] = Self::BLOCKED_TILE;

        // block bottom left tiles
        tiles[7][0] = Self::BLOCKED_TILE;
        tiles[7][1] = Self::BLOCKED_TILE;
        tiles[7][2] = Self::BLOCKED_TILE;
        tiles[7][3] = Self::BLOCKED_TILE;

        // Block date tiles
        // TODO by default I'll hardcode the date: Feb 12th Sunday
        tiles[0][1] = Self::ENGRAVINGS[0][1]; // Feb
        tiles[3][4] = Self::ENGRAVINGS[3][4]; // 12
        tiles[6][3] = Self::ENGRAVINGS[6][3]; // Sunday

        tiles
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

    pub(crate) fn solve(self, remaining_pieces: Vec<Piece>) -> Option<Board> {
        if remaining_pieces.is_empty() {
            return Some(self); // we were able to place all the pieces. It's solved!
        }

        let (row_start, col_start) = self.find_next_free_tile().unwrap();

        for (idx, piece) in remaining_pieces.iter().enumerate() {
            println! ("Level {}: Trying to add {} at ({}, {})", 10 - remaining_pieces.len(), &piece.name, row_start, col_start);
            println!("{}", self);
            let mut new_remaining_pieces = remaining_pieces.clone();
            new_remaining_pieces.remove(idx);
            for shape in piece.distinct_shapes.iter() {
                match self.add_shape_at_position(shape, (row_start, col_start)) {
                    Ok(new_board) => {
                        match new_board.solve(new_remaining_pieces.clone()) {
                            Some(solved_board) => return Some(solved_board),
                            None => continue,
                        }
                    },
                    Err(e) => {
                        println!("Error: {:?} adding {}", e, &piece.name);
                        continue
                    },
                };
            }
        }

        None
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
                date: self.date,
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
