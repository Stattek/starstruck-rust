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
        let enemy_list = Self::create_enemy_list();
        let status_list = Self::create_status_list();
        let move_list = Self::create_move_list(&status_list);

        //create a new random monster
        // TODO: this will cause issues if you want to start off with a specific enemy
        self.enemy = self.create_random_enemy(&enemy_list);

        while self.is_playing {
            //each loop through here is a full turn
            self.do_turns_in_order(&enemy_list, &move_list);
        }
    }

    fn create_enemy_list() -> Vec<Enemy> {
        vec![
            Enemy::new("Spider".to_string(), Stats::new(5, 0, 4, 2, 1, 0), 1, false),
            Enemy::new(
                "Skeleton".to_string(),
                Stats::new(3, 0, 3, 5, 4, 0),
                1,
                false,
            ),
            Enemy::new(
                "Dragon".to_string(),
                Stats::new(20, 100, 10, 10, 10, 10),
                8,
                false,
            ),
        ]
    }

    fn create_status_list() -> Vec<Status> {
        vec![
            Status::new(String::from("Burn"), 10, false, 0, 5),
            Status::new(String::from("Frostburn"), 12, false, 0, 5),
        ]
    }

    fn get_status_from(target_name: &str, status_list: &Vec<Status>) -> Option<Status> {
        let mut result: Option<Status> = None;

        // go through the status list and find the one that matches our target string
        for i in 0..status_list.len() {
            if target_name == status_list[i].name() {
                result = Some(status_list[i].clone());
                break;
            }
        }

        result
    }

    fn create_move_list(status_list: &Vec<Status>) -> Vec<Move<'_>> {
        vec![
            Move::new(
                "FireOne",
                12,
                2,
                1,
                ElementType::Fire,
                Self::get_status_from("Burn", status_list),
            ),
            Move::new("WindOne", 14, 2, 3, ElementType::Wind, None),
            Move::new("EarthOne", 16, 2, 5, ElementType::Earth, None),
            Move::new("WaterOne", 20, 2, 6, ElementType::Water, None),
        ]
    }

    ///Does turns in order of speed
    ///# Returns
    ///- A tuple with a `String` to represent the type of Entity that this is
    ///and a `u32` for the index into the entity `Vec`
    fn do_turns_in_order(&mut self, enemy_list: &Vec<Enemy>, move_list: &Vec<Move<'_>>) {
        if self.player.speed() >= self.enemy.speed() {
            //prefer player if speeds are equal

            // print info
            println!(); // make a new line now
            self.player.print_info();
            self.enemy.print_info();

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
                // print info
                println!(); //make new line
                self.player.print_info();
                self.enemy.print_info();

                // player turn
                self.player.get_turn_type();
            }
        }

        self.end_turn();
        self.check_entities(enemy_list);
    }

    /// Ends a turn and does any required activities before the turn is over.
    ///
    /// FUTURE: add more unique statuses
    fn end_turn(&mut self) {
        // always stop defending at the end of a turn
        self.player.stop_defending();
        self.enemy.stop_defending();

        self.player.tick_statuses();
        self.enemy.tick_statuses();
    }

    // TODO: move some of this code and similar code into the structs for the entities, so we can de-spaghettify before it gets bad
    fn do_player_turn(&mut self, move_list: &Vec<Move>) {
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
                    let move_list = Move::get_move_list(move_list, self.player.level());
                    let move_list_len = move_list.len(); // save the length to avoid borrowing moved value

                    // print all of the moves
                    for index in 0..move_list_len {
                        let cur_move = &move_list[index];

                        println!(
                            "{}:{} | Cost: {}",
                            (index + 1),
                            cur_move.name().on_blue().black(),
                            cur_move.cost()
                        )
                    }

                    // choose the move
                    let mut choice = -1;
                    while choice < 0 || choice >= (move_list_len as i32) {
                        println!("{}", "Choose a move:".on_white().black());

                        //take user input
                        let user_input = self.player.get_player_input();

                        //gives back -1 if the input is incorrect
                        choice = user_input.parse::<i32>().unwrap_or(-1) - 1; // minus one to get index
                    }

                    let random_damage = move_list[choice as usize]
                        .generate_random_amount(self.player.magic_strength());

                    let damage_dealt = self.player.attack_entity(random_damage, &mut self.enemy);
                    // display the damage that was dealt
                    self.display_attack_text(self.player.name(), self.enemy.name(), damage_dealt);

                    // roll for random chance to apply status if it exists
                    if move_list[choice as usize].roll_status_chance() {
                        self.enemy
                            .apply_status(&move_list[choice as usize].get_status().unwrap());
                    }
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

    // TODO: move code for this into the Enemy struct to avoid spaghetti code
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

    fn apply_status(&self, entity: &mut dyn Entity, status: &Status) {
        println!("{} appled to {}", status.name(), entity.name());
        entity.apply_status(status);
    }

    /// Displays text when an entity attacks another.
    fn display_attack_text(
        &self,
        from_entity_name: String,
        victim_entity_name: String,
        damage_dealt: u32,
    ) {
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
