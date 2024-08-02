//create modules
mod entity_components;
mod game;
mod moves;

//imports
use colored::Colorize;
use entity_components::{entity::Entity, player::Player, stats::Stats};
use game::GameState;

fn main() {
    println!("{}", "Starstruck is back once again.".bold().blue());

    let player = Player::new("test".to_string(), Stats::new(10, 10, 10, 10), 1, 0, false);

    //TODO: temporary code possibly
    if true {
        let mut the_game = GameState::new(player, None);

        //play the game
        the_game.game_loop();
    }
}
