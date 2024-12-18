use std::collections::VecDeque;
//simple turn-based game logic
use std::io;

use crate::entity_components::enemy::Enemy;
use crate::entity_components::moves::{Move, MoveType};
use crate::entity_components::status::Status;
use crate::entity_components::{entity::Entity, player::LevelUpType, player::Player, stats::Stats};
use rand::random;
use ratatui::widgets::{BorderType, ListState};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

const MAX_ATTACK_STR_HISTORY: usize = 200;
const RESET_MAGIC_CHOICE: bool = false; // if we want to reset the magic choice after a move is chosen

/// This keeps track of the current screen that the app is on.
#[derive(Clone, Copy)]
pub enum CurrentScreen {
    Main,       // Main gameplay screen
    LevelingUp, // player is leveling up
    Magic,      // choosing a magic move
    Warning,    // warning popup text
    Died,       // player died
    Exiting,
}

///Struct to hold the game state.
pub struct GameState {
    player: Player,
    enemy: Enemy,
    is_playing: bool,
    enemy_list: Vec<Enemy>,         // all game enemies
    status_list: Vec<Status>,       // all game statuses
    move_list: Vec<Move>,           // all game moves
    move_list_available_len: usize, // the length of available moves to the player

    // TUI
    current_screen: CurrentScreen,
    cur_attack_text_idx: usize, // for scrolling through the attack text
    cur_move_list_idx: usize,
    attack_text: VecDeque<String>, // NOTE: always push_front() to this.
    last_screen: CurrentScreen,    // Last screen to return to from the current (in case we need to)
    warning_text: String,
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
            the_enemy = create_temp_enemy();
        }

        let status_list = Status::create_status_list();
        let move_list = Move::create_move_list(&status_list);
        let move_list_available_len = Move::get_move_list(&move_list, player.level());

        GameState {
            player,
            enemy: the_enemy,
            is_playing,
            enemy_list: Enemy::create_enemy_list(),
            status_list,
            move_list,
            move_list_available_len,
            current_screen: CurrentScreen::Main,
            cur_attack_text_idx: 0, // start at the first index
            cur_move_list_idx: 0,   // start at first index
            attack_text: VecDeque::<String>::new(),
            last_screen: CurrentScreen::Main,
            warning_text: String::new(),
        }
    }

    ///the main game loop
    pub fn game_loop<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        //create a new random monster
        // TODO: this will cause issues if you want to start off with a specific enemy
        self.enemy = self.create_random_enemy();

        loop {
            // each loop is a tick, with the player or enemy able to try attacking.
            // but they can only both go again once both of them have gone

            terminal.draw(|frame| self.ui(frame))?;

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
                            self.change_screen(CurrentScreen::Exiting);
                        }
                        // TODO: for attacking moves, give back some value to let the game know when to change screens (like when a player levels up)
                        KeyCode::Char('1') => {
                            self.do_player_turn(MoveType::AttackMove);
                        }
                        KeyCode::Char('2') => {
                            self.change_screen(CurrentScreen::Magic);
                        }
                        KeyCode::Char('3') => {
                            self.do_player_turn(MoveType::DefendMove);
                        }
                        // nothing
                        _ => {}
                    },

                    // leveling up
                    CurrentScreen::LevelingUp => {
                        match key.code {
                            KeyCode::Char('q') => {
                                self.change_screen(CurrentScreen::Exiting);
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

                        // since we leveled up, we can now check what moves that are available.
                        self.move_list_available_len =
                            Move::get_move_list(&self.move_list, self.player.level());
                        // after we have chosen one of these, we now go back to the normal game
                        self.current_screen = CurrentScreen::Main;
                    }

                    CurrentScreen::Died => match key.code {
                        KeyCode::Char('q') => {
                            self.change_screen(CurrentScreen::Exiting);
                        }
                        // nothing
                        _ => {}
                    },

                    CurrentScreen::Warning => match key.code {
                        KeyCode::Char('q') => {
                            self.change_screen_no_save(self.last_screen);
                        }
                        KeyCode::Esc => {
                            self.change_screen_no_save(self.last_screen);
                        }
                        KeyCode::Enter => {
                            self.change_screen_no_save(self.last_screen);
                        }
                        // nothing
                        _ => {}
                    },

                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => {
                            break;
                        }
                        KeyCode::Char('n') => {
                            self.current_screen = self.last_screen;
                        }
                        _ => {}
                    },

                    CurrentScreen::Magic => match key.code {
                        KeyCode::Char('q') => {
                            self.change_screen(CurrentScreen::Main);
                        }
                        KeyCode::Esc => {
                            self.change_screen(CurrentScreen::Main);
                        }
                        // move up and down through the move list
                        KeyCode::Up => {
                            if self.cur_move_list_idx > 0 {
                                self.cur_move_list_idx -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.cur_move_list_idx < self.move_list_available_len - 1 {
                                self.cur_move_list_idx += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if self.do_player_turn(MoveType::MagicMove) {
                                self.change_screen(self.last_screen);
                            } else {
                                // display a warning if we can't do this move
                                self.display_warning(
                                    "Could not perform magic move! Have enough mana?",
                                );
                            }
                        }
                        _ => {}
                    },
                }
            }

            // only let the other events occur when the player is still playing
            if self.is_playing {
                self.perform_entity_check();
                self.do_enemy_turn();

                // do cleanup if both the player and enemy have gone
                self.perform_entity_check();
                self.truncate_attack_text();
            }
        }

        Ok(())
    }

    /// Changes the current screen and saves the
    /// state of the last screen.
    ///
    /// # Params
    /// - `new_screen` - The new screen to switch to.
    fn change_screen(&mut self, new_screen: CurrentScreen) {
        self.last_screen = self.current_screen;
        self.current_screen = new_screen;
    }

    /// Changes the current screen and without saving the
    /// state of the last screen. Typically used with warnings
    /// to avoid saving the warning as the last screen state.
    ///
    /// # Params
    /// - `new_screen` - The new screen to switch to.
    fn change_screen_no_save(&mut self, new_screen: CurrentScreen) {
        self.current_screen = new_screen;
    }

    /// Changes the current screen to display a warning and saves the
    /// the last screen.
    ///
    /// # Params
    /// - `warning_text` - The warning text to display.
    ///
    /// # Example
    /// ```rust,norun
    /// // something happens, causing an error
    /// self.display_warning("Could not perform magic attack.");
    /// ```
    fn display_warning(&mut self, warning_text: &str) {
        self.warning_text = String::from(warning_text);
        self.change_screen(CurrentScreen::Warning);
    }

    /// Checks the state of entities and ends the turn if an entity has died or
    /// all entities have gone.
    fn perform_entity_check(&mut self) {
        // end the turn if both entities have gone or if either one of the entities has died
        if (self.player.has_gone() && self.enemy.has_gone()) || self.check_entities() {
            self.end_turn();
        }
    }

    /// Truncates the attack text list so it doesn't overflow
    /// memory if the history gets too long.
    fn truncate_attack_text(&mut self) {
        let mut cur_len = self.attack_text.len();
        while cur_len > MAX_ATTACK_STR_HISTORY {
            self.attack_text.pop_front();
            cur_len = self.attack_text.len();
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

    /// Does the player's turn based on the player's choice of move.
    ///
    /// # Params
    /// - `turn_type` - The type of turn the player is making.
    fn do_player_turn(&mut self, turn_type: MoveType) -> bool {
        let mut ret = false; // move not done

        // if the player is faster and hasn't gone yet
        if (self.player.speed() >= self.enemy.speed() && !self.player.has_gone())
            || (self.enemy.has_gone() && !self.player.has_gone())
        {
            // do the action that the player wishes.
            // It is possible that these actions fail, due to the Player already having gone.
            // In this case, nothing occurs.
            ret = match turn_type {
                MoveType::AttackMove => self
                    .player
                    .attack_move(&mut self.enemy, &mut self.attack_text),

                MoveType::MagicMove => {
                    let temp = self.player.magic_move(
                        &mut self.enemy,
                        &self.move_list[self.cur_move_list_idx],
                        &mut self.attack_text,
                    );

                    if RESET_MAGIC_CHOICE {
                        // reset the move list index
                        self.cur_move_list_idx = 0;
                    }

                    temp
                }

                MoveType::DefendMove => self.player.defend_move(&mut self.attack_text),

                MoveType::NumMoveTypes => {
                    panic!("Invalid move type chosen by Player");
                }
            };
        }

        ret
    }

    /// Does the enemy's turn, allowing the enemy to choose
    /// what to do in this turn.
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

                    MoveType::MagicMove => {
                        self.enemy
                            .magic_move(&mut self.player, &mut self.attack_text);
                    }
                    MoveType::DefendMove => {
                        self.enemy.defend_move(&mut self.attack_text);
                    }
                    MoveType::NumMoveTypes => {
                        panic!("Invalid Enemy move type chosen");
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
    /// `true` if an entity died, false otherwise.
    ///
    /// # Notes
    /// - This function can change the value of the current screen,
    /// so ensure that care is taken that it does not override the current screen
    /// repeatedly in unwanted situations.
    fn check_entities(&mut self) -> bool {
        let mut output = false;

        if self.player.is_dead() {
            self.current_screen = CurrentScreen::Died;
            self.is_playing = false;

            // entity died
            output = true;
        } else if self.enemy.is_dead() {
            self.attack_text.push_back(String::from("The enemy died!"));
            let xp_dropped = self
                .enemy
                .drop_xp(self.player.level(), &mut self.attack_text);

            if self.player.gain_xp(xp_dropped) {
                self.current_screen = CurrentScreen::LevelingUp;
            }

            // create the enemy after the xp is dropped
            self.enemy = self.create_random_enemy();

            //entity died
            output = true;
        }

        output
    }

    ///Creates a new random monster
    fn create_random_enemy(&self) -> Enemy {
        let possible_enemies = self.get_possible_enemies();
        // pick a random enemy from the list
        let random_index = random::<usize>() % possible_enemies.len();

        possible_enemies[random_index].clone()
    }

    /// Gets the possible enemies that the player can fight.
    ///
    /// # Returns
    /// - A list of enemies that the player can fight, based on level.
    fn get_possible_enemies(&self) -> Vec<Enemy> {
        let mut result = Vec::new();

        // iterate through all enemies
        for i in 0..self.enemy_list.len() {
            // we can fight an enemy if it is below or close to the player's level
            if self.enemy_list[i].level() <= self.player.level() + 2 {
                result.push(self.enemy_list[i].clone());
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
                Constraint::Length(7), // the second section should never be smaller than one line tall but can expand if needed
                Constraint::Min(4),    // third section
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
                format!(
                    "    Mana: {}/{}",
                    self.player.mana(),
                    self.player.max_mana()
                ),
                Style::default().fg(Color::Blue),
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
            self.enemy.get_move_set_listitem(),
        ]);
        let enemy_ui_list = List::new(enemy_info_vec).block(enemy_info_block);

        frame.render_widget(enemy_ui_list, game_info_chunks[1]);

        // now we create a vector of ListItems to display the game text
        let game_text_block = Block::default().style(Style::default().bg(Color::Black));
        let mut list_items = Vec::<ListItem>::new();

        // create a new list from the attack list
        for element in &self.attack_text {
            list_items.push(ListItem::new(Line::from(Span::styled(
                element,
                Style::default().fg(Color::Yellow),
            ))));
        }

        let mut game_text_state = ListState::default().with_selected(Some(self.attack_text.len()));
        let game_text_list = List::new(list_items).block(game_text_block);

        //render the list
        frame.render_stateful_widget(game_text_list, chunks[2], &mut game_text_state);

        // create the bottom navigational bar
        // This has the current screen and what keybinds are available
        let current_navigation_text = vec![
            // The first half of the text
            match self.current_screen {
                CurrentScreen::Main => Span::styled("Playing", Style::default().fg(Color::Green)),
                CurrentScreen::LevelingUp => {
                    Span::styled("Leveling up", Style::default().fg(Color::Blue))
                }
                CurrentScreen::Magic => {
                    Span::styled("Choosing a magic move", Style::default().fg(Color::Blue))
                }
                CurrentScreen::Warning => {
                    Span::styled("Warning", Style::default().fg(Color::Yellow))
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
                    "(1) Attack, (2) Magic, (3) Defend (q) quit",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::LevelingUp => Span::styled(
                    "(1) Strength, (2) Magic, (3) Health",
                    Style::default().fg(Color::Blue),
                ),
                CurrentScreen::Magic => Span::styled(
                    "(↑↓) Change choice, (enter) Select choice, (q) Go back",
                    Style::default().fg(Color::Blue),
                ),
                CurrentScreen::Warning => Span::styled(
                    "(enter, q) Close Warning",
                    Style::default().fg(Color::Yellow),
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

        // popups
        match self.current_screen {
            CurrentScreen::LevelingUp => {
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

                let level_up_text =
                    Paragraph::new("Choose a trait to level up!").block(level_up_block);

                frame.render_widget(level_up_text, area);
            }

            CurrentScreen::Warning => {
                // create a block with a title and no borders
                let popup_block = Block::default()
                    .title("Warning!")
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::Yellow));

                // create a centered rectangle
                let area = self.centered_rect(60, 25, frame.area());
                frame.render_widget(Clear, area);
                frame.render_widget(popup_block, area);

                // show the player waht they have already entered
                let died_block = Block::default().title("Warning!").borders(Borders::ALL);

                let died_text = Paragraph::new(self.warning_text.clone()).block(died_block);

                frame.render_widget(died_text, area);
            }

            CurrentScreen::Died => {
                // create a block with a title and no borders
                let popup_block = Block::default()
                    .title("You died!")
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::DarkGray));

                // create a centered rectangle
                let area = self.centered_rect(60, 25, frame.area());
                frame.render_widget(Clear, area);
                frame.render_widget(popup_block, area);

                // show the player waht they have already entered
                let died_block = Block::default().title("You died!").borders(Borders::ALL);

                let died_text = Paragraph::new("(q) to quit").block(died_block);

                frame.render_widget(died_text, area);
            }

            CurrentScreen::Exiting => {
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

            CurrentScreen::Magic => {
                let popup_block = Block::default()
                    .title("Choosing Move")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .border_type(BorderType::Rounded);

                let mut ui_move_list_items = Vec::<ListItem>::new();

                // add the elements of move_list to the ui list (should be a small list)
                for i in 0..self.move_list_available_len {
                    if i == self.cur_move_list_idx {
                        ui_move_list_items.push(ListItem::new(Line::from(Span::styled(
                            format!(
                                "Name: {}, Cost: {}",
                                self.move_list[i].name(),
                                self.move_list[i].cost()
                            ),
                            Style::default().bg(Color::Blue).fg(Color::White),
                        ))));
                    } else {
                        ui_move_list_items.push(ListItem::new(Line::from(Span::styled(
                            format!(
                                "Name: {}, Cost: {}",
                                self.move_list[i].name(),
                                self.move_list[i].cost()
                            ),
                            Style::default().fg(Color::Blue),
                        ))));
                    };
                }

                let mut move_list_state =
                    ListState::default().with_selected(Some(self.cur_move_list_idx));
                let ui_move_list = List::new(ui_move_list_items).block(popup_block);

                let area = self.centered_rect(60, 60, frame.area());
                frame.render_stateful_widget(ui_move_list, area, &mut move_list_state);
            }

            _ => {}
        }
    }

    /// helper function to create a centered rect using up certain percentage of the available rect `r`.
    ///
    /// # Params
    /// - `percent_x` - The percentage of the screen the `Rect` will take up along the x-axis.
    /// - `percent_y` - The percentage of the screen the `Rect` will take up along the y-axis.
    /// - `r` - The `Rect` of the terminal we find the available space from.
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
fn create_temp_enemy() -> Enemy {
    let random_health_stat: u32 = (random::<u32>() % 10) + 1;
    Enemy::new(
        String::from("test_enemy"),
        Stats::new(random_health_stat, 10, 10, 10, 10, 0),
        1,
        false,
    )
}
