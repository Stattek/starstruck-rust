mod entities;
mod game;

use colored::Colorize;
use entities::*;
use game::GameState;

fn main() {
    println!("{}", "Starstruck is back once again.".bold().blue());

    let mut player = Player::new("test".to_string(), Stats::new(10, 10, 10), 1, 0, false);
    
    // TODO: temporary code possibly
    if true {
        let mut players = Vec::new();
        players.push(player);

        let enemies = Vec::new();

        let mut the_game = GameState::new(players, enemies);

        // play the game
        the_game.game_loop();
    }
}
