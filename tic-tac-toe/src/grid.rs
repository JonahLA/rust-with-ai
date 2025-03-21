// This file defines the Grid struct which represents the 3x3 grid for the Tic-Tac-Toe game.

pub struct Grid {
    pub cells: [[Option<char>; 3]; 3],
}

impl Grid {
    // Creates a new empty grid
    pub fn new() -> Self {
        Grid {
            cells: [[None; 3]; 3],
        }
    }

    // Displays the current state of the grid
    pub fn display(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Some(mark) => print!(" {} ", mark),
                    None => print!(" . "),
                }
            }
            println!();
        }
    }

    // Places a mark in the specified cell
    pub fn place_mark(&mut self, row: usize, col: usize, mark: char) {
        self.cells[row][col] = Some(mark);
    }

    // Checks if a cell is occupied
    pub fn is_occupied(&self, row: usize, col: usize) -> bool {
        self.cells[row][col].is_some()
    }
}