use crate::entity_components::{entity::Entity, moves::MoveType, stats::Stats};
use colored::Colorize;

///Struct to represent an enemy.
///Implements the Entity trait.
pub struct Enemy {
    name: String,
    health: u32,
    mana: u32,
    stats: Stats,
    level: u32,
    has_gone: bool,
}

impl Enemy {
    ///create new enemy
    pub fn new(name: String, stats: Stats, level: u32, has_gone: bool) -> Self {
        Self {
            name,
            health: stats.calculate_max_health(),
            mana: stats.calculate_max_mana(),
            stats,
            level,
            has_gone,
        }
    }
}

//entity implementation for enemy
impl Entity for Enemy {
    fn print_name(&self) {
        print!("{}", self.name);
    }

    ///Make the Enemy take damage
    fn take_damage(&mut self, amount: u32) {
        if amount > self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
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
        println!("{}:\n\t{}{}", self.name, "Health: ".green(), self.health);
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn level(&self) -> u32 {
        self.level
    }
}
