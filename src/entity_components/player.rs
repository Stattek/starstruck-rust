use crate::entity_components::entity::Entity;
use crate::entity_components::moves::Move;
use crate::entity_components::moves::MoveType;
use crate::entity_components::stats::Stats;
use std::collections::VecDeque;

use super::status::Status;

const XP_TO_LEVEL_UP: u32 = 100;
pub const DEFAULT_PLAYER_LEVEL: u32 = 1;
pub const DEFAULT_PLAYER_XP: u32 = 0;
pub const DEFAULT_HAS_GONE_STATE: bool = false;
pub const DEFAULT_NAME: &str = "Louie";

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

pub enum LevelUpType {
    Strength,
    Magic,
    Health,
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

    pub fn default() -> Self {
        // start with this mana and hp
        let stats = Stats::default();
        let starting_health = stats.calculate_max_health();
        let starting_mana = stats.calculate_max_mana();
        Self {
            name: String::from(DEFAULT_NAME),
            health: starting_health,
            max_health: starting_health,
            mana: starting_mana,
            max_mana: starting_mana,
            stats,
            level: DEFAULT_PLAYER_LEVEL,
            xp: DEFAULT_PLAYER_XP,
            has_gone: DEFAULT_HAS_GONE_STATE,
            statuses: Vec::new(), // start with no statuses
        }
    }

    /// Makes the Player gain xp and level up if they reach the xp to level up.
    ///
    /// # Returns
    /// - `true` if the `Player` is ready to level up, `false` otherwise
    pub fn gain_xp(&mut self, amount: u32) -> bool {
        let mut is_leveling_up = false;

        self.xp += amount;

        // for level-up chains
        while self.xp >= XP_TO_LEVEL_UP {
            self.xp -= XP_TO_LEVEL_UP;
            is_leveling_up = true;
        }

        is_leveling_up
    }

    /// The player levels up and gets to choose stats to increase
    pub fn level_up(&mut self, level_type: LevelUpType) {
        // increment level
        self.level += 1;

        match level_type {
            LevelUpType::Strength => self.stats.increase_physical(),
            LevelUpType::Magic => self.stats.increase_magic(),
            LevelUpType::Health => self.stats.increase_health(),
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
        text_vec: &mut VecDeque<String>,
    ) {
        text_vec.push_back(format!(
            "You did {} damage to {}",
            damage_dealt.to_string(),
            victim_entity_name
        ));
    }

    /// The `Player` performs a magic move against another `Entity`.
    ///
    /// # Params
    /// - `target` - The target of the attack.
    /// - `the_move` - The `Move` for the `Player` object to perform.
    pub fn magic_move(
        &mut self,
        target: &mut dyn Entity,
        the_move: &Move,
        text_vec: &mut VecDeque<String>,
    ) -> bool {
        // check whether the user has already gone or if the move costs too much
        if self.has_gone || self.mana < the_move.cost() {
            return false; // error
        }

        let random_damage = the_move.generate_random_amount(self.magic_strength());

        let damage_dealt = self.attack_entity(random_damage, target);
        // use the mana from this move
        self.use_mana(the_move.cost());
        // display the damage that was dealt
        self.display_attack_text(target.name(), damage_dealt, text_vec);

        // roll for random chance to apply status if it exists
        if the_move.roll_status_chance() {
            target.apply_status(&the_move.get_status().unwrap(), text_vec);
        }

        // the player has gone
        self.has_gone = true;

        // no error
        true
    }

    pub fn defend_move(&mut self, text_vec: &mut VecDeque<String>) -> bool {
        if self.has_gone {
            return false;
        }
        self.start_defending();

        // tell player that they started defending
        let mut output_str = String::new();
        output_str.push_str(self.name.as_str());
        output_str.push_str(" began defending for 1 turn.");

        text_vec.push_back(format!("{} began defending for 1 turn.", self.name));

        // no error
        true
    }

    pub fn experience(&self) -> u32 {
        self.xp
    }

    pub fn max_experience(&self) -> u32 {
        XP_TO_LEVEL_UP
    }

    pub fn mana(&self) -> u32 {
        self.mana
    }

    pub fn max_mana(&self) -> u32 {
        self.max_mana
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

    /// Makes the Player use mana
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
    /// # NOTE: This method is now defunct
    fn get_turn_type(&mut self) -> Option<MoveType> {
        panic!()
    }

    fn get_random_attack_dmg(&self) -> u32 {
        self.stats.generate_random_attack_dmg()
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
    fn tick_statuses(&mut self, text_vec: &mut VecDeque<String>) {
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
                text_vec.push_back(format!(
                    "{} healed {} health from {}",
                    self.name,
                    amount,
                    self.statuses[i].name()
                ));
                self.heal(amount);
            } else {
                text_vec.push_back(format!(
                    "{} took {} damage from {}",
                    self.name,
                    amount,
                    self.statuses[i].name()
                ));
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

    fn apply_status(&mut self, status: &Status, text_vec: &mut VecDeque<String>) {
        text_vec.push_back(format!("{} applied to {}", status.name(), self.name));
        self.statuses.push(status.clone());
    }

    fn attack_move(&mut self, target: &mut dyn Entity, text_vec: &mut VecDeque<String>) -> bool {
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
