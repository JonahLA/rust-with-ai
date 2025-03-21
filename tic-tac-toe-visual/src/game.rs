use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const GRID_SIZE: usize = 3;
const CELL_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct Cell {
    row: usize,
    col: usize,
}

#[derive(Resource)]
pub struct GameState {
    grid: [[Option<char>; GRID_SIZE]; GRID_SIZE],
    current_player: char,
    game_over: bool, // Track if the game is over
    winner: Option<char>, // Track the winner ('X', 'O', or None for a draw)
    message_displayed: bool, // Track if the game-over message has been displayed
}

pub fn setup(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
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
        game_state.message_displayed = true; // Set the flag to true
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
