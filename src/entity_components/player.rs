use crate::entity_components::entity::Entity;
use crate::entity_components::moves::Move;
use crate::entity_components::moves::MoveType;
use crate::entity_components::stats::Stats;
use colored::Colorize;
use ratatui::text;
use std::fmt::format;
use std::io;

use super::status::Status;

const XP_TO_LEVEL_UP: u32 = 100;

///Struct to represent the Player.
///Implements the Entity trait
pub struct Player {
    name: String,
    health: u32,
    max_health: u32,
    mana: u32,
    max_mana: u32,
    stats: Stats,
    level: u32,
    xp: u32,
    has_gone: bool,
    statuses: Vec<Status>,
}

impl Player {
    pub fn new(name: String, stats: Stats, level: u32, xp: u32, has_gone: bool) -> Self {
        // start with this mana and hp
        let starting_health = stats.calculate_max_health();
        let starting_mana = stats.calculate_max_mana();
        Self {
            name,
            health: starting_health,
            max_health: starting_health,
            mana: starting_mana,
            max_mana: starting_mana,
            stats,
            level,
            xp,
            has_gone,
            statuses: Vec::new(), // start with no statuses
        }
    }

    ///Makes the Player gain xp and level up if they reach 100 xp
    pub fn gain_xp(&mut self, amount: u32) {
        self.xp += amount;

        // for level-up chains
        while self.xp >= XP_TO_LEVEL_UP {
            self.xp -= XP_TO_LEVEL_UP;
            self.level_up();
        }
    }

    /// Take input from the user.
    ///
    /// # Returns
    ///
    /// - A `String` containing the trimmed input from the user.
    pub fn get_player_input(&self) -> String {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        // return the user input string
        String::from(user_input.trim())
    }

    /// The player levels up and gets to choose stats to increase
    fn level_up(&mut self) {
        // increment level
        self.level += 1;
        println!(
            "Choose a stat to increase:\n1. {}\n2. {}\n3. {}",
            "Strength".yellow(),
            "Magic".blue(),
            "Health".red()
        );

        let mut user_input = self.get_player_input();

        let mut choice = user_input.parse::<i32>().unwrap_or(-1);

        while choice <= 0 || choice > 3 {
            println!(
                "Choose a stat to increase:\n1. {}\n2. {}\n3. {}",
                "Strength".yellow(),
                "Magic".blue(),
                "Health".red()
            );

            //take user input
            user_input = self.get_player_input();

            //gives back -1 if the input is incorrect
            choice = user_input.parse::<i32>().unwrap_or(-1);
        }

        match choice {
            1 => self.stats.increase_physical(),
            2 => self.stats.increase_magic(),
            3 => self.stats.increase_health(),
            _ => panic!("Invalid choice"), // just panic if we get an invalid answer
        }
        self.reset_stats();
    }

    /// Recalculates stats and gives the player max health and mana
    fn reset_stats(&mut self) {
        let new_health = self.stats.calculate_max_health();
        let new_mana = self.stats.calculate_max_mana();
        self.health = new_health;
        self.max_health = new_health;
        self.mana = new_mana;
        self.max_mana = new_mana;
    }

    /// Displays attack text for the `Player` attacking another `Entity`.
    fn display_attack_text(
        &self,
        victim_entity_name: String,
        damage_dealt: u32,
        text_vec: &mut Vec<String>,
    ) {
        text_vec.push(format!(
            "You did {} damage to {}",
            damage_dealt.to_string(),
            victim_entity_name
        ));
    }

    /// The `Player` performs a magic move against another `Entity`.
    ///
    /// # Params
    /// - `target` - The target of the attack.
    /// - `move_list` - The full move list, for finding the moves this `Player` can use.
    pub fn magic_move(
        &mut self,
        target: &mut dyn Entity,
        move_list: &Vec<Move<'_>>,
        text_vec: &mut Vec<String>,
    ) -> bool {
        // get a list of moves that the player meets the requirements for
        let move_list = Move::get_move_list(move_list, self.level);
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
        while choice < 0
            || choice >= (move_list_len as i32)
            || move_list[choice as usize].cost() > self.mana
        {
            println!(
                "{}",
                "Choose a move (q/quit to go back):".on_white().black()
            );

            //take user input
            let user_input = self.get_player_input();
            if user_input.to_lowercase() == "q" || user_input.to_lowercase() == "quit" {
                // player wants to stop choosing a magic move
                return false;
            }

            //gives back -1 if the input is incorrect
            choice = user_input.parse::<i32>().unwrap_or(-1) - 1; // minus one to get index

            if choice >= 0
                && choice < (move_list_len as i32)
                && move_list[choice as usize].cost() > self.mana
            {
                println!("{}", "Move costs too much mana!".black().on_red());
            }
        }

        let random_damage =
            move_list[choice as usize].generate_random_amount(self.magic_strength());

        let damage_dealt = self.attack_entity(random_damage, target);
        // use the mana from this move
        self.use_mana(move_list[choice as usize].cost());
        // display the damage that was dealt
        self.display_attack_text(target.name(), damage_dealt, text_vec);

        // roll for random chance to apply status if it exists
        if move_list[choice as usize].roll_status_chance() {
            target.apply_status(&move_list[choice as usize].get_status().unwrap());
        }

        // no error
        true
    }

    pub fn defend_move(&mut self) -> bool {
        self.start_defending();

        // tell player that they started defending
        let mut output_str = String::new();
        output_str.push_str(self.name.as_str());
        output_str.push_str(" began defending for 1 turn.");

        println!("{}", output_str.green());

        // no error
        true
    }

    pub fn experience(&self) -> u32 {
        self.xp
    }

    pub fn max_experience(&self) -> u32 {
        XP_TO_LEVEL_UP
    }
}

//entity implementation for player
impl Entity for Player {
    ///Makes the Player take damage
    fn take_damage(&mut self, amount: u32) -> u32 {
        let damage_taken = self.stats.calc_damage_taken(amount);

        if damage_taken > self.health {
            self.health = 0;
        } else {
            self.health -= damage_taken;
        }

        damage_taken
    }

    ///Heals the Player
    fn heal(&mut self, amount: u32) {
        self.health += amount;
    }

    ///Makes the Plaer use mana
    fn use_mana(&mut self, amount: u32) {
        if amount > self.mana {
            self.mana = 0;
        } else {
            self.mana -= amount;
        }
    }

    ///Gets the speed of the Player
    fn speed(&self) -> u32 {
        self.stats.get_speed()
    }

    ///Checks to see if the Player is dead
    fn is_dead(&self) -> bool {
        self.health == 0
    }

    ///Checks to see if the Player has gone this turn
    fn gone_this_turn(&self) -> bool {
        self.has_gone
    }

    ///Player chooses attack type, and it is returned.
    ///
    /// # FUTURE: Move this to GameState
    fn get_turn_type(&mut self) -> Option<MoveType> {
        self.has_gone = true;

        let mut choice = -1;
        while choice <= 0 || choice > (MoveType::NumMoveTypes as i32) {
            // println!(
            //     "What do you want to do?\n\t1. {}\n\t2. {}\n\t3. {}",
            //     "Attack".red(),
            //     "Magic".blue(),
            //     "Defend".white()
            // );

            //take user input
            let user_input = self.get_player_input();

            //gives back -1 if the input is incorrect
            choice = user_input.parse::<i32>().unwrap_or(-1);
        }

        //return the correct type
        match choice {
            1 => Some(MoveType::AttackMove),
            2 => Some(MoveType::MagicMove),
            3 => Some(MoveType::DefendMove),
            _ => None,
        }
    }

    fn get_random_attack_dmg(&self) -> u32 {
        self.stats.generate_random_attack_dmg()
    }

    ///Print the Player info
    fn print_info(&self) {
        println!(
            "{}:\n\t{}{}\n\t{}{} / {}\n\t{}{} / {}\n\t{} {} / {}",
            self.name,
            "Level: ".blue(),
            self.level.to_string().on_blue(),
            "Health: ".green(),
            self.health,
            self.max_health,
            "Mana: ".blue(),
            self.mana,
            self.max_mana,
            "Experience".blue(),
            self.xp,
            XP_TO_LEVEL_UP
        );
    }

    /// Get the name of the Player
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Get the level of the Player
    fn level(&self) -> u32 {
        self.level
    }

    fn magic_strength(&self) -> u32 {
        self.stats.get_magic_strength()
    }

    fn start_defending(&mut self) {
        self.stats.start_defending();
    }

    fn stop_defending(&mut self) {
        self.stats.stop_defending();
    }

    /// Ticks statuses and goes through the list
    fn tick_statuses(&mut self) {
        let mut indicies_to_remove: Vec<usize> = Vec::new();

        for i in 0..self.statuses.len() {
            let amount = self.statuses[i].calculate_amount();

            // mark this status for removal if it has no turns left
            if self.statuses[i].tick() {
                // push this index
                indicies_to_remove.push(i);
            }

            // print what the status effect did and apply effect
            if self.statuses[i].is_healing() {
                println!(
                    "{} {} {} {} {}",
                    self.name.green(),
                    "healed".green(),
                    amount.to_string().as_str().on_green(),
                    "health from".green(),
                    self.statuses[i].name().on_blue().black()
                );
                self.heal(amount);
            } else {
                println!(
                    "{} {} {} {} {}",
                    self.name.green(),
                    "took".red(),
                    amount.to_string().as_str().on_red(),
                    "damage from".red(),
                    self.statuses[i].name().on_blue().black()
                );
                self.take_damage(amount);
            }
        }

        // remove all statuses that were marked for removal
        let mut cur_num_removed = 0;

        for i in 0..indicies_to_remove.len() {
            // since the indices of the elements will change due to the removal
            let index = indicies_to_remove[i] - cur_num_removed;
            self.statuses.remove(index);
            cur_num_removed += 1; // we have removed another status
        }
    }

    fn apply_status(&mut self, status: &Status) {
        println!("{} appled to {}", status.name(), self.name);
        self.statuses.push(status.clone());
    }

    fn attack_move(&mut self, target: &mut dyn Entity, text_vec: &mut Vec<String>) -> bool {
        if self.has_gone {
            return false; // error, did not go
        }
        // attack the enemy with a random amount of damage
        let random_damage = self.get_random_attack_dmg();

        let damage_dealt = self.attack_entity(random_damage, target);
        // display the damage dealt
        self.display_attack_text(target.name(), damage_dealt, text_vec);

        // the player has gone
        self.has_gone = true;

        // no error
        true
    }

    fn has_gone(&self) -> bool {
        self.has_gone
    }

    fn health(&self) -> u32 {
        self.health
    }

    fn max_health(&self) -> u32 {
        self.max_health
    }

    fn allow_move(&mut self) {
        // we can move again
        self.has_gone = false;
    }
}
