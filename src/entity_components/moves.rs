use rand::random;

use super::status::Status;

#[derive(Clone)]
pub enum ElementType {
    Fire,
    Wind,
    Earth,
    Water,
    NumElements, // This should always be the last value
}

pub enum MoveType {
    AttackMove,
    MagicMove,
    DefendMove,
    NumMoveTypes, // This should always be the last value
}

/// Struct for representing a move in the game.
/// This could be an attacking or healing move.
#[derive(Clone)]
pub struct Move<'a> {
    name: &'a str, // specify the lifetime of this variable (still don't know why)
    base_amount: u32,
    mana_cost: u32,
    level_requirement: u32,
    element: ElementType,
    applied_status: Option<Status>,
}

impl<'a> Move<'a> {
    pub const fn new(
        name: &'a str,
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
    /// # Returns
    /// - A `Vec` of all the `Move`s in the game.
    pub fn get_move_list(full_move_list: &Vec<Move<'a>>, entity_level: u32) -> Vec<Move<'a>> {
        let mut move_vector: Vec<Move<'a>> = Vec::new();

        for i in 0..full_move_list.len() {
            if full_move_list[i].is_meeting_requirements(entity_level) {
                // push a clone of the move to the resulting list
                move_vector.push(full_move_list[i].clone());
            } else {
                // break out of the for loop, as moves are ordered by level requirement
                break;
            }
        }

        move_vector
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
    pub fn name(&self) -> &'a str {
        self.name
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
}
