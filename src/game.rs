// simple turn-based game logic

use std::io::stdin;

use rand::random;

use crate::{entities, Enemy, Entity, Player, Stats};

const PLAYER_TYPE: &str = "Player";
const ENEMY_TYPE: &str = "Enemy";

/// Struct to hold the game state.
pub struct GameState {
    players: Vec<Player>,
    enemies: Vec<Enemy>,
}

impl GameState {
    pub fn new(players: Vec<Player>, enemies: Vec<Enemy>) -> Self {
        GameState { players, enemies }
    }
    /// the main game loop
    pub fn game_loop(&mut self) {
        // create a new random monster for now
        self.enemies.push(create_random_monster()); // TODO: implement and remove comment

        loop {
            // each loop through here is a full turn

            // do all turns for each entity
            // self.do_turns(); // TODO: implement and remove comment

            // TODO: get rid of this debug code
            if true {
                self.players[0].do_turn();
                self.enemies[0].do_turn();

                if self.enemies[0].is_dead() {
                    self.enemies.pop();
                    self.enemies.push(create_random_monster());
                }
            }

            // self.check_and_handle_dead_entities(); // TODO: implement and remove comment
        }
    }

    /// Chooses to do a turn from either of these two
    /// # Returns
    /// - A tuple with a `String` to represent the type of Entity that this is
    /// and a `u32` for the index into the entity `Vec`
    fn choose_turn_from_all(&mut self) -> (String, u32) {
        // hold any entity

        let return_index: u32;
        let return_string: String;

        let the_player = self.get_fastest_player();
        let the_enemy = self.get_fastest_enemy();

        // get fastest player

        // in case of a tie, prefer the player
        if self.players[the_player as usize].get_speed()
            >= self.enemies[the_enemy as usize].get_speed()
        {
            return_index = the_player;
            return_string = String::from(PLAYER_TYPE);
        } else {
            return_index = the_enemy;
            return_string = String::from(ENEMY_TYPE);
        }

        (return_string, return_index)
    }

    /// Gets the fastest player in the list
    fn get_fastest_player(&self) -> u32 {
        let mut fastest_player_index = 0;
        let mut max_speed = self.players[0].get_speed();

        for i in 1..self.players.len() {
            if max_speed < self.players[i].get_speed() {
                // new highest speed
                max_speed = self.players[i].get_speed();
                fastest_player_index = i as u32; // forgor how to typecast 💀🫃
            }
        }

        fastest_player_index
    }

    /// Gets the fastest enemy in the list
    fn get_fastest_enemy(&self) -> u32 {
        let mut fastest_enemy_index = 0;
        let mut max_speed = self.enemies[0].get_speed();

        for i in 1..self.enemies.len() {
            if max_speed < self.enemies[i].get_speed() {
                // new highest speed
                max_speed = self.enemies[i].get_speed();
                fastest_enemy_index = i as u32;
            }
        }

        fastest_enemy_index
    }
}

/// Creates a new random monster
fn create_random_monster() -> Enemy {
    // enemy with health between 10 and 250
    let random_health_stat: u32 = (random::<u32>() % 10) + 1;

    Enemy::new(
        String::from("test_enemy"),
        Stats::new(random_health_stat, 10, 10),
        1,
        false,
    )
}
