use bevy::prelude::*;

const GRID_SIZE: usize = 3;
const CELL_SIZE: f32 = 100.0;

#[derive(Component)]
struct Cell {
    row: usize,
    col: usize,
}

#[derive(Resource)]
struct GameState {
    grid: [[Option<char>; GRID_SIZE]; GRID_SIZE],
    current_player: char,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Initialize game state
    commands.insert_resource(GameState {
        grid: [[None; GRID_SIZE]; GRID_SIZE],
        current_player: 'X',
    });

    // Spawn grid cells
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.8, 0.8, 0.8),
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        col as f32 * CELL_SIZE - CELL_SIZE,
                        row as f32 * CELL_SIZE - CELL_SIZE,
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
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    query: Query<(&Cell, &Transform)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.get_primary() {
            if let Some(cursor_pos) = window.cursor_position() {
                for (cell, transform) in query.iter() {
                    let cell_pos = transform.translation.truncate();
                    let half_size = CELL_SIZE / 2.0;

                    if cursor_pos.x > cell_pos.x - half_size
                        && cursor_pos.x < cell_pos.x + half_size
                        && cursor_pos.y > cell_pos.y - half_size
                        && cursor_pos.y < cell_pos.y + half_size
                    {
                        if game_state.grid[cell.row][cell.col].is_none() {
                            game_state.grid[cell.row][cell.col] = Some(game_state.current_player);
                            game_state.current_player = if game_state.current_player == 'X' {
                                'O'
                            } else {
                                'X'
                            };
                        }
                    }
                }
            }
        }
    }
}

pub fn update_grid(
    game_state: Res<GameState>,
    mut query: Query<(&Cell, &mut Sprite)>,
) {
    for (cell, mut sprite) in query.iter_mut() {
        if let Some(mark) = game_state.grid[cell.row][cell.col] {
            sprite.color = if mark == 'X' {
                Color::rgb(1.0, 0.0, 0.0)
            } else {
                Color::rgb(0.0, 0.0, 1.0)
            };
        }
    }
}
