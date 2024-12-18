//create modules
mod entity_components;
mod game;

use std::{error::Error, io};

use crate::entity_components::player::Player;
use crate::game::GameState;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // create backend and terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // create app and run it
    let player = Player::default();
    let mut the_game = GameState::new(player, None);
    the_game.game_loop(&mut terminal)?;

    // undo changes made to the user's terminal to exit
    // NOTE: if an application exits without running this closing biolerplate, the terminal will act very strange,
    // so we should handle our error in a way that we can call this last piece of code
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
