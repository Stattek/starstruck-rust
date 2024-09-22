//simple turn-based game logic

use colored::Colorize;
use rand::random;

use crate::entity_components::enemy::Enemy;
use crate::entity_components::moves::{ElementType, Move, MoveType};
use crate::entity_components::status::Status;
use crate::{Entity, Player, Stats};

///Struct to hold the game state.
pub struct GameState {
    player: Player,
    enemy: Enemy,
    is_playing: bool,
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
        }
    }

    ///the main game loop
    pub fn game_loop(&mut self) {
        //create lists for creating enemies and statuses
        let enemy_list = Enemy::create_enemy_list();
        let status_list = Status::create_status_list();
        let move_list = Move::create_move_list(&status_list);

        //create a new random monster
        // TODO: this will cause issues if you want to start off with a specific enemy
        self.enemy = self.create_random_enemy(&enemy_list);

        while self.is_playing {
            //each loop through here is a full turn
            self.do_turns_in_order(&enemy_list, &move_list);
        }
    }

    ///Does turns in order of speed
    ///# Returns
    ///- A tuple with a `String` to represent the type of Entity that this is
    ///and a `u32` for the index into the entity `Vec`
    fn do_turns_in_order(&mut self, enemy_list: &Vec<Enemy>, move_list: &Vec<Move<'_>>) {
        if self.player.speed() >= self.enemy.speed() {
            //prefer player if speeds are equal

            self.print_basic_hud();

            // do turns
            self.do_player_turn(move_list);

            // check entities before next turn is done
            if !self.check_entities(enemy_list) {
                self.do_enemy_turn();
            }
        } else {
            // enemy is faster

            // do enemy turn
            self.do_enemy_turn();

            // check entities before doing the player's turn
            if !self.check_entities(enemy_list) {
                self.print_basic_hud();

                // player turn
                self.player.get_turn_type();
            }
        }

        self.end_turn();
        self.check_entities(enemy_list);
    }

    /// Ends a turn and does any required activities before the turn is over.
    fn end_turn(&mut self) {
        // always stop defending at the end of a turn
        self.player.stop_defending();
        self.enemy.stop_defending();

        self.player.tick_statuses();
        self.enemy.tick_statuses();
    }

    fn print_basic_hud(&self) {
        // print info
        println!(); // make a new line now
        self.player.print_info();
        self.enemy.print_info();
    }

    fn do_player_turn(&mut self, move_list: &Vec<Move>) {
        let mut done_turn = false;
        while !done_turn {
            //get the turn type
            if let Some(turn_type) = self.player.get_turn_type() {
                //now act upon this turn type
                match turn_type {
                    MoveType::AttackMove => {
                        done_turn = self.player.attack_move(&mut self.enemy);
                    }

                    MoveType::MagicMove => {
                        done_turn = self.player.magic_move(&mut self.enemy, move_list);
                    }

                    MoveType::DefendMove => {
                        done_turn = self.player.defend_move();
                    }

                    _ => {
                        // we should never reach this unless something has gone wrong
                        panic!("Invalid move type");
                    }
                }
            }
        }
    }

    // TODO: move code for this into the Enemy struct to avoid spaghetti code
    fn do_enemy_turn(&mut self) {
        //get the turn type
        if let Some(turn_type) = self.enemy.get_turn_type() {
            match turn_type {
                MoveType::AttackMove => {
                    self.enemy.attack_move(&mut self.player);
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

    ///Checks if entities are dead and creates
    ///new random enemies if they die.
    ///
    ///If the player dies, the game is over.
    /// # Returns
    /// True if an entity died, false otherwise.
    fn check_entities(&mut self, enemy_list: &Vec<Enemy>) -> bool {
        let mut output = false;

        if self.player.is_dead() {
            println!("{}", "\nYou died!".red().bold());
            self.is_playing = false;

            // entity died
            output = true;
        } else if self.enemy.is_dead() {
            println!("{}", "\nThe enemy died!".green());
            let xp_dropped = self.enemy.drop_xp(self.player.level());
            self.player.gain_xp(xp_dropped);

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
