// trait for entities
pub trait Entity {
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

    // get the speed of the entity
    fn get_speed(&self) -> &u32;

    // see if this entity is faster than the other
    fn is_faster<T: Entity>(&self, entity: T) -> bool;

    fn is_dead(&self) -> bool;
}

pub struct Stats {
    health_stat: u32,
    mana_stat: u32,
    speed_stat: u32,
}

impl Stats {
    // creates a new Stats object
    pub fn new(health_stat: u32, mana_stat: u32, speed_stat: u32) -> Self {
        Self {
            health_stat,
            mana_stat,
            speed_stat,
        }
    }
    // generates the health of the entity
    fn generate_health(&self) -> u32 {
        (self.health_stat as f32 * 5.5) as u32
    }

    // generates the mana of the entity
    fn generate_mana(&self) -> u32 {
        (self.mana_stat as f32 * 2.5) as u32
    }
}

// struct to represent the player
pub struct Player {
    name: String,
    health: u32,
    mana: u32,
    stats: Stats,
    level: u32,
    xp: u32,
}

// struct to represent enemies
pub struct Enemy {
    name: String,
    health: u32,
    mana: u32,
    stats: Stats,
    level: u32,
}

// entity implementation for player
impl Entity for Player {
    fn print_name(&self) {
        print!("{}", self.name);
    }

    fn take_damage(&mut self, amount: u32) {
        if amount > self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
    }

    fn heal(&mut self, amount: u32) {
        self.health += amount;
    }

    fn use_mana(&mut self, amount: u32) {
        if amount > self.mana {
            self.mana = 0;
        } else {
            self.mana -= amount;
        }
    }

    fn gain_xp(&mut self, amount: u32) {
        self.xp += amount;
    }

    fn get_speed(&self) -> &u32 {
        &self.stats.speed_stat
    }

    fn is_faster<T: Entity>(&self, entity: T) -> bool {
        self.stats.speed_stat > *entity.get_speed()
    }

    fn is_dead(&self) -> bool {
        self.health == 0
    }
}

impl Player {
    pub fn new(name: String, stats: Stats, level: u32, xp: u32) -> Self {
        Self {
            name,
            health: stats.generate_health(),
            mana: stats.generate_mana(),
            stats,
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
        if amount > self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
    }

    fn heal(&mut self, amount: u32) {
        self.health += amount;
    }

    fn use_mana(&mut self, amount: u32) {
        if amount > self.mana {
            self.mana = 0;
        } else {
            self.mana -= amount;
        }
    }

    fn get_speed(&self) -> &u32 {
        &self.stats.speed_stat
    }

    fn is_faster<T: Entity>(&self, entity: T) -> bool {
        self.stats.speed_stat > *entity.get_speed()
    }

    fn is_dead(&self) -> bool {
        self.health == 0
    }
}

impl Enemy {
    // create new enemy
    pub fn new(name: String, stats: Stats, level: u32) -> Self {
        Self {
            name,
            health: stats.generate_health(),
            mana: stats.generate_mana(),
            stats,
            level,
        }
    }

    pub fn print_info(&self) {
        println!("{}:\n\tHealth:{}", self.name, self.health);
    }
}
