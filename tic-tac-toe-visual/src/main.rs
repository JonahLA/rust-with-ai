use bevy::prelude::*;
use bevy::log::LogPlugin; // Import LogPlugin for logging
use std::env; // Import for command-line argument parsing

mod game;

fn main() {
    // Parse the grid size from command-line arguments
    let args: Vec<String> = env::args().collect();
    let grid_size = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(3) // Default to 3x3 if parsing fails
    } else {
        3 // Default to 3x3 if no argument is provided
    };

    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::INFO, // Set log level to INFO
            filter: "wgpu=warn".to_string(), // Suppress wgpu warnings
        }))
        .insert_resource(game::Score {
            wins_x: 0,
            wins_o: 0,
            draws: 0,
        }) // Initialize score tracking
        .insert_resource(game::GridConfig { size: grid_size }) // Use the grid size from the command-line argument
        .add_systems(Startup, game::setup)
        .add_systems(Update, game::handle_clicks)
        .add_systems(Update, game::update_grid)
        .add_systems(Update, game::handle_restart) // Add restart system
        .add_systems(Update, game::handle_ai_turn) // Add AI turn system
        .add_systems(Update, game::log_game_record) // Log game record system
        .add_systems(Update, game::update_winner_text) // Update winner banner
        .add_systems(Update, game::update_score_text)  // Update score text
        .run();
}
