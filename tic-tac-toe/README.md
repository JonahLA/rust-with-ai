# Tic-Tac-Toe Game

## Overview
This is a simple, interactive Tic-Tac-Toe game implemented in Rust. The game allows two players to compete against each other on a 3x3 grid. Players take turns placing their marks (X or O) and the game determines the winner or declares a draw when appropriate.

## Features
- Two-player gameplay on a 3x3 grid.
- Players alternate turns, starting with Player X.
- Win conditions include three marks in a row, column, or diagonal.
- Draw condition if all cells are filled without a winner.
- Clear display of the grid and game status after each move.

## Getting Started

### Prerequisites
- Rust installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Running the Game
1. Clone the repository:
   ```
   git clone <repository-url>
   cd tic-tac-toe
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the game:
   ```
   cargo run
   ```

### Game Instructions
- Players will be prompted to enter their moves by selecting a grid cell using row and column numbers.
- The game will display the current state of the grid and indicate whose turn it is.
- The game will announce the winner or declare a draw when the game ends.
- Players can restart the game after it concludes.

## Project Structure
- `src/main.rs`: Entry point of the application, manages the game loop and user input.
- `src/game.rs`: Contains the `Game` struct that manages game state and logic.
- `src/grid.rs`: Defines the `Grid` struct for the 3x3 grid representation.
- `src/player.rs`: Defines the `Player` struct representing each player.

## License
This project is licensed under the MIT License. See the LICENSE file for details.