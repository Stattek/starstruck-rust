mod entities;
mod game;

use entities::*;
use game::game_loop;

fn main() {
    println!("Starstruck is back once again.");

    let player = Player::new("test".to_string(), 100, 50, 10, 1, 0);

    // play the game
    game_loop(&player);
}
