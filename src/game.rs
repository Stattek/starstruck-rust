//simple turn-based game logic
use std::{error::Error, io};

use crate::entity_components::enemy::Enemy;
use crate::entity_components::moves::{ElementType, Move, MoveType};
use crate::entity_components::status::Status;
use crate::entity_components::{entity::Entity, player::LevelUpType, player::Player, stats::Stats};
use colored::Colorize;
use rand::random;
use ratatui::style::Styled;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

/// This keeps track of the current screen that the app is on.
#[derive(Clone, Copy)]
pub enum CurrentScreen {
    Main,       // Main gameplay screen
    LevelingUp, // player is leveling up
    // Magic,      // choosing a magic move
    Died, // player died
    Exiting,
}

///Struct to hold the game state.
pub struct GameState {
    player: Player,
    enemy: Enemy,
    is_playing: bool,

    // TUI
    current_screen: CurrentScreen,
    attack_text: Vec<String>, // TODO: should there be a sane limit to this vector?
    last_screen: CurrentScreen, // Last screen to return to from the current (in case we need to)
}

impl GameState {
    ///Create new GameState object
    pub fn new(player: Player, enemy: Option<Enemy>) -> Self {
        let is_playing = !player.is_dead();
        let the_enemy;

        if let Some(temp_enemy) = enemy {
            //if we were given an enemy, use this
            the_enemy = temp_enemy;
        } else {
            //otherwise, create a new temporary one until we create a random one
            the_enemy = create_temp_monster();
        }

        GameState {
            player,
            enemy: the_enemy,
            is_playing,
            current_screen: CurrentScreen::Main,
            attack_text: Vec::<String>::new(),
            last_screen: CurrentScreen::Main,
        }
    }

    ///the main game loop
    pub fn game_loop<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        //create lists for creating enemies and statuses
        let enemy_list = Enemy::create_enemy_list();
        let status_list = Status::create_status_list();
        let move_list = Move::create_move_list(&status_list);

        //create a new random monster
        // TODO: this will cause issues if you want to start off with a specific enemy
        self.enemy = self.create_random_enemy(&enemy_list);

        loop {
            // each loop is a tick, with the player or enemy able to try attacking.
            // but they can only both go again once both of them have gone

            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // skip events that are not KeyEventKind::Press
                    continue;
                }
                match self.current_screen {
                    // we are currently on the Main screen
                    CurrentScreen::Main => match key.code {
                        // stop playing
                        KeyCode::Char('q') => {
                            self.last_screen = self.current_screen;
                            self.current_screen = CurrentScreen::Exiting;
                        }
                        // TODO: for attacking moves, give back some value to let the game know when to change screens (like when a player levels up)
                        KeyCode::Char('1') => {
                            self.do_player_turn(&move_list, MoveType::AttackMove);
                        }
                        KeyCode::Char('2') => {
                            self.do_player_turn(&move_list, MoveType::MagicMove);
                        }
                        KeyCode::Char('3') => {
                            self.do_player_turn(&move_list, MoveType::DefendMove);
                        }
                        // nothing
                        _ => {}
                    },

                    // leveling up
                    CurrentScreen::LevelingUp => {
                        match key.code {
                            KeyCode::Char('q') => {
                                self.last_screen = self.current_screen; // so we can return back to this screen
                                self.current_screen = CurrentScreen::Exiting;
                            }
                            KeyCode::Char('1') => {
                                self.player.level_up(LevelUpType::Strength);
                            }
                            KeyCode::Char('2') => {
                                self.player.level_up(LevelUpType::Magic);
                            }
                            KeyCode::Char('3') => {
                                self.player.level_up(LevelUpType::Health);
                            }
                            // nothing
                            _ => {}
                        }

                        // after we have chosen one of these, we now go back to the normal game
                        self.current_screen = CurrentScreen::Main;
                    }

                    CurrentScreen::Died => match key.code {
                        KeyCode::Char('q') => {
                            self.last_screen = self.current_screen;
                            self.current_screen = CurrentScreen::Exiting;
                        }
                        // nothing
                        _ => {}
                    },

                    // TODO: make screen for choosing magic skill
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => {
                            break;
                        }
                        KeyCode::Char('n') => {
                            self.current_screen = self.last_screen;
                        }
                        _ => {}
                    },

                    _ => {}
                }
            }

            // FIXME: this should be removed and bring logic into this function
            self.do_enemy_turn();

            // do cleanup if both the player and enemy have gone
            self.perform_entity_check(&enemy_list);
        }

        Ok(())
    }

    fn perform_entity_check(&mut self, enemy_list: &Vec<Enemy>) {
        // do cleanup if both have gone this turn
        if self.player.has_gone() && self.enemy.has_gone() {
            self.end_turn();
            self.check_entities(enemy_list);
        }
    }

    /// Ends a turn and does any required activities before the turn is over.
    fn end_turn(&mut self) {
        // always stop defending at the end of a turn
        self.player.stop_defending();
        self.enemy.stop_defending();

        self.player.tick_statuses(&mut self.attack_text);
        self.enemy.tick_statuses(&mut self.attack_text);

        self.player.allow_move();
        self.enemy.allow_move();
    }

    fn do_player_turn(&mut self, move_list: &Vec<Move>, turn_type: MoveType) {
        // if the  is faster and hasn't gone yet
        if (self.player.speed() >= self.enemy.speed() && !self.player.has_gone())
            || (self.enemy.has_gone() && !self.player.has_gone())
        {
            // do the action that the player wishes.
            // It is possible that these actions fail, due to the Player already having gone.
            // In this case, nothing occurs.
            match turn_type {
                MoveType::AttackMove => {
                    self.player
                        .attack_move(&mut self.enemy, &mut self.attack_text);
                }

                MoveType::MagicMove => {
                    self.player
                        .magic_move(&mut self.enemy, move_list, &mut self.attack_text);
                }

                MoveType::DefendMove => {
                    self.player.defend_move(&mut self.attack_text);
                }

                _ => {
                    // we should never reach this unless something has gone wrong
                    panic!("Invalid move type");
                }
            }
        }
    }

    // TODO: move code for this into the Enemy struct to avoid spaghetti code
    fn do_enemy_turn(&mut self) {
        // if the enemy is faster and hasn't gone yet
        if (self.enemy.speed() > self.player.speed() && !self.enemy.has_gone())
            || (self.player.has_gone() && !self.enemy.has_gone())
        {
            //get the turn type
            if let Some(turn_type) = self.enemy.get_turn_type() {
                match turn_type {
                    MoveType::AttackMove => {
                        self.enemy
                            .attack_move(&mut self.player, &mut self.attack_text);
                    }

                    MoveType::MagicMove => {}
                    MoveType::DefendMove => {}
                    _ => {
                        // we should never reach this unless something has gone wrong
                        panic!("Invalid move type");
                    }
                }
            }
        }
    }

    ///Checks if entities are dead and creates
    ///new random enemies if they die.
    ///
    ///If the player dies, the game is over.
    /// # Returns
    /// True if an entity died, false otherwise.
    fn check_entities(&mut self, enemy_list: &Vec<Enemy>) -> bool {
        let mut output = false;

        if self.player.is_dead() {
            self.current_screen = CurrentScreen::Died;
            self.is_playing = false;

            // entity died
            output = true;
        } else if self.enemy.is_dead() {
            self.attack_text.push(String::from("The enemy died!"));
            let xp_dropped = self
                .enemy
                .drop_xp(self.player.level(), &mut self.attack_text);

            if self.player.gain_xp(xp_dropped) {
                self.current_screen = CurrentScreen::LevelingUp;
            }

            // create the enemy after the xp is dropped
            self.enemy = self.create_random_enemy(&enemy_list);

            //entity died
            output = true;
        }

        output
    }

    ///Creates a new random monster
    fn create_random_enemy(&self, enemy_list: &Vec<Enemy>) -> Enemy {
        let possible_enemies = self.get_possible_enemies(enemy_list);
        // pick a random enemy from the list
        let random_index = random::<usize>() % possible_enemies.len();

        possible_enemies[random_index].clone()
    }

    /// Gets the possible enemies that the player can fight.
    ///
    /// # Returns
    /// - A list of enemies that the player can fight, based on level.
    fn get_possible_enemies(&self, enemy_list: &Vec<Enemy>) -> Vec<Enemy> {
        let mut result = Vec::new();

        // iterate through all enemies
        for i in 0..enemy_list.len() {
            // we can fight an enemy if it is below or close to the player's level
            if enemy_list[i].level() <= self.player.level() + 2 {
                result.push(enemy_list[i].clone());
            }
        }

        result
    }

    /// Basically, Widgets are constructed and drawn onto the screen using a `Frame`, which is placed
    /// within a specified `Rect`.
    /// If we want to divide our renderable `Rect` area into three distinct areas, we can use the `Layout`
    /// functionality in Ratatui.
    ///
    /// This function creates our UI elements. The `Frame` which contains the size of the terminal at render time
    /// allows us to take resizable terminals into account.
    ///
    /// # Params
    /// - `frame` - The frame of the terminal, which contains the size of the terminal at render time (to allow resizing)
    /// - `app` - The application state, so we know what to render
    fn ui(&mut self, frame: &mut Frame) {
        // Create the layout sections.
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7), // top segment is 7 lines tall
                Constraint::Min(4), // the second section should never be smaller than one line tall but can expand if needed
                Constraint::Min(4), // third section
                Constraint::Length(3), // bottom section is 3 lines tall
            ])
            .split(frame.area());

        /* we render the Main screen first so it's below the popup */

        // Create the title of the program using a Paragraph widget (which is used to display only text)
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Magenta).bg(Color::Black));

        // create a paragraph widget with text styled green
        let title = Paragraph::new(Text::styled(
            " ___ _               _               _   \n\
/ __| |_ __ _ _ _ __| |_ _ _ _  _ __| |__\n\
\\__ \\  _/ _` | '_(_-<  _| '_| || / _| / /\n\
|___/\\__\\__,_|_| /__/\\__|_|  \\_,_\\__|_\\_\\",
            Style::default().fg(Color::Magenta),
        ))
        .centered()
        .block(title_block); // tells it that we want to be part of the title_block

        // now we render it
        frame.render_widget(title, chunks[0]);

        /* render the enemy and player health */
        let game_info_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let player_info_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Blue).bg(Color::Black));

        let player_info_vec = Vec::<ListItem>::from([
            ListItem::new(Line::styled(
                self.player.name(),
                Style::default().fg(Color::Blue),
            )),
            ListItem::new(Line::styled(
                format!(
                    "    Health: {}/{}",
                    self.player.health(),
                    self.player.max_health()
                ),
                Style::default().fg(Color::Green),
            )),
            ListItem::new(Line::styled(
                format!("    Level: {}", self.player.level(),),
                Style::default().fg(Color::Blue),
            )),
            ListItem::new(Line::styled(
                format!(
                    "    Experience: {}/{}",
                    self.player.experience(),
                    self.player.max_experience()
                ),
                Style::default().fg(Color::Blue),
            )),
        ]);
        let player_ui_list = List::new(player_info_vec).block(player_info_block);

        frame.render_widget(player_ui_list, game_info_chunks[0]);

        let enemy_info_block = Block::new()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red).bg(Color::Black));

        let enemy_info_vec = Vec::<ListItem>::from([
            ListItem::new(Line::styled(
                self.enemy.name(),
                Style::default().fg(Color::Red),
            )),
            ListItem::new(Line::styled(
                format!(
                    "    Health: {}/{}",
                    self.enemy.health(),
                    self.enemy.max_health()
                ),
                Style::default().fg(Color::Green),
            )),
        ]);
        let enemy_ui_list = List::new(enemy_info_vec).block(enemy_info_block);

        frame.render_widget(enemy_ui_list, game_info_chunks[1]);

        // now we create a vector of ListItems to display the game text
        let game_text_block = Block::default().style(Style::default().bg(Color::Black));
        let mut list_items = Vec::<ListItem>::new();

        // loop through the key-value pairs and add them to the list
        for element in &self.attack_text {
            list_items.push(ListItem::new(Line::from(Span::styled(
                element,
                Style::default().fg(Color::Yellow),
            ))));
        }

        let game_text_list = List::new(list_items).block(game_text_block);

        //render the list
        frame.render_widget(game_text_list, chunks[2]);

        // create the bottom navigational bar
        // This has the current screen and what keybinds are available
        let current_navigation_text = vec![
            // The first half of the text
            match self.current_screen {
                CurrentScreen::Main => Span::styled("Playing", Style::default().fg(Color::Green)),
                CurrentScreen::LevelingUp => {
                    Span::styled("Leveling up", Style::default().fg(Color::Blue))
                }
                CurrentScreen::Died => Span::styled("Died", Style::default().fg(Color::Red)),
                CurrentScreen::Exiting => {
                    Span::styled("Exiting", Style::default().fg(Color::LightRed))
                }
            }
            .to_owned(),
        ];

        let mode_footer = Paragraph::new(Line::from(current_navigation_text)).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black)),
        );

        // Create a hint with available keys
        let current_keys_hint = {
            match self.current_screen {
                CurrentScreen::Main => Span::styled(
                    "(1) Attack, (2) Magic, (3) Defend (q) to quit",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::LevelingUp => Span::styled(
                    "(1) Strength, (2) Magic, (3) Health",
                    Style::default().fg(Color::Blue),
                ),
                CurrentScreen::Died => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
                CurrentScreen::Exiting => {
                    Span::styled("(q) to quit", Style::default().fg(Color::Red))
                }
            }
        };

        let key_notes_footer = Paragraph::new(Line::from(current_keys_hint)).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black)),
        );

        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[3]);

        // render footer paragraphs in their appropriate spaces
        frame.render_widget(mode_footer, footer_chunks[0]);
        frame.render_widget(key_notes_footer, footer_chunks[1]);

        if let CurrentScreen::LevelingUp = self.current_screen {
            // create a block with a title and no borders
            let popup_block = Block::default()
                .title("Level up!")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            // create a centered rectangle
            let area = self.centered_rect(60, 25, frame.area());
            frame.render_widget(Clear, area);
            frame.render_widget(popup_block, area);

            // show the player waht they have already entered
            let level_up_block = Block::default().title("Level Up!").borders(Borders::ALL);

            let level_up_text = Paragraph::new("Choose a trait to level up!").block(level_up_block);

            frame.render_widget(level_up_text, area);
        }

        // popup if the player died
        if let CurrentScreen::Died = self.current_screen {
            // create a block with a title and no borders
            let popup_block = Block::default()
                .title("You died!")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            // create a centered rectangle
            let area = self.centered_rect(60, 25, frame.area());
            frame.render_widget(popup_block, area);

            // show the player waht they have already entered
            let died_block = Block::default().title("You died!").borders(Borders::ALL);

            let died_text = Paragraph::new("(q) to quit").block(died_block);

            frame.render_widget(died_text, area);
        }

        // let the user choose to output the key-value pairs or close without printing anything
        if let CurrentScreen::Exiting = self.current_screen {
            frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Are you sure you want to quit? (y/n)",
                Style::default().fg(Color::Red),
            );
            // the `trim: false` will stop the text from being cut off when over the edge of the block
            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = self.centered_rect(60, 25, frame.area());
            frame.render_widget(exit_paragraph, area);
        }
    }

    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    ///
    /// # Params
    /// - `percent_x` - The percentage of the screen the `Rect` will take up along the x-axis
    /// - `percent_y` - The percentage of the screen the `Rect` will take up along the y-axis
    /// - `r` - The `Rect` of the terminal for finding the available space
    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }
}

// TODO: do we really need this??
fn create_temp_monster() -> Enemy {
    let random_health_stat: u32 = (random::<u32>() % 10) + 1;
    Enemy::new(
        String::from("test_enemy"),
        Stats::new(random_health_stat, 10, 10, 10, 10, 0),
        1,
        false,
    )
}
