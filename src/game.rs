//simple turn-based game logic

use colored::Colorize;
use rand::random;

use crate::entity_components::enemy::Enemy;
use crate::entity_components::moves::{Move, MoveType};
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
            //otherwise, create a new random one
            the_enemy = create_random_monster();
        }

        GameState {
            player,
            enemy: the_enemy,
            is_playing,
        }
    }

    ///the main game loop
    pub fn game_loop(&mut self) {
        //create a new random monster for now
        self.enemy = create_random_monster();

        while self.is_playing {
            //each loop through here is a full turn
            self.do_turns_in_order();
        }
    }

    ///Does turns in order of speed
    ///# Returns
    ///- A tuple with a `String` to represent the type of Entity that this is
    ///and a `u32` for the index into the entity `Vec`
    fn do_turns_in_order(&mut self) {
        if self.player.speed() >= self.enemy.speed() {
            //prefer player if speeds are equal

            // print info
            println!(); // make a new line now
            self.player.print_info();
            self.enemy.print_info();

            // do turns
            self.do_player_turn();

            // check entities before next turn is done
            if !self.check_entities() {
                self.do_enemy_turn();
            }
        } else {
            // enemy is faster

            // do enemy turn
            self.do_enemy_turn();

            // check entities before doing the player's turn
            if !self.check_entities() {
                // print info
                println!(); //make new line
                self.player.print_info();
                self.enemy.print_info();

                // player turn
                self.player.get_turn_type();
            }
        }

        self.check_remove_entity_statuses();
        self.check_entities();
    }

    /// Removes all status effects from entities that need to be done at the
    /// end of a turn.
    ///
    /// FUTURE: add more unique statuses
    fn check_remove_entity_statuses(&mut self) {
        // always stop defending at the end of a turn
        self.player.stop_defending();
        self.enemy.stop_defending();

        // TODO: remove more statuses
    }

    fn do_player_turn(&mut self) {
        //get the turn type
        if let Some(turn_type) = self.player.get_turn_type() {
            //now act upon this turn type
            match turn_type {
                MoveType::AttackMove => {
                    // attack the enemy with a random amount of damage
                    let random_damage = self.player.get_random_attack_dmg();

                    let damage_dealt = self.player.attack_entity(random_damage, &mut self.enemy);
                    // display the damage dealt
                    self.display_attack_text(self.player.name(), self.enemy.name(), damage_dealt);
                }

                MoveType::MagicMove => {
                    // get a list of moves that the player meets the requirements for
                    let move_list = Move::get_move_list(self.player.level());
                    let move_list_len = move_list.len(); // save the length to avoid borrowing moved value

                    // print all of the moves
                    for index in 0..move_list_len {
                        let cur_move = &move_list[index];

                        println!(
                            "{} | Cost: {}",
                            cur_move.name().on_blue().black(),
                            cur_move.cost()
                        )
                    }

                    // choose thlet =e move
                    let mut choice = -1;
                    while choice < 0 || choice >= (move_list_len as i32) {
                        println!("{}", "Choose a move:".on_white().black());

                        //take user input
                        let user_input = self.player.get_player_input();

                        //gives back -1 if the input is incorrect
                        choice = user_input.parse::<i32>().unwrap_or(-1) - 1; // minus one to get index
                    }

                    // TODO: attack the enemy with the move
                    let random_damage = move_list[choice as usize]
                        .generate_random_amount(self.player.magic_strength());

                    let damage_dealt = self.player.attack_entity(random_damage, &mut self.enemy);
                    // display the damage that was dealt
                    self.display_attack_text(self.player.name(), self.enemy.name(), damage_dealt);
                }
                MoveType::DefendMove => {
                    self.player.start_defending();

                    // tell player that they started defending
                    let mut output_str = String::new();
                    output_str.push_str(self.player.name().as_str());
                    output_str.push_str(" began defending for 1 turn.");

                    println!("{}", output_str.green());
                }
                _ => {
                    // we should never reach this unless something has gone wrong
                    panic!("Invalid move type");
                }
            }
        }
    }

    fn do_enemy_turn(&mut self) {
        //get the turn type
        if let Some(turn_type) = self.enemy.get_turn_type() {
            match turn_type {
                MoveType::AttackMove => {
                    // attack the player with a random amount of damage
                    let random_damage = self.enemy.get_random_attack_dmg();

                    let damage_dealt = self.enemy.attack_entity(random_damage, &mut self.player);
                    // display the text for an attack
                    self.display_attack_text(self.enemy.name(), self.player.name(), damage_dealt);
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

    /// Displays text when an entity attacks another.
    fn display_attack_text(
        &self,
        from_entity_name: String,
        victim_entity_name: String,
        damage_dealt: u32,
    ) {
        // TODO: display text for this
        // cursed string creation to colorize this string when we print it out 💀
        let mut output_str = String::new();
        output_str.push_str(from_entity_name.as_str());
        output_str.push_str(" did ");
        output_str.push_str(damage_dealt.to_string().as_str());
        output_str.push_str(" damage to ");
        output_str.push_str(victim_entity_name.as_str());

        println!("{}", output_str.red());
    }

    ///Checks if entities are dead and creates
    ///new random enemies if they die.
    ///
    ///If the player dies, the game is over.
    /// # Returns
    /// True if an entity died, false otherwise.
    fn check_entities(&mut self) -> bool {
        let mut output = false;

        if self.player.is_dead() {
            println!("{}", "\nYou died!".red().bold());
            self.is_playing = false;

            // entity died
            output = true;
        } else if self.enemy.is_dead() {
            println!("{}", "\nThe enemy died!".green());
            self.enemy = create_random_monster();

            //entity died
            output = true;
        }

        output
    }
}

///Creates a new random monster
fn create_random_monster() -> Enemy {
    //enemy with health between 10 and 250
    let random_health_stat: u32 = (random::<u32>() % 10) + 1;

    Enemy::new(
        String::from("test_enemy"),
        Stats::new(random_health_stat, 10, 10, 10, 10, 0),
        1,
        false,
    )
}
