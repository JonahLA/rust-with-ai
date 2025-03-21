use bevy::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(game::setup)
        .add_system(game::handle_clicks)
        .add_system(game::update_grid)
        .run();
}
