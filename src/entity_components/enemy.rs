use crate::entity_components::{entity::Entity, moves::MoveType, stats::Stats};
use colored::Colorize;

use super::status::Status;

///Struct to represent an enemy.
///Implements the Entity trait.
#[derive(Clone)]
pub struct Enemy {
    name: String,
    health: u32,
    max_health: u32,
    mana: u32,
    max_mana: u32,
    stats: Stats,
    level: u32,
    has_gone: bool,
    statuses: Vec<Status>,
}

const BASE_XP: u32 = 8;

impl Enemy {
    ///create new enemy
    pub fn new(name: String, stats: Stats, level: u32, has_gone: bool) -> Self {
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
            has_gone,
            statuses: Vec::new(), // start with no statuses
        }
    }

    pub fn drop_xp(&self, player_level: u32) -> u32 {
        let mut amount = BASE_XP; // start with a base xp

        let num_levels_above_player = self.level as i64 - player_level as i64;
        for _i in 0..num_levels_above_player {
            amount *= 2; // just crazy xp as enemies get way higher leveled than you
        }

        println!("{} dropped {} xp!", self.name, amount.to_string().blue());

        amount
    }
}

//entity implementation for enemy
impl Entity for Enemy {
    ///Make the Enemy take damage
    fn take_damage(&mut self, amount: u32) -> u32 {
        let damage_taken = self.stats.calc_damage_taken(amount);

        if damage_taken > self.health {
            self.health = 0;
        } else {
            self.health -= damage_taken;
        }

        damage_taken
    }

    ///Heal the Enemy
    fn heal(&mut self, amount: u32) {
        self.health += amount;
    }

    ///Makes the Enemy use mana
    fn use_mana(&mut self, amount: u32) {
        if amount > self.mana {
            self.mana = 0;
        } else {
            self.mana -= amount;
        }
    }

    ///Gets the speed of the Enemy
    fn speed(&self) -> u32 {
        self.stats.get_speed()
    }

    ///Checks to see if the Enemy is dead
    fn is_dead(&self) -> bool {
        self.health == 0
    }

    ///Checks to see if the Enemy has gone yet
    fn gone_this_turn(&self) -> bool {
        self.has_gone
    }

    ///The Enemy makes a choice as to what type of move it wants to do this turn
    /// FUTURE: implement AI for this
    fn get_turn_type(&mut self) -> Option<MoveType> {
        Some(MoveType::AttackMove)
    }

    fn get_random_attack_dmg(&self) -> u32 {
        self.stats.generate_random_attack_dmg()
    }

    ///Print the Enemy info
    fn print_info(&self) {
        println!(
            "{}:\n\t{}{} / {}",
            self.name,
            "Health: ".green(),
            self.health,
            self.max_health
        );
    }

    fn name(&self) -> String {
        self.name.clone()
    }

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
        self.stats.stop_defending()
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
                    self.name.red(),
                    "healed".green(),
                    amount.to_string().as_str().on_green(),
                    "health from".green(),
                    self.statuses[i].name().on_blue().black()
                );
                self.heal(amount);
            } else {
                println!(
                    "{} {} {} {} {}",
                    self.name.red(),
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
