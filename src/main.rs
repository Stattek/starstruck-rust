//create modules
mod entity_components;
mod game;

use std::error::Error;

use crate::entity_components::player::Player;
use crate::game::GameState;

fn main() -> Result<(), Box<dyn Error>> {
    // set up terminal
    let mut terminal = ratatui::init();
    terminal.clear()?;

    // create app and run it
    let player = Player::default();
    let mut the_game = GameState::new(player, None);
    the_game.game_loop(&mut terminal)?;

    // undo changes made to the user's terminal to exit
    // NOTE: in older versions, if an application exited without running this closing biolerplate,
    // the terminal would act very strange.
    ratatui::restore();

    Ok(())
}
