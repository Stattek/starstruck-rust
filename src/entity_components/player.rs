use crate::entity_components::entity::Entity;
use crate::entity_components::moves::MoveType;
use crate::entity_components::stats::Stats;
use colored::Colorize;
use std::io;

use super::status::Status;

///Struct to represent the Player.
///Implements the Entity trait
pub struct Player {
    name: String,
    health: u32,
    mana: u32,
    stats: Stats,
    level: u32,
    xp: u32,
    xp_to_next_level: u32,
    has_gone: bool,
    statuses: Vec<Status>,
}

impl Player {
    pub fn new(name: String, stats: Stats, level: u32, xp: u32, has_gone: bool) -> Self {
        Self {
            name,
            health: stats.calculate_max_health(),
            mana: stats.calculate_max_mana(),
            stats,
            level,
            xp,
            xp_to_next_level: level * 10,
            has_gone,
            statuses: Vec::new(), // start with no statuses
        }
    }

    ///Makes the Player gain xp and level up if they reach 100 xp
    pub fn gain_xp(&mut self, amount: u32) {
        self.xp += amount;

        // for level-up chains
        while self.xp >= 100 {
            self.xp -= 100;
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
        /*health: u32,
        mana: u32,
        speed: u32,
        strength: u32,
        magic_strength: u32,
        defense: u32 */
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
        self.health = self.stats.calculate_max_health();
        self.mana = self.stats.calculate_max_mana();
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
            println!(
                "What do you want to do?\n\t1. {}\n\t2. {}\n\t3. {}",
                "Attack".red(),
                "Magic".blue(),
                "Defend".white()
            );

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
        println!("{}:\n\t{}{}", self.name, "Health:".green(), self.health);
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

    fn tick_statuses(&mut self) {
        let mut indicies_to_remove: Vec<usize> = Vec::new();

        for i in 0..self.statuses.len() {
            let amount = self.statuses[i].calculate_amount();

            // mark this status for removal if it has no turns left
            if self.statuses[i].tick() {
                // push this index
                indicies_to_remove.push(i);
            }

            if self.statuses[i].is_healing() {
                // cursed println for text coloring
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

            let mut cur_num_removed = 0;
            // remove all statuses that were marked for removal
            for i in 0..indicies_to_remove.len() {
                // since the indices of the elements will change due to the removal
                let index = indicies_to_remove[i] - cur_num_removed;
                self.statuses.remove(index);
                cur_num_removed += 1; // we have removed another status
            }
        }
    }

    fn apply_status(&mut self, status: &Status) {
        self.statuses.push(status.clone());
    }
}
