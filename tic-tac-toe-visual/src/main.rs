use bevy::prelude::*;
use bevy::log::LogPlugin; // Import LogPlugin for logging

mod game;

fn main() {
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
