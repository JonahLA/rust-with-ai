// This file defines the Player struct which represents a player in the game.
// It includes fields for the player's name and mark (X or O).
// It exports methods such as new and get_mark.

pub struct Player {
    pub name: String,
    pub mark: char,
}

impl Player {
    // Creates a new Player with the given name and mark.
    pub fn new(name: String, mark: char) -> Self {
        Player { name, mark }
    }

    // Returns the mark of the player (X or O).
    pub fn get_mark(&self) -> char {
        self.mark
    }
}