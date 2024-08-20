//create modules
mod entity_components;
mod game;

//imports
use colored::Colorize;
use entity_components::{entity::Entity, player::Player, stats::Stats};
use game::GameState;

fn main() {
    println!(
        "{}",
        "  _____ _                 _                   _
 / ____| |               | |                 | |
| (___ | |_ __ _ _ __ ___| |_ _ __ _   _  ___| | __
 \\___ \\| __/ _` | '__/ __| __| '__| | | |/ __| |/ /
 ____) | || (_| | |  \\__ \\ |_| |  | |_| | (__|   <
|_____/ \\__\\__,_|_|  |___/\\__|_|   \\__,_|\\___|_|\\_\\
"
        .blue()
    );

    let player = Player::new(
        "test".to_string(),
        Stats::new(10, 10, 10, 10, 10, 0),
        1,
        0,
        false,
    );

    //TODO: temporary code possibly
    if true {
        let mut the_game = GameState::new(player, None);

        //play the game
        the_game.game_loop();
    }
}
