mod entities;
mod game;

use entities::*;
use game::game_loop;

fn main() {
    println!("Starstruck is back once again.");

    let mut player = Player::new("test".to_string(), Stats::new(10, 10, 10), 1, 0, false);

    // play the game
    game_loop(&mut player);
}
