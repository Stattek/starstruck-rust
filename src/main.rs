//create modules
mod entity_components;
mod game;
mod ui;

use std::{error::Error, io};

use crate::{
    game::{CurrentScreen, GameState},
    ui::ui,
};
use entity_components::{enemy::Enemy, entity::Entity, player::Player, stats::Stats};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    if false {
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
    }

    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // create backend and terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // create app and run it
    let player = Player::new(
        "test".to_string(),
        Stats::new(10, 10, 10, 10, 10, 0),
        1,
        0,
        false,
    );
    let mut the_game = GameState::new(player, None);
    the_game.game_loop(&mut terminal);
    run_app(&mut terminal, &mut the_game); // TODO: remove

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

/// This function will run the app.
/// It takes an object of type `Terminal` which implements the
/// `ratatui::backend::Backend` trait, so we can use any terminal backend.
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut GameState) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                // we are currently on the Main screen
                CurrentScreen::Main => match key.code {
                    // switch screen to editing
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    // switch screen to exiting
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    // nothing
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
