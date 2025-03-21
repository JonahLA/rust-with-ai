use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::text::Text2dBundle; // Import for text rendering
use rand::seq::IteratorRandom; // Import for random selection

const GRID_SIZE: usize = 3;
const CELL_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct Cell {
    row: usize,
    col: usize,
}

#[derive(Component)]
pub struct WinnerText; // Marker for the winner banner text

#[derive(Component)]
pub struct ScoreText; // Marker for the score text

#[derive(Resource)]
pub struct GameState {
    grid: [[Option<char>; GRID_SIZE]; GRID_SIZE],
    current_player: char,
    game_over: bool, // Track if the game is over
    winner: Option<char>, // Track the winner ('X', 'O', or None for a draw)
    message_displayed: bool, // Track if the game-over message has been displayed
}

#[derive(Resource)]
pub struct Score {
    pub wins_x: u32,
    pub wins_o: u32,
    pub draws: u32,
}

pub fn setup(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    // Get the primary window dimensions
    let window = windows.single();
    let _window_width = window.width();
    let _window_height = window.height();

    // Spawn a 2D camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 999.0)), // Ensure the camera is positioned correctly
        ..default()
    });

    // Initialize game state
    commands.insert_resource(GameState {
        grid: [[None; GRID_SIZE]; GRID_SIZE],
        current_player: 'X',
        game_over: false,
        winner: None,
        message_displayed: false, // Initialize the flag as false
    });

    // Initialize score tracking
    commands.insert_resource(Score {
        wins_x: 0,
        wins_o: 0,
        draws: 0,
    });

    // Spawn grid cells, centered on the screen
    let grid_width = GRID_SIZE as f32 * CELL_SIZE;
    let grid_height = GRID_SIZE as f32 * CELL_SIZE;
    let start_x = -grid_width / 2.0 + CELL_SIZE / 2.0;
    let start_y = -grid_height / 2.0 + CELL_SIZE / 2.0;

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            // Alternate cell colors for better visibility
            let color = if (row + col) % 2 == 0 {
                Color::rgb(0.9, 0.9, 0.9) // Light grey
            } else {
                Color::rgb(0.6, 0.6, 0.6) // Dark grey
            };

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        start_x + col as f32 * CELL_SIZE,
                        start_y + row as f32 * CELL_SIZE,
                        0.0,
                    )),
                    ..default()
                })
                .insert(Cell { row, col });
        }
    }

    // Add a text entity for the winner banner
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "", // Initially empty
                TextStyle {
                    font: asset_server.load("fonts/Fira_Sans/FiraSans-Bold.ttf"), // Ensure the font path is correct
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 250.0, 1.0)), // Move further above the grid
            ..default()
        })
        .insert(WinnerText);

    // Add a text entity for the score display
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "Scores: Player X - 0, Player O - 0, Draws - 0", // Initial score
                TextStyle {
                    font: asset_server.load("fonts/Fira_Sans/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, -250.0, 1.0)), // Position below the grid
            ..default()
        })
        .insert(ScoreText);
}

pub fn handle_clicks(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    query: Query<(&Cell, &Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>, // Query the camera to adjust cursor position
) {
    if game_state.game_over {
        return; // Prevent further moves if the game is over
    }

    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Get the camera and its global transform
                if let Ok((camera, camera_transform)) = camera_query.get_single() {
                    // Convert cursor position to world coordinates
                    if let Some(world_pos) = camera.viewport_to_world(camera_transform, cursor_pos) {
                        let world_pos = world_pos.origin.truncate(); // Get the 2D world position

                        for (cell, transform) in query.iter() {
                            let cell_pos = transform.translation.truncate();
                            let half_size = CELL_SIZE / 2.0;

                            // Check if the world position is within the bounds of the cell
                            if world_pos.x > cell_pos.x - half_size
                                && world_pos.x < cell_pos.x + half_size
                                && world_pos.y > cell_pos.y - half_size
                                && world_pos.y < cell_pos.y + half_size
                            {
                                if game_state.grid[cell.row][cell.col].is_none() {
                                    game_state.grid[cell.row][cell.col] = Some(game_state.current_player);
                                    game_state.current_player = if game_state.current_player == 'X' {
                                        'O'
                                    } else {
                                        'X'
                                    };

                                    // Check for a win or draw after the move
                                    if let Some(winner) = check_winner(&game_state.grid) {
                                        game_state.game_over = true;
                                        game_state.winner = Some(winner);
                                    } else if is_draw(&game_state.grid) {
                                        game_state.game_over = true;
                                        game_state.winner = None;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn update_grid(
    mut game_state: ResMut<GameState>, // Make game_state mutable to update the flag
    mut score: ResMut<Score>, // Add score resource
    mut query: Query<(&Cell, &mut Sprite)>,
) {
    for (cell, mut sprite) in query.iter_mut() {
        // Update the cell color based on the game state
        if let Some(mark) = game_state.grid[cell.row][cell.col] {
            sprite.color = if mark == 'X' {
                Color::rgb(1.0, 0.0, 0.0) // Red for X
            } else {
                Color::rgb(0.0, 0.0, 1.0) // Blue for O
            };
        } else {
            // Reset to default color if the cell is empty
            sprite.color = if (cell.row + cell.col) % 2 == 0 {
                Color::rgb(0.9, 0.9, 0.9) // Light grey
            } else {
                Color::rgb(0.6, 0.6, 0.6) // Dark grey
            };
        }
    }

    if game_state.game_over && !game_state.message_displayed {
        if let Some(winner) = game_state.winner {
            if winner == 'X' {
                score.wins_x += 1;
            } else if winner == 'O' {
                score.wins_o += 1;
            }
        } else {
            score.draws += 1;
        }
        game_state.message_displayed = true; // Set the flag to true
    }
}

pub fn update_winner_text(
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<WinnerText>>,
) {
    if game_state.game_over && game_state.message_displayed {
        if let Ok(mut text) = query.get_single_mut() {
            if let Some(winner) = game_state.winner {
                text.sections[0].value = format!("Player {} wins!", winner);
            } else {
                text.sections[0].value = "It's a draw!".to_string();
            }
        }
    }
}

pub fn update_score_text(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!(
            "Scores: Player X - {}, Player O - {}, Draws - {}",
            score.wins_x, score.wins_o, score.draws
        );
    }
}

pub fn handle_restart(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut winner_text_query: Query<&mut Text, With<WinnerText>>,
    mut query: Query<(&Cell, &mut Sprite)>,
) {
    if keys.just_pressed(KeyCode::R) {
        // Reset the game state
        game_state.grid = [[None; GRID_SIZE]; GRID_SIZE];
        game_state.current_player = 'X';
        game_state.game_over = false;
        game_state.winner = None;
        game_state.message_displayed = false;

        // Reset the grid visuals
        for (cell, mut sprite) in query.iter_mut() {
            sprite.color = if (cell.row + cell.col) % 2 == 0 {
                Color::rgb(0.9, 0.9, 0.9) // Light grey
            } else {
                Color::rgb(0.6, 0.6, 0.6) // Dark grey
            };
        }

        // Clear the winner banner text
        if let Ok(mut text) = winner_text_query.get_single_mut() {
            text.sections[0].value = "".to_string();
        }
    }
}

pub fn handle_ai_turn(
    mut game_state: ResMut<GameState>,
    mut query: Query<&Cell>,
) {
    if game_state.game_over || game_state.current_player != 'O' {
        return; // Skip if the game is over or it's not AI's turn
    }

    // Find all empty cells
    let empty_cells: Vec<&Cell> = query
        .iter()
        .filter(|cell| game_state.grid[cell.row][cell.col].is_none())
        .collect();

    // Randomly select an empty cell
    if let Some(cell) = empty_cells.iter().choose(&mut rand::thread_rng()) {
        game_state.grid[cell.row][cell.col] = Some('O');
        game_state.current_player = 'X'; // Switch back to the human player

        // Check for a win or draw after the AI's move
        if let Some(winner) = check_winner(&game_state.grid) {
            game_state.game_over = true;
            game_state.winner = Some(winner);
        } else if is_draw(&game_state.grid) {
            game_state.game_over = true;
            game_state.winner = None;
        }
    }
}

// Helper function to check for a winner
fn check_winner(grid: &[[Option<char>; GRID_SIZE]; GRID_SIZE]) -> Option<char> {
    // Check rows and columns
    for i in 0..GRID_SIZE {
        if grid[i][0].is_some() && grid[i][0] == grid[i][1] && grid[i][1] == grid[i][2] {
            return grid[i][0];
        }
        if grid[0][i].is_some() && grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] {
            return grid[0][i];
        }
    }

    // Check diagonals
    if grid[0][0].is_some() && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
        return grid[0][0];
    }
    if grid[0][2].is_some() && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
        return grid[0][2];
    }

    None // No winner
}

// Helper function to check for a draw
fn is_draw(grid: &[[Option<char>; GRID_SIZE]; GRID_SIZE]) -> bool {
    grid.iter().all(|row| row.iter().all(|&cell| cell.is_some()))
}

pub fn log_game_record(
    mut game_state: ResMut<GameState>, // Make game_state mutable to reset the flag
    mut score: ResMut<Score>,         // Make score mutable to ensure it updates only once
) {
    if game_state.game_over && !game_state.message_displayed {
        // Log the game result
        println!("Game Over!");
        if let Some(winner) = game_state.winner {
            println!("Winner: Player {}", winner);
            if winner == 'X' {
                score.wins_x += 1; // Increment score for Player X
            } else if winner == 'O' {
                score.wins_o += 1; // Increment score for Player O
            }
        } else {
            println!("It's a draw!");
            score.draws += 1; // Increment draw count
        }

        // Log the updated scores
        println!(
            "Scores: Player X - {}, Player O - {}, Draws - {}",
            score.wins_x, score.wins_o, score.draws
        );

        // Set the message_displayed flag to prevent repeated logging
        game_state.message_displayed = true;
    }
}
