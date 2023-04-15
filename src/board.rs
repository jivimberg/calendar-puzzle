use std::fmt;
use colored::Colorize;
use time::Date;

pub(crate) struct Board {
    date: Date,
    available_tiles: [[bool; 7]; 8],
    engravings: [[&'static str; 7]; 8],
}

impl Board {
    pub fn new(date: Date) -> Self {
        let mut board = Self {
            date,
            available_tiles: Self::init_board(),
            engravings: Self::init_engravings(),
        };

        Self::block_date_tiles(&mut board.available_tiles, date);

        board
    }

    fn block_date_tiles(available_tiles: &mut [[bool; 7]; 8], date: Date, ) {
        // TODO we should be able to pass in a date to the constructor
        // TODO by default I'll hardcode the date: Feb 12th Sunday
        available_tiles[0][1] = false; // Feb
        available_tiles[3][4] = false; // 12
        available_tiles[6][3] = false; // Sunday
    }

    fn init_board() -> [[bool; 7]; 8] {
        let mut empty_board = [[true; 7]; 8]; // rows x columns
        // block top right tiles
        empty_board[0][6] = false;
        empty_board[1][6] = false;

        // block bottom left tiles
        empty_board[7][0] = false;
        empty_board[7][1] = false;
        empty_board[7][2] = false;
        empty_board[7][3] = false;

        empty_board
    }

    fn init_engravings() -> [[&'static str; 7]; 8] {
        [
            ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "  " ],
            ["Jul", "Aug", "Sep", "Oct", "Nov", "Dec", "  " ],
            ["  1", "  2", "  3", "  4", "  5", "  6", "  7"],
            ["  8", "  9", " 10", " 11", " 12", " 13", " 14"],
            [" 15", " 16", " 17", " 18", " 19", " 20", " 21"],
            [" 22", " 23", " 24", " 25", " 26", " 27", " 28"],
            [" 29", " 30", " 31", "Sun", "Mon", "Tue", "Wed"],
            ["   ", "   ", "   ", "   ", "Thu", "Fri", "Sat"],
        ]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (row_idx, row) in self.available_tiles.iter().enumerate() {
            for (col_idx, is_available) in row.iter().enumerate() {
                if *is_available {
                    write!(f, " {}", self.engravings[row_idx][col_idx])?;
                } else {
                    write!(f, " {}", self.engravings[row_idx][col_idx].bold())?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
