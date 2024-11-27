use rand::random;

use super::status::Status;

#[derive(Clone)]
pub enum ElementType {
    Fire,
    Wind,
    Earth,
    Water,
    None,
}

pub enum MoveType {
    AttackMove,
    MagicMove,
    DefendMove,
}

/// Struct for representing a move in the game.
/// This could be an attacking or healing move.
#[derive(Clone)]
pub struct Move {
    name: String, // specify the lifetime of this variable (still don't know why)
    base_amount: u32,
    mana_cost: u32,
    level_requirement: u32,
    element: ElementType,
    applied_status: Option<Status>,
}

impl Move {
    pub const fn new(
        name: String,
        base_amount: u32,
        mana_cost: u32,
        level_requirement: u32,
        element: ElementType,
        applied_status: Option<Status>,
    ) -> Self {
        Self {
            name,
            base_amount,
            mana_cost,
            level_requirement,
            element,
            applied_status,
        }
    }

    fn default() -> Self {
        Self {
            name: String::from("DEFAULT_MOVE"),
            base_amount: 1,
            mana_cost: 1,
            level_requirement: 1,
            element: ElementType::None,
            applied_status: None,
        }
    }

    /// Builder function for easily builing moves.
    /// Sets the name of the Move object and returns it.
    ///
    /// # Params
    /// - `name` - The name of the move.
    ///
    /// # Returns
    /// - The `Move` object.
    fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// Builder function for easily builing moves.
    /// Sets the base amount of the Move object and returns it.
    ///
    /// # Params
    /// - `base_amount` - The base amount of damage/healing for the move.
    ///
    /// # Returns
    /// - The `Move` object.
    fn with_base_amount(mut self, base_amount: u32) -> Self {
        self.base_amount = base_amount;
        self
    }

    /// Builder function for easily builing moves.
    /// Sets the mana cost of the Move object and returns it.
    ///
    /// # Params
    /// - `mana_cost` - The mana cost of the move.
    ///
    /// # Returns
    /// - The `Move` object.
    fn with_mana_cost(mut self, mana_cost: u32) -> Self {
        self.mana_cost = mana_cost;
        self
    }

    /// Builder function for easily builing moves.
    /// Sets the level requirement of the Move object and returns it.
    ///
    /// # Params
    /// - `level_requirement` - The name of the move.
    ///
    /// # Returns
    /// - The `Move` object.
    fn with_level_requirement(mut self, level_requirement: u32) -> Self {
        self.level_requirement = level_requirement;
        self
    }

    /// Builder function for easily builing moves.
    /// Sets the element of the Move object and returns it.
    ///
    /// # Params
    /// - `element` - The element of the move.
    ///
    /// # Returns
    /// - The `Move` object.
    fn with_element(mut self, element: ElementType) -> Self {
        self.element = element;
        self
    }

    /// Builder function for easily builing moves.
    /// Sets the applied of the Move object and returns it.
    ///
    /// # Params
    /// - `applied_status` - The applied status of the move.
    ///
    /// # Returns
    /// - The `Move` object.
    fn with_applied_status(mut self, applied_status: Option<Status>) -> Self {
        self.applied_status = applied_status;
        self
    }

    /// Generates a random damage/healing value for this `Move`.
    ///
    /// # Params
    /// - `magic_strength` - The magic strength stat of the user to enhance the Move's damage/healing.
    ///
    /// # Returns
    /// - The random value of healing/damage this `Move` will do.
    pub fn generate_random_amount(&self, magic_strength: u32) -> u32 {
        // magic_strength + (random number between 0 and magic_strength/2)
        magic_strength
            + self.base_amount
            + (rand::random::<u32>() % (magic_strength + self.base_amount / 2))
    }

    /// Creates and returns a `Move` list of all the moves that are in the game.
    ///
    /// FUTURE: since this move list is ordered by level requirement, we could do a binary search
    ///
    /// # Returns
    /// - A `Vec` of all the `Move`s in the game.
    pub fn get_move_list(full_move_list: &Vec<Move>, entity_level: u32) -> usize {
        let mut valid_len: usize = 0;

        for i in 0..full_move_list.len() {
            if full_move_list[i].is_meeting_requirements(entity_level) {
                valid_len += 1;
            } else {
                // break out of the for loop, as moves are ordered by level requirement
                break;
            }
        }

        valid_len
    }

    /// Checks that the entity with this level meets the requirements for using
    /// this `Move`.
    ///
    /// # Params
    /// - `entity_level` - The level of the entity using this `Move`.
    ///
    /// # Returns
    /// - Returns `true` if the entity meets the requirements, `false` otherwise
    pub fn is_meeting_requirements(&self, entity_level: u32) -> bool {
        self.level_requirement <= entity_level
    }

    /// Get the mana cost of this move.
    ///
    /// # Returns
    /// - the mana cost of the `Move`.
    pub fn cost(&self) -> u32 {
        self.mana_cost
    }

    /// Get the name of this move.
    ///
    /// # Returns
    /// - the name of the `Move`.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn roll_status_chance(&self) -> bool {
        let mut result = false;
        if self.applied_status.is_some() {
            let rand_num = (random::<u32>() % 100) + 1;
            let chance = (Status::status_chance() * 100 as f64) as u32;

            if rand_num <= chance {
                result = true
            }
        }

        result
    }

    pub fn get_status(&self) -> Option<Status> {
        self.applied_status.clone()
    }

    /// Creates a `Move` list for the game
    ///
    /// # Returns
    /// - The full `Move` list for the game.
    pub fn create_move_list(status_list: &Vec<Status>) -> Vec<Move> {
        vec![
            Move::new(
                String::from("FireOne"),
                12,
                2,
                1,
                ElementType::Fire,
                Status::get_status_from("Burn", status_list),
            ),
            Move::new(String::from("WindOne"), 14, 2, 3, ElementType::Wind, None),
            Move::new(String::from("EarthOne"), 16, 2, 5, ElementType::Earth, None),
            Move::new(String::from("WaterOne"), 20, 2, 6, ElementType::Water, None),
        ]
    }
}
