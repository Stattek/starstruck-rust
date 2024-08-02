use rand::random;

///struct for the stats of an entity
pub struct Stats {
    health: u32,
    mana: u32,
    speed: u32,
    strength: u32,
}

impl Stats {
    ///Creates a new Stats object
    pub fn new(health: u32, mana: u32, speed: u32, strength: u32) -> Self {
        Self {
            health,
            mana,
            speed,
            strength,
        }
    }

    ///Generates the health of the entity
    pub fn calculate_max_health(&self) -> u32 {
        //TODO: change this with
        (self.health as f32 * 5.5) as u32
    }

    ///Generates the mana of the entity
    pub fn calculate_max_mana(&self) -> u32 {
        //TODO: change this
        (self.mana as f32 * 2.5) as u32
    }

    ///Generates random attack damage based on stats
    pub fn generate_random_attack_dmg(&self) -> u32 {
        ((random::<u32>() % 10) + 1) * self.strength
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }
}
