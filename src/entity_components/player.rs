//FIXME: fix the private methods and members
use crate::entity_components::entity::{Entity, MoveType};
use crate::entity_components::stats::Stats;
use std::io;

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
}

impl Player {
    pub fn new(name: String, stats: Stats, level: u32, xp: u32, has_gone: bool) -> Self {
        Self {
            name,
            //FIXME: private method, change this to be public maybe
            health: stats.calculate_max_health(),
            mana: stats.calculate_max_mana(),
            stats,
            level,
            xp,
            xp_to_next_level: level * 10,
            has_gone,
        }
    }

    ///Makes the Player gain xp
    fn gain_xp(&mut self, amount: u32) {
        self.xp += amount;
    }
}

//entity implementation for player
impl Entity for Player {
    ///prints the name of the Player
    fn print_name(&self) {
        print!("{}", self.name);
    }

    ///Makes the Player take damage
    fn take_damage(&mut self, amount: u32) {
        if amount > self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
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

    ///Player chooses attack type, and it is returned
    fn get_turn_type(&mut self) -> Option<MoveType> {
        self.has_gone = true;

        let mut choice = -1;
        while choice < 0 || choice > (MoveType::NumMoveTypes as i32) {
            println!("Choose an attack type:\n\t1. Attack\n\t2. Magic\n\t3. Defend");

            //take user input
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();
            user_input = String::from(user_input.trim());

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
        println!("{}:\n\tHealth:{}", self.name, self.health);
    }
}
