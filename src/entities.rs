use rand::random;
use std::io;

/// Represents the type of move that an entity is making
pub enum Move {
    AttackMove,
    MagicMove,
    DefendMove,
}

const MAX_NUM_MOVES: u32 = 3;

///trait for entities
pub trait Entity {
    // prints the entity's name
    fn print_name(&self);

    ///entity takes damage
    fn take_damage(&mut self, amount: u32);

    ///entity heals
    fn heal(&mut self, amount: u32);

    ///uses mana
    fn use_mana(&mut self, amount: u32);

    /// get the speed of the entity
    fn speed(&self) -> u32;

    /// checks to see if this entity is dead
    fn is_dead(&self) -> bool;

    /// checks to see if the entity has gone this turn
    fn gone_this_turn(&self) -> bool;

    ///Makes this Entity do its turn and make a choice
    fn get_turn_type(&mut self) -> Option<Move>;

    fn get_random_attack_dmg(&self) -> u32;
}

pub struct Stats {
    health: u32,
    mana: u32,
    speed: u32,
    strength: u32,
}

impl Stats {
    ///creates a new Stats object
    pub fn new(health: u32, mana: u32, speed: u32, strength: u32) -> Self {
        Self {
            health,
            mana,
            speed,
            strength,
        }
    }

    ///generates the health of the entity
    fn generate_health(&self) -> u32 {
        // TODO: change this
        (self.health as f32 * 5.5) as u32
    }

    ///generates the mana of the entity
    fn generate_mana(&self) -> u32 {
        // TODO: change this
        (self.mana as f32 * 2.5) as u32
    }

    fn generate_random_attack_dmg(&self) -> u32 {
        ((random::<u32>() % 10) + 1) * self.strength
    }
}

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
        self.stats.speed
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
    fn get_turn_type(&mut self) -> Option<Move> {
        self.has_gone = true;

        let mut choice = -1;
        while choice < 0 || choice > (MAX_NUM_MOVES as i32) {
            println!("Choose an attack type:\n\t1. Attack\n\t2. Magic\n\t3. Defend");

            //take user input
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();
            user_input = String::from(user_input.trim());

            //gives back -1 if the input is incorrect
            choice = user_input.parse::<i32>().unwrap_or(-1);
        }

        // return the correct type
        match choice {
            1 => Some(Move::AttackMove),
            2 => Some(Move::MagicMove),
            3 => Some(Move::DefendMove),
            _ => None,
        }
    }

    fn get_random_attack_dmg(&self) -> u32 {
        self.stats.generate_random_attack_dmg()
    }
}

impl Player {
    pub fn new(name: String, stats: Stats, level: u32, xp: u32, has_gone: bool) -> Self {
        Self {
            name,
            health: stats.generate_health(),
            mana: stats.generate_mana(),
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

// entity implementation for enemy
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
        self.stats.speed
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
    fn get_turn_type(&mut self) -> Option<Move> {
        Some(Move::AttackMove)
    }

    fn get_random_attack_dmg(&self) -> u32 {
        self.stats.generate_random_attack_dmg()
    }
}

impl Enemy {
    // create new enemy
    pub fn new(name: String, stats: Stats, level: u32, has_gone: bool) -> Self {
        Self {
            name,
            health: stats.generate_health(),
            mana: stats.generate_mana(),
            stats,
            level,
            has_gone,
        }
    }

    ///Print the Enemy info
    pub fn print_info(&self) {
        println!("{}:\n\tHealth:{}", self.name, self.health);
    }
}
