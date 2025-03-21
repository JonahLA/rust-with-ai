use bevy::prelude::*;

mod game;

fn main() {
    App::new() // Updated method
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, game::setup)
        .add_systems(Update, game::handle_clicks)
        .add_systems(Update, game::update_grid)
        .run();
}
