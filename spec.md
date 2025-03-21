# Tic-Tac-Toe Specification (MVP)

## Objective
Create a simple, interactive Tic-Tac-Toe game that allows two players to compete against each other on a 3x3 grid. The game should determine the winner or declare a draw when appropriate.

---

## Features

### 1. Core Gameplay
- **Players**: Two players take turns (Player X and Player O).
- **Grid**: A 3x3 grid where players place their marks (X or O).
- **Turns**: Players alternate turns, starting with Player X.
- **Win Condition**: A player wins if they place three of their marks in:
  - A horizontal row.
  - A vertical column.
  - A diagonal line.
- **Draw Condition**: The game ends in a draw if all grid spaces are filled and no player has won.

### 2. User Interface
- **Grid Display**: Show the 3x3 grid with current marks.
- **Player Input**: Allow players to select a grid cell for their move.
  - Input validation: Prevent players from selecting an already occupied cell.
- **Game Status**: Display the current player's turn and the game result (win/draw).

### 3. Game Flow
- Start the game with an empty grid.
- Alternate turns between Player X and Player O.
- Check for a win or draw after each move.
- End the game when a win or draw condition is met.
- Allow players to restart the game.

---

## Technical Requirements
- **Programming Language**: Rust.
- **Platform**: Command-line interface (CLI).
- **Input Method**: Keyboard input for selecting grid cells (e.g., row and column numbers).
- **Output**: Text-based display of the grid and game status.

---

## Stretch Goals (Not part of MVP)
1. **AI Opponent**: Add a single-player mode with an AI opponent.
2. **Graphical Interface**: Create a graphical version of the game.
3. **Score Tracking**: Keep track of wins, losses, and draws across multiple games.
4. **Custom Grid Size**: Allow players to choose a grid size (e.g., 4x4, 5x5).

---

## Acceptance Criteria
- The game must allow two players to complete a full game of Tic-Tac-Toe.
- The game must correctly identify a winner or a draw.
- The game must prevent invalid moves (e.g., selecting an occupied cell).
- The game must display the grid and game status clearly after each move.