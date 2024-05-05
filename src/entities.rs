// trait for entities
trait Entity {
    // prints the entity's name
    fn print_name(&self);

    //entity takes damage
    fn take_damage(&mut self, amount: u32);

    //entity heals
    fn heal(&mut self, amount: u32);

    // uses mana
    fn use_mana(&mut self, amount: u32);

    // gain xp - not every entity will be able to
    fn gain_xp(&mut self, _amount: u32) {}
}

// struct to represent the player
pub struct Player {
    name: String,
    health: u32,
    mana: u32,
    level: u32,
    xp: u32,
}

// struct to represent enemies
pub struct Enemy {
    name: String,
    health: u32,
    mana: u32,
    level: u32,
}

// entity implementation for player
impl Entity for Player {
    fn print_name(&self) {
        print!("{}", self.name);
    }

    fn take_damage(&mut self, amount: u32) {
        self.health -= amount;
    }

    fn heal(&mut self, amount: u32) {
        self.health += amount;
    }

    fn use_mana(&mut self, amount: u32) {
        self.mana -= amount;
    }

    fn gain_xp(&mut self, amount: u32) {
        self.xp += amount;
    }
}

impl Player {
    pub fn new(name: String, health: u32, mana: u32, level: u32, xp: u32) -> Self {
        Self {
            name,
            health,
            mana,
            level,
            xp,
        }
    }
}

// entity implementation for enemy
impl Entity for Enemy {
    fn print_name(&self) {
        print!("{}", self.name);
    }

    fn take_damage(&mut self, amount: u32) {
        self.health -= amount;
    }

    fn heal(&mut self, amount: u32) {
        self.health += amount;
    }

    fn use_mana(&mut self, amount: u32) {
        self.mana -= amount;
    }
}

impl Enemy {}
