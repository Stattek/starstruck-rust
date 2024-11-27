use rand::random;

const DEFENSE_AMOUNT: u32 = 50;

///struct for the stats of an entity
#[derive(Clone)]
pub struct Stats {
    health: u32,
    mana: u32,
    speed: u32, // TODO: might remove
    strength: u32,
    magic_strength: u32,
    defense: u32,
    is_defending: bool, // if the entity is defending (this value is here so we can change the other stats)
}

impl Stats {
    ///Creates a new Stats object
    pub fn new(
        health: u32,
        mana: u32,
        speed: u32,
        strength: u32,
        magic_strength: u32,
        defense: u32,
    ) -> Self {
        Self {
            health,
            mana,
            speed,
            strength,
            magic_strength,
            defense,
            is_defending: false, // always start off not defending
        }
    }

    /// Create a default Stats object
    pub fn default() -> Self {
        Self {
            health: 10,
            mana: 10,
            speed: 10,
            strength: 10,
            magic_strength: 10,
            defense: 0,
            is_defending: false,
        }
    }

    ///Generates the health of the entity
    pub fn calculate_max_health(&self) -> u32 {
        //TODO: change this with
        (self.health as f64 * 5.5) as u32
    }

    ///Generates the mana of the entity
    pub fn calculate_max_mana(&self) -> u32 {
        //TODO: change this
        (self.mana as f64 * 2.5) as u32
    }

    ///Generates random attack damage based on stats
    pub fn generate_random_attack_dmg(&self) -> u32 {
        // TODO: random crits?

        // strength + (random number between 0 and strength / 2)
        self.strength + (random::<u32>() % (self.strength / 2))
    }

    pub fn calc_damage_taken(&self, damage_amount: u32) -> u32 {
        (damage_amount as f64 - ((self.defense as f64 / 100.0) * damage_amount as f64)) as u32
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }

    pub fn get_magic_strength(&self) -> u32 {
        self.magic_strength
    }

    pub fn start_defending(&mut self) {
        if !self.is_defending {
            self.defense += DEFENSE_AMOUNT;
            self.is_defending = true;
        }
    }

    pub fn stop_defending(&mut self) {
        if self.is_defending {
            self.defense -= DEFENSE_AMOUNT;
            self.is_defending = false;
        }
    }

    pub fn increase_physical(&mut self) {
        self.strength += 1;
    }

    pub fn increase_magic(&mut self) {
        self.mana += 1;
        self.magic_strength += 1;
    }

    pub fn increase_health(&mut self) {
        self.health += 1;
    }
}
