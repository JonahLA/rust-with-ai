// src/main.rs

mod grid;
mod player;
mod game;

use game::Game;
use player::Player;

use std::io;

fn main() {
    let player_x = Player::new("Player X".to_string(), 'X');
    let player_o = Player::new("Player O".to_string(), 'O');
    let mut game = Game::new(player_x, player_o);

    println!("Welcome to Tic-Tac-Toe!");
    loop {
        game.get_grid().display();
        println!(
            "{}'s turn. Enter your move (row and column):",
            game.get_current_player().name
        );

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        if input.len() != 2 || input[0] > 2 || input[1] > 2 {
            println!("Invalid input. Please enter row and column numbers between 0 and 2.");
            continue;
        }

        match game.play_turn(input[0], input[1]) {
            Ok(_) => {
                if game.check_winner() {
                    game.get_grid().display();
                    println!("{} wins!", game.get_current_player().name);
                    break;
                } else if game.is_draw() {
                    game.get_grid().display();
                    println!("It's a draw!");
                    break;
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}