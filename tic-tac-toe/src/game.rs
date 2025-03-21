// src/game.rs

use crate::grid::Grid;
use crate::player::Player;

pub struct Game {
    grid: Grid,
    current_player: Player,
}

impl Game {
    pub fn new(player_x: Player, _player_o: Player) -> Self {
        Game {
            grid: Grid::new(),
            current_player: player_x,
        }
    }

    pub fn play_turn(&mut self, row: usize, col: usize) -> Result<(), String> {
        if self.grid.is_occupied(row, col) {
            return Err("Cell is already occupied".to_string());
        }
        self.grid.place_mark(row, col, self.current_player.get_mark());
        if self.check_winner() {
            return Ok(());
        }
        if self.is_draw() {
            return Ok(());
        }
        self.switch_player();
        Ok(())
    }

    pub fn check_winner(&self) -> bool {
        let cells = &self.grid.cells;

        // Check rows and columns
        for i in 0..3 {
            if cells[i][0] == cells[i][1] && cells[i][1] == cells[i][2] && cells[i][0].is_some() {
                return true;
            }
            if cells[0][i] == cells[1][i] && cells[1][i] == cells[2][i] && cells[0][i].is_some() {
                return true;
            }
        }

        // Check diagonals
        if cells[0][0] == cells[1][1] && cells[1][1] == cells[2][2] && cells[0][0].is_some() {
            return true;
        }
        if cells[0][2] == cells[1][1] && cells[1][1] == cells[2][0] && cells[0][2].is_some() {
            return true;
        }

        false
    }

    pub fn is_draw(&self) -> bool {
        self.grid.cells.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }

    fn switch_player(&mut self) {
        // Switch between Player X and Player O
        if self.current_player.get_mark() == 'X' {
            self.current_player = Player::new("Player O".to_string(), 'O');
        } else {
            self.current_player = Player::new("Player X".to_string(), 'X');
        }
    }

    // Getter for the grid
    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    // Getter for the current player
    pub fn get_current_player(&self) -> &Player {
        &self.current_player
    }
}

// Additional structs and methods for Grid and Player would be defined in their respective files.